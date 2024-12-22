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

use std::{fmt::Debug, hash::Hash, ops::Range};

use serde::{Deserialize, Serialize};
use url::Url;

use crate::frontend::span::{Span, Spanned};

pub type DiagnosticResult<S, T> = Result<T, Diagnostic<S>>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct InlayHint<S> {
    pub span: S,
    pub contents: String,
}

impl InlayHint<Span> {
    pub fn to_lsp(&self) -> tower_lsp::lsp_types::InlayHint {
        use tower_lsp::lsp_types::*;
        InlayHint {
            position: self.span.end.into(),
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
    pub message: String,
    pub labels: Vec<Spanned<S, String>>,
}

impl<S> Diagnostic<S> {
    pub fn span_set(self) -> impl Iterator<Item = S> {
        std::iter::once(self.span).chain(self.labels.into_iter().map(|label| label.span))
    }
}

impl<ID> Diagnostic<(ID, Range<usize>)>
where
    ID: Clone + Debug + Hash + Eq,
{
    pub fn to_ariadne(&self) -> ariadne::Report<'static, (ID, Range<usize>)> {
        use ariadne::*;

        let kind = match self.kind {
            DiagnosticKind::Error => ReportKind::Error,
            DiagnosticKind::Warning => ReportKind::Warning,
            DiagnosticKind::Info => ReportKind::Custom("Info", Color::White),
            DiagnosticKind::Note => ReportKind::Custom("Note", Color::Blue),
        };

        let span = self.span.clone();

        let labels = self
            .labels
            .iter()
            .map(|label| Label::new(label.span.clone()).with_message(label.inner.clone()));

        Report::build(kind, span.clone())
            .with_message(self.message.clone())
            .with_labels(labels)
            .finish()
    }
}

impl Diagnostic<(Url, Span)> {
    pub fn to_lsp(&self) -> tower_lsp::lsp_types::Diagnostic {
        use tower_lsp::lsp_types::DiagnosticSeverity;
        let severity = match self.kind {
            DiagnosticKind::Error => DiagnosticSeverity::ERROR,
            DiagnosticKind::Warning => DiagnosticSeverity::WARNING,
            DiagnosticKind::Info => DiagnosticSeverity::INFORMATION,
            DiagnosticKind::Note => DiagnosticSeverity::HINT,
        };

        tower_lsp::lsp_types::Diagnostic {
            range: self.span.1.into(),
            severity: Some(severity),
            message: self.message.clone(),
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
