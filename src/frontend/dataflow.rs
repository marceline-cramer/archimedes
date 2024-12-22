// Copyright (c) 2024 Marceline Cramer.
// SPDX-License-Identifier: AGPL-3.0-or-later
//
// This file is part of Archimedes.
//
// Archimedes is free software: you can redistribute it and/or modify it under
// the terms of the GNU Affero General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version.
//
// Archimedes is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
// details.
//
// You should have received a copy of the GNU Affero General Public License
// along with Archimedes. If not, see <https://www.gnu.org/licenses/>.

use std::{collections::BTreeMap, sync::Arc};

use differential_dataflow::{
    input::Input,
    lattice::Lattice,
    operators::{arrange::ArrangeBySelf, iterate::Variable, Join, Reduce, Threshold},
    Collection, Hashable,
};
use timely::{
    communication::Allocate,
    dataflow::{operators::Probe, ProbeHandle, Scope},
    order::Product,
    worker::Worker,
};
use url::Url;

use crate::{
    frontend::{
        span::{MapSpan, Point, Span, Spanned},
        types::*,
    },
    utils::*,
};

pub fn frontend<G: Input>(inputs: FrontendInputs<G>) -> FrontendOutputs<G>
where
    G::Timestamp: Lattice,
{
    let mut scope = inputs.items.scope();
    let hover = scope.new_collection().1;

    // index all items: convert string variable names to indices
    let indexed_items = inputs.items.map(ModuleItem::index_variables);
    let diagnostics = indexed_items.flat_map(value);
    let indexed_items = indexed_items.map(key);

    // unspan items: decouple the contents of each (sub-)item from its physical span
    let unspanned = indexed_items.map(unspan);
    let span_keys = unspanned.flat_map(value);
    let items = unspanned.map(key);

    // create unique identifiers for each item
    let item_keys = items.map(Key::pair);

    // find all of the typing combinators from the given items
    let base_types = items.flat_map(IndexedItem::base_type);
    let head_types = item_keys.flat_map(map_value(IndexedItem::head_type));
    let body_types = item_keys.flat_map(IndexedItem::body_types);

    // iteratively derive the types of all relations using bottom-up fixed-point evaluation
    let (relation_types, type_diagnostics, item_types) = scope.iterative::<u16, _, _>(|scope| {
        // init loop variables
        let step = Product::new(Default::default(), 1);
        let proposed_types = Variable::new_from(base_types.enter(scope), step.clone());
        let item_types = Variable::new(scope, step.clone());

        let resolved_types: Variable<_, (ResourceId, Spanned<SpanKey, Type<SpanKey>>), isize> =
            Variable::new(scope, step.clone());

        // create a bag of diagnostics to aggregate from each operation
        let diagnostics = Variable::new(scope, step.clone());
        let mut new_diagnostics = Aggregate::from_collection(diagnostics.clone());

        // unify the patterns of each body type with its relation's type
        let new_body = new_diagnostics.with_errs(
            &body_types
                .enter(scope)
                .join(&resolved_types)
                .map(value)
                .flat_map(|((key, src), dst)| src.unify(key, dst.inner)),
        );

        // reduce the variable types of each item into a single map
        let var_maps = new_diagnostics.with_errs(&new_body.reduce(merge_var_types).map(value));

        // infer types of items from complete variable maps
        let resolved_body = var_maps
            .join(&head_types.enter(scope))
            .map(value)
            .flat_map(|(vars, (relation, dst))| {
                dst.map(|dst| dst.flat_quantify(&mut |var| vars.get(&var).cloned()))
                    .flatten()
                    .map(|ty| (relation, ty))
            })
            .distinct();

        // resolve proposed types for each relation
        let new_resolved = new_diagnostics.with_errs(
            &proposed_types
                .reduce(resolve_proposed_types)
                .map(value)
                .consolidate(),
        );

        // feed forward all new results to next iteration
        let resolved = resolved_types.set_concat(&new_resolved);
        let diagnostics = diagnostics.set_concat(&new_diagnostics.as_ref().distinct());
        let item_types = item_types.set_concat(&var_maps);
        proposed_types.set_concat(&resolved_body);

        // pass completed results to caller
        (resolved.leave(), diagnostics.leave(), item_types.leave())
    });

    // fill missing item typings with blank variable maps
    let var_types = item_keys
        .map(|(key, _item)| (key, BTreeMap::default()))
        .antijoin(&item_types.map(key))
        .concat(&item_types)
        .distinct();

    // combine all inlay hints
    let inlay_hints = var_types
        .join(&item_keys)
        .map(value)
        .flat_map(|(vars, item)| item.type_hints(&vars))
        .map(|h| (h.span, h))
        .join(&span_keys)
        .map(|(_key, (h, (url, span)))| (url, h.with_span(span)))
        .distinct();

    // combine all diagnostics
    let diagnostics = type_diagnostics
        .map(|d| (d.span, d))
        .join(&span_keys)
        .map(|(_key, (d, (url, span)))| (url, d.with_span(span)))
        .concat(&inputs.items.flat_map(ModuleItem::diagnostic))
        .concat(&diagnostics)
        .distinct();

    FrontendOutputs {
        diagnostics,
        hover,
        inlay_hints,
    }
}

