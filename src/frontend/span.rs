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

use std::{collections::BTreeMap, fmt::Display, ops::Deref};

use serde::{Deserialize, Serialize};

use crate::frontend::types::*;

impl<S, O> MapSpan<S, O> for InlayHint<S> {
    type Target = InlayHint<O>;

    fn map_span(self, cb: &mut impl FnMut(S) -> O) -> Self::Target {
        InlayHint {
            span: cb(self.span),
            contents: self.contents,
        }
    }
}

impl<S, O> MapSpan<S, O> for Diagnostic<S> {
    type Target = Diagnostic<O>;

    fn map_span(self, cb: &mut impl FnMut(S) -> O) -> Self::Target {
        Diagnostic {
            span: cb(self.span),
            kind: self.kind,
            message: self.message,
            labels: self.labels.map_span(cb),
        }
    }
}

impl<S, O, R> MapSpan<S, O> for IndexedItem<S, R>
where
    R: MapSpan<S, O>,
{
    type Target = IndexedItem<O, R::Target>;

    fn map_span(self, cb: &mut impl FnMut(S) -> O) -> Self::Target {
        IndexedItem {
            url: self.url,
            variables: self.variables.map_span(cb),
            inner: self.inner.map_span(cb),
        }
    }
}

impl<S, O, R, T> MapSpan<S, O> for ModuleItem<S, R, T>
where
    R: MapSpan<S, O>,
    T: MapSpan<S, O>,
{
    type Target = ModuleItem<O, R::Target, T::Target>;

    fn map_span(self, cb: &mut impl FnMut(S) -> O) -> Self::Target {
        use ModuleItem::*;
        match self {
            Rule(el) => Rule(el.map_span(cb)),
            Decision(el) => Decision(el.map_span(cb)),
            Constraint(el) => Constraint(el.map_span(cb)),
            Diagnostic(el) => Diagnostic(el.map_span(cb)),
        }
    }
}

impl<S, O, R, T> MapSpan<S, O> for Decision<S, R, T>
where
    R: MapSpan<S, O>,
    T: MapSpan<S, O>,
{
    type Target = Decision<O, R::Target, T::Target>;

    fn map_span(self, cb: &mut impl FnMut(S) -> O) -> Self::Target {
        Decision(self.0.map_span(cb))
    }
}

impl<S, O, R, T> MapSpan<S, O> for Rule<S, R, T>
where
    R: MapSpan<S, O>,
    T: MapSpan<S, O>,
{
    type Target = Rule<O, R::Target, T::Target>;

    fn map_span(self, cb: &mut impl FnMut(S) -> O) -> Self::Target {
        let head = self.head.map_span(cb);
        let body = self.body.map_span(cb);
        Rule { head, body }
    }
}

impl<S, O, R, T> MapSpan<S, O> for Constraint<S, R, T>
where
    R: MapSpan<S, O>,
    T: MapSpan<S, O>,
{
    type Target = Constraint<O, R::Target, T::Target>;

    fn map_span(self, cb: &mut impl FnMut(S) -> O) -> Self::Target {
        let captures = self.captures.map_span(cb);
        let kind = self.kind.map_span(cb);
        let body = self.body.map_span(cb);

        Constraint {
            captures,
            kind,
            body,
        }
    }
}

impl Spanless for ConstraintKind {}

impl<S, O, R, T> MapSpan<S, O> for Atom<S, R, T>
where
    R: MapSpan<S, O>,
    T: MapSpan<S, O>,
{
    type Target = Atom<O, R::Target, T::Target>;

    fn map_span(self, cb: &mut impl FnMut(S) -> O) -> Self::Target {
        let relation = self.relation.map_span(cb);
        let pattern = self.pattern.map_span(cb);
        Atom { relation, pattern }
    }
}

