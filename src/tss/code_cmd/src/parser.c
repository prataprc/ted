#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 11
#define STATE_COUNT 19
#define LARGE_STATE_COUNT 3
#define SYMBOL_COUNT 22
#define ALIAS_COUNT 0
#define TOKEN_COUNT 14
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 3

enum {
  anon_sym_COLON = 1,
  sym_newline = 2,
  anon_sym_COMMA = 3,
  sym_range_start = 4,
  sym_range_end = 5,
  anon_sym_set = 6,
  anon_sym_wrap = 7,
  anon_sym_nowrap = 8,
  anon_sym_edit = 9,
  aux_sym_edit_token1 = 10,
  anon_sym_buffer = 11,
  aux_sym_buffer_token1 = 12,
  anon_sym_buffers = 13,
  sym_s = 14,
  sym_cmd = 15,
  sym_range = 16,
  sym_set = 17,
  sym_config_param = 18,
  sym_edit = 19,
  sym_buffer = 20,
  sym_buffers = 21,
};

static const char *ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_COLON] = ":",
  [sym_newline] = "newline",
  [anon_sym_COMMA] = ",",
  [sym_range_start] = "range_start",
  [sym_range_end] = "range_end",
  [anon_sym_set] = "set",
  [anon_sym_wrap] = "wrap",
  [anon_sym_nowrap] = "nowrap",
  [anon_sym_edit] = "edit",
  [aux_sym_edit_token1] = "edit_token1",
  [anon_sym_buffer] = "buffer",
  [aux_sym_buffer_token1] = "buffer_token1",
  [anon_sym_buffers] = "buffers",
  [sym_s] = "s",
  [sym_cmd] = "cmd",
  [sym_range] = "range",
  [sym_set] = "set",
  [sym_config_param] = "config_param",
  [sym_edit] = "edit",
  [sym_buffer] = "buffer",
  [sym_buffers] = "buffers",
};

static TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_COLON] = anon_sym_COLON,
  [sym_newline] = sym_newline,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [sym_range_start] = sym_range_start,
  [sym_range_end] = sym_range_end,
  [anon_sym_set] = anon_sym_set,
  [anon_sym_wrap] = anon_sym_wrap,
  [anon_sym_nowrap] = anon_sym_nowrap,
  [anon_sym_edit] = anon_sym_edit,
  [aux_sym_edit_token1] = aux_sym_edit_token1,
  [anon_sym_buffer] = anon_sym_buffer,
  [aux_sym_buffer_token1] = aux_sym_buffer_token1,
  [anon_sym_buffers] = anon_sym_buffers,
  [sym_s] = sym_s,
  [sym_cmd] = sym_cmd,
  [sym_range] = sym_range,
  [sym_set] = sym_set,
  [sym_config_param] = sym_config_param,
  [sym_edit] = sym_edit,
  [sym_buffer] = sym_buffer,
  [sym_buffers] = sym_buffers,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
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
  [anon_sym_buffer] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_buffer_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_buffers] = {
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
  [sym_config_param] = {
    .visible = true,
    .named = true,
  },
  [sym_edit] = {
    .visible = true,
    .named = true,
  },
  [sym_buffer] = {
    .visible = true,
    .named = true,
  },
  [sym_buffers] = {
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
      if (eof) ADVANCE(28);
      if (lookahead == '\n') ADVANCE(30);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '$') ADVANCE(38);
      if (lookahead == '%') ADVANCE(33);
      if (lookahead == '\'') ADVANCE(24);
      if (lookahead == ',') ADVANCE(31);
      if (lookahead == '.') ADVANCE(32);
      if (lookahead == '/') ADVANCE(4);
      if (lookahead == ':') ADVANCE(29);
      if (lookahead == '?') ADVANCE(26);
      if (lookahead == 'b') ADVANCE(22);
      if (lookahead == 'e') ADVANCE(8);
      if (lookahead == 'n') ADVANCE(14);
      if (lookahead == 's') ADVANCE(9);
      if (lookahead == 'w') ADVANCE(17);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(32);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(30);
      END_STATE();
    case 2:
      if (lookahead == '\n') ADVANCE(30);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '\'') ADVANCE(25);
      if (lookahead == '/') ADVANCE(4);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(2)
      if (lookahead == '$' ||
          ('.' <= lookahead && lookahead <= '9')) ADVANCE(38);
      END_STATE();
    case 3:
      if (lookahead == '\n') ADVANCE(30);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(3)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(49);
      END_STATE();
    case 4:
      if (lookahead == '/') ADVANCE(36);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(4);
      END_STATE();
    case 5:
      if (lookahead == '?') ADVANCE(34);
      if (lookahead != 0) ADVANCE(5);
      END_STATE();
    case 6:
      if (lookahead == 'a') ADVANCE(15);
      END_STATE();
    case 7:
      if (lookahead == 'a') ADVANCE(16);
      END_STATE();
    case 8:
      if (lookahead == 'd') ADVANCE(13);
      END_STATE();
    case 9:
      if (lookahead == 'e') ADVANCE(20);
      END_STATE();
    case 10:
      if (lookahead == 'e') ADVANCE(18);
      END_STATE();
    case 11:
      if (lookahead == 'f') ADVANCE(10);
      END_STATE();
    case 12:
      if (lookahead == 'f') ADVANCE(11);
      END_STATE();
    case 13:
      if (lookahead == 'i') ADVANCE(21);
      END_STATE();
    case 14:
      if (lookahead == 'o') ADVANCE(23);
      END_STATE();
    case 15:
      if (lookahead == 'p') ADVANCE(42);
      END_STATE();
    case 16:
      if (lookahead == 'p') ADVANCE(43);
      END_STATE();
    case 17:
      if (lookahead == 'r') ADVANCE(6);
      END_STATE();
    case 18:
      if (lookahead == 'r') ADVANCE(48);
      END_STATE();
    case 19:
      if (lookahead == 'r') ADVANCE(7);
      END_STATE();
    case 20:
      if (lookahead == 't') ADVANCE(41);
      END_STATE();
    case 21:
      if (lookahead == 't') ADVANCE(44);
      END_STATE();
    case 22:
      if (lookahead == 'u') ADVANCE(12);
      END_STATE();
    case 23:
      if (lookahead == 'w') ADVANCE(19);
      END_STATE();
    case 24:
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(34);
      END_STATE();
    case 25:
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(39);
      END_STATE();
    case 26:
      if (lookahead != 0 &&
          lookahead != '?') ADVANCE(5);
      END_STATE();
    case 27:
      if (eof) ADVANCE(28);
      if (lookahead == '\n') ADVANCE(30);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '\'') ADVANCE(24);
      if (lookahead == '?') ADVANCE(26);
      if (lookahead == 'b') ADVANCE(22);
      if (lookahead == 'e') ADVANCE(8);
      if (lookahead == 's') ADVANCE(9);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(27)
      if (lookahead == '%' ||
          lookahead == '.' ||
          ('0' <= lookahead && lookahead <= '9')) ADVANCE(33);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(sym_newline);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(sym_range_start);
      if (lookahead == '$') ADVANCE(38);
      if (lookahead == '%') ADVANCE(33);
      if (lookahead == '.') ADVANCE(32);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(35);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(32);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(sym_range_start);
      if (lookahead == '%' ||
          lookahead == '.') ADVANCE(33);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(35);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(33);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(sym_range_start);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(35);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(35);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(sym_range_start);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(35);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(sym_range_end);
      if (lookahead == '/') ADVANCE(36);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(37);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(37);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(4);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(sym_range_end);
      if (lookahead == '/') ADVANCE(36);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(37);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(4);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(sym_range_end);
      if (lookahead == '$' ||
          lookahead == '.') ADVANCE(38);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(40);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(38);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(sym_range_end);
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(40);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(40);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(sym_range_end);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(40);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(anon_sym_set);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(anon_sym_wrap);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(anon_sym_nowrap);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(anon_sym_edit);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(aux_sym_edit_token1);
      if (lookahead == '\n') ADVANCE(30);
      if (lookahead != 0) ADVANCE(47);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(aux_sym_edit_token1);
      if (lookahead == '\r') ADVANCE(45);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(46);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(47);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(aux_sym_edit_token1);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(47);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(anon_sym_buffer);
      if (lookahead == 's') ADVANCE(50);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(aux_sym_buffer_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(49);
      END_STATE();
    case 50:
      ACCEPT_TOKEN(anon_sym_buffers);
      END_STATE();
    default:
      return false;
  }
}

static TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 27},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 46},
  [9] = {.lex_state = 3},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 2},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 0},
};

static uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [sym_newline] = ACTIONS(3),
    [anon_sym_COMMA] = ACTIONS(1),
    [sym_range_start] = ACTIONS(1),
    [sym_range_end] = ACTIONS(1),
    [anon_sym_set] = ACTIONS(1),
    [anon_sym_wrap] = ACTIONS(1),
    [anon_sym_nowrap] = ACTIONS(1),
    [anon_sym_edit] = ACTIONS(1),
    [anon_sym_buffer] = ACTIONS(1),
    [aux_sym_buffer_token1] = ACTIONS(1),
    [anon_sym_buffers] = ACTIONS(1),
  },
  [1] = {
    [sym_s] = STATE(7),
    [anon_sym_COLON] = ACTIONS(5),
    [sym_newline] = ACTIONS(3),
  },
  [2] = {
    [sym_cmd] = STATE(11),
    [sym_range] = STATE(3),
    [sym_set] = STATE(12),
    [sym_edit] = STATE(12),
    [sym_buffer] = STATE(12),
    [sym_buffers] = STATE(12),
    [ts_builtin_sym_end] = ACTIONS(7),
    [sym_newline] = ACTIONS(3),
    [sym_range_start] = ACTIONS(9),
    [anon_sym_set] = ACTIONS(11),
    [anon_sym_edit] = ACTIONS(13),
    [anon_sym_buffer] = ACTIONS(15),
    [anon_sym_buffers] = ACTIONS(17),
  },
};

static uint16_t ts_small_parse_table[] = {
  [0] = 8,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(11), 1,
      anon_sym_set,
    ACTIONS(13), 1,
      anon_sym_edit,
    ACTIONS(15), 1,
      anon_sym_buffer,
    ACTIONS(17), 1,
      anon_sym_buffers,
    ACTIONS(19), 1,
      ts_builtin_sym_end,
    STATE(18), 1,
      sym_cmd,
    STATE(12), 4,
      sym_set,
      sym_edit,
      sym_buffer,
      sym_buffers,
  [28] = 4,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(23), 1,
      anon_sym_COMMA,
    ACTIONS(25), 1,
      anon_sym_buffer,
    ACTIONS(21), 4,
      ts_builtin_sym_end,
      anon_sym_set,
      anon_sym_edit,
      anon_sym_buffers,
  [44] = 3,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(29), 1,
      anon_sym_buffer,
    ACTIONS(27), 4,
      ts_builtin_sym_end,
      anon_sym_set,
      anon_sym_edit,
      anon_sym_buffers,
  [57] = 3,
    ACTIONS(3), 1,
      sym_newline,
    STATE(15), 1,
      sym_config_param,
    ACTIONS(31), 2,
      anon_sym_wrap,
      anon_sym_nowrap,
  [68] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(33), 1,
      ts_builtin_sym_end,
  [75] = 2,
    ACTIONS(35), 1,
      sym_newline,
    ACTIONS(37), 1,
      aux_sym_edit_token1,
  [82] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(39), 1,
      aux_sym_buffer_token1,
  [89] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(41), 1,
      ts_builtin_sym_end,
  [96] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(19), 1,
      ts_builtin_sym_end,
  [103] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(43), 1,
      ts_builtin_sym_end,
  [110] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(45), 1,
      sym_range_end,
  [117] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(47), 1,
      ts_builtin_sym_end,
  [124] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(49), 1,
      ts_builtin_sym_end,
  [131] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(51), 1,
      ts_builtin_sym_end,
  [138] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(53), 1,
      ts_builtin_sym_end,
  [145] = 2,
    ACTIONS(3), 1,
      sym_newline,
    ACTIONS(55), 1,
      ts_builtin_sym_end,
};

static uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(3)] = 0,
  [SMALL_STATE(4)] = 28,
  [SMALL_STATE(5)] = 44,
  [SMALL_STATE(6)] = 57,
  [SMALL_STATE(7)] = 68,
  [SMALL_STATE(8)] = 75,
  [SMALL_STATE(9)] = 82,
  [SMALL_STATE(10)] = 89,
  [SMALL_STATE(11)] = 96,
  [SMALL_STATE(12)] = 103,
  [SMALL_STATE(13)] = 110,
  [SMALL_STATE(14)] = 117,
  [SMALL_STATE(15)] = 124,
  [SMALL_STATE(16)] = 131,
  [SMALL_STATE(17)] = 138,
  [SMALL_STATE(18)] = 145,
};

static TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [7] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_s, 1),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(9),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [19] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_s, 2),
  [21] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_range, 1),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [25] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_range, 1),
  [27] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_range, 3),
  [29] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_range, 3),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [33] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [35] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [37] = {.entry = {.count = 1, .reusable = false}}, SHIFT(16),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [41] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_buffers, 1),
  [43] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_cmd, 1),
  [45] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [47] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_config_param, 1),
  [49] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_set, 2),
  [51] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_edit, 2),
  [53] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_buffer, 2),
  [55] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_s, 3),
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
