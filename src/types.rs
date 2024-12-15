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

use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use differential_dataflow::Hashable;
use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Serialize};

use crate::span::{MapSpan, Point, Span, Spanned};

pub type DiagnosticResult<S, T> = Result<T, Diagnostic<S>>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct InlayHint<S> {
    pub span: S,
    pub contents: String,
}

impl InlayHint<Point> {
    pub fn to_lsp(&self) -> tower_lsp::lsp_types::InlayHint {
        use tower_lsp::lsp_types::*;
        InlayHint {
            position: self.span.into(),
            label: InlayHintLabel::String(self.contents.clone()),
            kind: Some(InlayHintKind::TYPE),
            text_edits: None,
            tooltip: None,
            padding_left: None,
            padding_right: None,
            data: None,
        }
    }
}

impl<S> InlayHint<S> {
    pub fn with_span<O>(self, span: O) -> InlayHint<O> {
        InlayHint {
            span,
            contents: self.contents,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Diagnostic<S> {
    pub span: S,
    pub kind: DiagnosticKind,
    pub contents: String,
}

impl<S> Diagnostic<S> {
    pub fn with_span<O>(self, span: O) -> Diagnostic<O> {
        Diagnostic {
            span,
            kind: self.kind,
            contents: self.contents,
        }
    }
}

impl Diagnostic<Span> {
    pub fn to_lsp(&self) -> tower_lsp::lsp_types::Diagnostic {
        use tower_lsp::lsp_types::DiagnosticSeverity;
        let severity = match self.kind {
            DiagnosticKind::Error => DiagnosticSeverity::ERROR,
            DiagnosticKind::Warning => DiagnosticSeverity::WARNING,
            DiagnosticKind::Info => DiagnosticSeverity::INFORMATION,
            DiagnosticKind::Note => DiagnosticSeverity::HINT,
        };

        tower_lsp::lsp_types::Diagnostic {
            range: self.span.into(),
            severity: Some(severity),
            message: self.contents.clone(),
            ..Default::default()
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum DiagnosticKind {
    Error,
    Warning,
    Info,
    Note,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ModuleItem<S, T> {
    Rule(Rule<S, T>),
    Decision(Decision<S, T>),
    Constraint(Constraint<S, T>),
    Diagnostic(Diagnostic<S>),
}

pub struct ModuleItems<S, T> {
    pub rules: Vec<Rule<S, T>>,
    pub decisions: Vec<Decision<S, T>>,
    pub constraints: Vec<Constraint<S, T>>,
    pub diagnostics: Vec<Diagnostic<S>>,
}

impl<S, O, T> MapSpan<S, O> for ModuleItems<S, T>
where
    T: MapSpan<S, O>,
{
    type Target = ModuleItems<O, T::Target>;

    fn map_span(self, cb: &mut impl FnMut(S) -> O) -> Self::Target {
        ModuleItems {
            rules: self.rules.map_span(cb),
            decisions: self.decisions.map_span(cb),
            constraints: self.constraints.map_span(cb),
            diagnostics: self.diagnostics.map_span(cb),
        }
    }
}

impl<S, T> ModuleItems<S, T> {
    pub fn to_vec(self) -> Vec<ModuleItem<S, T>> {
        let rules = self.rules.into_iter().map(ModuleItem::Rule);
        let decisions = self.decisions.into_iter().map(ModuleItem::Decision);
        let constraints = self.constraints.into_iter().map(ModuleItem::Constraint);
        let diagnostics = self.diagnostics.into_iter().map(ModuleItem::Diagnostic);

        rules
            .chain(decisions)
            .chain(constraints)
            .chain(diagnostics)
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Decision<S, T>(pub Rule<S, T>);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct IndexedRule<S> {
    pub variables: Vec<Spanned<S, String>>,
    pub inner: Rule<S, usize>,
}

impl<S> IndexedRule<S> {
    pub fn infer_head<C: Hash + Hashable<Output = u64>>(
        self,
        ctx: C,
    ) -> (u64, Pattern<S, TypeTerm<S, usize>>) {
        let relation = self.inner.head.relation.inner.clone();
        let key = (ctx, relation).hashed();

        (
            key,
            self.inner
                .head
                .inner
                .pattern
                .inner
                .map_leaves(&mut |term| match term {
                    AnyTerm::Variable(idx) => AnyTerm::Variable(idx),
                    AnyTerm::Value(val) => AnyTerm::Value(val.map(|inner| inner.ty())),
                }),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Rule<S, T> {
    pub head: Spanned<S, Atom<S, Term<S, T>>>,
    pub body: Vec<Spanned<S, Atom<S, Term<S, T>>>>,
}

impl<S: Clone, T> Rule<S, T> {
    pub fn base_type(self) -> Option<(String, Spanned<S, Type<S>>)> {
        if !self.body.is_empty() {
            return None;
        }

        let relation = self.head.relation.inner.clone();

        let ty = self
            .head
            .inner
            .pattern
            .inner
            .flat_quantify(&mut |_| None)?
            .map_leaves(&mut |leaf| leaf.ty());

        let ty = Spanned {
            span: self.head.span.clone(),
            inner: ty,
        };

        Some((relation, ty))
    }
}

impl<S: Clone> Rule<S, String> {
    pub fn index_variables(self) -> (IndexedRule<S>, Vec<Diagnostic<S>>) {
        let mut variables = IndexMap::new();
        let mut diagnostics = Vec::new();
        let mut body = Vec::new();

        for atom in self.body {
            let relation = atom.relation.clone();

            let pattern = atom
                .pattern
                .clone()
                .map(|inner| inner.index_variables(&mut variables, &mut diagnostics, false));

            body.push(atom.map(|_| Atom { relation, pattern }));
        }

        let relation = self.head.relation.clone();
        let pattern = self
            .head
            .pattern
            .clone()
            .map(|inner| inner.index_variables(&mut variables, &mut diagnostics, true));

        let variables = variables
            .into_iter()
            .map(|(inner, span)| Spanned { inner, span })
            .collect();

        let indexed = IndexedRule {
            variables,
            inner: Rule {
                head: self.head.map(|_| Atom { relation, pattern }),
                body,
            },
        };

        (indexed, diagnostics)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Constraint<S, T> {
    pub captures: Vec<Spanned<S, String>>,
    pub kind: Spanned<S, ConstraintKind>,
    pub bound: Spanned<S, i64>,
    pub body: Vec<Spanned<S, Atom<S, T>>>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum ConstraintKind {
    Only,
    AtMost,
    AtLeast,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Atom<S, T> {
    pub relation: Spanned<S, String>,
    pub pattern: Spanned<S, Pattern<S, T>>,
}

impl<S: Clone + Debug + Eq, T> Pattern<S, TypeTerm<S, T>> {
    pub fn unify(
        self,
        target: Type<S>,
        diagnostics: &mut Vec<Diagnostic<S>>,
        cb: &mut impl FnMut(Spanned<S, T>, Type<S>),
    ) -> bool {
        match self {
            Pattern::Tuple(lhs) => match target {
                Pattern::Tuple(rhs) => {
                    if lhs.len() != rhs.len() {
                        diagnostics.push(Diagnostic {
                            kind: DiagnosticKind::Error,
                            contents: format!(
                                "Expected tuple of arity {}, got tuple of arity {}",
                                lhs.len(),
                                rhs.len()
                            ),
                            span: lhs.span,
                        });

                        false
                    } else {
                        lhs.inner
                            .into_iter()
                            .zip(rhs.inner)
                            .all(|(lhs, rhs)| lhs.inner.unify(rhs.inner, diagnostics, cb))
                    }
                }
                Pattern::Leaf(prim) => {
                    diagnostics.push(Diagnostic {
                        kind: DiagnosticKind::Error,
                        contents: format!(
                            "Expected tuple of arity {}, got {:?}",
                            lhs.len(),
                            prim.inner
                        ),
                        span: lhs.span,
                    });

                    false
                }
            },
            Pattern::Leaf(leaf) => match leaf.inner {
                AnyTerm::Variable(var) => {
                    cb(var, target);
                    true
                }
                AnyTerm::Value(prim) => {
                    if Pattern::Leaf(prim.clone()) != target {
                        diagnostics.push(Diagnostic {
                            span: prim.span,
                            kind: DiagnosticKind::Error,
                            contents: format!("Expected {:?}, got {target:?}", prim.inner),
                        });

                        false
                    } else {
                        true
                    }
                }
            },
        }
    }
}

pub type Type<S> = Pattern<S, PrimitiveType>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum Pattern<S, T> {
    Leaf(Spanned<S, T>),
    Tuple(Spanned<S, Vec<Spanned<S, Self>>>),
}

impl<S, T: Display> Display for Pattern<S, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pattern::Leaf(leaf) => write!(f, "{leaf}"),
            Pattern::Tuple(tuple) => write!(
                f,
                "({})",
                tuple
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

impl<S: Clone> Pattern<S, Term<S, String>> {
    pub fn index_variables(
        self,
        scope: &mut IndexMap<String, S>,
        diagnostics: &mut Vec<Diagnostic<S>>,
        disallow_unbound: bool,
    ) -> Pattern<S, Term<S, usize>> {
        let mut vars = IndexSet::new();
        self.map_variables(&mut |span, var| {
            if !vars.insert(var.clone()) {
                diagnostics.push(Diagnostic {
                    span: span.clone(),
                    kind: DiagnosticKind::Error,
                    contents: format!("Cannot rebind {var:?} within same pattern"),
                });
            }

            let entry = scope.entry(var.clone());
            let idx = entry.index();

            use indexmap::map::Entry;
            let new = match entry {
                Entry::Occupied(_) => false,
                Entry::Vacant(entry) => {
                    entry.insert(span.clone());
                    true
                }
            };

            if new && disallow_unbound {
                diagnostics.push(Diagnostic {
                    span: span.clone(),
                    kind: DiagnosticKind::Error,
                    contents: format!("{var:?} does not appear within body"),
                });
            }

            idx
        })
    }
}

impl<S, T, V> Pattern<S, AnyTerm<S, T, V>> {
    pub fn map_variables<O>(self, cb: &mut impl FnMut(&S, T) -> O) -> Pattern<S, AnyTerm<S, O, V>> {
        self.map_leaves(&mut |term| term.map_variable(cb))
    }

    pub fn flat_quantify(
        self,
        cb: &mut impl FnMut(T) -> Option<Pattern<S, V>>,
    ) -> Option<Pattern<S, V>> {
        use Pattern::*;
        Some(match self {
            Leaf(leaf) => match leaf.inner {
                AnyTerm::Variable(var) => cb(var.inner)?,
                AnyTerm::Value(val) => Leaf(val),
            },
            Tuple(tuple) => {
                let mut els = Vec::with_capacity(tuple.len());
                for el in tuple.inner {
                    let el = el.map(|inner| inner.flat_quantify(cb)).flatten()?;
                    els.push(el);
                }

                Tuple(Spanned {
                    span: tuple.span,
                    inner: els,
                })
            }
        })
    }

    pub fn quantify(self, cb: &mut impl FnMut(T) -> Pattern<S, V>) -> Pattern<S, V> {
        self.flat_quantify(&mut |term| Some(cb(term))).unwrap()
    }
}

impl<S, T> Pattern<S, T> {
    pub fn flat_map_leaves<O>(self, cb: &mut impl FnMut(T) -> Option<O>) -> Option<Pattern<S, O>> {
        use Pattern::*;
        Some(match self {
            Leaf(leaf) => Leaf(leaf.map(cb).flatten()?),
            Tuple(tuple) => {
                let mut els = Vec::with_capacity(tuple.len());
                for el in tuple.inner {
                    let el = el.map(|inner| inner.flat_map_leaves(cb)).flatten()?;
                    els.push(el);
                }

                Tuple(Spanned {
                    span: tuple.span,
                    inner: els,
                })
            }
        })
    }

    pub fn map_leaves<O>(self, cb: &mut impl FnMut(T) -> O) -> Pattern<S, O> {
        self.flat_map_leaves(&mut |var| Some(cb(var))).unwrap()
    }
}

pub type Term<S, T> = AnyTerm<S, T, Value>;
pub type TypeTerm<S, T> = AnyTerm<S, T, PrimitiveType>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum AnyTerm<S, T, V> {
    Variable(Spanned<S, T>),
    Value(Spanned<S, V>),
}

impl<S, T, V> AnyTerm<S, T, V> {
    pub fn flat_map_variable<O>(
        self,
        cb: &mut impl FnMut(&S, T) -> Option<O>,
    ) -> Option<AnyTerm<S, O, V>> {
        use AnyTerm::*;
        Some(match self {
            Variable(var) => Variable(Spanned {
                inner: cb(&var.span, var.inner)?,
                span: var.span,
            }),
            Value(val) => Value(val),
        })
    }

    pub fn map_variable<O>(self, cb: &mut impl FnMut(&S, T) -> O) -> AnyTerm<S, O, V> {
        self.flat_map_variable(&mut |span, var| Some(cb(span, var)))
            .unwrap()
    }

    pub fn flat_quantify(self, cb: &mut impl FnMut(&S, T) -> Option<V>) -> Option<Spanned<S, V>> {
        use AnyTerm::*;
        Some(match self {
            Variable(var) => Spanned {
                inner: cb(&var.span, var.inner)?,
                span: var.span,
            },
            Value(val) => val,
        })
    }

    pub fn quantify(self, cb: &mut impl FnMut(&S, T) -> V) -> Spanned<S, V> {
        self.flat_quantify(&mut |span, var| Some(cb(span, var)))
            .unwrap()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum Value {
    Symbol(String),
    Integer(i64),
}

impl Value {
    pub fn ty(&self) -> PrimitiveType {
        match self {
            Value::Symbol(_) => PrimitiveType::Symbol,
            Value::Integer(_) => PrimitiveType::Integer,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum PrimitiveType {
    Symbol,
    Integer,
}

impl Display for PrimitiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveType::Symbol => write!(f, "Symbol"),
            PrimitiveType::Integer => write!(f, "Integer"),
        }
    }
}
