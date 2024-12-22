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

use std::{collections::BTreeMap, fmt::Debug};

use indexmap::IndexMap;
use url::Url;

use crate::{
    frontend::{span::Spanned, types::*},
    utils::Key,
};

impl<S: Clone> IndexedItem<S, ResourceId> {
    pub fn type_hints(self, vars: &BTreeMap<usize, Type<S>>) -> Vec<InlayHint<S>> {
        self.variables
            .iter()
            .enumerate()
            .map(|(idx, name)| InlayHint {
                span: name.span.clone(),
                contents: match vars.get(&idx) {
                    Some(ty) => format!(": {ty}"),
                    None => ": {unknown}".to_string(),
                },
            })
            .collect()
    }
}

impl<S: Clone, R: Clone> IndexedItem<S, R> {
    pub fn base_type(self) -> Option<(R, Spanned<S, Type<S>>)> {
        let rule = self.rule_or_decision()?;

        if !rule.body.is_empty() {
            return None;
        }

        let relation = rule.head.relation.inner.clone();

        let ty = rule
            .head
            .inner
            .pattern
            .inner
            .flat_quantify(&mut |_| None)?
            .map_leaves(&mut |leaf| leaf.ty());

        let ty = Spanned {
            span: rule.head.span.clone(),
            inner: ty,
        };

        Some((relation, ty))
    }

    pub fn head_type(
        self,
    ) -> Option<(R, Spanned<S, Pattern<S, AnyTerm<S, usize, PrimitiveType>>>)> {
        let rule = self.rule_or_decision()?;

        // TODO: this actually would replace the work of base_types if unblocked
        if rule.body.is_empty() {
            return None;
        }

        let relation = rule.head.inner.relation.inner.clone();

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

        Some((relation, ty))
    }

    pub fn body_types(
        (key, item): (Key<Self>, Self),
    ) -> Vec<(R, (Key<Self>, Pattern<S, TypeTerm<S, usize>>))> {
        let body = match item.inner {
            ModuleItem::Decision(Decision(rule)) => rule.body,
            ModuleItem::Rule(rule) => rule.body,
            ModuleItem::Constraint(constraint) => constraint.body,
            _ => return vec![],
        };

        body.into_iter()
            .map(move |atom| {
                let relation = atom.relation.inner.clone();

                let ty = atom.inner.pattern.inner.map_leaves(&mut |term| match term {
                    AnyTerm::Variable(name) => AnyTerm::Variable(name),
                    AnyTerm::Value(val) => AnyTerm::Value(val.map(|inner| inner.ty())),
                });

                (relation, (key, ty))
            })
            .collect()
    }

    pub fn rule_or_decision(&self) -> Option<Rule<S, R, usize>> {
        match &self.inner {
            ModuleItem::Decision(Decision(rule)) | ModuleItem::Rule(rule) => Some(rule.clone()),
            _ => None,
        }
    }
}

impl<S, R> IndexedItem<S, R> {
    pub fn map_relations<O>(self, cb: &mut impl FnMut(&S, R) -> O) -> IndexedItem<S, O> {
        IndexedItem {
            url: self.url,
            variables: self.variables,
            inner: self.inner.map_relations(cb),
        }
    }
}

impl<S: Clone, R: Clone> ModuleItem<S, R, String> {
    pub fn index_variables(
        (url, item): (Url, Self),
    ) -> (IndexedItem<S, R>, Vec<(Url, Diagnostic<S>)>) {
        let mut variables = IndexMap::new();
        let mut diagnostics = Vec::new();

        let mut map = |is_head, span: &S, name: String| {
            let entry = variables.entry(name.clone());
            let idx = entry.index();

            use indexmap::map::Entry;
            let new = match entry {
                Entry::Occupied(_) => false,
                Entry::Vacant(entry) => {
                    entry.insert(span.clone());
                    true
                }
            };

            if is_head && new {
                let d = Diagnostic {
                    span: span.clone(),
                    kind: DiagnosticKind::Error,
                    message: format!("{name:?} does not appear within body"),
                    labels: vec![Spanned {
                        span: span.clone(),
                        inner: "Defined here.".to_string(),
                    }],
                };

                diagnostics.push((url.clone(), d));
            }

            idx
        };

        let inner = {
            use ModuleItem::*;
            match item {
                Decision(el) => Decision(el.map_variables(&mut map)),
                Rule(el) => Rule(el.map_variables(&mut map)),
                Constraint(el) => Constraint(el.map_variables(&mut map)),
                Diagnostic(el) => Diagnostic(el),
            }
        };

        let variables = variables
            .into_iter()
            .map(|(inner, span)| Spanned { inner, span })
            .collect();

        let indexed = IndexedItem {
            url,
            inner,
            variables,
        };

        (indexed, diagnostics)
    }
}

impl<S, R, T> ModuleItem<S, R, T> {
    pub fn map_relations<O>(self, cb: &mut impl FnMut(&S, R) -> O) -> ModuleItem<S, O, T> {
        use ModuleItem::*;
        match self {
            Decision(el) => Decision(el.map_relations(cb)),
            Rule(el) => Rule(el.map_relations(cb)),
            Constraint(el) => Constraint(el.map_relations(cb)),
            Diagnostic(el) => return Diagnostic(el),
        }
    }
}

impl<S, R, T> Decision<S, R, T> {
    pub fn map_variables<O>(self, cb: &mut impl FnMut(bool, &S, T) -> O) -> Decision<S, R, O> {
        Decision(self.0.map_variables(cb))
    }

