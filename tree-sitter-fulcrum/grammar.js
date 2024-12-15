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

/// <reference types="tree-sitter-cli/dsl" />

const list = el => seq(el, repeat(seq(",", el)));
const paren_list = el => seq("(", list(el), ")");

module.exports = grammar({
  name: "fulcrum",

  extras: $ => [$._whitespace, $.comment],

  rules: {
    file: $ => repeat(choice($.import, $.rule, $.decision, $.constraint)),

    _whitespace: _ => /[ \n\r\t]/,
    comment: _ => /;.*\n/,

    variable: _ => /[a-z][a-z0-9_]*/,
    symbol: _ => /[A-Z][a-zA-Z0-9]*/,
    _ident: $ => choice($.variable, $.symbol),

    integer: _ => choice("0", /-?[1-9][0-9]*/),

    import: $ => seq(
      "import",
      $.symbol,
      repeat(seq(".", $.symbol)),
      ".",
      paren_list($.symbol),
    ),

    decision: $ => seq("decide", $.rule),
    rule: $ => seq(field("head", $.atom), field("body", optional(seq("if", $._rule_body))), "."),
    _rule_body: $ => list($.atom),

    constraint: $ => seq(
      "constrain",
      field("soft", optional(seq("soft", "(", $.integer, ")"))),
      field("captures", optional($.captures)),
      field("kind", $.constraint_kind),
      field("body", $._rule_body),
      "."
    ),

    captures: $ => paren_list($._ident),

    constraint_kind: $ => choice($.uniform, $.cardinality),

    uniform: _ => "uniform",
    cardinality: $ => seq(
      "cardinality", "to",
      choice($.only, $.at_most, $.at_least),
      $.integer,
    ),

    only: _ => "only",
    at_most: _ => seq("at", "most"),
    at_least: _ => seq("at", "least"),

    atom: $ => seq(field("relation", $.symbol), $.pattern),

    pattern: $ => choice( $.tuple, $._ident, $.integer),
    tuple: $ => paren_list($.pattern),
    record: $ => seq("{", list($.field), "}"),
    field: $ => seq(field("key", $.symbol), ":", field("value", $.pattern)),
  }
});
