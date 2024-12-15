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

use serde::{Deserialize, Serialize};
use url::Url;

use crate::frontend::span::{Point, Span, Spanned};

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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ModuleItem<S, R, T> {
    Rule(Rule<S, R, T>),
    Decision(Decision<S, R, T>),
    Constraint(Constraint<S, R, T>),
    Diagnostic(Diagnostic<S>),
}

impl<S, R, T> ModuleItem<S, R, T> {
    pub fn rule<C>((ctx, item): (C, Self)) -> Option<(C, Rule<S, R, T>)> {
        match item {
            ModuleItem::Rule(r) => Some((ctx, r)),
            _ => None,
        }
    }

    pub fn diagnostic<C>((ctx, item): (C, Self)) -> Option<(C, Diagnostic<S>)> {
        match item {
            ModuleItem::Diagnostic(d) => Some((ctx, d)),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Decision<S, R, T>(pub Rule<S, R, T>);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct IndexedRule<S> {
    pub variables: Vec<Spanned<S, String>>,
    pub inner: Rule<S, String, usize>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Rule<S, R, T> {
    pub head: Spanned<S, Atom<S, R, Term<S, T>>>,
    pub body: Vec<Spanned<S, Atom<S, R, Term<S, T>>>>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Constraint<S, R, T> {
    pub captures: Vec<Spanned<S, String>>,
    pub kind: Spanned<S, ConstraintKind>,
    pub body: Vec<Spanned<S, Atom<S, R, Term<S, T>>>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum ConstraintKind {
    Uniform,
    Cardinality(CardinalityConstraintKind, i64),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum CardinalityConstraintKind {
    Only,
    AtMost,
    AtLeast,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Atom<S, R, T> {
    pub relation: Spanned<S, R>,
    pub pattern: Spanned<S, Pattern<S, T>>,
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

pub type Term<S, T> = AnyTerm<S, T, Value>;
pub type TypeTerm<S, T> = AnyTerm<S, T, PrimitiveType>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum AnyTerm<S, T, V> {
    Variable(Spanned<S, T>),
    Value(Spanned<S, V>),
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum ResourceId {
    Module(Url, String),
}
