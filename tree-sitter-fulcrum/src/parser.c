#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 54
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 39
#define ALIAS_COUNT 0
#define TOKEN_COUNT 20
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 6
#define MAX_ALIAS_SEQUENCE_LENGTH 6
#define PRODUCTION_ID_COUNT 6

enum ts_symbol_identifiers {
  sym__whitespace = 1,
  sym_comment = 2,
  sym_variable = 3,
  sym_symbol = 4,
  anon_sym_0 = 5,
  aux_sym_integer_token1 = 6,
  anon_sym_decide = 7,
  anon_sym_if = 8,
  anon_sym_DOT = 9,
  anon_sym_COMMA = 10,
  anon_sym_constrain = 11,
  anon_sym_LPAREN = 12,
  anon_sym_RPAREN = 13,
  anon_sym_cardinality = 14,
  anon_sym_to = 15,
  sym_only = 16,
  anon_sym_at = 17,
  anon_sym_most = 18,
  anon_sym_least = 19,
  sym_file = 20,
  sym__ident = 21,
  sym_integer = 22,
  sym_decision = 23,
  sym_rule = 24,
  sym__rule_body = 25,
  sym_constraint = 26,
  sym_captures = 27,
  sym_constraint_kind = 28,
  sym_cardinality = 29,
  sym_at_most = 30,
  sym_at_least = 31,
  sym_atom = 32,
  sym_pattern = 33,
  sym_tuple = 34,
  aux_sym_file_repeat1 = 35,
  aux_sym__rule_body_repeat1 = 36,
  aux_sym_captures_repeat1 = 37,
  aux_sym_tuple_repeat1 = 38,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym__whitespace] = "_whitespace",
  [sym_comment] = "comment",
  [sym_variable] = "variable",
  [sym_symbol] = "symbol",
  [anon_sym_0] = "0",
  [aux_sym_integer_token1] = "integer_token1",
  [anon_sym_decide] = "decide",
  [anon_sym_if] = "if",
  [anon_sym_DOT] = ".",
  [anon_sym_COMMA] = ",",
  [anon_sym_constrain] = "constrain",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_cardinality] = "cardinality",
  [anon_sym_to] = "to",
  [sym_only] = "only",
  [anon_sym_at] = "at",
  [anon_sym_most] = "most",
  [anon_sym_least] = "least",
  [sym_file] = "file",
  [sym__ident] = "_ident",
  [sym_integer] = "integer",
  [sym_decision] = "decision",
  [sym_rule] = "rule",
  [sym__rule_body] = "_rule_body",
  [sym_constraint] = "constraint",
  [sym_captures] = "captures",
  [sym_constraint_kind] = "constraint_kind",
  [sym_cardinality] = "cardinality",
  [sym_at_most] = "at_most",
  [sym_at_least] = "at_least",
  [sym_atom] = "atom",
  [sym_pattern] = "pattern",
  [sym_tuple] = "tuple",
  [aux_sym_file_repeat1] = "file_repeat1",
  [aux_sym__rule_body_repeat1] = "_rule_body_repeat1",
  [aux_sym_captures_repeat1] = "captures_repeat1",
  [aux_sym_tuple_repeat1] = "tuple_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym__whitespace] = sym__whitespace,
  [sym_comment] = sym_comment,
  [sym_variable] = sym_variable,
  [sym_symbol] = sym_symbol,
  [anon_sym_0] = anon_sym_0,
  [aux_sym_integer_token1] = aux_sym_integer_token1,
  [anon_sym_decide] = anon_sym_decide,
  [anon_sym_if] = anon_sym_if,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_constrain] = anon_sym_constrain,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_cardinality] = anon_sym_cardinality,
  [anon_sym_to] = anon_sym_to,
  [sym_only] = sym_only,
  [anon_sym_at] = anon_sym_at,
  [anon_sym_most] = anon_sym_most,
  [anon_sym_least] = anon_sym_least,
  [sym_file] = sym_file,
  [sym__ident] = sym__ident,
  [sym_integer] = sym_integer,
  [sym_decision] = sym_decision,
  [sym_rule] = sym_rule,
  [sym__rule_body] = sym__rule_body,
  [sym_constraint] = sym_constraint,
  [sym_captures] = sym_captures,
  [sym_constraint_kind] = sym_constraint_kind,
  [sym_cardinality] = sym_cardinality,
  [sym_at_most] = sym_at_most,
  [sym_at_least] = sym_at_least,
  [sym_atom] = sym_atom,
  [sym_pattern] = sym_pattern,
  [sym_tuple] = sym_tuple,
  [aux_sym_file_repeat1] = aux_sym_file_repeat1,
  [aux_sym__rule_body_repeat1] = aux_sym__rule_body_repeat1,
  [aux_sym_captures_repeat1] = aux_sym_captures_repeat1,
  [aux_sym_tuple_repeat1] = aux_sym_tuple_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym__whitespace] = {
    .visible = false,
    .named = true,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [sym_variable] = {
    .visible = true,
    .named = true,
  },
  [sym_symbol] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_0] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_integer_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_decide] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_if] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_constrain] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_cardinality] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_to] = {
    .visible = true,
    .named = false,
  },
  [sym_only] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_at] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_most] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_least] = {
    .visible = true,
    .named = false,
  },
  [sym_file] = {
    .visible = true,
    .named = true,
  },
  [sym__ident] = {
    .visible = false,
    .named = true,
  },
  [sym_integer] = {
    .visible = true,
    .named = true,
  },
  [sym_decision] = {
    .visible = true,
    .named = true,
  },
  [sym_rule] = {
    .visible = true,
    .named = true,
  },
  [sym__rule_body] = {
    .visible = false,
    .named = true,
  },
  [sym_constraint] = {
    .visible = true,
    .named = true,
  },
  [sym_captures] = {
    .visible = true,
    .named = true,
  },
  [sym_constraint_kind] = {
    .visible = true,
    .named = true,
  },
  [sym_cardinality] = {
    .visible = true,
    .named = true,
  },
  [sym_at_most] = {
    .visible = true,
    .named = true,
  },
  [sym_at_least] = {
    .visible = true,
    .named = true,
  },
  [sym_atom] = {
    .visible = true,
    .named = true,
  },
  [sym_pattern] = {
    .visible = true,
    .named = true,
  },
  [sym_tuple] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym__rule_body_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_captures_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_tuple_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum ts_field_identifiers {
  field_body = 1,
  field_bound = 2,
  field_captures = 3,
  field_head = 4,
  field_kind = 5,
  field_relation = 6,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_body] = "body",
  [field_bound] = "bound",
  [field_captures] = "captures",
  [field_head] = "head",
  [field_kind] = "kind",
  [field_relation] = "relation",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 1},
  [3] = {.index = 2, .length = 3},
  [4] = {.index = 5, .length = 3},
  [5] = {.index = 8, .length = 4},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_relation, 0},
  [1] =
    {field_head, 0},
  [2] =
    {field_body, 1},
    {field_body, 2},
    {field_head, 0},
  [5] =
    {field_body, 3},
    {field_bound, 2},
    {field_kind, 1},
  [8] =
    {field_body, 4},
    {field_bound, 3},
    {field_captures, 1},
    {field_kind, 2},
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 32,
  [33] = 33,
  [34] = 34,
  [35] = 35,
  [36] = 36,
  [37] = 37,
  [38] = 38,
  [39] = 39,
  [40] = 40,
  [41] = 41,
  [42] = 42,
  [43] = 43,
  [44] = 44,
  [45] = 45,
  [46] = 46,
  [47] = 47,
  [48] = 48,
  [49] = 49,
  [50] = 50,
  [51] = 51,
  [52] = 52,
  [53] = 53,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(39);
      ADVANCE_MAP(
        '(', 51,
        ')', 52,
        ',', 49,
        '-', 38,
        '.', 48,
        '0', 44,
        ';', 1,
        'a', 31,
        'c', 3,
        'd', 10,
        'i', 13,
        'l', 11,
        'm', 24,
        'o', 20,
        't', 25,
        '\t', 40,
        '\n', 40,
        '\r', 40,
        ' ', 40,
      );
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(45);
      if (('A' <= lookahead && lookahead <= 'Z')) ADVANCE(43);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(41);
      if (lookahead != 0) ADVANCE(1);
      END_STATE();
    case 2:
      if (lookahead == '(') ADVANCE(51);
      if (lookahead == '-') ADVANCE(38);
      if (lookahead == '0') ADVANCE(44);
      if (lookahead == ';') ADVANCE(1);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(40);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(45);
      if (('A' <= lookahead && lookahead <= 'Z')) ADVANCE(43);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(42);
      END_STATE();
    case 3:
      if (lookahead == 'a') ADVANCE(26);
      if (lookahead == 'o') ADVANCE(23);
      END_STATE();
    case 4:
      if (lookahead == 'a') ADVANCE(19);
      END_STATE();
    case 5:
      if (lookahead == 'a') ADVANCE(16);
      END_STATE();
    case 6:
      if (lookahead == 'a') ADVANCE(30);
      END_STATE();
    case 7:
      if (lookahead == 'c') ADVANCE(14);
      END_STATE();
    case 8:
      if (lookahead == 'd') ADVANCE(15);
      END_STATE();
    case 9:
      if (lookahead == 'd') ADVANCE(12);
      END_STATE();
    case 10:
      if (lookahead == 'e') ADVANCE(7);
      END_STATE();
    case 11:
      if (lookahead == 'e') ADVANCE(6);
      END_STATE();
    case 12:
      if (lookahead == 'e') ADVANCE(46);
      END_STATE();
    case 13:
      if (lookahead == 'f') ADVANCE(47);
      END_STATE();
    case 14:
      if (lookahead == 'i') ADVANCE(9);
      END_STATE();
    case 15:
      if (lookahead == 'i') ADVANCE(22);
      END_STATE();
    case 16:
      if (lookahead == 'i') ADVANCE(21);
      END_STATE();
    case 17:
      if (lookahead == 'i') ADVANCE(35);
      END_STATE();
    case 18:
      if (lookahead == 'l') ADVANCE(36);
      END_STATE();
    case 19:
      if (lookahead == 'l') ADVANCE(17);
      END_STATE();
    case 20:
      if (lookahead == 'n') ADVANCE(18);
      END_STATE();
    case 21:
      if (lookahead == 'n') ADVANCE(50);
      END_STATE();
    case 22:
      if (lookahead == 'n') ADVANCE(4);
      END_STATE();
    case 23:
      if (lookahead == 'n') ADVANCE(29);
      END_STATE();
    case 24:
      if (lookahead == 'o') ADVANCE(28);
      END_STATE();
    case 25:
      if (lookahead == 'o') ADVANCE(54);
      END_STATE();
    case 26:
      if (lookahead == 'r') ADVANCE(8);
      END_STATE();
    case 27:
      if (lookahead == 'r') ADVANCE(5);
      END_STATE();
    case 28:
      if (lookahead == 's') ADVANCE(32);
      END_STATE();
    case 29:
      if (lookahead == 's') ADVANCE(34);
      END_STATE();
    case 30:
      if (lookahead == 's') ADVANCE(33);
      END_STATE();
    case 31:
      if (lookahead == 't') ADVANCE(56);
      END_STATE();
    case 32:
      if (lookahead == 't') ADVANCE(57);
      END_STATE();
    case 33:
      if (lookahead == 't') ADVANCE(58);
      END_STATE();
    case 34:
      if (lookahead == 't') ADVANCE(27);
      END_STATE();
    case 35:
      if (lookahead == 't') ADVANCE(37);
      END_STATE();
    case 36:
      if (lookahead == 'y') ADVANCE(55);
      END_STATE();
    case 37:
      if (lookahead == 'y') ADVANCE(53);
      END_STATE();
    case 38:
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(45);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(sym__whitespace);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(sym_comment);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(sym_variable);
      if (('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(42);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(sym_symbol);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(43);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(anon_sym_0);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(aux_sym_integer_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(45);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(anon_sym_decide);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(anon_sym_if);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 50:
      ACCEPT_TOKEN(anon_sym_constrain);
      END_STATE();
    case 51:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 52:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 53:
      ACCEPT_TOKEN(anon_sym_cardinality);
      END_STATE();
    case 54:
      ACCEPT_TOKEN(anon_sym_to);
      END_STATE();
    case 55:
      ACCEPT_TOKEN(sym_only);
      END_STATE();
    case 56:
      ACCEPT_TOKEN(anon_sym_at);
      END_STATE();
    case 57:
      ACCEPT_TOKEN(anon_sym_most);
      END_STATE();
    case 58:
      ACCEPT_TOKEN(anon_sym_least);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 2},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 2},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 2},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 0},
  [22] = {.lex_state = 0},
  [23] = {.lex_state = 0},
  [24] = {.lex_state = 0},
  [25] = {.lex_state = 0},
  [26] = {.lex_state = 0},
  [27] = {.lex_state = 2},
  [28] = {.lex_state = 0},
  [29] = {.lex_state = 0},
  [30] = {.lex_state = 0},
  [31] = {.lex_state = 0},
  [32] = {.lex_state = 0},
  [33] = {.lex_state = 0},
  [34] = {.lex_state = 0},
  [35] = {.lex_state = 0},
  [36] = {.lex_state = 2},
  [37] = {.lex_state = 0},
  [38] = {.lex_state = 0},
  [39] = {.lex_state = 0},
  [40] = {.lex_state = 0},
  [41] = {.lex_state = 0},
  [42] = {.lex_state = 0},
  [43] = {.lex_state = 0},
  [44] = {.lex_state = 0},
  [45] = {.lex_state = 0},
  [46] = {.lex_state = 0},
  [47] = {.lex_state = 0},
  [48] = {.lex_state = 0},
  [49] = {.lex_state = 0},
  [50] = {.lex_state = 0},
  [51] = {.lex_state = 0},
  [52] = {.lex_state = 0},
  [53] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym__whitespace] = ACTIONS(3),
    [sym_comment] = ACTIONS(3),
    [sym_symbol] = ACTIONS(1),
    [anon_sym_0] = ACTIONS(1),
    [aux_sym_integer_token1] = ACTIONS(1),
    [anon_sym_decide] = ACTIONS(1),
    [anon_sym_if] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_constrain] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_cardinality] = ACTIONS(1),
    [anon_sym_to] = ACTIONS(1),
    [sym_only] = ACTIONS(1),
    [anon_sym_at] = ACTIONS(1),
    [anon_sym_most] = ACTIONS(1),
    [anon_sym_least] = ACTIONS(1),
  },
  [1] = {
    [sym_file] = STATE(48),
    [sym_decision] = STATE(3),
    [sym_rule] = STATE(3),
    [sym_constraint] = STATE(3),
    [sym_atom] = STATE(42),
    [aux_sym_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(5),
    [sym__whitespace] = ACTIONS(3),
    [sym_comment] = ACTIONS(3),
    [sym_symbol] = ACTIONS(7),
    [anon_sym_decide] = ACTIONS(9),
    [anon_sym_constrain] = ACTIONS(11),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 6,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    STATE(33), 1,
      sym_pattern,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(13), 2,
      sym_variable,
      sym_symbol,
    ACTIONS(15), 2,
      anon_sym_0,
      aux_sym_integer_token1,
    STATE(15), 3,
      sym__ident,
      sym_integer,
      sym_tuple,
  [24] = 7,
    ACTIONS(7), 1,
      sym_symbol,
    ACTIONS(9), 1,
      anon_sym_decide,
    ACTIONS(11), 1,
      anon_sym_constrain,
    ACTIONS(19), 1,
      ts_builtin_sym_end,
    STATE(42), 1,
      sym_atom,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    STATE(5), 4,
      sym_decision,
      sym_rule,
      sym_constraint,
      aux_sym_file_repeat1,
  [50] = 6,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    STATE(20), 1,
      sym_pattern,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(13), 2,
      sym_variable,
      sym_symbol,
    ACTIONS(15), 2,
      anon_sym_0,
      aux_sym_integer_token1,
    STATE(15), 3,
      sym__ident,
      sym_integer,
      sym_tuple,
  [74] = 7,
    ACTIONS(21), 1,
      ts_builtin_sym_end,
    ACTIONS(23), 1,
      sym_symbol,
    ACTIONS(26), 1,
      anon_sym_decide,
    ACTIONS(29), 1,
      anon_sym_constrain,
    STATE(42), 1,
      sym_atom,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    STATE(5), 4,
      sym_decision,
      sym_rule,
      sym_constraint,
      aux_sym_file_repeat1,
  [100] = 6,
    ACTIONS(17), 1,
      anon_sym_LPAREN,
    STATE(41), 1,
      sym_pattern,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(13), 2,
      sym_variable,
      sym_symbol,
    ACTIONS(15), 2,
      anon_sym_0,
      aux_sym_integer_token1,
    STATE(15), 3,
      sym__ident,
      sym_integer,
      sym_tuple,
  [124] = 6,
    ACTIONS(32), 1,
      anon_sym_LPAREN,
    ACTIONS(34), 1,
      anon_sym_cardinality,
    STATE(25), 1,
      sym_captures,
    STATE(26), 1,
      sym_constraint_kind,
    STATE(38), 1,
      sym_cardinality,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [144] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(36), 5,
      sym_symbol,
      anon_sym_if,
      anon_sym_DOT,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [156] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(38), 4,
      anon_sym_if,
      anon_sym_DOT,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [167] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(40), 4,
      ts_builtin_sym_end,
      sym_symbol,
      anon_sym_decide,
      anon_sym_constrain,
  [178] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(42), 4,
      ts_builtin_sym_end,
      sym_symbol,
      anon_sym_decide,
      anon_sym_constrain,
  [189] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(44), 4,
      ts_builtin_sym_end,
      sym_symbol,
      anon_sym_decide,
      anon_sym_constrain,
  [200] = 4,
    ACTIONS(46), 1,
      sym_only,
    ACTIONS(48), 1,
      anon_sym_at,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    STATE(40), 2,
      sym_at_most,
      sym_at_least,
  [215] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(50), 4,
      anon_sym_if,
      anon_sym_DOT,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [226] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(52), 4,
      anon_sym_if,
      anon_sym_DOT,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [237] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(54), 4,
      ts_builtin_sym_end,
      sym_symbol,
      anon_sym_decide,
      anon_sym_constrain,
  [248] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(56), 4,
      ts_builtin_sym_end,
      sym_symbol,
      anon_sym_decide,
      anon_sym_constrain,
  [259] = 4,
    ACTIONS(58), 1,
      anon_sym_COMMA,
    ACTIONS(60), 1,
      anon_sym_RPAREN,
    STATE(31), 1,
      aux_sym_tuple_repeat1,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [273] = 4,
    ACTIONS(7), 1,
      sym_symbol,
    STATE(11), 1,
      sym_rule,
    STATE(42), 1,
      sym_atom,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [287] = 4,
    ACTIONS(58), 1,
      anon_sym_COMMA,
    ACTIONS(62), 1,
      anon_sym_RPAREN,
    STATE(18), 1,
      aux_sym_tuple_repeat1,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [301] = 4,
    ACTIONS(64), 1,
      anon_sym_COMMA,
    ACTIONS(66), 1,
      anon_sym_RPAREN,
    STATE(28), 1,
      aux_sym_captures_repeat1,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [315] = 3,
    STATE(29), 1,
      sym_integer,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(15), 2,
      anon_sym_0,
      aux_sym_integer_token1,
  [327] = 4,
    ACTIONS(7), 1,
      sym_symbol,
    STATE(24), 1,
      sym_atom,
    STATE(53), 1,
      sym__rule_body,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [341] = 4,
    ACTIONS(68), 1,
      anon_sym_DOT,
    ACTIONS(70), 1,
      anon_sym_COMMA,
    STATE(30), 1,
      aux_sym__rule_body_repeat1,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [355] = 4,
    ACTIONS(34), 1,
      anon_sym_cardinality,
    STATE(22), 1,
      sym_constraint_kind,
    STATE(38), 1,
      sym_cardinality,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [369] = 3,
    STATE(23), 1,
      sym_integer,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(15), 2,
      anon_sym_0,
      aux_sym_integer_token1,
  [381] = 3,
    STATE(43), 1,
      sym__ident,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(72), 2,
      sym_variable,
      sym_symbol,
  [393] = 4,
    ACTIONS(64), 1,
      anon_sym_COMMA,
    ACTIONS(74), 1,
      anon_sym_RPAREN,
    STATE(32), 1,
      aux_sym_captures_repeat1,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [407] = 4,
    ACTIONS(7), 1,
      sym_symbol,
    STATE(24), 1,
      sym_atom,
    STATE(51), 1,
      sym__rule_body,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [421] = 4,
    ACTIONS(70), 1,
      anon_sym_COMMA,
    ACTIONS(76), 1,
      anon_sym_DOT,
    STATE(35), 1,
      aux_sym__rule_body_repeat1,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [435] = 4,
    ACTIONS(78), 1,
      anon_sym_COMMA,
    ACTIONS(81), 1,
      anon_sym_RPAREN,
    STATE(31), 1,
      aux_sym_tuple_repeat1,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [449] = 4,
    ACTIONS(83), 1,
      anon_sym_COMMA,
    ACTIONS(86), 1,
      anon_sym_RPAREN,
    STATE(32), 1,
      aux_sym_captures_repeat1,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [463] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(88), 3,
      anon_sym_if,
      anon_sym_DOT,
      anon_sym_COMMA,
  [473] = 4,
    ACTIONS(7), 1,
      sym_symbol,
    STATE(24), 1,
      sym_atom,
    STATE(49), 1,
      sym__rule_body,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [487] = 4,
    ACTIONS(90), 1,
      anon_sym_DOT,
    ACTIONS(92), 1,
      anon_sym_COMMA,
    STATE(35), 1,
      aux_sym__rule_body_repeat1,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [501] = 3,
    STATE(21), 1,
      sym__ident,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(95), 2,
      sym_variable,
      sym_symbol,
  [513] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(97), 2,
      anon_sym_0,
      aux_sym_integer_token1,
  [522] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(99), 2,
      anon_sym_0,
      aux_sym_integer_token1,
  [531] = 3,
    ACTIONS(7), 1,
      sym_symbol,
    STATE(46), 1,
      sym_atom,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [542] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(101), 2,
      anon_sym_0,
      aux_sym_integer_token1,
  [551] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(81), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [560] = 3,
    ACTIONS(103), 1,
      anon_sym_if,
    ACTIONS(105), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [571] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(86), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [580] = 3,
    ACTIONS(107), 1,
      anon_sym_most,
    ACTIONS(109), 1,
      anon_sym_least,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [591] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(111), 2,
      anon_sym_0,
      aux_sym_integer_token1,
  [600] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(90), 2,
      anon_sym_DOT,
      anon_sym_COMMA,
  [609] = 2,
    ACTIONS(113), 1,
      anon_sym_cardinality,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [617] = 2,
    ACTIONS(115), 1,
      ts_builtin_sym_end,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [625] = 2,
    ACTIONS(117), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [633] = 2,
    ACTIONS(119), 1,
      anon_sym_cardinality,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [641] = 2,
    ACTIONS(121), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [649] = 2,
    ACTIONS(123), 1,
      anon_sym_to,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [657] = 2,
    ACTIONS(125), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 24,
  [SMALL_STATE(4)] = 50,
  [SMALL_STATE(5)] = 74,
  [SMALL_STATE(6)] = 100,
  [SMALL_STATE(7)] = 124,
  [SMALL_STATE(8)] = 144,
  [SMALL_STATE(9)] = 156,
  [SMALL_STATE(10)] = 167,
  [SMALL_STATE(11)] = 178,
  [SMALL_STATE(12)] = 189,
  [SMALL_STATE(13)] = 200,
  [SMALL_STATE(14)] = 215,
  [SMALL_STATE(15)] = 226,
  [SMALL_STATE(16)] = 237,
  [SMALL_STATE(17)] = 248,
  [SMALL_STATE(18)] = 259,
  [SMALL_STATE(19)] = 273,
  [SMALL_STATE(20)] = 287,
  [SMALL_STATE(21)] = 301,
  [SMALL_STATE(22)] = 315,
  [SMALL_STATE(23)] = 327,
  [SMALL_STATE(24)] = 341,
  [SMALL_STATE(25)] = 355,
  [SMALL_STATE(26)] = 369,
  [SMALL_STATE(27)] = 381,
  [SMALL_STATE(28)] = 393,
  [SMALL_STATE(29)] = 407,
  [SMALL_STATE(30)] = 421,
  [SMALL_STATE(31)] = 435,
  [SMALL_STATE(32)] = 449,
  [SMALL_STATE(33)] = 463,
  [SMALL_STATE(34)] = 473,
  [SMALL_STATE(35)] = 487,
  [SMALL_STATE(36)] = 501,
  [SMALL_STATE(37)] = 513,
  [SMALL_STATE(38)] = 522,
  [SMALL_STATE(39)] = 531,
  [SMALL_STATE(40)] = 542,
  [SMALL_STATE(41)] = 551,
  [SMALL_STATE(42)] = 560,
  [SMALL_STATE(43)] = 571,
  [SMALL_STATE(44)] = 580,
  [SMALL_STATE(45)] = 591,
  [SMALL_STATE(46)] = 600,
  [SMALL_STATE(47)] = 609,
  [SMALL_STATE(48)] = 617,
  [SMALL_STATE(49)] = 625,
  [SMALL_STATE(50)] = 633,
  [SMALL_STATE(51)] = 641,
  [SMALL_STATE(52)] = 649,
  [SMALL_STATE(53)] = 657,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_file, 0, 0, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [19] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_file, 1, 0, 0),
  [21] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_file_repeat1, 2, 0, 0),
  [23] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_file_repeat1, 2, 0, 0), SHIFT_REPEAT(2),
  [26] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_file_repeat1, 2, 0, 0), SHIFT_REPEAT(19),
  [29] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_file_repeat1, 2, 0, 0), SHIFT_REPEAT(7),
  [32] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [34] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [36] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_integer, 1, 0, 0),
  [38] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tuple, 4, 0, 0),
  [40] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint, 6, 0, 5),
  [42] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_decision, 2, 0, 0),
  [44] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_rule, 2, 0, 2),
  [46] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [48] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [50] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tuple, 3, 0, 0),
  [52] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pattern, 1, 0, 0),
  [54] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_rule, 4, 0, 3),
  [56] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint, 5, 0, 4),
  [58] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [60] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [62] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [64] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [66] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [68] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__rule_body, 1, 0, 0),
  [70] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [72] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [74] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [76] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__rule_body, 2, 0, 0),
  [78] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_tuple_repeat1, 2, 0, 0), SHIFT_REPEAT(6),
  [81] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_tuple_repeat1, 2, 0, 0),
  [83] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_captures_repeat1, 2, 0, 0), SHIFT_REPEAT(27),
  [86] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_captures_repeat1, 2, 0, 0),
  [88] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_atom, 2, 0, 1),
  [90] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__rule_body_repeat1, 2, 0, 0),
  [92] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__rule_body_repeat1, 2, 0, 0), SHIFT_REPEAT(39),
  [95] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [97] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_at_most, 2, 0, 0),
  [99] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint_kind, 1, 0, 0),
  [101] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_cardinality, 3, 0, 0),
  [103] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [105] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [107] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [109] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [111] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_at_least, 2, 0, 0),
  [113] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_captures, 3, 0, 0),
  [115] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [117] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [119] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_captures, 4, 0, 0),
  [121] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [123] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [125] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef TREE_SITTER_HIDE_SYMBOLS
#define TS_PUBLIC
#elif defined(_WIN32)
#define TS_PUBLIC __declspec(dllexport)
#else
#define TS_PUBLIC __attribute__((visibility("default")))
#endif

TS_PUBLIC const TSLanguage *tree_sitter_fulcrum(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .field_names = ts_field_names,
    .field_map_slices = ts_field_map_slices,
    .field_map_entries = ts_field_map_entries,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
