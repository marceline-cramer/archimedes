#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 84
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 45
#define ALIAS_COUNT 0
#define TOKEN_COUNT 23
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 6
#define MAX_ALIAS_SEQUENCE_LENGTH 9
#define PRODUCTION_ID_COUNT 8

enum ts_symbol_identifiers {
  sym__whitespace = 1,
  sym_comment = 2,
  sym_variable = 3,
  sym_symbol = 4,
  anon_sym_0 = 5,
  aux_sym_integer_token1 = 6,
  anon_sym_import = 7,
  anon_sym_DOT = 8,
  anon_sym_LPAREN = 9,
  anon_sym_COMMA = 10,
  anon_sym_RPAREN = 11,
  anon_sym_decide = 12,
  anon_sym_if = 13,
  anon_sym_constrain = 14,
  anon_sym_soft = 15,
  sym_uniform = 16,
  anon_sym_cardinality = 17,
  anon_sym_to = 18,
  sym_only = 19,
  anon_sym_at = 20,
  anon_sym_most = 21,
  anon_sym_least = 22,
  sym_file = 23,
  sym__ident = 24,
  sym_integer = 25,
  sym_import = 26,
  sym_decision = 27,
  sym_rule = 28,
  sym__rule_body = 29,
  sym_constraint = 30,
  sym_captures = 31,
  sym_constraint_kind = 32,
  sym_cardinality = 33,
  sym_at_most = 34,
  sym_at_least = 35,
  sym_atom = 36,
  sym_pattern = 37,
  sym_tuple = 38,
  aux_sym_file_repeat1 = 39,
  aux_sym_import_repeat1 = 40,
  aux_sym_import_repeat2 = 41,
  aux_sym__rule_body_repeat1 = 42,
  aux_sym_captures_repeat1 = 43,
  aux_sym_tuple_repeat1 = 44,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym__whitespace] = "_whitespace",
  [sym_comment] = "comment",
  [sym_variable] = "variable",
  [sym_symbol] = "symbol",
  [anon_sym_0] = "0",
  [aux_sym_integer_token1] = "integer_token1",
  [anon_sym_import] = "import",
  [anon_sym_DOT] = ".",
  [anon_sym_LPAREN] = "(",
  [anon_sym_COMMA] = ",",
  [anon_sym_RPAREN] = ")",
  [anon_sym_decide] = "decide",
  [anon_sym_if] = "if",
  [anon_sym_constrain] = "constrain",
  [anon_sym_soft] = "soft",
  [sym_uniform] = "uniform",
  [anon_sym_cardinality] = "cardinality",
  [anon_sym_to] = "to",
  [sym_only] = "only",
  [anon_sym_at] = "at",
  [anon_sym_most] = "most",
  [anon_sym_least] = "least",
  [sym_file] = "file",
  [sym__ident] = "_ident",
  [sym_integer] = "integer",
  [sym_import] = "import",
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
  [aux_sym_import_repeat1] = "import_repeat1",
  [aux_sym_import_repeat2] = "import_repeat2",
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
  [anon_sym_import] = anon_sym_import,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_decide] = anon_sym_decide,
  [anon_sym_if] = anon_sym_if,
  [anon_sym_constrain] = anon_sym_constrain,
  [anon_sym_soft] = anon_sym_soft,
  [sym_uniform] = sym_uniform,
  [anon_sym_cardinality] = anon_sym_cardinality,
  [anon_sym_to] = anon_sym_to,
  [sym_only] = sym_only,
  [anon_sym_at] = anon_sym_at,
  [anon_sym_most] = anon_sym_most,
  [anon_sym_least] = anon_sym_least,
  [sym_file] = sym_file,
  [sym__ident] = sym__ident,
  [sym_integer] = sym_integer,
  [sym_import] = sym_import,
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
  [aux_sym_import_repeat1] = aux_sym_import_repeat1,
  [aux_sym_import_repeat2] = aux_sym_import_repeat2,
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
  [anon_sym_import] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
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
  [anon_sym_constrain] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_soft] = {
    .visible = true,
    .named = false,
  },
  [sym_uniform] = {
    .visible = true,
    .named = true,
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
  [sym_import] = {
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
  [aux_sym_import_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_import_repeat2] = {
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
  field_captures = 2,
  field_head = 3,
  field_kind = 4,
  field_relation = 5,
  field_soft = 6,
};

static const char * const ts_field_names[] = {
  [0] = NULL,
  [field_body] = "body",
  [field_captures] = "captures",
  [field_head] = "head",
  [field_kind] = "kind",
  [field_relation] = "relation",
  [field_soft] = "soft",
};

static const TSFieldMapSlice ts_field_map_slices[PRODUCTION_ID_COUNT] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 1},
  [3] = {.index = 2, .length = 2},
  [4] = {.index = 4, .length = 3},
  [5] = {.index = 7, .length = 3},
  [6] = {.index = 10, .length = 6},
  [7] = {.index = 16, .length = 7},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_relation, 0},
  [1] =
    {field_head, 0},
  [2] =
    {field_body, 2},
    {field_kind, 1},
  [4] =
    {field_body, 1},
    {field_body, 2},
    {field_head, 0},
  [7] =
    {field_body, 3},
    {field_captures, 1},
    {field_kind, 2},
  [10] =
    {field_body, 6},
    {field_kind, 5},
    {field_soft, 1},
    {field_soft, 2},
    {field_soft, 3},
    {field_soft, 4},
  [16] =
    {field_body, 7},
    {field_captures, 5},
    {field_kind, 6},
    {field_soft, 1},
    {field_soft, 2},
    {field_soft, 3},
    {field_soft, 4},
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
  [54] = 54,
  [55] = 55,
  [56] = 56,
  [57] = 57,
  [58] = 58,
  [59] = 59,
  [60] = 60,
  [61] = 61,
  [62] = 62,
  [63] = 63,
  [64] = 64,
  [65] = 65,
  [66] = 66,
  [67] = 67,
  [68] = 68,
  [69] = 69,
  [70] = 70,
  [71] = 71,
  [72] = 72,
  [73] = 73,
  [74] = 74,
  [75] = 75,
  [76] = 76,
  [77] = 77,
  [78] = 78,
  [79] = 79,
  [80] = 80,
  [81] = 81,
  [82] = 82,
  [83] = 83,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(52);
      ADVANCE_MAP(
        '(', 61,
        ')', 63,
        ',', 62,
        '-', 51,
        '.', 60,
        '0', 57,
        ';', 1,
        'a', 42,
        'c', 3,
        'd', 10,
        'i', 13,
        'l', 11,
        'm', 29,
        'o', 24,
        's', 30,
        't', 31,
        'u', 25,
        '\t', 53,
        '\n', 53,
        '\r', 53,
        ' ', 53,
      );
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(58);
      if (('A' <= lookahead && lookahead <= 'Z')) ADVANCE(56);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(54);
      if (lookahead != 0) ADVANCE(1);
      END_STATE();
    case 2:
      if (lookahead == '(') ADVANCE(61);
      if (lookahead == '-') ADVANCE(51);
      if (lookahead == '0') ADVANCE(57);
      if (lookahead == ';') ADVANCE(1);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(53);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(58);
      if (('A' <= lookahead && lookahead <= 'Z')) ADVANCE(56);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(55);
      END_STATE();
    case 3:
      if (lookahead == 'a') ADVANCE(35);
      if (lookahead == 'o') ADVANCE(28);
      END_STATE();
    case 4:
      if (lookahead == 'a') ADVANCE(22);
      END_STATE();
    case 5:
      if (lookahead == 'a') ADVANCE(19);
      END_STATE();
    case 6:
      if (lookahead == 'a') ADVANCE(41);
      END_STATE();
    case 7:
      if (lookahead == 'c') ADVANCE(17);
      END_STATE();
    case 8:
      if (lookahead == 'd') ADVANCE(12);
      END_STATE();
    case 9:
      if (lookahead == 'd') ADVANCE(18);
      END_STATE();
    case 10:
      if (lookahead == 'e') ADVANCE(7);
      END_STATE();
    case 11:
      if (lookahead == 'e') ADVANCE(6);
      END_STATE();
    case 12:
      if (lookahead == 'e') ADVANCE(64);
      END_STATE();
    case 13:
      if (lookahead == 'f') ADVANCE(65);
      if (lookahead == 'm') ADVANCE(34);
      END_STATE();
    case 14:
      if (lookahead == 'f') ADVANCE(44);
      END_STATE();
    case 15:
      if (lookahead == 'f') ADVANCE(33);
      END_STATE();
    case 16:
      if (lookahead == 'i') ADVANCE(15);
      END_STATE();
    case 17:
      if (lookahead == 'i') ADVANCE(8);
      END_STATE();
    case 18:
      if (lookahead == 'i') ADVANCE(27);
      END_STATE();
    case 19:
      if (lookahead == 'i') ADVANCE(26);
      END_STATE();
    case 20:
      if (lookahead == 'i') ADVANCE(47);
      END_STATE();
    case 21:
      if (lookahead == 'l') ADVANCE(49);
      END_STATE();
    case 22:
      if (lookahead == 'l') ADVANCE(20);
      END_STATE();
    case 23:
      if (lookahead == 'm') ADVANCE(68);
      END_STATE();
    case 24:
      if (lookahead == 'n') ADVANCE(21);
      END_STATE();
    case 25:
      if (lookahead == 'n') ADVANCE(16);
      END_STATE();
    case 26:
      if (lookahead == 'n') ADVANCE(66);
      END_STATE();
    case 27:
      if (lookahead == 'n') ADVANCE(4);
      END_STATE();
    case 28:
      if (lookahead == 'n') ADVANCE(40);
      END_STATE();
    case 29:
      if (lookahead == 'o') ADVANCE(39);
      END_STATE();
    case 30:
      if (lookahead == 'o') ADVANCE(14);
      END_STATE();
    case 31:
      if (lookahead == 'o') ADVANCE(70);
      END_STATE();
    case 32:
      if (lookahead == 'o') ADVANCE(38);
      END_STATE();
    case 33:
      if (lookahead == 'o') ADVANCE(36);
      END_STATE();
    case 34:
      if (lookahead == 'p') ADVANCE(32);
      END_STATE();
    case 35:
      if (lookahead == 'r') ADVANCE(9);
      END_STATE();
    case 36:
      if (lookahead == 'r') ADVANCE(23);
      END_STATE();
    case 37:
      if (lookahead == 'r') ADVANCE(5);
      END_STATE();
    case 38:
      if (lookahead == 'r') ADVANCE(46);
      END_STATE();
    case 39:
      if (lookahead == 's') ADVANCE(43);
      END_STATE();
    case 40:
      if (lookahead == 's') ADVANCE(48);
      END_STATE();
    case 41:
      if (lookahead == 's') ADVANCE(45);
      END_STATE();
    case 42:
      if (lookahead == 't') ADVANCE(72);
      END_STATE();
    case 43:
      if (lookahead == 't') ADVANCE(73);
      END_STATE();
    case 44:
      if (lookahead == 't') ADVANCE(67);
      END_STATE();
    case 45:
      if (lookahead == 't') ADVANCE(74);
      END_STATE();
    case 46:
      if (lookahead == 't') ADVANCE(59);
      END_STATE();
    case 47:
      if (lookahead == 't') ADVANCE(50);
      END_STATE();
    case 48:
      if (lookahead == 't') ADVANCE(37);
      END_STATE();
    case 49:
      if (lookahead == 'y') ADVANCE(71);
      END_STATE();
    case 50:
      if (lookahead == 'y') ADVANCE(69);
      END_STATE();
    case 51:
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(58);
      END_STATE();
    case 52:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 53:
      ACCEPT_TOKEN(sym__whitespace);
      END_STATE();
    case 54:
      ACCEPT_TOKEN(sym_comment);
      END_STATE();
    case 55:
      ACCEPT_TOKEN(sym_variable);
      if (('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(55);
      END_STATE();
    case 56:
      ACCEPT_TOKEN(sym_symbol);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(56);
      END_STATE();
    case 57:
      ACCEPT_TOKEN(anon_sym_0);
      END_STATE();
    case 58:
      ACCEPT_TOKEN(aux_sym_integer_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(58);
      END_STATE();
    case 59:
      ACCEPT_TOKEN(anon_sym_import);
      END_STATE();
    case 60:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 61:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 62:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 63:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 64:
      ACCEPT_TOKEN(anon_sym_decide);
      END_STATE();
    case 65:
      ACCEPT_TOKEN(anon_sym_if);
      END_STATE();
    case 66:
      ACCEPT_TOKEN(anon_sym_constrain);
      END_STATE();
    case 67:
      ACCEPT_TOKEN(anon_sym_soft);
      END_STATE();
    case 68:
      ACCEPT_TOKEN(sym_uniform);
      END_STATE();
    case 69:
      ACCEPT_TOKEN(anon_sym_cardinality);
      END_STATE();
    case 70:
      ACCEPT_TOKEN(anon_sym_to);
      END_STATE();
    case 71:
      ACCEPT_TOKEN(sym_only);
      END_STATE();
    case 72:
      ACCEPT_TOKEN(anon_sym_at);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(anon_sym_most);
      END_STATE();
    case 74:
      ACCEPT_TOKEN(anon_sym_least);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 2},
  [5] = {.lex_state = 2},
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
  [27] = {.lex_state = 0},
  [28] = {.lex_state = 0},
  [29] = {.lex_state = 0},
  [30] = {.lex_state = 2},
  [31] = {.lex_state = 0},
  [32] = {.lex_state = 2},
  [33] = {.lex_state = 0},
  [34] = {.lex_state = 0},
  [35] = {.lex_state = 0},
  [36] = {.lex_state = 0},
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
  [54] = {.lex_state = 0},
  [55] = {.lex_state = 0},
  [56] = {.lex_state = 0},
  [57] = {.lex_state = 0},
  [58] = {.lex_state = 0},
  [59] = {.lex_state = 0},
  [60] = {.lex_state = 0},
  [61] = {.lex_state = 0},
  [62] = {.lex_state = 0},
  [63] = {.lex_state = 0},
  [64] = {.lex_state = 0},
  [65] = {.lex_state = 0},
  [66] = {.lex_state = 0},
  [67] = {.lex_state = 0},
  [68] = {.lex_state = 0},
  [69] = {.lex_state = 0},
  [70] = {.lex_state = 0},
  [71] = {.lex_state = 0},
  [72] = {.lex_state = 0},
  [73] = {.lex_state = 0},
  [74] = {.lex_state = 0},
  [75] = {.lex_state = 0},
  [76] = {.lex_state = 0},
  [77] = {.lex_state = 0},
  [78] = {.lex_state = 0},
  [79] = {.lex_state = 0},
  [80] = {.lex_state = 0},
  [81] = {.lex_state = 0},
  [82] = {.lex_state = 0},
  [83] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym__whitespace] = ACTIONS(3),
    [sym_comment] = ACTIONS(3),
    [sym_symbol] = ACTIONS(1),
    [anon_sym_0] = ACTIONS(1),
    [aux_sym_integer_token1] = ACTIONS(1),
    [anon_sym_import] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_decide] = ACTIONS(1),
    [anon_sym_if] = ACTIONS(1),
    [anon_sym_constrain] = ACTIONS(1),
    [anon_sym_soft] = ACTIONS(1),
    [sym_uniform] = ACTIONS(1),
    [anon_sym_cardinality] = ACTIONS(1),
    [anon_sym_to] = ACTIONS(1),
    [sym_only] = ACTIONS(1),
    [anon_sym_at] = ACTIONS(1),
    [anon_sym_most] = ACTIONS(1),
    [anon_sym_least] = ACTIONS(1),
  },
  [1] = {
    [sym_file] = STATE(71),
    [sym_import] = STATE(3),
    [sym_decision] = STATE(3),
    [sym_rule] = STATE(3),
    [sym_constraint] = STATE(3),
    [sym_atom] = STATE(55),
    [aux_sym_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(5),
    [sym__whitespace] = ACTIONS(3),
    [sym_comment] = ACTIONS(3),
    [sym_symbol] = ACTIONS(7),
    [anon_sym_import] = ACTIONS(9),
    [anon_sym_decide] = ACTIONS(11),
    [anon_sym_constrain] = ACTIONS(13),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 8,
    ACTIONS(15), 1,
      ts_builtin_sym_end,
    ACTIONS(17), 1,
      sym_symbol,
    ACTIONS(20), 1,
      anon_sym_import,
    ACTIONS(23), 1,
      anon_sym_decide,
    ACTIONS(26), 1,
      anon_sym_constrain,
    STATE(55), 1,
      sym_atom,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    STATE(2), 5,
      sym_import,
      sym_decision,
      sym_rule,
      sym_constraint,
      aux_sym_file_repeat1,
  [30] = 8,
    ACTIONS(7), 1,
      sym_symbol,
    ACTIONS(9), 1,
      anon_sym_import,
    ACTIONS(11), 1,
      anon_sym_decide,
    ACTIONS(13), 1,
      anon_sym_constrain,
    ACTIONS(29), 1,
      ts_builtin_sym_end,
    STATE(55), 1,
      sym_atom,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    STATE(2), 5,
      sym_import,
      sym_decision,
      sym_rule,
      sym_constraint,
      aux_sym_file_repeat1,
  [60] = 6,
    ACTIONS(35), 1,
      anon_sym_LPAREN,
    STATE(49), 1,
      sym_pattern,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(31), 2,
      sym_variable,
      sym_symbol,
    ACTIONS(33), 2,
      anon_sym_0,
      aux_sym_integer_token1,
    STATE(21), 3,
      sym__ident,
      sym_integer,
      sym_tuple,
  [84] = 6,
    ACTIONS(35), 1,
      anon_sym_LPAREN,
    STATE(45), 1,
      sym_pattern,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(31), 2,
      sym_variable,
      sym_symbol,
    ACTIONS(33), 2,
      anon_sym_0,
      aux_sym_integer_token1,
    STATE(21), 3,
      sym__ident,
      sym_integer,
      sym_tuple,
  [108] = 6,
    ACTIONS(35), 1,
      anon_sym_LPAREN,
    STATE(51), 1,
      sym_pattern,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(31), 2,
      sym_variable,
      sym_symbol,
    ACTIONS(33), 2,
      anon_sym_0,
      aux_sym_integer_token1,
    STATE(21), 3,
      sym__ident,
      sym_integer,
      sym_tuple,
  [132] = 8,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(39), 1,
      anon_sym_soft,
    ACTIONS(41), 1,
      sym_uniform,
    ACTIONS(43), 1,
      anon_sym_cardinality,
    STATE(20), 1,
      sym_captures,
    STATE(39), 1,
      sym_constraint_kind,
    STATE(83), 1,
      sym_cardinality,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [158] = 7,
    ACTIONS(37), 1,
      anon_sym_LPAREN,
    ACTIONS(41), 1,
      sym_uniform,
    ACTIONS(43), 1,
      anon_sym_cardinality,
    STATE(24), 1,
      sym_captures,
    STATE(44), 1,
      sym_constraint_kind,
    STATE(83), 1,
      sym_cardinality,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [181] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(45), 5,
      ts_builtin_sym_end,
      sym_symbol,
      anon_sym_import,
      anon_sym_decide,
      anon_sym_constrain,
  [193] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(47), 5,
      sym_symbol,
      anon_sym_DOT,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_if,
  [205] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(49), 5,
      ts_builtin_sym_end,
      sym_symbol,
      anon_sym_import,
      anon_sym_decide,
      anon_sym_constrain,
  [217] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(51), 5,
      ts_builtin_sym_end,
      sym_symbol,
      anon_sym_import,
      anon_sym_decide,
      anon_sym_constrain,
  [229] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(53), 5,
      ts_builtin_sym_end,
      sym_symbol,
      anon_sym_import,
      anon_sym_decide,
      anon_sym_constrain,
  [241] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(55), 5,
      ts_builtin_sym_end,
      sym_symbol,
      anon_sym_import,
      anon_sym_decide,
      anon_sym_constrain,
  [253] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(57), 5,
      ts_builtin_sym_end,
      sym_symbol,
      anon_sym_import,
      anon_sym_decide,
      anon_sym_constrain,
  [265] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(59), 5,
      ts_builtin_sym_end,
      sym_symbol,
      anon_sym_import,
      anon_sym_decide,
      anon_sym_constrain,
  [277] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(61), 5,
      ts_builtin_sym_end,
      sym_symbol,
      anon_sym_import,
      anon_sym_decide,
      anon_sym_constrain,
  [289] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(63), 5,
      ts_builtin_sym_end,
      sym_symbol,
      anon_sym_import,
      anon_sym_decide,
      anon_sym_constrain,
  [301] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(65), 5,
      ts_builtin_sym_end,
      sym_symbol,
      anon_sym_import,
      anon_sym_decide,
      anon_sym_constrain,
  [313] = 5,
    ACTIONS(41), 1,
      sym_uniform,
    ACTIONS(43), 1,
      anon_sym_cardinality,
    STATE(28), 1,
      sym_constraint_kind,
    STATE(83), 1,
      sym_cardinality,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [330] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(67), 4,
      anon_sym_DOT,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_if,
  [341] = 4,
    ACTIONS(69), 1,
      sym_only,
    ACTIONS(71), 1,
      anon_sym_at,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    STATE(34), 2,
      sym_at_most,
      sym_at_least,
  [356] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(73), 4,
      anon_sym_DOT,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_if,
  [367] = 5,
    ACTIONS(41), 1,
      sym_uniform,
    ACTIONS(43), 1,
      anon_sym_cardinality,
    STATE(48), 1,
      sym_constraint_kind,
    STATE(83), 1,
      sym_cardinality,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [384] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(75), 4,
      anon_sym_DOT,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_if,
  [395] = 4,
    ACTIONS(7), 1,
      sym_symbol,
    STATE(9), 1,
      sym_rule,
    STATE(55), 1,
      sym_atom,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [409] = 3,
    STATE(72), 1,
      sym_integer,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(33), 2,
      anon_sym_0,
      aux_sym_integer_token1,
  [421] = 4,
    ACTIONS(7), 1,
      sym_symbol,
    STATE(29), 1,
      sym_atom,
    STATE(76), 1,
      sym__rule_body,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [435] = 4,
    ACTIONS(77), 1,
      anon_sym_DOT,
    ACTIONS(79), 1,
      anon_sym_COMMA,
    STATE(35), 1,
      aux_sym__rule_body_repeat1,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [449] = 3,
    STATE(37), 1,
      sym__ident,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(81), 2,
      sym_variable,
      sym_symbol,
  [461] = 4,
    ACTIONS(83), 1,
      anon_sym_COMMA,
    ACTIONS(85), 1,
      anon_sym_RPAREN,
    STATE(36), 1,
      aux_sym_tuple_repeat1,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [475] = 3,
    STATE(52), 1,
      sym__ident,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(87), 2,
      sym_variable,
      sym_symbol,
  [487] = 4,
    ACTIONS(89), 1,
      anon_sym_COMMA,
    ACTIONS(91), 1,
      anon_sym_RPAREN,
    STATE(38), 1,
      aux_sym_captures_repeat1,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [501] = 3,
    STATE(74), 1,
      sym_integer,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(33), 2,
      anon_sym_0,
      aux_sym_integer_token1,
  [513] = 4,
    ACTIONS(79), 1,
      anon_sym_COMMA,
    ACTIONS(93), 1,
      anon_sym_DOT,
    STATE(40), 1,
      aux_sym__rule_body_repeat1,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [527] = 4,
    ACTIONS(95), 1,
      anon_sym_COMMA,
    ACTIONS(98), 1,
      anon_sym_RPAREN,
    STATE(36), 1,
      aux_sym_tuple_repeat1,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [541] = 4,
    ACTIONS(89), 1,
      anon_sym_COMMA,
    ACTIONS(100), 1,
      anon_sym_RPAREN,
    STATE(33), 1,
      aux_sym_captures_repeat1,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [555] = 4,
    ACTIONS(102), 1,
      anon_sym_COMMA,
    ACTIONS(105), 1,
      anon_sym_RPAREN,
    STATE(38), 1,
      aux_sym_captures_repeat1,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [569] = 4,
    ACTIONS(7), 1,
      sym_symbol,
    STATE(29), 1,
      sym_atom,
    STATE(73), 1,
      sym__rule_body,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [583] = 4,
    ACTIONS(107), 1,
      anon_sym_DOT,
    ACTIONS(109), 1,
      anon_sym_COMMA,
    STATE(40), 1,
      aux_sym__rule_body_repeat1,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [597] = 4,
    ACTIONS(112), 1,
      anon_sym_COMMA,
    ACTIONS(114), 1,
      anon_sym_RPAREN,
    STATE(46), 1,
      aux_sym_import_repeat2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [611] = 4,
    ACTIONS(112), 1,
      anon_sym_COMMA,
    ACTIONS(114), 1,
      anon_sym_RPAREN,
    STATE(47), 1,
      aux_sym_import_repeat2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [625] = 4,
    ACTIONS(7), 1,
      sym_symbol,
    STATE(29), 1,
      sym_atom,
    STATE(67), 1,
      sym__rule_body,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [639] = 4,
    ACTIONS(7), 1,
      sym_symbol,
    STATE(29), 1,
      sym_atom,
    STATE(79), 1,
      sym__rule_body,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [653] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(116), 3,
      anon_sym_DOT,
      anon_sym_COMMA,
      anon_sym_if,
  [663] = 4,
    ACTIONS(118), 1,
      anon_sym_COMMA,
    ACTIONS(121), 1,
      anon_sym_RPAREN,
    STATE(46), 1,
      aux_sym_import_repeat2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [677] = 4,
    ACTIONS(112), 1,
      anon_sym_COMMA,
    ACTIONS(123), 1,
      anon_sym_RPAREN,
    STATE(46), 1,
      aux_sym_import_repeat2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [691] = 4,
    ACTIONS(7), 1,
      sym_symbol,
    STATE(29), 1,
      sym_atom,
    STATE(81), 1,
      sym__rule_body,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [705] = 4,
    ACTIONS(83), 1,
      anon_sym_COMMA,
    ACTIONS(125), 1,
      anon_sym_RPAREN,
    STATE(31), 1,
      aux_sym_tuple_repeat1,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [719] = 4,
    ACTIONS(112), 1,
      anon_sym_COMMA,
    ACTIONS(127), 1,
      anon_sym_RPAREN,
    STATE(41), 1,
      aux_sym_import_repeat2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [733] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(98), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [742] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(105), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [751] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(129), 2,
      sym_uniform,
      anon_sym_cardinality,
  [760] = 3,
    ACTIONS(131), 1,
      sym_symbol,
    ACTIONS(133), 1,
      anon_sym_LPAREN,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [771] = 3,
    ACTIONS(135), 1,
      anon_sym_DOT,
    ACTIONS(137), 1,
      anon_sym_if,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [782] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(139), 2,
      anon_sym_0,
      aux_sym_integer_token1,
  [791] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(141), 2,
      anon_sym_0,
      aux_sym_integer_token1,
  [800] = 3,
    ACTIONS(143), 1,
      anon_sym_DOT,
    STATE(58), 1,
      aux_sym_import_repeat1,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [811] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(107), 2,
      anon_sym_DOT,
      anon_sym_COMMA,
  [820] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(146), 2,
      sym_uniform,
      anon_sym_cardinality,
  [829] = 2,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(121), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [838] = 3,
    ACTIONS(148), 1,
      anon_sym_most,
    ACTIONS(150), 1,
      anon_sym_least,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [849] = 3,
    ACTIONS(7), 1,
      sym_symbol,
    STATE(59), 1,
      sym_atom,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [860] = 3,
    ACTIONS(152), 1,
      anon_sym_DOT,
    STATE(66), 1,
      aux_sym_import_repeat1,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [871] = 3,
    ACTIONS(131), 1,
      sym_symbol,
    ACTIONS(154), 1,
      anon_sym_LPAREN,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [882] = 3,
    ACTIONS(156), 1,
      anon_sym_DOT,
    STATE(58), 1,
      aux_sym_import_repeat1,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [893] = 2,
    ACTIONS(158), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [901] = 2,
    ACTIONS(160), 1,
      sym_symbol,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [909] = 2,
    ACTIONS(162), 1,
      sym_symbol,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [917] = 2,
    ACTIONS(164), 1,
      sym_symbol,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [925] = 2,
    ACTIONS(166), 1,
      ts_builtin_sym_end,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [933] = 2,
    ACTIONS(168), 1,
      anon_sym_RPAREN,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [941] = 2,
    ACTIONS(170), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [949] = 2,
    ACTIONS(172), 1,
      sym_symbol,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [957] = 2,
    ACTIONS(131), 1,
      sym_symbol,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [965] = 2,
    ACTIONS(174), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [973] = 2,
    ACTIONS(176), 1,
      anon_sym_LPAREN,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [981] = 2,
    ACTIONS(178), 1,
      anon_sym_to,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [989] = 2,
    ACTIONS(180), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [997] = 2,
    ACTIONS(182), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [1005] = 2,
    ACTIONS(184), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [1013] = 2,
    ACTIONS(186), 1,
      sym_symbol,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [1021] = 2,
    ACTIONS(188), 1,
      sym_symbol,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 30,
  [SMALL_STATE(4)] = 60,
  [SMALL_STATE(5)] = 84,
  [SMALL_STATE(6)] = 108,
  [SMALL_STATE(7)] = 132,
  [SMALL_STATE(8)] = 158,
  [SMALL_STATE(9)] = 181,
  [SMALL_STATE(10)] = 193,
  [SMALL_STATE(11)] = 205,
  [SMALL_STATE(12)] = 217,
  [SMALL_STATE(13)] = 229,
  [SMALL_STATE(14)] = 241,
  [SMALL_STATE(15)] = 253,
  [SMALL_STATE(16)] = 265,
  [SMALL_STATE(17)] = 277,
  [SMALL_STATE(18)] = 289,
  [SMALL_STATE(19)] = 301,
  [SMALL_STATE(20)] = 313,
  [SMALL_STATE(21)] = 330,
  [SMALL_STATE(22)] = 341,
  [SMALL_STATE(23)] = 356,
  [SMALL_STATE(24)] = 367,
  [SMALL_STATE(25)] = 384,
  [SMALL_STATE(26)] = 395,
  [SMALL_STATE(27)] = 409,
  [SMALL_STATE(28)] = 421,
  [SMALL_STATE(29)] = 435,
  [SMALL_STATE(30)] = 449,
  [SMALL_STATE(31)] = 461,
  [SMALL_STATE(32)] = 475,
  [SMALL_STATE(33)] = 487,
  [SMALL_STATE(34)] = 501,
  [SMALL_STATE(35)] = 513,
  [SMALL_STATE(36)] = 527,
  [SMALL_STATE(37)] = 541,
  [SMALL_STATE(38)] = 555,
  [SMALL_STATE(39)] = 569,
  [SMALL_STATE(40)] = 583,
  [SMALL_STATE(41)] = 597,
  [SMALL_STATE(42)] = 611,
  [SMALL_STATE(43)] = 625,
  [SMALL_STATE(44)] = 639,
  [SMALL_STATE(45)] = 653,
  [SMALL_STATE(46)] = 663,
  [SMALL_STATE(47)] = 677,
  [SMALL_STATE(48)] = 691,
  [SMALL_STATE(49)] = 705,
  [SMALL_STATE(50)] = 719,
  [SMALL_STATE(51)] = 733,
  [SMALL_STATE(52)] = 742,
  [SMALL_STATE(53)] = 751,
  [SMALL_STATE(54)] = 760,
  [SMALL_STATE(55)] = 771,
  [SMALL_STATE(56)] = 782,
  [SMALL_STATE(57)] = 791,
  [SMALL_STATE(58)] = 800,
  [SMALL_STATE(59)] = 811,
  [SMALL_STATE(60)] = 820,
  [SMALL_STATE(61)] = 829,
  [SMALL_STATE(62)] = 838,
  [SMALL_STATE(63)] = 849,
  [SMALL_STATE(64)] = 860,
  [SMALL_STATE(65)] = 871,
  [SMALL_STATE(66)] = 882,
  [SMALL_STATE(67)] = 893,
  [SMALL_STATE(68)] = 901,
  [SMALL_STATE(69)] = 909,
  [SMALL_STATE(70)] = 917,
  [SMALL_STATE(71)] = 925,
  [SMALL_STATE(72)] = 933,
  [SMALL_STATE(73)] = 941,
  [SMALL_STATE(74)] = 949,
  [SMALL_STATE(75)] = 957,
  [SMALL_STATE(76)] = 965,
  [SMALL_STATE(77)] = 973,
  [SMALL_STATE(78)] = 981,
  [SMALL_STATE(79)] = 989,
  [SMALL_STATE(80)] = 997,
  [SMALL_STATE(81)] = 1005,
  [SMALL_STATE(82)] = 1013,
  [SMALL_STATE(83)] = 1021,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_file, 0, 0, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [15] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_file_repeat1, 2, 0, 0),
  [17] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_file_repeat1, 2, 0, 0), SHIFT_REPEAT(5),
  [20] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_file_repeat1, 2, 0, 0), SHIFT_REPEAT(69),
  [23] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_file_repeat1, 2, 0, 0), SHIFT_REPEAT(26),
  [26] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_file_repeat1, 2, 0, 0), SHIFT_REPEAT(7),
  [29] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_file, 1, 0, 0),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(77),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [45] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_decision, 2, 0, 0),
  [47] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_integer, 1, 0, 0),
  [49] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_rule, 2, 0, 2),
  [51] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import, 8, 0, 0),
  [53] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_rule, 4, 0, 4),
  [55] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint, 8, 0, 6),
  [57] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint, 4, 0, 3),
  [59] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint, 5, 0, 5),
  [61] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import, 6, 0, 0),
  [63] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint, 9, 0, 7),
  [65] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_import, 7, 0, 0),
  [67] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pattern, 1, 0, 0),
  [69] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [71] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
  [73] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tuple, 3, 0, 0),
  [75] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_tuple, 4, 0, 0),
  [77] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__rule_body, 1, 0, 0),
  [79] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [81] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [83] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [85] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [87] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [89] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [91] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [93] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__rule_body, 2, 0, 0),
  [95] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_tuple_repeat1, 2, 0, 0), SHIFT_REPEAT(6),
  [98] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_tuple_repeat1, 2, 0, 0),
  [100] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [102] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_captures_repeat1, 2, 0, 0), SHIFT_REPEAT(32),
  [105] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_captures_repeat1, 2, 0, 0),
  [107] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__rule_body_repeat1, 2, 0, 0),
  [109] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__rule_body_repeat1, 2, 0, 0), SHIFT_REPEAT(63),
  [112] = {.entry = {.count = 1, .reusable = true}}, SHIFT(68),
  [114] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [116] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_atom, 2, 0, 1),
  [118] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_import_repeat2, 2, 0, 0), SHIFT_REPEAT(68),
  [121] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_import_repeat2, 2, 0, 0),
  [123] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [125] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [127] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [129] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_captures, 4, 0, 0),
  [131] = {.entry = {.count = 1, .reusable = true}}, SHIFT(80),
  [133] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [135] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [137] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [139] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_at_most, 2, 0, 0),
  [141] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_at_least, 2, 0, 0),
  [143] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_import_repeat1, 2, 0, 0), SHIFT_REPEAT(75),
  [146] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_captures, 3, 0, 0),
  [148] = {.entry = {.count = 1, .reusable = true}}, SHIFT(56),
  [150] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [152] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [154] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [156] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [158] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [160] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [162] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [164] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [166] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [168] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [170] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [172] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_cardinality, 4, 0, 0),
  [174] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [176] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [178] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [180] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [182] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_import_repeat1, 2, 0, 0),
  [184] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [186] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [188] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constraint_kind, 1, 0, 0),
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
