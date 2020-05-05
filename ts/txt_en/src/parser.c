#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 11
#define STATE_COUNT 20
#define LARGE_STATE_COUNT 6
#define SYMBOL_COUNT 14
#define ALIAS_COUNT 0
#define TOKEN_COUNT 6
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 3

enum {
  sym_word = 1,
  sym_wword = 2,
  sym_dot = 3,
  sym_nl = 4,
  sym_ws = 5,
  sym_source_file = 6,
  sym_sentence = 7,
  sym_sentence_line = 8,
  sym_sentence_fin = 9,
  sym_wordws = 10,
  aux_sym_source_file_repeat1 = 11,
  aux_sym_sentence_repeat1 = 12,
  aux_sym_sentence_line_repeat1 = 13,
};

static const char *ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_word] = "word",
  [sym_wword] = "wword",
  [sym_dot] = "dot",
  [sym_nl] = "nl",
  [sym_ws] = "ws",
  [sym_source_file] = "source_file",
  [sym_sentence] = "sentence",
  [sym_sentence_line] = "sentence_line",
  [sym_sentence_fin] = "sentence_fin",
  [sym_wordws] = "wordws",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_sentence_repeat1] = "sentence_repeat1",
  [aux_sym_sentence_line_repeat1] = "sentence_line_repeat1",
};

static TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_word] = sym_word,
  [sym_wword] = sym_wword,
  [sym_dot] = sym_dot,
  [sym_nl] = sym_nl,
  [sym_ws] = sym_ws,
  [sym_source_file] = sym_source_file,
  [sym_sentence] = sym_sentence,
  [sym_sentence_line] = sym_sentence_line,
  [sym_sentence_fin] = sym_sentence_fin,
  [sym_wordws] = sym_wordws,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_sentence_repeat1] = aux_sym_sentence_repeat1,
  [aux_sym_sentence_line_repeat1] = aux_sym_sentence_line_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_word] = {
    .visible = true,
    .named = true,
  },
  [sym_wword] = {
    .visible = true,
    .named = true,
  },
  [sym_dot] = {
    .visible = true,
    .named = true,
  },
  [sym_nl] = {
    .visible = true,
    .named = true,
  },
  [sym_ws] = {
    .visible = true,
    .named = true,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym_sentence] = {
    .visible = true,
    .named = true,
  },
  [sym_sentence_line] = {
    .visible = true,
    .named = true,
  },
  [sym_sentence_fin] = {
    .visible = true,
    .named = true,
  },
  [sym_wordws] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_sentence_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_sentence_line_repeat1] = {
    .visible = false,
    .named = false,
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
      if (eof) ADVANCE(4);
      if (lookahead == '\n') ADVANCE(10);
      if (lookahead == '\r') ADVANCE(13);
      if (lookahead == '.') ADVANCE(8);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(13);
      if (lookahead == 11 ||
          lookahead == '\f') ADVANCE(5);
      if (lookahead != 0) ADVANCE(6);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(12);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '.') ADVANCE(8);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(1)
      if (lookahead != 0) ADVANCE(6);
      END_STATE();
    case 2:
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '.') ADVANCE(7);
      END_STATE();
    case 3:
      if (eof) ADVANCE(4);
      if (lookahead == '\n') ADVANCE(12);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '.') ADVANCE(8);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(3)
      if (lookahead != 0) ADVANCE(6);
      END_STATE();
    case 4:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(sym_word);
      if (lookahead == '.') ADVANCE(2);
      if (lookahead == 11 ||
          lookahead == '\f') ADVANCE(5);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(14);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(6);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(sym_word);
      if (lookahead == '.') ADVANCE(2);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ') ADVANCE(6);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(sym_wword);
      if (lookahead == '.') ADVANCE(2);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ') ADVANCE(7);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(sym_dot);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(sym_nl);
      if (lookahead == '\n') ADVANCE(10);
      if (lookahead == '\r') ADVANCE(13);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(13);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(sym_nl);
      if (lookahead == '\n') ADVANCE(10);
      if (lookahead == '\r') ADVANCE(9);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(13);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(sym_nl);
      if (lookahead == '\n') ADVANCE(12);
      if (lookahead == '\r') ADVANCE(1);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(sym_nl);
      if (lookahead == '\n') ADVANCE(12);
      if (lookahead == '\r') ADVANCE(11);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(sym_ws);
      if (lookahead == '\n') ADVANCE(10);
      if (lookahead == '\r') ADVANCE(13);
      if (lookahead == '\t' ||
          lookahead == ' ') ADVANCE(13);
      if (lookahead == 11 ||
          lookahead == '\f') ADVANCE(5);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(sym_ws);
      if (lookahead == '\t' ||
          (11 <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(14);
      END_STATE();
    default:
      return false;
  }
}

static TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 3},
  [2] = {.lex_state = 3},
  [3] = {.lex_state = 3},
  [4] = {.lex_state = 3},
  [5] = {.lex_state = 3},
  [6] = {.lex_state = 3},
  [7] = {.lex_state = 3},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 3},
  [10] = {.lex_state = 3},
  [11] = {.lex_state = 3},
  [12] = {.lex_state = 3},
  [13] = {.lex_state = 3},
  [14] = {.lex_state = 3},
  [15] = {.lex_state = 3},
  [16] = {.lex_state = 3},
  [17] = {.lex_state = 3},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 3},
};

static uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_word] = ACTIONS(1),
    [sym_wword] = ACTIONS(1),
    [sym_dot] = ACTIONS(1),
    [sym_nl] = ACTIONS(1),
    [sym_ws] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(18),
    [sym_sentence] = STATE(2),
    [sym_sentence_line] = STATE(4),
    [sym_sentence_fin] = STATE(9),
    [sym_wordws] = STATE(6),
    [aux_sym_source_file_repeat1] = STATE(2),
    [aux_sym_sentence_repeat1] = STATE(4),
    [aux_sym_sentence_line_repeat1] = STATE(6),
    [ts_builtin_sym_end] = ACTIONS(3),
    [sym_word] = ACTIONS(5),
    [sym_wword] = ACTIONS(5),
    [sym_dot] = ACTIONS(7),
    [sym_nl] = ACTIONS(9),
  },
  [2] = {
    [sym_sentence] = STATE(3),
    [sym_sentence_line] = STATE(4),
    [sym_sentence_fin] = STATE(9),
    [sym_wordws] = STATE(6),
    [aux_sym_source_file_repeat1] = STATE(3),
    [aux_sym_sentence_repeat1] = STATE(4),
    [aux_sym_sentence_line_repeat1] = STATE(6),
    [ts_builtin_sym_end] = ACTIONS(11),
    [sym_word] = ACTIONS(5),
    [sym_wword] = ACTIONS(5),
    [sym_dot] = ACTIONS(7),
    [sym_nl] = ACTIONS(9),
  },
  [3] = {
    [sym_sentence] = STATE(3),
    [sym_sentence_line] = STATE(4),
    [sym_sentence_fin] = STATE(9),
    [sym_wordws] = STATE(6),
    [aux_sym_source_file_repeat1] = STATE(3),
    [aux_sym_sentence_repeat1] = STATE(4),
    [aux_sym_sentence_line_repeat1] = STATE(6),
    [ts_builtin_sym_end] = ACTIONS(13),
    [sym_word] = ACTIONS(15),
    [sym_wword] = ACTIONS(15),
    [sym_dot] = ACTIONS(18),
    [sym_nl] = ACTIONS(21),
  },
  [4] = {
    [sym_sentence_line] = STATE(5),
    [sym_sentence_fin] = STATE(11),
    [sym_wordws] = STATE(6),
    [aux_sym_sentence_repeat1] = STATE(5),
    [aux_sym_sentence_line_repeat1] = STATE(6),
    [sym_word] = ACTIONS(5),
    [sym_wword] = ACTIONS(5),
    [sym_dot] = ACTIONS(7),
    [sym_nl] = ACTIONS(9),
  },
  [5] = {
    [sym_sentence_line] = STATE(5),
    [sym_wordws] = STATE(12),
    [aux_sym_sentence_repeat1] = STATE(5),
    [aux_sym_sentence_line_repeat1] = STATE(12),
    [sym_word] = ACTIONS(24),
    [sym_wword] = ACTIONS(24),
    [sym_dot] = ACTIONS(27),
    [sym_nl] = ACTIONS(29),
  },
};

static uint16_t ts_small_parse_table[] = {
  [0] = 4,
    ACTIONS(32), 1,
      sym_dot,
    ACTIONS(34), 1,
      sym_nl,
    ACTIONS(5), 2,
      sym_word,
      sym_wword,
    STATE(7), 2,
      sym_wordws,
      aux_sym_sentence_line_repeat1,
  [15] = 4,
    ACTIONS(39), 1,
      sym_dot,
    ACTIONS(41), 1,
      sym_nl,
    ACTIONS(36), 2,
      sym_word,
      sym_wword,
    STATE(7), 2,
      sym_wordws,
      aux_sym_sentence_line_repeat1,
  [30] = 2,
    ACTIONS(45), 1,
      sym_ws,
    ACTIONS(43), 4,
      sym_word,
      sym_wword,
      sym_dot,
      sym_nl,
  [40] = 2,
    ACTIONS(47), 2,
      ts_builtin_sym_end,
      sym_nl,
    ACTIONS(49), 3,
      sym_word,
      sym_wword,
      sym_dot,
  [50] = 2,
    ACTIONS(51), 2,
      ts_builtin_sym_end,
      sym_nl,
    ACTIONS(53), 3,
      sym_word,
      sym_wword,
      sym_dot,
  [60] = 2,
    ACTIONS(55), 2,
      ts_builtin_sym_end,
      sym_nl,
    ACTIONS(57), 3,
      sym_word,
      sym_wword,
      sym_dot,
  [70] = 3,
    ACTIONS(34), 1,
      sym_nl,
    ACTIONS(5), 2,
      sym_word,
      sym_wword,
    STATE(7), 2,
      sym_wordws,
      aux_sym_sentence_line_repeat1,
  [82] = 2,
    ACTIONS(59), 2,
      ts_builtin_sym_end,
      sym_nl,
    ACTIONS(61), 3,
      sym_word,
      sym_wword,
      sym_dot,
  [92] = 2,
    ACTIONS(65), 1,
      sym_nl,
    ACTIONS(63), 3,
      sym_word,
      sym_wword,
      sym_dot,
  [101] = 2,
    ACTIONS(69), 1,
      sym_nl,
    ACTIONS(67), 3,
      sym_word,
      sym_wword,
      sym_dot,
  [110] = 2,
    ACTIONS(73), 1,
      sym_nl,
    ACTIONS(71), 3,
      sym_word,
      sym_wword,
      sym_dot,
  [119] = 1,
    ACTIONS(75), 1,
      sym_nl,
  [123] = 1,
    ACTIONS(77), 1,
      ts_builtin_sym_end,
  [127] = 1,
    ACTIONS(79), 1,
      sym_nl,
};

static uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(6)] = 0,
  [SMALL_STATE(7)] = 15,
  [SMALL_STATE(8)] = 30,
  [SMALL_STATE(9)] = 40,
  [SMALL_STATE(10)] = 50,
  [SMALL_STATE(11)] = 60,
  [SMALL_STATE(12)] = 70,
  [SMALL_STATE(13)] = 82,
  [SMALL_STATE(14)] = 92,
  [SMALL_STATE(15)] = 101,
  [SMALL_STATE(16)] = 110,
  [SMALL_STATE(17)] = 119,
  [SMALL_STATE(18)] = 123,
  [SMALL_STATE(19)] = 127,
};

static TSParseActionEntry ts_parse_actions[] = {
  [0] = {.count = 0, .reusable = false},
  [1] = {.count = 1, .reusable = false}, RECOVER(),
  [3] = {.count = 1, .reusable = true}, REDUCE(sym_source_file, 0),
  [5] = {.count = 1, .reusable = false}, SHIFT(8),
  [7] = {.count = 1, .reusable = false}, SHIFT(17),
  [9] = {.count = 1, .reusable = true}, SHIFT(14),
  [11] = {.count = 1, .reusable = true}, REDUCE(sym_source_file, 1),
  [13] = {.count = 1, .reusable = true}, REDUCE(aux_sym_source_file_repeat1, 2),
  [15] = {.count = 2, .reusable = false}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(8),
  [18] = {.count = 2, .reusable = false}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(17),
  [21] = {.count = 2, .reusable = true}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(14),
  [24] = {.count = 2, .reusable = false}, REDUCE(aux_sym_sentence_repeat1, 2), SHIFT_REPEAT(8),
  [27] = {.count = 1, .reusable = false}, REDUCE(aux_sym_sentence_repeat1, 2),
  [29] = {.count = 2, .reusable = true}, REDUCE(aux_sym_sentence_repeat1, 2), SHIFT_REPEAT(14),
  [32] = {.count = 1, .reusable = false}, SHIFT(19),
  [34] = {.count = 1, .reusable = true}, SHIFT(16),
  [36] = {.count = 2, .reusable = false}, REDUCE(aux_sym_sentence_line_repeat1, 2), SHIFT_REPEAT(8),
  [39] = {.count = 1, .reusable = false}, REDUCE(aux_sym_sentence_line_repeat1, 2),
  [41] = {.count = 1, .reusable = true}, REDUCE(aux_sym_sentence_line_repeat1, 2),
  [43] = {.count = 1, .reusable = false}, REDUCE(sym_wordws, 1),
  [45] = {.count = 1, .reusable = false}, SHIFT(15),
  [47] = {.count = 1, .reusable = true}, REDUCE(sym_sentence, 1),
  [49] = {.count = 1, .reusable = false}, REDUCE(sym_sentence, 1),
  [51] = {.count = 1, .reusable = true}, REDUCE(sym_sentence_fin, 2),
  [53] = {.count = 1, .reusable = false}, REDUCE(sym_sentence_fin, 2),
  [55] = {.count = 1, .reusable = true}, REDUCE(sym_sentence, 2),
  [57] = {.count = 1, .reusable = false}, REDUCE(sym_sentence, 2),
  [59] = {.count = 1, .reusable = true}, REDUCE(sym_sentence_fin, 3),
  [61] = {.count = 1, .reusable = false}, REDUCE(sym_sentence_fin, 3),
  [63] = {.count = 1, .reusable = false}, REDUCE(sym_sentence_line, 1),
  [65] = {.count = 1, .reusable = true}, REDUCE(sym_sentence_line, 1),
  [67] = {.count = 1, .reusable = false}, REDUCE(sym_wordws, 2),
  [69] = {.count = 1, .reusable = true}, REDUCE(sym_wordws, 2),
  [71] = {.count = 1, .reusable = false}, REDUCE(sym_sentence_line, 2),
  [73] = {.count = 1, .reusable = true}, REDUCE(sym_sentence_line, 2),
  [75] = {.count = 1, .reusable = true}, SHIFT(10),
  [77] = {.count = 1, .reusable = true},  ACCEPT_INPUT(),
  [79] = {.count = 1, .reusable = true}, SHIFT(13),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_txt_en(void) {
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