pub fn frontend_worker<A: Allocate>(
    worker: &mut Worker<A>,
) -> (FrontendWorkerInput, FrontendWorkerOutput) {
    let mut input = FrontendWorkerInput {
        items: InputSession::new(),
    };

    let output = worker.dataflow(|scope| {
        let inputs = FrontendInputs {
            items: input.items.to_collection(scope),
        };

        let outputs = frontend(inputs);

        let diagnostics = outputs.diagnostics.arrange_by_self();
        let hover = outputs.hover.arrange_by_self();
        let inlay_hints = outputs.inlay_hints.arrange_by_self();

        FrontendWorkerOutput {
            probes: vec![
                diagnostics.stream.probe(),
                hover.stream.probe(),
                inlay_hints.stream.probe(),
            ],
            diagnostics: Box::new(TraceMap::new(diagnostics.trace)),
            hover: Box::new(TraceMap::new(hover.trace)),
            inlay_hints: Box::new(TraceMap::new(inlay_hints.trace)),
        }
    });

    (input, output)
}

pub struct FrontendWorkerInput {
    pub items: InputSession<(Url, ModuleItem<Span, String, String>)>,
}

impl WorkerInput for FrontendWorkerInput {
    type Update = FrontendUpdate;

    fn advance_to(&mut self, time: Time) {
        self.items.advance_to(time);
    }

    fn on_update(&mut self, update: Self::Update) {
        let diff = |add| if add { 1 } else { -1 };

        use FrontendUpdate::*;
        match update {
            Item(url, el, add) => self.items.update((url, el), diff(add)),
        }
    }

    fn flush(&mut self) {
        self.items.flush();
    }
}

#[derive(Clone, Debug)]
pub enum FrontendUpdate {
    Item(Url, ModuleItem<Span, String, String>, bool),
}

pub struct FrontendWorkerOutput {
    pub probes: Vec<ProbeHandle<Time>>,
    pub diagnostics: Box<dyn DynTraceMap<(Url, Diagnostic<Span>), ()>>,
    pub inlay_hints: Box<dyn DynTraceMap<(Url, InlayHint<Span>), ()>>,
    pub hover: Box<dyn DynTraceMap<(Url, (Point, (Point, String))), ()>>,
}

impl WorkerOutput for FrontendWorkerOutput {
    type Result = FrontendResult;

    fn advance_to(&mut self, time: Time) {
        self.diagnostics.advance_to(time);
        self.hover.advance_to(time);
        self.inlay_hints.advance_to(time);
    }

    fn pending(&self, time: &Time) -> bool {
        self.probes.iter().any(|probe| probe.less_than(time))
    }

    fn results(&mut self) -> Vec<Self::Result> {
        self.diagnostics.update();
        self.hover.update();
        self.inlay_hints.update();

        let diagnostics = self
            .diagnostics
            .distinct_keys()
            .into_iter()
            .map(|(url, diag)| (url, FrontendResultKind::Diagnostic(diag)));

        let hover = self
            .hover
            .distinct_keys()
            .into_iter()
            .map(|(url, hover)| (url, FrontendResultKind::Hover(hover)));

        let inlay_hints = self
            .inlay_hints
            .distinct_keys()
            .into_iter()
            .map(|(url, hint)| (url, FrontendResultKind::InlayHint(hint)));

        diagnostics.chain(hover).chain(inlay_hints).collect()
    }
}

