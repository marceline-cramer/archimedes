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

use std::collections::BTreeMap;

use differential_dataflow::{
    algorithms::identifiers::Identifiers,
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
    dataflow::*,
    span::{MapSpan, Point, Span, Spanned},
    types::*,
};

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
    pub inlay_hints: Box<dyn DynTraceMap<(Url, InlayHint<Point>), ()>>,
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
    InlayHint(InlayHint<Point>),
    Hover((Point, (Point, String))),
}

pub fn frontend<G: Input>(inputs: FrontendInputs<G>) -> FrontendOutputs<G>
where
    G::Timestamp: Lattice,
{
    let mut scope = inputs.items.scope();
    let unspanned = inputs.clone().unspan();
    let hover = scope.new_collection().1;

    let indexed_rules = unspanned
        .items
        .flat_map(|r| ModuleItem::rule(((), r)))
        .map(value)
        .map(Rule::index_variables);

    let diagnostics = indexed_rules.flat_map(value).consolidate();

    let rules = inputs.items.flat_map(ModuleItem::rule);

    let base_types = rules.flat_map(|(url, rule)| {
        rule.clone()
            .base_type()
            .map(|(relation, ty)| ((url, relation), ty))
    });

    let rule_keys = rules.identifiers().map(|(rule, key)| (key, rule));

    let head_types = rule_keys.flat_map(|(key, (url, rule))| {
        // TODO: this actually would replace the work of base_types if unblocked
        if rule.body.is_empty() {
            return None;
        }

        let relation = (url.clone(), rule.head.inner.relation.inner.clone());

        let ty = rule
            .head
            .inner
            .pattern
            .inner
            .map_leaves(&mut |term| match term {
                AnyTerm::Variable(name) => AnyTerm::Variable(name),
                AnyTerm::Value(val) => AnyTerm::Value(val.map(|inner| inner.ty())),
            });

        let ty = Spanned {
            span: rule.head.span,
            inner: ty,
        };

        Some((key, (relation, ty)))
    });

    let body_types = rule_keys.flat_map(|(key, (url, rule))| {
        rule.body.into_iter().map(move |atom| {
            let relation = (url.clone(), atom.relation.inner.clone());

            let ty = atom.inner.pattern.inner.map_leaves(&mut |term| match term {
                AnyTerm::Variable(name) => AnyTerm::Variable(name),
                AnyTerm::Value(val) => AnyTerm::Value(val.map(|inner| inner.ty())),
            });

            (relation, (key, ty))
        })
    });

    let (types, type_diagnostics, var_types) = scope.iterative::<u16, _, _>(|scope| {
        let step = Product::new(Default::default(), 1);
        let proposed_types = Variable::new_from(base_types.enter(scope), step.clone());
        let var_types = Variable::new(scope, step.clone());
        let resolved_types = Variable::new(scope, step.clone());
        let diagnostics = Variable::new(scope, step.clone());

        let mut new_diagnostics = Aggregate::from_collection(diagnostics.clone());

        let new_resolved = proposed_types
            .reduce(|_key, input, output| {
                let mut resolved = None;
                for (ty, _diff) in input.iter().cloned() {
                    match resolved.as_ref() {
                        None => {
                            output.push((Ok(ty.clone()), 1));
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
            })
            .map(|((url, relation), v)| {
                v.map(|ok| ((url.clone(), relation), ok))
                    .map_err(|err| (url, err))
            })
            .consolidate();

        let new_types = new_diagnostics.with_errs(&new_resolved);

        let new_body = body_types.enter(scope).join(&new_types).flat_map(
            |((url, _relation), ((key, src), dst))| {
                let mut diagnostics = Vec::new();
                let mut resolved = Vec::new();

                src.unify(dst.inner, &mut diagnostics, &mut |var, ty| {
                    resolved.push((var, ty));
                });

                resolved
                    .into_iter()
                    .map(|(var, ty)| Ok((key, (url.clone(), var, ty))))
                    .chain(diagnostics.into_iter().map(|d| Err((url.clone(), d))))
                    .collect::<Vec<_>>()
            },
        );

        let new_resolved_vars = new_diagnostics
            .with_errs(&new_body)
            .reduce(|_key, input, output| {
                let mut resolved = BTreeMap::new();
                for ((url, var, ty), _diff) in input.iter().cloned() {
                    use std::collections::btree_map::Entry;
                    match resolved.entry(var.inner.clone()) {
                        Entry::Vacant(entry) => {
                            entry.insert(ty.clone());
                        }
                        Entry::Occupied(entry) => {
                            // TODO: recursive unification
                            let lhs = entry.get().clone().map_span(&mut |_| ());
                            let rhs = ty.clone().map_span(&mut |_| ());
                            if lhs != rhs {
                                let d = Diagnostic {
                                    contents: format!(
                                        "Pattern expects {rhs} but {:?} is {lhs}",
                                        var.inner
                                    ),
                                    span: var.span,
                                    kind: DiagnosticKind::Error,
                                };

                                output.push((Err((url.clone(), d)), 1));
                            }
                        }
                    }
                }

                if !resolved.is_empty() {
                    output.push((Ok(resolved), 1));
                }
            })
            .map(|(k, v)| v.map(|ok| (k, ok)));

        let new_resolved_vars = new_diagnostics.with_errs(&new_resolved_vars);

        let resolved_body = new_resolved_vars
            .join(&head_types.enter(scope))
            .flat_map(|(_key, (vars, (relation, dst)))| {
                dst.map(|dst| dst.flat_quantify(&mut |var| vars.get(&var).cloned()))
                    .flatten()
                    .map(|ty| (relation, ty))
            })
            .distinct();

        proposed_types.set_concat(&resolved_body);
        let resolved = resolved_types.set_concat(&new_types);
        let diagnostics = diagnostics.set_concat(&new_diagnostics.as_ref().distinct());
        let var_types = var_types.set_concat(&new_resolved_vars);

        (resolved.leave(), diagnostics.leave(), var_types.leave())
    });

    let type_hints = var_types
        .join(&rule_keys)
        .flat_map(|(_key, (vars, (url, rule)))| {
            let mut touched = BTreeMap::new();

            rule.head
                .inner
                .pattern
                .inner
                .map_variables(&mut |span, var| {
                    touched.entry(var).or_insert(*span);
                });

            for body in rule.body {
                body.inner.pattern.inner.map_variables(&mut |span, var| {
                    touched.entry(var).or_insert(*span);
                });
            }

            touched.into_iter().map(move |(name, span)| {
                let contents = match vars.get(&name) {
                    Some(ty) => format!(": {ty}"),
                    None => ": {unknown}".to_string(),
                };

                let span = span.end;
                let hint = InlayHint { span, contents };

                (url.clone(), hint)
            })
        });

    let diagnostics = diagnostics
        .map(|d| (d.span, d))
        .join(&unspanned.spans)
        .map(|(_key, (d, (url, span)))| (url, d.with_span(span)))
        .concat(&inputs.items.flat_map(ModuleItem::diagnostic))
        .concat(&type_diagnostics)
        .distinct();

    let inlay_hints = type_hints.distinct();

    FrontendOutputs {
        diagnostics,
        hover,
        inlay_hints,
    }
}

#[derive(Clone)]
pub struct FrontendInputs<G: Scope> {
    pub items: Collection<G, (Url, ModuleItem<Span, String, String>)>,
}

pub type SpanKey = (u64, usize);

pub fn indirect_spans<T>(
    (url, item): (Url, T),
) -> (
    Vec<(SpanKey, (Url, Span))>,
    <T::Target as MapSpan<usize, SpanKey>>::Target,
)
where
    T: MapSpan<Span, usize>,
    T::Target: MapSpan<usize, SpanKey> + Hashable<Output = u64>,
{
    let mut spans = Vec::new();

    let unspanned = item.map_span(&mut |span| {
        let idx = spans.len();
        spans.push(span);
        idx
    });

    let key = unspanned.hashed();
    let respanned = unspanned.map_span(&mut |span| (key, span));

    let span_map = spans
        .into_iter()
        .enumerate()
        .map(|(idx, span)| ((key, idx), (url.clone(), span)))
        .collect();

    (span_map, respanned)
}

impl<G: Scope> FrontendInputs<G>
where
    G::Timestamp: Lattice,
{
    pub fn unspan(self) -> FrontendUnspanned<G> {
        let items = self.items.map(indirect_spans);
        let spans = items.flat_map(key).distinct();

        FrontendUnspanned {
            items: items.map(value),
            spans,
        }
    }
}

pub struct FrontendUnspanned<G: Scope> {
    pub items: Collection<G, ModuleItem<SpanKey, String, String>>,
    pub spans: Collection<G, (SpanKey, (Url, Span))>,
}

pub struct FrontendOutputs<G: Scope> {
    pub diagnostics: Collection<G, (Url, Diagnostic<Span>)>,
    pub hover: Collection<G, (Url, (Point, (Point, String)))>,
    pub inlay_hints: Collection<G, (Url, InlayHint<Point>)>,
}