impl<S, O, T> MapSpan<S, O> for Pattern<S, T>
where
    T: MapSpan<S, O>,
{
    type Target = Pattern<O, T::Target>;

    fn map_span(self, cb: &mut impl FnMut(S) -> O) -> Self::Target {
        use Pattern::*;
        match self {
            Leaf(el) => Leaf(el.map_span(cb)),
            Tuple(el) => Tuple(el.map_span(cb)),
        }
    }
}

impl<S, O, T, V> MapSpan<S, O> for AnyTerm<S, T, V>
where
    T: MapSpan<S, O>,
    V: MapSpan<S, O>,
{
    type Target = AnyTerm<O, T::Target, V::Target>;

    fn map_span(self, cb: &mut impl FnMut(S) -> O) -> Self::Target {
        use AnyTerm::*;
        match self {
            Variable(el) => Variable(el.map_span(cb)),
            Value(el) => Value(el.map_span(cb)),
        }
    }
}

impl Spanless for Value {}
impl Spanless for PrimitiveType {}
impl Spanless for usize {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Spanned<S, T> {
    pub span: S,
    pub inner: T,
}

impl<S, T> Deref for Spanned<S, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<S, T: Display> Display for Spanned<S, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<S, T> Spanned<S, Option<T>> {
    pub fn flatten(self) -> Option<Spanned<S, T>> {
        Some(Spanned {
            span: self.span,
            inner: self.inner?,
        })
    }
}

impl<S, T> Spanned<S, T> {
    pub fn map<O>(self, cb: impl FnOnce(T) -> O) -> Spanned<S, O> {
        Spanned {
            span: self.span,
            inner: cb(self.inner),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Span<T = Point> {
    pub start: T,
    pub end: T,
}

impl From<tree_sitter::Range> for Span {
    fn from(range: tree_sitter::Range) -> Self {
        Self {
            start: range.start_point.into(),
            end: range.end_point.into(),
        }
    }
}

impl From<Span> for tower_lsp::lsp_types::Range {
    fn from(span: Span) -> Self {
        Self {
            start: span.start.into(),
            end: span.end.into(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl From<tree_sitter::Point> for Point {
    fn from(pt: tree_sitter::Point) -> Self {
        Self {
            row: pt.row,
            col: pt.column,
        }
    }
}

impl From<Point> for tower_lsp::lsp_types::Position {
    fn from(pt: Point) -> Self {
        tower_lsp::lsp_types::Position {
            line: pt.row as u32,
            character: pt.col as u32,
        }
    }
}

pub trait MapMultiSpan<S, O>: MapSpan<S, O> {
    fn map_multi_span(self, map: &BTreeMap<S, O>) -> Self::Target;
}

impl<S, O, T> MapMultiSpan<S, O> for T
where
    T: MapSpan<S, O>,
    S: Ord,
    O: Clone,
{
    fn map_multi_span(self, map: &BTreeMap<S, O>) -> Self::Target {
        self.map_span(&mut |span| {
            map.get(&span)
                .expect("missing span key from multi-span map")
                .to_owned()
        })
    }
}

pub trait Spanless {}

impl<S, O, T: Spanless> MapSpan<S, O> for T {
    type Target = T;

    fn map_span(self, _cb: &mut impl FnMut(S) -> O) -> Self::Target {
        self
    }
}

pub trait MapSpan<S, O> {
    type Target;

    fn map_span(self, cb: &mut impl FnMut(S) -> O) -> Self::Target;
}

impl<S, T, O> MapSpan<S, O> for Spanned<S, T>
where
    T: MapSpan<S, O>,
{
    type Target = Spanned<O, T::Target>;

    fn map_span(self, cb: &mut impl FnMut(S) -> O) -> Self::Target {
        let span = cb(self.span);
        let inner = self.inner.map_span(cb);
        Spanned { span, inner }
    }
}

impl<S, O, T> MapSpan<S, O> for Vec<T>
where
    T: MapSpan<S, O>,
{
    type Target = Vec<T::Target>;

    fn map_span(self, cb: &mut impl FnMut(S) -> O) -> Self::Target {
        self.into_iter().map(|el| el.map_span(cb)).collect()
    }
}
