#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 11
#define STATE_COUNT 13
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 13
#define ALIAS_COUNT 0
#define TOKEN_COUNT 8
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
  sym_s = 8,
  sym_cmd = 9,
  sym_range = 10,
  sym_set = 11,
  sym_set_flags = 12,
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
  [sym_s] = "s",
  [sym_cmd] = "cmd",
  [sym_range] = "range",
  [sym_set] = "set",
  [sym_set_flags] = "set_flags",
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
  [sym_s] = sym_s,
  [sym_cmd] = sym_cmd,
  [sym_range] = sym_range,
  [sym_set] = sym_set,
  [sym_set_flags] = sym_set_flags,
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
  [sym_set_flags] = {
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
      if (eof) ADVANCE(19);
      if (lookahead == '\n') ADVANCE(20);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '$') ADVANCE(28);
      if (lookahead == '%') ADVANCE(23);
      if (lookahead == '\'') ADVANCE(15);
      if (lookahead == ',') ADVANCE(21);
      if (lookahead == '/') ADVANCE(3);
      if (lookahead == '?') ADVANCE(17);
      if (lookahead == 'n') ADVANCE(8);
      if (lookahead == 's') ADVANCE(7);
      if (lookahead == 'w') ADVANCE(11);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(0)
      if (('.' <= lookahead && lookahead <= '9')) ADVANCE(22);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(20);
      END_STATE();
    case 2:
      if (lookahead == '\n') ADVANCE(20);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '\'') ADVANCE(16);
      if (lookahead == '/') ADVANCE(3);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(2)
      if (lookahead == '$' ||
          ('.' <= lookahead && lookahead <= '9')) ADVANCE(28);
      END_STATE();
    case 3:
      if (lookahead == '/') ADVANCE(26);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(3);
      END_STATE();
    case 4:
      if (lookahead == '?') ADVANCE(24);
      if (lookahead != 0) ADVANCE(4);
      END_STATE();
    case 5:
      if (lookahead == 'a') ADVANCE(9);
      END_STATE();
    case 6:
      if (lookahead == 'a') ADVANCE(10);
      END_STATE();
    case 7:
      if (lookahead == 'e') ADVANCE(13);
      END_STATE();
    case 8:
      if (lookahead == 'o') ADVANCE(14);
      END_STATE();
    case 9:
      if (lookahead == 'p') ADVANCE(32);
      END_STATE();
    case 10:
      if (lookahead == 'p') ADVANCE(33);
      END_STATE();
    case 11:
      if (lookahead == 'r') ADVANCE(5);
      END_STATE();
    case 12:
      if (lookahead == 'r') ADVANCE(6);
      END_STATE();
    case 13:
      if (lookahead == 't') ADVANCE(31);
      END_STATE();
    case 14:
      if (lookahead == 'w') ADVANCE(12);
      END_STATE();
    case 15:
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(24);
      END_STATE();
    case 16:
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(29);
      END_STATE();
    case 17:
      if (lookahead != 0 &&
          lookahead != '?') ADVANCE(4);
      END_STATE();
    case 18:
      if (eof) ADVANCE(19);
      if (lookahead == '\n') ADVANCE(20);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '\'') ADVANCE(15);
      if (lookahead == '?') ADVANCE(17);
      if (lookahead == 's') ADVANCE(7);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(18)
      if (lookahead == '%' ||
          lookahead == '.' ||
          ('0' <= lookahead && lookahead <= '9')) ADVANCE(23);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(sym_newline);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(sym_range_start);
      if (lookahead == '$') ADVANCE(28);
      if (lookahead == '%') ADVANCE(23);
      if (lookahead == '.') ADVANCE(22);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(25);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(22);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(sym_range_start);
      if (lookahead == '%' ||
          lookahead == '.') ADVANCE(23);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(25);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(23);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(sym_range_start);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(25);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(25);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(sym_range_start);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(25);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(sym_range_end);
      if (lookahead == '/') ADVANCE(26);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(27);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(27);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(3);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(sym_range_end);
      if (lookahead == '/') ADVANCE(26);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(27);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(3);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(sym_range_end);
      if (lookahead == '$' ||
          lookahead == '.') ADVANCE(28);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(30);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(28);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(sym_range_end);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(30);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(30);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(sym_range_end);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(30);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(anon_sym_set);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(anon_sym_wrap);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(anon_sym_nowrap);
      END_STATE();
    default:
      return false;
  }
}

static TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 18},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 2},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
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
  },
  [1] = {
    [sym_s] = STATE(6),
    [sym_cmd] = STATE(7),
    [sym_range] = STATE(2),
    [sym_set] = STATE(8),
    [ts_builtin_sym_end] = ACTIONS(5),
    [sym_newline] = ACTIONS(3),
    [sym_range_start] = ACTIONS(7),
    [anon_sym_set] = ACTIONS(9),
  },
};

static uint16_t ts_small_parse_table[] = {
  [0] = 5,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(9), 1,
      anon_sym_set,
    ACTIONS(11), 1,
      ts_builtin_sym_end,
    STATE(8), 1,
      sym_set,
    STATE(12), 1,
      sym_cmd,
  [16] = 3,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(15), 1,
      anon_sym_COMMA,
    ACTIONS(13), 2,
      ts_builtin_sym_end,
      anon_sym_set,
  [27] = 3,
    ACTIONS(3), 1,
      sym_newline,
    STATE(11), 1,
      sym_set_flags,
    ACTIONS(17), 2,
      anon_sym_wrap,
      anon_sym_nowrap,
  [38] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(19), 2,
      ts_builtin_sym_end,
      anon_sym_set,
  [46] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(21), 1,
      ts_builtin_sym_end,
  [53] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(11), 1,
      ts_builtin_sym_end,
  [60] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(23), 1,
      ts_builtin_sym_end,
  [67] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(25), 1,
      sym_range_end,
  [74] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(27), 1,
      ts_builtin_sym_end,
  [81] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(29), 1,
      ts_builtin_sym_end,
  [88] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(31), 1,
      ts_builtin_sym_end,
};

static uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 16,
  [SMALL_STATE(4)] = 27,
  [SMALL_STATE(5)] = 38,
  [SMALL_STATE(6)] = 46,
  [SMALL_STATE(7)] = 53,
  [SMALL_STATE(8)] = 60,
  [SMALL_STATE(9)] = 67,
  [SMALL_STATE(10)] = 74,
  [SMALL_STATE(11)] = 81,
  [SMALL_STATE(12)] = 88,
};

static TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_s, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [11] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_s, 1),
  [13] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_range, 1),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [19] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_range, 3),
  [21] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [23] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_cmd, 1),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [27] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_set_flags, 1),
  [29] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_set, 2),
  [31] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_s, 2),
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
