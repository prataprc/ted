#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 11
#define STATE_COUNT 4
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 32
#define ALIAS_COUNT 0
#define TOKEN_COUNT 31
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 1

enum {
  anon_sym_black = 1,
  anon_sym_darkgrey = 2,
  anon_sym_dark_DASHgrey = 3,
  anon_sym_dark_grey = 4,
  anon_sym_red = 5,
  anon_sym_darkred = 6,
  anon_sym_dark_DASHred = 7,
  anon_sym_dark_red = 8,
  anon_sym_green = 9,
  anon_sym_darkgreen = 10,
  anon_sym_dark_DASHgreen = 11,
  anon_sym_dark_green = 12,
  anon_sym_yellow = 13,
  anon_sym_darkyellow = 14,
  anon_sym_dark_DASHyellow = 15,
  anon_sym_dark_yellow = 16,
  anon_sym_blue = 17,
  anon_sym_darkblue = 18,
  anon_sym_dark_DASHblue = 19,
  anon_sym_dark_blue = 20,
  anon_sym_magenta = 21,
  anon_sym_darkmagenta = 22,
  anon_sym_dark_DASHmagenta = 23,
  anon_sym_dark_magenta = 24,
  anon_sym_cyan = 25,
  anon_sym_darkcyan = 26,
  anon_sym_dark_DASHcyan = 27,
  anon_sym_dark_cyan = 28,
  anon_sym_white = 29,
  anon_sym_grey = 30,
  sym_color_name = 31,
};

static const char *ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_black] = "black",
  [anon_sym_darkgrey] = "darkgrey",
  [anon_sym_dark_DASHgrey] = "dark-grey",
  [anon_sym_dark_grey] = "dark_grey",
  [anon_sym_red] = "red",
  [anon_sym_darkred] = "darkred",
  [anon_sym_dark_DASHred] = "dark-red",
  [anon_sym_dark_red] = "dark_red",
  [anon_sym_green] = "green",
  [anon_sym_darkgreen] = "darkgreen",
  [anon_sym_dark_DASHgreen] = "dark-green",
  [anon_sym_dark_green] = "dark_green",
  [anon_sym_yellow] = "yellow",
  [anon_sym_darkyellow] = "darkyellow",
  [anon_sym_dark_DASHyellow] = "dark-yellow",
  [anon_sym_dark_yellow] = "dark_yellow",
  [anon_sym_blue] = "blue",
  [anon_sym_darkblue] = "darkblue",
  [anon_sym_dark_DASHblue] = "dark-blue",
  [anon_sym_dark_blue] = "dark_blue",
  [anon_sym_magenta] = "magenta",
  [anon_sym_darkmagenta] = "darkmagenta",
  [anon_sym_dark_DASHmagenta] = "dark-magenta",
  [anon_sym_dark_magenta] = "dark_magenta",
  [anon_sym_cyan] = "cyan",
  [anon_sym_darkcyan] = "darkcyan",
  [anon_sym_dark_DASHcyan] = "dark-cyan",
  [anon_sym_dark_cyan] = "dark_cyan",
  [anon_sym_white] = "white",
  [anon_sym_grey] = "grey",
  [sym_color_name] = "color_name",
};

static TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_black] = anon_sym_black,
  [anon_sym_darkgrey] = anon_sym_darkgrey,
  [anon_sym_dark_DASHgrey] = anon_sym_dark_DASHgrey,
  [anon_sym_dark_grey] = anon_sym_dark_grey,
  [anon_sym_red] = anon_sym_red,
  [anon_sym_darkred] = anon_sym_darkred,
  [anon_sym_dark_DASHred] = anon_sym_dark_DASHred,
  [anon_sym_dark_red] = anon_sym_dark_red,
  [anon_sym_green] = anon_sym_green,
  [anon_sym_darkgreen] = anon_sym_darkgreen,
  [anon_sym_dark_DASHgreen] = anon_sym_dark_DASHgreen,
  [anon_sym_dark_green] = anon_sym_dark_green,
  [anon_sym_yellow] = anon_sym_yellow,
  [anon_sym_darkyellow] = anon_sym_darkyellow,
  [anon_sym_dark_DASHyellow] = anon_sym_dark_DASHyellow,
  [anon_sym_dark_yellow] = anon_sym_dark_yellow,
  [anon_sym_blue] = anon_sym_blue,
  [anon_sym_darkblue] = anon_sym_darkblue,
  [anon_sym_dark_DASHblue] = anon_sym_dark_DASHblue,
  [anon_sym_dark_blue] = anon_sym_dark_blue,
  [anon_sym_magenta] = anon_sym_magenta,
  [anon_sym_darkmagenta] = anon_sym_darkmagenta,
  [anon_sym_dark_DASHmagenta] = anon_sym_dark_DASHmagenta,
  [anon_sym_dark_magenta] = anon_sym_dark_magenta,
  [anon_sym_cyan] = anon_sym_cyan,
  [anon_sym_darkcyan] = anon_sym_darkcyan,
  [anon_sym_dark_DASHcyan] = anon_sym_dark_DASHcyan,
  [anon_sym_dark_cyan] = anon_sym_dark_cyan,
  [anon_sym_white] = anon_sym_white,
  [anon_sym_grey] = anon_sym_grey,
  [sym_color_name] = sym_color_name,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_black] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_darkgrey] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_dark_DASHgrey] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_dark_grey] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_red] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_darkred] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_dark_DASHred] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_dark_red] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_green] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_darkgreen] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_dark_DASHgreen] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_dark_green] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_yellow] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_darkyellow] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_dark_DASHyellow] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_dark_yellow] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_blue] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_darkblue] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_dark_DASHblue] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_dark_blue] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_magenta] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_darkmagenta] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_dark_DASHmagenta] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_dark_magenta] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_cyan] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_darkcyan] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_dark_DASHcyan] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_dark_cyan] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_white] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_grey] = {
    .visible = true,
    .named = false,
  },
  [sym_color_name] = {
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
      if (eof) ADVANCE(105);
      if (lookahead == 'b') ADVANCE(56);
      if (lookahead == 'c') ADVANCE(101);
      if (lookahead == 'd') ADVANCE(9);
      if (lookahead == 'g') ADVANCE(85);
      if (lookahead == 'm') ADVANCE(2);
      if (lookahead == 'r') ADVANCE(23);
      if (lookahead == 'w') ADVANCE(52);
      if (lookahead == 'y') ADVANCE(30);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      END_STATE();
    case 1:
      if (lookahead == '-') ADVANCE(16);
      if (lookahead == '_') ADVANCE(17);
      if (lookahead == 'b') ADVANCE(58);
      if (lookahead == 'c') ADVANCE(102);
      if (lookahead == 'g') ADVANCE(86);
      if (lookahead == 'm') ADVANCE(13);
      if (lookahead == 'r') ADVANCE(31);
      if (lookahead == 'y') ADVANCE(45);
      END_STATE();
    case 2:
      if (lookahead == 'a') ADVANCE(48);
      END_STATE();
    case 3:
      if (lookahead == 'a') ADVANCE(18);
      if (lookahead == 'u') ADVANCE(25);
      END_STATE();
    case 4:
      if (lookahead == 'a') ADVANCE(68);
      END_STATE();
    case 5:
      if (lookahead == 'a') ADVANCE(126);
      END_STATE();
    case 6:
      if (lookahead == 'a') ADVANCE(127);
      END_STATE();
    case 7:
      if (lookahead == 'a') ADVANCE(128);
      END_STATE();
    case 8:
      if (lookahead == 'a') ADVANCE(129);
      END_STATE();
    case 9:
      if (lookahead == 'a') ADVANCE(84);
      END_STATE();
    case 10:
      if (lookahead == 'a') ADVANCE(70);
      END_STATE();
    case 11:
      if (lookahead == 'a') ADVANCE(71);
      END_STATE();
    case 12:
      if (lookahead == 'a') ADVANCE(72);
      END_STATE();
    case 13:
      if (lookahead == 'a') ADVANCE(49);
      END_STATE();
    case 14:
      if (lookahead == 'a') ADVANCE(50);
      END_STATE();
    case 15:
      if (lookahead == 'a') ADVANCE(51);
      END_STATE();
    case 16:
      if (lookahead == 'b') ADVANCE(61);
      if (lookahead == 'c') ADVANCE(103);
      if (lookahead == 'g') ADVANCE(87);
      if (lookahead == 'm') ADVANCE(14);
      if (lookahead == 'r') ADVANCE(34);
      if (lookahead == 'y') ADVANCE(46);
      END_STATE();
    case 17:
      if (lookahead == 'b') ADVANCE(64);
      if (lookahead == 'c') ADVANCE(104);
      if (lookahead == 'g') ADVANCE(88);
      if (lookahead == 'm') ADVANCE(15);
      if (lookahead == 'r') ADVANCE(37);
      if (lookahead == 'y') ADVANCE(47);
      END_STATE();
    case 18:
      if (lookahead == 'c') ADVANCE(55);
      END_STATE();
    case 19:
      if (lookahead == 'd') ADVANCE(110);
      END_STATE();
    case 20:
      if (lookahead == 'd') ADVANCE(111);
      END_STATE();
    case 21:
      if (lookahead == 'd') ADVANCE(112);
      END_STATE();
    case 22:
      if (lookahead == 'd') ADVANCE(113);
      END_STATE();
    case 23:
      if (lookahead == 'e') ADVANCE(19);
      END_STATE();
    case 24:
      if (lookahead == 'e') ADVANCE(32);
      END_STATE();
    case 25:
      if (lookahead == 'e') ADVANCE(122);
      END_STATE();
    case 26:
      if (lookahead == 'e') ADVANCE(134);
      END_STATE();
    case 27:
      if (lookahead == 'e') ADVANCE(123);
      END_STATE();
    case 28:
      if (lookahead == 'e') ADVANCE(124);
      END_STATE();
    case 29:
      if (lookahead == 'e') ADVANCE(125);
      END_STATE();
    case 30:
      if (lookahead == 'e') ADVANCE(59);
      END_STATE();
    case 31:
      if (lookahead == 'e') ADVANCE(20);
      END_STATE();
    case 32:
      if (lookahead == 'e') ADVANCE(69);
      if (lookahead == 'y') ADVANCE(135);
      END_STATE();
    case 33:
      if (lookahead == 'e') ADVANCE(39);
      END_STATE();
    case 34:
      if (lookahead == 'e') ADVANCE(21);
      END_STATE();
    case 35:
      if (lookahead == 'e') ADVANCE(76);
      END_STATE();
    case 36:
      if (lookahead == 'e') ADVANCE(40);
      END_STATE();
    case 37:
      if (lookahead == 'e') ADVANCE(22);
      END_STATE();
    case 38:
      if (lookahead == 'e') ADVANCE(41);
      END_STATE();
    case 39:
      if (lookahead == 'e') ADVANCE(73);
      if (lookahead == 'y') ADVANCE(107);
      END_STATE();
    case 40:
      if (lookahead == 'e') ADVANCE(74);
      if (lookahead == 'y') ADVANCE(108);
      END_STATE();
    case 41:
      if (lookahead == 'e') ADVANCE(75);
      if (lookahead == 'y') ADVANCE(109);
      END_STATE();
    case 42:
      if (lookahead == 'e') ADVANCE(77);
      END_STATE();
    case 43:
      if (lookahead == 'e') ADVANCE(78);
      END_STATE();
    case 44:
      if (lookahead == 'e') ADVANCE(79);
      END_STATE();
    case 45:
      if (lookahead == 'e') ADVANCE(62);
      END_STATE();
    case 46:
      if (lookahead == 'e') ADVANCE(65);
      END_STATE();
    case 47:
      if (lookahead == 'e') ADVANCE(67);
      END_STATE();
    case 48:
      if (lookahead == 'g') ADVANCE(35);
      END_STATE();
    case 49:
      if (lookahead == 'g') ADVANCE(42);
      END_STATE();
    case 50:
      if (lookahead == 'g') ADVANCE(43);
      END_STATE();
    case 51:
      if (lookahead == 'g') ADVANCE(44);
      END_STATE();
    case 52:
      if (lookahead == 'h') ADVANCE(53);
      END_STATE();
    case 53:
      if (lookahead == 'i') ADVANCE(91);
      END_STATE();
    case 54:
      if (lookahead == 'k') ADVANCE(1);
      END_STATE();
    case 55:
      if (lookahead == 'k') ADVANCE(106);
      END_STATE();
    case 56:
      if (lookahead == 'l') ADVANCE(3);
      END_STATE();
    case 57:
      if (lookahead == 'l') ADVANCE(80);
      END_STATE();
    case 58:
      if (lookahead == 'l') ADVANCE(94);
      END_STATE();
    case 59:
      if (lookahead == 'l') ADVANCE(57);
      END_STATE();
    case 60:
      if (lookahead == 'l') ADVANCE(81);
      END_STATE();
    case 61:
      if (lookahead == 'l') ADVANCE(95);
      END_STATE();
    case 62:
      if (lookahead == 'l') ADVANCE(60);
      END_STATE();
    case 63:
      if (lookahead == 'l') ADVANCE(82);
      END_STATE();
    case 64:
      if (lookahead == 'l') ADVANCE(96);
      END_STATE();
    case 65:
      if (lookahead == 'l') ADVANCE(63);
      END_STATE();
    case 66:
      if (lookahead == 'l') ADVANCE(83);
      END_STATE();
    case 67:
      if (lookahead == 'l') ADVANCE(66);
      END_STATE();
    case 68:
      if (lookahead == 'n') ADVANCE(130);
      END_STATE();
    case 69:
      if (lookahead == 'n') ADVANCE(114);
      END_STATE();
    case 70:
      if (lookahead == 'n') ADVANCE(131);
      END_STATE();
    case 71:
      if (lookahead == 'n') ADVANCE(132);
      END_STATE();
    case 72:
      if (lookahead == 'n') ADVANCE(133);
      END_STATE();
    case 73:
      if (lookahead == 'n') ADVANCE(115);
      END_STATE();
    case 74:
      if (lookahead == 'n') ADVANCE(116);
      END_STATE();
    case 75:
      if (lookahead == 'n') ADVANCE(117);
      END_STATE();
    case 76:
      if (lookahead == 'n') ADVANCE(89);
      END_STATE();
    case 77:
      if (lookahead == 'n') ADVANCE(90);
      END_STATE();
    case 78:
      if (lookahead == 'n') ADVANCE(92);
      END_STATE();
    case 79:
      if (lookahead == 'n') ADVANCE(93);
      END_STATE();
    case 80:
      if (lookahead == 'o') ADVANCE(97);
      END_STATE();
    case 81:
      if (lookahead == 'o') ADVANCE(98);
      END_STATE();
    case 82:
      if (lookahead == 'o') ADVANCE(99);
      END_STATE();
    case 83:
      if (lookahead == 'o') ADVANCE(100);
      END_STATE();
    case 84:
      if (lookahead == 'r') ADVANCE(54);
      END_STATE();
    case 85:
      if (lookahead == 'r') ADVANCE(24);
      END_STATE();
    case 86:
      if (lookahead == 'r') ADVANCE(33);
      END_STATE();
    case 87:
      if (lookahead == 'r') ADVANCE(36);
      END_STATE();
    case 88:
      if (lookahead == 'r') ADVANCE(38);
      END_STATE();
    case 89:
      if (lookahead == 't') ADVANCE(5);
      END_STATE();
    case 90:
      if (lookahead == 't') ADVANCE(6);
      END_STATE();
    case 91:
      if (lookahead == 't') ADVANCE(26);
      END_STATE();
    case 92:
      if (lookahead == 't') ADVANCE(7);
      END_STATE();
    case 93:
      if (lookahead == 't') ADVANCE(8);
      END_STATE();
    case 94:
      if (lookahead == 'u') ADVANCE(27);
      END_STATE();
    case 95:
      if (lookahead == 'u') ADVANCE(28);
      END_STATE();
    case 96:
      if (lookahead == 'u') ADVANCE(29);
      END_STATE();
    case 97:
      if (lookahead == 'w') ADVANCE(118);
      END_STATE();
    case 98:
      if (lookahead == 'w') ADVANCE(119);
      END_STATE();
    case 99:
      if (lookahead == 'w') ADVANCE(120);
      END_STATE();
    case 100:
      if (lookahead == 'w') ADVANCE(121);
      END_STATE();
    case 101:
      if (lookahead == 'y') ADVANCE(4);
      END_STATE();
    case 102:
      if (lookahead == 'y') ADVANCE(10);
      END_STATE();
    case 103:
      if (lookahead == 'y') ADVANCE(11);
      END_STATE();
    case 104:
      if (lookahead == 'y') ADVANCE(12);
      END_STATE();
    case 105:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 106:
      ACCEPT_TOKEN(anon_sym_black);
      END_STATE();
    case 107:
      ACCEPT_TOKEN(anon_sym_darkgrey);
      END_STATE();
    case 108:
      ACCEPT_TOKEN(anon_sym_dark_DASHgrey);
      END_STATE();
    case 109:
      ACCEPT_TOKEN(anon_sym_dark_grey);
      END_STATE();
    case 110:
      ACCEPT_TOKEN(anon_sym_red);
      END_STATE();
    case 111:
      ACCEPT_TOKEN(anon_sym_darkred);
      END_STATE();
    case 112:
      ACCEPT_TOKEN(anon_sym_dark_DASHred);
      END_STATE();
    case 113:
      ACCEPT_TOKEN(anon_sym_dark_red);
      END_STATE();
    case 114:
      ACCEPT_TOKEN(anon_sym_green);
      END_STATE();
    case 115:
      ACCEPT_TOKEN(anon_sym_darkgreen);
      END_STATE();
    case 116:
      ACCEPT_TOKEN(anon_sym_dark_DASHgreen);
      END_STATE();
    case 117:
      ACCEPT_TOKEN(anon_sym_dark_green);
      END_STATE();
    case 118:
      ACCEPT_TOKEN(anon_sym_yellow);
      END_STATE();
    case 119:
      ACCEPT_TOKEN(anon_sym_darkyellow);
      END_STATE();
    case 120:
      ACCEPT_TOKEN(anon_sym_dark_DASHyellow);
      END_STATE();
    case 121:
      ACCEPT_TOKEN(anon_sym_dark_yellow);
      END_STATE();
    case 122:
      ACCEPT_TOKEN(anon_sym_blue);
      END_STATE();
    case 123:
      ACCEPT_TOKEN(anon_sym_darkblue);
      END_STATE();
    case 124:
      ACCEPT_TOKEN(anon_sym_dark_DASHblue);
      END_STATE();
    case 125:
      ACCEPT_TOKEN(anon_sym_dark_blue);
      END_STATE();
    case 126:
      ACCEPT_TOKEN(anon_sym_magenta);
      END_STATE();
    case 127:
      ACCEPT_TOKEN(anon_sym_darkmagenta);
      END_STATE();
    case 128:
      ACCEPT_TOKEN(anon_sym_dark_DASHmagenta);
      END_STATE();
    case 129:
      ACCEPT_TOKEN(anon_sym_dark_magenta);
      END_STATE();
    case 130:
      ACCEPT_TOKEN(anon_sym_cyan);
      END_STATE();
    case 131:
      ACCEPT_TOKEN(anon_sym_darkcyan);
      END_STATE();
    case 132:
      ACCEPT_TOKEN(anon_sym_dark_DASHcyan);
      END_STATE();
    case 133:
      ACCEPT_TOKEN(anon_sym_dark_cyan);
      END_STATE();
    case 134:
      ACCEPT_TOKEN(anon_sym_white);
      END_STATE();
    case 135:
      ACCEPT_TOKEN(anon_sym_grey);
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
};

static uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_black] = ACTIONS(1),
    [anon_sym_darkgrey] = ACTIONS(1),
    [anon_sym_dark_DASHgrey] = ACTIONS(1),
    [anon_sym_dark_grey] = ACTIONS(1),
    [anon_sym_red] = ACTIONS(1),
    [anon_sym_darkred] = ACTIONS(1),
    [anon_sym_dark_DASHred] = ACTIONS(1),
    [anon_sym_dark_red] = ACTIONS(1),
    [anon_sym_green] = ACTIONS(1),
    [anon_sym_darkgreen] = ACTIONS(1),
    [anon_sym_dark_DASHgreen] = ACTIONS(1),
    [anon_sym_dark_green] = ACTIONS(1),
    [anon_sym_yellow] = ACTIONS(1),
    [anon_sym_darkyellow] = ACTIONS(1),
    [anon_sym_dark_DASHyellow] = ACTIONS(1),
    [anon_sym_dark_yellow] = ACTIONS(1),
    [anon_sym_blue] = ACTIONS(1),
    [anon_sym_darkblue] = ACTIONS(1),
    [anon_sym_dark_DASHblue] = ACTIONS(1),
    [anon_sym_dark_blue] = ACTIONS(1),
    [anon_sym_magenta] = ACTIONS(1),
    [anon_sym_darkmagenta] = ACTIONS(1),
    [anon_sym_dark_DASHmagenta] = ACTIONS(1),
    [anon_sym_dark_magenta] = ACTIONS(1),
    [anon_sym_cyan] = ACTIONS(1),
    [anon_sym_darkcyan] = ACTIONS(1),
    [anon_sym_dark_DASHcyan] = ACTIONS(1),
    [anon_sym_dark_cyan] = ACTIONS(1),
    [anon_sym_white] = ACTIONS(1),
    [anon_sym_grey] = ACTIONS(1),
  },
  [1] = {
    [sym_color_name] = STATE(3),
    [anon_sym_black] = ACTIONS(3),
    [anon_sym_darkgrey] = ACTIONS(3),
    [anon_sym_dark_DASHgrey] = ACTIONS(3),
    [anon_sym_dark_grey] = ACTIONS(3),
    [anon_sym_red] = ACTIONS(3),
    [anon_sym_darkred] = ACTIONS(3),
    [anon_sym_dark_DASHred] = ACTIONS(3),
    [anon_sym_dark_red] = ACTIONS(3),
    [anon_sym_green] = ACTIONS(3),
    [anon_sym_darkgreen] = ACTIONS(3),
    [anon_sym_dark_DASHgreen] = ACTIONS(3),
    [anon_sym_dark_green] = ACTIONS(3),
    [anon_sym_yellow] = ACTIONS(3),
    [anon_sym_darkyellow] = ACTIONS(3),
    [anon_sym_dark_DASHyellow] = ACTIONS(3),
    [anon_sym_dark_yellow] = ACTIONS(3),
    [anon_sym_blue] = ACTIONS(3),
    [anon_sym_darkblue] = ACTIONS(3),
    [anon_sym_dark_DASHblue] = ACTIONS(3),
    [anon_sym_dark_blue] = ACTIONS(3),
    [anon_sym_magenta] = ACTIONS(3),
    [anon_sym_darkmagenta] = ACTIONS(3),
    [anon_sym_dark_DASHmagenta] = ACTIONS(3),
    [anon_sym_dark_magenta] = ACTIONS(3),
    [anon_sym_cyan] = ACTIONS(3),
    [anon_sym_darkcyan] = ACTIONS(3),
    [anon_sym_dark_DASHcyan] = ACTIONS(3),
    [anon_sym_dark_cyan] = ACTIONS(3),
    [anon_sym_white] = ACTIONS(3),
    [anon_sym_grey] = ACTIONS(3),
  },
};

static uint16_t ts_small_parse_table[] = {
  [0] = 1,
    ACTIONS(5), 1,
      ts_builtin_sym_end,
  [4] = 1,
    ACTIONS(7), 1,
      ts_builtin_sym_end,
};

static uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 4,
};

static TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_color_name, 1),
  [7] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
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