    pub fn map_relations<O>(self, cb: &mut impl FnMut(&S, R) -> O) -> Decision<S, O, T> {
        Decision(self.0.map_relations(cb))
    }
}

impl<S, R, T> Rule<S, R, T> {
    pub fn map_variables<O>(self, cb: &mut impl FnMut(bool, &S, T) -> O) -> Rule<S, R, O> {
        let body = self
            .body
            .into_iter()
            .map(|atom| {
                atom.map(|inner| inner.map_variables(&mut |span, var| cb(false, span, var)))
            })
            .collect();

        let head = self
            .head
            .map(|inner| inner.map_variables(&mut |span, var| cb(true, span, var)));

        Rule { head, body }
    }

    pub fn map_relations<O>(self, cb: &mut impl FnMut(&S, R) -> O) -> Rule<S, O, T> {
        Rule {
            head: self.head.map(|inner| inner.map_relation(cb)),
            body: self
                .body
                .into_iter()
                .map(|atom| atom.map(|inner| inner.map_relation(cb)))
                .collect(),
        }
    }
}

impl<S, R, T> Constraint<S, R, T> {
    pub fn map_variables<O>(self, cb: &mut impl FnMut(bool, &S, T) -> O) -> Constraint<S, R, O> {
        let body = self
            .body
            .into_iter()
            .map(|atom| {
                atom.map(|inner| inner.map_variables(&mut |span, var| cb(false, span, var)))
            })
            .collect();

        let captures = self
            .captures
            .into_iter()
            .map(|var| Spanned {
                inner: cb(true, &var.span, var.inner),
                span: var.span,
            })
            .collect();

        Constraint {
            captures,
            kind: self.kind,
            body,
        }
    }

    pub fn map_relations<O>(self, cb: &mut impl FnMut(&S, R) -> O) -> Constraint<S, O, T> {
        Constraint {
            captures: self.captures,
            kind: self.kind,
            body: self
                .body
                .into_iter()
                .map(|atom| atom.map(|inner| inner.map_relation(cb)))
                .collect(),
        }
    }
}

impl<S, R, T, V> Atom<S, R, AnyTerm<S, T, V>> {
    pub fn map_variables<O>(self, cb: &mut impl FnMut(&S, T) -> O) -> Atom<S, R, AnyTerm<S, O, V>> {
        Atom {
            relation: self.relation,
            pattern: self.pattern.map(|inner| inner.map_variables(cb)),
        }
    }
}

impl<S, R, T> Atom<S, R, T> {
    pub fn map_relation<O>(self, cb: &mut impl FnMut(&S, R) -> O) -> Atom<S, O, T> {
        Atom {
            relation: Spanned {
                inner: cb(&self.relation.span, self.relation.inner),
                span: self.relation.span,
            },
            pattern: self.pattern,
        }
    }
}

impl<S: Clone + Debug + Eq, T> Pattern<S, TypeTerm<S, T>> {
    pub fn unify<K: Clone>(
        self,
        key: K,
        dst: Type<S>,
    ) -> Vec<Result<(K, (Spanned<S, T>, Type<S>)), Diagnostic<S>>> {
        let mut diagnostics = Vec::new();
        let mut resolved = Vec::new();

        self.unify_inner(dst, &mut diagnostics, &mut |var, ty| {
            resolved.push((var, ty));
        });

        resolved
            .into_iter()
            .map(|(var, ty)| Ok((key.clone(), (var, ty))))
            .chain(diagnostics.into_iter().map(Err))
            .collect()
    }

    pub fn unify_inner(
        self,
        target: Type<S>,
        diagnostics: &mut Vec<Diagnostic<S>>,
        cb: &mut impl FnMut(Spanned<S, T>, Type<S>),
    ) {
        match self {
            Pattern::Tuple(lhs) => match target {
                Pattern::Tuple(rhs) => {
                    if lhs.len() != rhs.len() {
                        diagnostics.push(Diagnostic {
                            kind: DiagnosticKind::Error,
                            message: format!(
                                "Expected tuple of arity {}, got tuple of arity {}",
                                lhs.len(),
                                rhs.len()
                            ),
                            span: lhs.span,
                            // TODO
                            labels: vec![],
                        });
                    } else {
                        lhs.inner.into_iter().zip(rhs.inner).for_each(|(lhs, rhs)| {
                            lhs.inner.unify_inner(rhs.inner, diagnostics, cb)
                        });
                    }
                }
                Pattern::Leaf(prim) => {
                    diagnostics.push(Diagnostic {
                        kind: DiagnosticKind::Error,
                        message: format!(
                            "Expected tuple of arity {}, got {:?}",
                            lhs.len(),
                            prim.inner
                        ),
                        span: lhs.span,
                        // TODO
                        labels: vec![],
                    });
                }
            },
            Pattern::Leaf(leaf) => match leaf.inner {
                AnyTerm::Variable(var) => {
                    cb(var, target);
                }
                AnyTerm::Value(prim) => {
                    if Pattern::Leaf(prim.clone()) != target {
                        diagnostics.push(Diagnostic {
                            span: prim.span,
                            kind: DiagnosticKind::Error,
                            message: format!("Expected {:?}, got {target:?}", prim.inner),
                            // TODO
                            labels: vec![],
                        });
                    }
                }
            },
        }
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
