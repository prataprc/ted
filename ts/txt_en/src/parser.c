#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 11
#define STATE_COUNT 18
#define LARGE_STATE_COUNT 6
#define SYMBOL_COUNT 11
#define ALIAS_COUNT 0
#define TOKEN_COUNT 4
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 3

enum {
  sym_word = 1,
  sym_dot = 2,
  sym_nl = 3,
  sym_source_file = 4,
  sym_sentence = 5,
  sym_sentence_line = 6,
  sym_sentence_fin = 7,
  aux_sym_source_file_repeat1 = 8,
  aux_sym_sentence_repeat1 = 9,
  aux_sym_sentence_line_repeat1 = 10,
};

static const char *ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_word] = "word",
  [sym_dot] = "dot",
  [sym_nl] = "nl",
  [sym_source_file] = "source_file",
  [sym_sentence] = "sentence",
  [sym_sentence_line] = "sentence_line",
  [sym_sentence_fin] = "sentence_fin",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_sentence_repeat1] = "sentence_repeat1",
  [aux_sym_sentence_line_repeat1] = "sentence_line_repeat1",
};

static TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_word] = sym_word,
  [sym_dot] = sym_dot,
  [sym_nl] = sym_nl,
  [sym_source_file] = sym_source_file,
  [sym_sentence] = sym_sentence,
  [sym_sentence_line] = sym_sentence_line,
  [sym_sentence_fin] = sym_sentence_fin,
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
  [sym_dot] = {
    .visible = true,
    .named = true,
  },
  [sym_nl] = {
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
      if (eof) ADVANCE(2);
      if (lookahead == '\n') ADVANCE(6);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '.') ADVANCE(4);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(0)
      if (lookahead != 0) ADVANCE(3);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(6);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '.') ADVANCE(4);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(1)
      if (lookahead != 0) ADVANCE(3);
      END_STATE();
    case 2:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 3:
      ACCEPT_TOKEN(sym_word);
      if (lookahead != 0 &&
          lookahead != '\t' &&
          lookahead != '\n' &&
          lookahead != '\r' &&
          lookahead != ' ' &&
          lookahead != '.') ADVANCE(3);
      END_STATE();
    case 4:
      ACCEPT_TOKEN(sym_dot);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(sym_nl);
      if (lookahead == '\n') ADVANCE(6);
      if (lookahead == '\r') ADVANCE(1);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(sym_nl);
      if (lookahead == '\n') ADVANCE(6);
      if (lookahead == '\r') ADVANCE(5);
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
};

static uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_word] = ACTIONS(1),
    [sym_dot] = ACTIONS(1),
    [sym_nl] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(16),
    [sym_sentence] = STATE(2),
    [sym_sentence_line] = STATE(4),
    [sym_sentence_fin] = STATE(6),
    [aux_sym_source_file_repeat1] = STATE(2),
    [aux_sym_sentence_repeat1] = STATE(4),
    [aux_sym_sentence_line_repeat1] = STATE(7),
    [ts_builtin_sym_end] = ACTIONS(3),
    [sym_word] = ACTIONS(5),
    [sym_dot] = ACTIONS(7),
    [sym_nl] = ACTIONS(9),
  },
  [2] = {
    [sym_sentence] = STATE(3),
    [sym_sentence_line] = STATE(4),
    [sym_sentence_fin] = STATE(6),
    [aux_sym_source_file_repeat1] = STATE(3),
    [aux_sym_sentence_repeat1] = STATE(4),
    [aux_sym_sentence_line_repeat1] = STATE(7),
    [ts_builtin_sym_end] = ACTIONS(11),
    [sym_word] = ACTIONS(5),
    [sym_dot] = ACTIONS(7),
    [sym_nl] = ACTIONS(9),
  },
  [3] = {
    [sym_sentence] = STATE(3),
    [sym_sentence_line] = STATE(4),
    [sym_sentence_fin] = STATE(6),
    [aux_sym_source_file_repeat1] = STATE(3),
    [aux_sym_sentence_repeat1] = STATE(4),
    [aux_sym_sentence_line_repeat1] = STATE(7),
    [ts_builtin_sym_end] = ACTIONS(13),
    [sym_word] = ACTIONS(15),
    [sym_dot] = ACTIONS(18),
    [sym_nl] = ACTIONS(21),
  },
  [4] = {
    [sym_sentence_line] = STATE(5),
    [sym_sentence_fin] = STATE(9),
    [aux_sym_sentence_repeat1] = STATE(5),
    [aux_sym_sentence_line_repeat1] = STATE(7),
    [sym_word] = ACTIONS(5),
    [sym_dot] = ACTIONS(7),
    [sym_nl] = ACTIONS(9),
  },
  [5] = {
    [sym_sentence_line] = STATE(5),
    [aux_sym_sentence_repeat1] = STATE(5),
    [aux_sym_sentence_line_repeat1] = STATE(14),
    [sym_word] = ACTIONS(24),
    [sym_dot] = ACTIONS(27),
    [sym_nl] = ACTIONS(29),
  },
};

