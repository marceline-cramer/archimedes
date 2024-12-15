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

use std::fmt::Debug;

use indexmap::{IndexMap, IndexSet};

use crate::frontend::{span::Spanned, types::*};

impl<S: Clone, R: Clone, T> Rule<S, R, T> {
    pub fn base_type(self) -> Option<(R, Spanned<S, Type<S>>)> {
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

impl<S: Clone> Rule<S, String, String> {
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
