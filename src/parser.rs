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

use std::{fmt::Debug, str::FromStr};

use tree_sitter::{InputEdit, Node, Parser, Range, Tree, TreeCursor};
use tree_sitter_fulcrum::LANGUAGE;

use crate::{
    span::{MapSpan, Span, Spanless, Spanned},
    types::*,
};

pub struct Module {
    parser: Parser,
    tree: Tree,
    src: String,
}

impl Module {
    pub fn new(src: &str) -> Self {
        let mut parser = Parser::new();
        parser.set_language(&LANGUAGE.into()).unwrap();
        let src = src.to_string();
        let tree = parser.parse(&src, None).unwrap();
        Module { parser, tree, src }
    }

    pub fn update(&mut self, src: &str) {
        self.src = src.to_string();
        self.tree = self.parser.parse(src, None).unwrap();
    }

    pub fn edit(&mut self, src: &str, edit: InputEdit) {
        self.src = src.to_string();
        self.tree.edit(&edit);
        self.tree = self.parser.parse(src, Some(&self.tree)).unwrap();
    }

    pub fn items(&self) -> ModuleItems<Span, String> {
        let mut diagnostics = Vec::new();
        let mut rules = Vec::new();
        let mut decisions = Vec::new();
        let mut cursor = self.tree.walk();
        for node in cursor.node().children(&mut cursor) {
            if node.has_error() {
                let mut stack = vec![node];
                let mut cursor = node.walk();
                while let Some(node) = stack.pop() {
                    stack.extend(node.children(&mut cursor).filter(|node| node.has_error()));

                    if node.is_error() {
                        diagnostics.push(Diagnostic {
                            span: node.range(),
                            kind: DiagnosticKind::Error,
                            contents: "Syntax error".to_string(),
                        });
                    }
                }

                continue;
            }

            let mut cursor = node.walk();
            match node.kind() {
                "rule" => rules.push(Rule::parse(&self.src, &node, &mut cursor)),
                "decision" => decisions.push(Decision::parse(&self.src, &node, &mut cursor)),
                "comment" | "constraint" => continue,
                other => unimplemented!("unexpected node {other:?}"),
            }
        }

        ModuleItems {
            rules,
            decisions,
            constraints: Default::default(),
            diagnostics,
        }
        .map_span(&mut |range| range.into())
    }
}

impl Parse for Decision<Range, String> {
    fn parse<'tree>(src: &str, node: &Node<'tree>, cursor: &mut TreeCursor<'tree>) -> Self {
        Self(Parse::parse(src, &node.named_child(0).unwrap(), cursor))
    }
}

impl Parse for Rule<Range, String> {
    fn parse<'tree>(src: &str, node: &Node<'tree>, cursor: &mut TreeCursor<'tree>) -> Self {
        let head = Parse::parse(src, &node.child_by_field_name("head").unwrap(), cursor);

        let body: Vec<_> = node
            .children_by_field_name("body", cursor)
            .filter(|node| node.is_named())
            .collect();

        let body = body
            .into_iter()
            .map(|node| Parse::parse(src, &node, cursor))
            .collect();

        Self { head, body }
    }
}

impl Parse for Constraint<Range, String> {
    fn parse(src: &str, node: &Node, cursor: &mut TreeCursor) -> Self {
        todo!()
    }
}

impl Parse for Atom<Range, Term<Range, String>> {
    fn parse<'tree>(src: &str, node: &Node<'tree>, cursor: &mut TreeCursor<'tree>) -> Self {
        let relation_node = node.child_by_field_name("relation").unwrap();
        let relation = Parse::parse(src, &relation_node, cursor);
        let pattern_node = relation_node.next_sibling().unwrap();
        let pattern = Parse::parse(src, &pattern_node, cursor);
        Self { relation, pattern }
    }
}

impl<T: Parse> Parse for Pattern<Range, T> {
    fn parse<'tree>(src: &str, node: &Node<'tree>, cursor: &mut TreeCursor<'tree>) -> Self {
        let node = if node.kind() == "pattern" {
            node.child(0).unwrap()
        } else {
            *node
        };

        let node = &node;
        match node.kind() {
            "tuple" => Self::Tuple(Parse::parse(src, node, cursor)),
            _ => Self::Leaf(Parse::parse(src, node, cursor)),
        }
    }
}

impl<T: Parse> Parse for Term<Range, T> {
    fn parse<'tree>(src: &str, node: &Node<'tree>, cursor: &mut TreeCursor<'tree>) -> Self {
        let node = &node;
        match node.kind() {
            "variable" => Term::Variable(Parse::parse(src, node, cursor)),
            _ => Term::Value(Parse::parse(src, node, cursor)),
        }
    }
}

impl Parse for Value {
    fn parse<'tree>(src: &str, node: &Node<'tree>, _cursor: &mut TreeCursor<'tree>) -> Self {
        let node = &node;
        let start = node.range().start_byte;
        let end = node.range().end_byte;
        let slice = &src[start..end];
        match node.kind() {
            "symbol" => Self::Symbol(slice.to_string()),
            "integer" => Self::Integer(slice.parse().unwrap()),
            other => unimplemented!("unexpected node kind {other:?}"),
        }
    }
}

pub trait Parse {
    fn parse<'tree>(src: &str, node: &Node<'tree>, cursor: &mut TreeCursor<'tree>) -> Self;
}

impl<T: Parse> Parse for Spanned<Range, T> {
    fn parse<'tree>(src: &str, node: &Node<'tree>, cursor: &mut TreeCursor<'tree>) -> Self {
        let span = node.range();
        let inner = Parse::parse(src, node, cursor);
        Self { span, inner }
    }
}

impl<T: Parse> Parse for Vec<T> {
    fn parse<'tree>(src: &str, node: &Node<'tree>, cursor: &mut TreeCursor<'tree>) -> Self {
        let mut nodes = Vec::new();
        cursor.reset(*node);
        cursor.goto_first_child();

        loop {
            if cursor.node().is_named() {
                nodes.push(cursor.node());
            }

            if !cursor.goto_next_sibling() {
                break;
            }
        }

        nodes
            .into_iter()
            .map(|node| Parse::parse(src, &node, cursor))
            .collect()
    }
}

trait ParseFromStr: FromStr<Err: Debug> {}
impl ParseFromStr for String {}
impl ParseFromStr for i64 {}

impl<T: ParseFromStr> Parse for T {
    fn parse(src: &str, node: &Node, _cursor: &mut TreeCursor) -> Self {
        let start = node.range().start_byte;
        let end = node.range().end_byte;
        let slice = &src[start..end];
        Self::from_str(slice).unwrap()
    }
}

impl<T: ParseFromStr> Spanless for T {}