static uint16_t ts_small_parse_table[] = {
  [0] = 2,
    ACTIONS(32), 2,
      ts_builtin_sym_end,
      sym_nl,
    ACTIONS(34), 2,
      sym_word,
      sym_dot,
  [9] = 4,
    ACTIONS(36), 1,
      sym_word,
    ACTIONS(38), 1,
      sym_dot,
    ACTIONS(40), 1,
      sym_nl,
    STATE(10), 1,
      aux_sym_sentence_line_repeat1,
  [22] = 2,
    ACTIONS(42), 2,
      ts_builtin_sym_end,
      sym_nl,
    ACTIONS(44), 2,
      sym_word,
      sym_dot,
  [31] = 2,
    ACTIONS(46), 2,
      ts_builtin_sym_end,
      sym_nl,
    ACTIONS(48), 2,
      sym_word,
      sym_dot,
  [40] = 4,
    ACTIONS(50), 1,
      sym_word,
    ACTIONS(53), 1,
      sym_dot,
    ACTIONS(55), 1,
      sym_nl,
    STATE(10), 1,
      aux_sym_sentence_line_repeat1,
  [53] = 2,
    ACTIONS(57), 2,
      ts_builtin_sym_end,
      sym_nl,
    ACTIONS(59), 2,
      sym_word,
      sym_dot,
  [62] = 2,
    ACTIONS(63), 1,
      sym_nl,
    ACTIONS(61), 2,
      sym_word,
      sym_dot,
  [70] = 2,
    ACTIONS(67), 1,
      sym_nl,
    ACTIONS(65), 2,
      sym_word,
      sym_dot,
  [78] = 3,
    ACTIONS(36), 1,
      sym_word,
    ACTIONS(40), 1,
      sym_nl,
    STATE(10), 1,
      aux_sym_sentence_line_repeat1,
  [88] = 1,
    ACTIONS(69), 1,
      sym_nl,
  [92] = 1,
    ACTIONS(71), 1,
      ts_builtin_sym_end,
  [96] = 1,
    ACTIONS(73), 1,
      sym_nl,
};

static uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(6)] = 0,
  [SMALL_STATE(7)] = 9,
  [SMALL_STATE(8)] = 22,
  [SMALL_STATE(9)] = 31,
  [SMALL_STATE(10)] = 40,
  [SMALL_STATE(11)] = 53,
  [SMALL_STATE(12)] = 62,
  [SMALL_STATE(13)] = 70,
  [SMALL_STATE(14)] = 78,
  [SMALL_STATE(15)] = 88,
  [SMALL_STATE(16)] = 92,
  [SMALL_STATE(17)] = 96,
};

static TSParseActionEntry ts_parse_actions[] = {
  [0] = {.count = 0, .reusable = false},
  [1] = {.count = 1, .reusable = false}, RECOVER(),
  [3] = {.count = 1, .reusable = true}, REDUCE(sym_source_file, 0),
  [5] = {.count = 1, .reusable = false}, SHIFT(7),
  [7] = {.count = 1, .reusable = false}, SHIFT(15),
  [9] = {.count = 1, .reusable = true}, SHIFT(12),
  [11] = {.count = 1, .reusable = true}, REDUCE(sym_source_file, 1),
  [13] = {.count = 1, .reusable = true}, REDUCE(aux_sym_source_file_repeat1, 2),
  [15] = {.count = 2, .reusable = false}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(7),
  [18] = {.count = 2, .reusable = false}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(15),
  [21] = {.count = 2, .reusable = true}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(12),
  [24] = {.count = 2, .reusable = false}, REDUCE(aux_sym_sentence_repeat1, 2), SHIFT_REPEAT(14),
  [27] = {.count = 1, .reusable = false}, REDUCE(aux_sym_sentence_repeat1, 2),
  [29] = {.count = 2, .reusable = true}, REDUCE(aux_sym_sentence_repeat1, 2), SHIFT_REPEAT(12),
  [32] = {.count = 1, .reusable = true}, REDUCE(sym_sentence, 1),
  [34] = {.count = 1, .reusable = false}, REDUCE(sym_sentence, 1),
  [36] = {.count = 1, .reusable = false}, SHIFT(10),
  [38] = {.count = 1, .reusable = false}, SHIFT(17),
  [40] = {.count = 1, .reusable = true}, SHIFT(13),
  [42] = {.count = 1, .reusable = true}, REDUCE(sym_sentence_fin, 2),
  [44] = {.count = 1, .reusable = false}, REDUCE(sym_sentence_fin, 2),
  [46] = {.count = 1, .reusable = true}, REDUCE(sym_sentence, 2),
  [48] = {.count = 1, .reusable = false}, REDUCE(sym_sentence, 2),
  [50] = {.count = 2, .reusable = false}, REDUCE(aux_sym_sentence_line_repeat1, 2), SHIFT_REPEAT(10),
  [53] = {.count = 1, .reusable = false}, REDUCE(aux_sym_sentence_line_repeat1, 2),
  [55] = {.count = 1, .reusable = true}, REDUCE(aux_sym_sentence_line_repeat1, 2),
  [57] = {.count = 1, .reusable = true}, REDUCE(sym_sentence_fin, 3),
  [59] = {.count = 1, .reusable = false}, REDUCE(sym_sentence_fin, 3),
  [61] = {.count = 1, .reusable = false}, REDUCE(sym_sentence_line, 1),
  [63] = {.count = 1, .reusable = true}, REDUCE(sym_sentence_line, 1),
  [65] = {.count = 1, .reusable = false}, REDUCE(sym_sentence_line, 2),
  [67] = {.count = 1, .reusable = true}, REDUCE(sym_sentence_line, 2),
  [69] = {.count = 1, .reusable = true}, SHIFT(8),
  [71] = {.count = 1, .reusable = true},  ACCEPT_INPUT(),
  [73] = {.count = 1, .reusable = true}, SHIFT(11),
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
