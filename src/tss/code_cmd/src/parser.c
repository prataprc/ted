#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 11
#define STATE_COUNT 7
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 8
#define ALIAS_COUNT 0
#define TOKEN_COUNT 5
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 2

enum {
  sym_newline = 1,
  anon_sym_set = 2,
  anon_sym_wrap = 3,
  anon_sym_nowrap = 4,
  sym_s = 5,
  sym_set = 6,
  sym_set_flags = 7,
};

static const char *ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_newline] = "newline",
  [anon_sym_set] = "set",
  [anon_sym_wrap] = "wrap",
  [anon_sym_nowrap] = "nowrap",
  [sym_s] = "s",
  [sym_set] = "set",
  [sym_set_flags] = "set_flags",
};

static TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_newline] = sym_newline,
  [anon_sym_set] = anon_sym_set,
  [anon_sym_wrap] = anon_sym_wrap,
  [anon_sym_nowrap] = anon_sym_nowrap,
  [sym_s] = sym_s,
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
      if (eof) ADVANCE(12);
      if (lookahead == '\n') ADVANCE(13);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == 'n') ADVANCE(5);
      if (lookahead == 's') ADVANCE(4);
      if (lookahead == 'w') ADVANCE(8);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(0)
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(13);
      END_STATE();
    case 2:
      if (lookahead == 'a') ADVANCE(6);
      END_STATE();
    case 3:
      if (lookahead == 'a') ADVANCE(7);
      END_STATE();
    case 4:
      if (lookahead == 'e') ADVANCE(10);
      END_STATE();
    case 5:
      if (lookahead == 'o') ADVANCE(11);
      END_STATE();
    case 6:
      if (lookahead == 'p') ADVANCE(15);
      END_STATE();
    case 7:
      if (lookahead == 'p') ADVANCE(16);
      END_STATE();
    case 8:
      if (lookahead == 'r') ADVANCE(2);
      END_STATE();
    case 9:
      if (lookahead == 'r') ADVANCE(3);
      END_STATE();
    case 10:
      if (lookahead == 't') ADVANCE(14);
      END_STATE();
    case 11:
      if (lookahead == 'w') ADVANCE(9);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(sym_newline);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(anon_sym_set);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_wrap);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(anon_sym_nowrap);
      END_STATE();
    default:
      return false;
  }
}

static TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0},
};

static uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_newline] = ACTIONS(3),
    [anon_sym_set] = ACTIONS(1),
    [anon_sym_wrap] = ACTIONS(1),
    [anon_sym_nowrap] = ACTIONS(1),
  },
  [1] = {
    [sym_s] = STATE(3),
    [sym_set] = STATE(4),
    [sym_newline] = ACTIONS(3),
    [anon_sym_set] = ACTIONS(5),
  },
};

static uint16_t ts_small_parse_table[] = {
  [0] = 3,
    ACTIONS(3), 1,
      sym_newline,
    STATE(6), 1,
      sym_set_flags,
    ACTIONS(7), 2,
      anon_sym_wrap,
      anon_sym_nowrap,
  [11] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(9), 1,
      ts_builtin_sym_end,
  [18] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(11), 1,
      ts_builtin_sym_end,
  [25] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(13), 1,
      ts_builtin_sym_end,
  [32] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(15), 1,
      ts_builtin_sym_end,
};

static uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 11,
  [SMALL_STATE(4)] = 18,
  [SMALL_STATE(5)] = 25,
  [SMALL_STATE(6)] = 32,
};

static TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [9] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [11] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_s, 1),
  [13] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_set_flags, 1),
  [15] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_set, 2),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_tss(void) {
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
