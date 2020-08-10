#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 11
#define STATE_COUNT 15
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 16
#define ALIAS_COUNT 0
#define TOKEN_COUNT 10
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 3

enum {
  sym_newline = 1,
  anon_sym_COMMA = 2,
  sym_range_start = 3,
  sym_range_end = 4,
  anon_sym_set = 5,
  anon_sym_wrap = 6,
  anon_sym_nowrap = 7,
  anon_sym_edit = 8,
  aux_sym_edit_token1 = 9,
  sym_s = 10,
  sym_cmd = 11,
  sym_range = 12,
  sym_set = 13,
  sym_config_param = 14,
  sym_edit = 15,
};

static const char *ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_newline] = "newline",
  [anon_sym_COMMA] = ",",
  [sym_range_start] = "range_start",
  [sym_range_end] = "range_end",
  [anon_sym_set] = "set",
  [anon_sym_wrap] = "wrap",
  [anon_sym_nowrap] = "nowrap",
  [anon_sym_edit] = "edit",
  [aux_sym_edit_token1] = "edit_token1",
  [sym_s] = "s",
  [sym_cmd] = "cmd",
  [sym_range] = "range",
  [sym_set] = "set",
  [sym_config_param] = "config_param",
  [sym_edit] = "edit",
};

static TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_newline] = sym_newline,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [sym_range_start] = sym_range_start,
  [sym_range_end] = sym_range_end,
  [anon_sym_set] = anon_sym_set,
  [anon_sym_wrap] = anon_sym_wrap,
  [anon_sym_nowrap] = anon_sym_nowrap,
  [anon_sym_edit] = anon_sym_edit,
  [aux_sym_edit_token1] = aux_sym_edit_token1,
  [sym_s] = sym_s,
  [sym_cmd] = sym_cmd,
  [sym_range] = sym_range,
  [sym_set] = sym_set,
  [sym_config_param] = sym_config_param,
  [sym_edit] = sym_edit,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_newline] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [sym_range_start] = {
    .visible = true,
    .named = true,
  },
  [sym_range_end] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_set] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_wrap] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_nowrap] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_edit] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_edit_token1] = {
    .visible = false,
    .named = false,
  },
  [sym_s] = {
    .visible = true,
    .named = true,
  },
  [sym_cmd] = {
    .visible = true,
    .named = true,
  },
  [sym_range] = {
    .visible = true,
    .named = true,
  },
  [sym_set] = {
    .visible = true,
    .named = true,
  },
  [sym_config_param] = {
    .visible = true,
    .named = true,
  },
  [sym_edit] = {
    .visible = true,
    .named = true,
  },
};

static TSSymbol ts_alias_sequences[1][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(22);
      if (lookahead == '\n') ADVANCE(23);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '$') ADVANCE(31);
      if (lookahead == '%') ADVANCE(26);
      if (lookahead == '\'') ADVANCE(18);
      if (lookahead == ',') ADVANCE(24);
      if (lookahead == '/') ADVANCE(3);
      if (lookahead == '?') ADVANCE(20);
      if (lookahead == 'e') ADVANCE(7);
      if (lookahead == 'n') ADVANCE(10);
      if (lookahead == 's') ADVANCE(8);
      if (lookahead == 'w') ADVANCE(13);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(0)
      if (('.' <= lookahead && lookahead <= '9')) ADVANCE(25);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(23);
      END_STATE();
    case 2:
      if (lookahead == '\n') ADVANCE(23);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '\'') ADVANCE(19);
      if (lookahead == '/') ADVANCE(3);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(2)
      if (lookahead == '$' ||
          ('.' <= lookahead && lookahead <= '9')) ADVANCE(31);
      END_STATE();
    case 3:
      if (lookahead == '/') ADVANCE(29);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(3);
      END_STATE();
    case 4:
      if (lookahead == '?') ADVANCE(27);
      if (lookahead != 0) ADVANCE(4);
      END_STATE();
    case 5:
      if (lookahead == 'a') ADVANCE(11);
      END_STATE();
    case 6:
      if (lookahead == 'a') ADVANCE(12);
      END_STATE();
    case 7:
      if (lookahead == 'd') ADVANCE(9);
      END_STATE();
    case 8:
      if (lookahead == 'e') ADVANCE(15);
      END_STATE();
    case 9:
      if (lookahead == 'i') ADVANCE(16);
      END_STATE();
    case 10:
      if (lookahead == 'o') ADVANCE(17);
      END_STATE();
    case 11:
      if (lookahead == 'p') ADVANCE(35);
      END_STATE();
    case 12:
      if (lookahead == 'p') ADVANCE(36);
      END_STATE();
    case 13:
      if (lookahead == 'r') ADVANCE(5);
      END_STATE();
    case 14:
      if (lookahead == 'r') ADVANCE(6);
      END_STATE();
    case 15:
      if (lookahead == 't') ADVANCE(34);
      END_STATE();
    case 16:
      if (lookahead == 't') ADVANCE(37);
      END_STATE();
    case 17:
      if (lookahead == 'w') ADVANCE(14);
      END_STATE();
    case 18:
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(27);
      END_STATE();
    case 19:
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(32);
      END_STATE();
    case 20:
      if (lookahead != 0 &&
          lookahead != '?') ADVANCE(4);
      END_STATE();
    case 21:
      if (eof) ADVANCE(22);
      if (lookahead == '\n') ADVANCE(23);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '\'') ADVANCE(18);
      if (lookahead == '?') ADVANCE(20);
      if (lookahead == 'e') ADVANCE(7);
      if (lookahead == 's') ADVANCE(8);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(21)
      if (lookahead == '%' ||
          lookahead == '.' ||
          ('0' <= lookahead && lookahead <= '9')) ADVANCE(26);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(sym_newline);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(sym_range_start);
      if (lookahead == '$') ADVANCE(31);
      if (lookahead == '%') ADVANCE(26);
      if (lookahead == '.') ADVANCE(25);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(28);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(25);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(sym_range_start);
      if (lookahead == '%' ||
          lookahead == '.') ADVANCE(26);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(28);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(26);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(sym_range_start);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(28);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(28);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(sym_range_start);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(28);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(sym_range_end);
      if (lookahead == '/') ADVANCE(29);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(30);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(30);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(3);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(sym_range_end);
      if (lookahead == '/') ADVANCE(29);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(30);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(3);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(sym_range_end);
      if (lookahead == '$' ||
          lookahead == '.') ADVANCE(31);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(33);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(31);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(sym_range_end);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(33);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(33);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(sym_range_end);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(33);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(anon_sym_set);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(anon_sym_wrap);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(anon_sym_nowrap);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(anon_sym_edit);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(aux_sym_edit_token1);
      if (lookahead == '\n') ADVANCE(23);
      if (lookahead != 0) ADVANCE(40);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(aux_sym_edit_token1);
      if (lookahead == '\r') ADVANCE(38);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(39);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(40);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(aux_sym_edit_token1);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(40);
      END_STATE();
    default:
      return false;
  }
}

static TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 21},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 39},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 2},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
};

static uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_newline] = ACTIONS(3),
    [anon_sym_COMMA] = ACTIONS(1),
    [sym_range_start] = ACTIONS(1),
    [sym_range_end] = ACTIONS(1),
    [anon_sym_set] = ACTIONS(1),
    [anon_sym_wrap] = ACTIONS(1),
    [anon_sym_nowrap] = ACTIONS(1),
    [anon_sym_edit] = ACTIONS(1),
  },
  [1] = {
    [sym_s] = STATE(7),
    [sym_cmd] = STATE(8),
    [sym_range] = STATE(2),
    [sym_set] = STATE(9),
    [sym_edit] = STATE(9),
    [ts_builtin_sym_end] = ACTIONS(5),
    [sym_newline] = ACTIONS(3),
    [sym_range_start] = ACTIONS(7),
    [anon_sym_set] = ACTIONS(9),
    [anon_sym_edit] = ACTIONS(11),
  },
};

static uint16_t ts_small_parse_table[] = {
  [0] = 6,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(9), 1,
      anon_sym_set,
    ACTIONS(11), 1,
      anon_sym_edit,
    ACTIONS(13), 1,
      ts_builtin_sym_end,
    STATE(14), 1,
      sym_cmd,
    STATE(9), 2,
      sym_set,
      sym_edit,
  [20] = 3,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(17), 1,
      anon_sym_COMMA,
    ACTIONS(15), 3,
      ts_builtin_sym_end,
      anon_sym_set,
      anon_sym_edit,
  [32] = 3,
    ACTIONS(3), 1,
      sym_newline,
    STATE(12), 1,
      sym_config_param,
    ACTIONS(19), 2,
      anon_sym_wrap,
      anon_sym_nowrap,
  [43] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(21), 3,
      ts_builtin_sym_end,
      anon_sym_set,
      anon_sym_edit,
  [52] = 2,
    ACTIONS(23), 1,
      sym_newline,
    ACTIONS(25), 1,
      aux_sym_edit_token1,
  [59] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(27), 1,
      ts_builtin_sym_end,
  [66] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(13), 1,
      ts_builtin_sym_end,
  [73] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(29), 1,
      ts_builtin_sym_end,
  [80] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(31), 1,
      sym_range_end,
  [87] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(33), 1,
      ts_builtin_sym_end,
  [94] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(35), 1,
      ts_builtin_sym_end,
  [101] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(37), 1,
      ts_builtin_sym_end,
  [108] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(39), 1,
      ts_builtin_sym_end,
};

static uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 20,
  [SMALL_STATE(4)] = 32,
  [SMALL_STATE(5)] = 43,
  [SMALL_STATE(6)] = 52,
  [SMALL_STATE(7)] = 59,
  [SMALL_STATE(8)] = 66,
  [SMALL_STATE(9)] = 73,
  [SMALL_STATE(10)] = 80,
  [SMALL_STATE(11)] = 87,
  [SMALL_STATE(12)] = 94,
  [SMALL_STATE(13)] = 101,
  [SMALL_STATE(14)] = 108,
};

static TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_s, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [13] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_s, 1),
  [15] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_range, 1),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [21] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_range, 3),
  [23] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [25] = {.entry = {.count = 1, .reusable = false}}, SHIFT(13),
  [27] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [29] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_cmd, 1),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [33] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_config_param, 1),
  [35] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_set, 2),
  [37] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_edit, 2),
  [39] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_s, 2),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_code_cmd(void) {
  static TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .symbol_metadata = ts_symbol_metadata,
    .parse_table = (const unsigned short *)ts_parse_table,
    .small_parse_table = (const uint16_t *)ts_small_parse_table,
    .small_parse_table_map = (const uint32_t *)ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .lex_modes = ts_lex_modes,
    .symbol_names = ts_symbol_names,
    .public_symbol_map = ts_symbol_map,
    .alias_sequences = (const TSSymbol *)ts_alias_sequences,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .lex_fn = ts_lex,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