pub type FrontendResult = (Url, FrontendResultKind);

#[derive(Clone, Debug)]
pub enum FrontendResultKind {
    Diagnostic(Diagnostic<Span>),
    InlayHint(InlayHint<Span>),
    Hover((Point, (Point, String))),
}

#[derive(Clone)]
pub struct FrontendInputs<G: Scope> {
    pub items: Collection<G, (Url, ModuleItem<Span, String, String>)>,
}

pub type SpanKey = (u64, usize);

pub fn unspan(
    item: IndexedItem<Span, String>,
) -> (
    IndexedItem<SpanKey, ResourceId>,
    Vec<(SpanKey, (Url, Span))>,
) {
    let url = item.url.clone();
    let mut spans = Vec::new();

    let unspanned = item.map_span(&mut |span| {
        let idx = spans.len();
        spans.push(span);
        idx
    });

    let key = unspanned.hashed();

    let span_map = spans
        .into_iter()
        .enumerate()
        .map(|(idx, span)| ((key, idx), (url.clone(), span)))
        .collect();

    let url = Arc::new(url);
    let respanned = unspanned
        .map_span(&mut |span| (key, span))
        .map_relations(&mut |_span, sym| ResourceId::SourceSymbol(url.clone(), sym));

    (respanned, span_map)
}

pub fn resolve_proposed_types(
    key: &ResourceId,
    input: &[(&Spanned<SpanKey, Type<SpanKey>>, isize)],
    output: &mut Vec<(
        Result<(ResourceId, Spanned<SpanKey, Type<SpanKey>>), Diagnostic<SpanKey>>,
        isize,
    )>,
) {
    let mut resolved = None;
    for (ty, _diff) in input.iter().cloned() {
        match resolved.as_ref() {
            None => {
                output.push((Ok((key.clone(), ty.clone())), 1));
                resolved = Some(ty.to_owned());
            }
            Some(target) => {
                // TODO: recursive unification
                let lhs = target.clone().map_span(&mut |_| ());
                let rhs = ty.clone().map_span(&mut |_| ());
                if lhs != rhs {
                    let d = Diagnostic {
                        span: ty.span,
                        kind: DiagnosticKind::Error,
                        contents: format!("Expected {lhs}, got {rhs}"),
                    };

                    output.push((Err(d), 1));
                }
            }
        }
    }
}

pub fn merge_var_types(
    key: &Key<IndexedItem<(u64, usize), ResourceId>>,
    input: &[(&(Spanned<SpanKey, usize>, Type<SpanKey>), isize)],
    output: &mut Vec<(
        Result<
            (
                Key<IndexedItem<SpanKey, ResourceId>>,
                BTreeMap<usize, Type<SpanKey>>,
            ),
            Diagnostic<SpanKey>,
        >,
        isize,
    )>,
) {
    let mut resolved = BTreeMap::new();
    for ((var, ty), _diff) in input.iter().cloned() {
        use std::collections::btree_map::Entry;
        match resolved.entry(var.inner) {
            Entry::Vacant(entry) => {
                entry.insert(ty.clone());
            }
            Entry::Occupied(entry) => {
                // TODO: recursive unification
                let lhs = entry.get().clone().map_span(&mut |_| ());
                let rhs = ty.clone().map_span(&mut |_| ());
                if lhs != rhs {
                    let d = Diagnostic {
                        contents: format!("Pattern expects {rhs} but {:?} is {lhs}", var.inner),
                        span: var.span,
                        kind: DiagnosticKind::Error,
                    };

                    output.push((Err(d), 1));
                }
            }
        }
    }

    if !resolved.is_empty() {
        output.push((Ok((*key, resolved)), 1));
    }
}

pub struct FrontendOutputs<G: Scope> {
    pub diagnostics: Collection<G, (Url, Diagnostic<Span>)>,
    pub hover: Collection<G, (Url, (Point, (Point, String)))>,
    pub inlay_hints: Collection<G, (Url, InlayHint<Span>)>,
}
