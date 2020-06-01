#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 11
#define STATE_COUNT 59
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 81
#define ALIAS_COUNT 0
#define TOKEN_COUNT 53
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 3
#define MAX_ALIAS_SEQUENCE_LENGTH 3

enum {
  anon_sym_COMMA = 1,
  anon_sym_GT = 2,
  anon_sym_PLUS = 3,
  anon_sym_DASH = 4,
  anon_sym_DOT = 5,
  aux_sym_symbol_name_token1 = 6,
  anon_sym_LBRACE = 7,
  anon_sym_RBRACE = 8,
  anon_sym_fg = 9,
  anon_sym_COLON = 10,
  anon_sym_bg = 11,
  anon_sym_attr = 12,
  anon_sym_attribute = 13,
  anon_sym_PIPE = 14,
  anon_sym_bold = 15,
  anon_sym_italic = 16,
  anon_sym_underlined = 17,
  anon_sym_underline = 18,
  anon_sym_reverse = 19,
  sym_rgb_color = 20,
  aux_sym_ansi_color_token1 = 21,
  aux_sym_ansi_color_token2 = 22,
  anon_sym_black = 23,
  anon_sym_darkgrey = 24,
  anon_sym_dark_DASHgrey = 25,
  anon_sym_dark_grey = 26,
  anon_sym_red = 27,
  anon_sym_darkred = 28,
  anon_sym_dark_DASHred = 29,
  anon_sym_dark_red = 30,
  anon_sym_green = 31,
  anon_sym_darkgreen = 32,
  anon_sym_dark_DASHgreen = 33,
  anon_sym_dark_green = 34,
  anon_sym_yellow = 35,
  anon_sym_darkyellow = 36,
  anon_sym_dark_DASHyellow = 37,
  anon_sym_dark_yellow = 38,
  anon_sym_blue = 39,
  anon_sym_darkblue = 40,
  anon_sym_dark_DASHblue = 41,
  anon_sym_dark_blue = 42,
  anon_sym_magenta = 43,
  anon_sym_darkmagenta = 44,
  anon_sym_dark_DASHmagenta = 45,
  anon_sym_dark_magenta = 46,
  anon_sym_cyan = 47,
  anon_sym_darkcyan = 48,
  anon_sym_dark_DASHcyan = 49,
  anon_sym_dark_cyan = 50,
  anon_sym_white = 51,
  anon_sym_grey = 52,
  sym_source_file = 53,
  sym_selectors = 54,
  sym_selector = 55,
  sym_sel_symbol = 56,
  sym_sel_child = 57,
  sym_sel_next_child = 58,
  sym_sel_prev_child = 59,
  sym_sel_field = 60,
  sym_sel_symbol_field = 61,
  sym_symbol_name = 62,
  sym_field_name = 63,
  sym_properties = 64,
  sym_property = 65,
  sym_fg = 66,
  sym_bg = 67,
  sym_attr1 = 68,
  sym_attr2 = 69,
  sym_attrs = 70,
  sym_attr_or = 71,
  sym_attr = 72,
  sym_ansi_color = 73,
  sym_color_name = 74,
  aux_sym_source_file_repeat1 = 75,
  aux_sym_selectors_repeat1 = 76,
  aux_sym_selector_repeat1 = 77,
  aux_sym_properties_repeat1 = 78,
  aux_sym_attr1_repeat1 = 79,
  aux_sym_attrs_repeat1 = 80,
};

static const char *ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_COMMA] = ",",
  [anon_sym_GT] = ">",
  [anon_sym_PLUS] = "+",
  [anon_sym_DASH] = "-",
  [anon_sym_DOT] = ".",
  [aux_sym_symbol_name_token1] = "symbol_name_token1",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_fg] = "fg",
  [anon_sym_COLON] = ":",
  [anon_sym_bg] = "bg",
  [anon_sym_attr] = "attr",
  [anon_sym_attribute] = "attribute",
  [anon_sym_PIPE] = "|",
  [anon_sym_bold] = "bold",
  [anon_sym_italic] = "italic",
  [anon_sym_underlined] = "underlined",
  [anon_sym_underline] = "underline",
  [anon_sym_reverse] = "reverse",
  [sym_rgb_color] = "rgb_color",
  [aux_sym_ansi_color_token1] = "ansi_color_token1",
  [aux_sym_ansi_color_token2] = "ansi_color_token2",
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
  [sym_source_file] = "source_file",
  [sym_selectors] = "selectors",
  [sym_selector] = "selector",
  [sym_sel_symbol] = "sel_symbol",
  [sym_sel_child] = "sel_child",
  [sym_sel_next_child] = "sel_next_child",
  [sym_sel_prev_child] = "sel_prev_child",
  [sym_sel_field] = "sel_field",
  [sym_sel_symbol_field] = "sel_symbol_field",
  [sym_symbol_name] = "symbol_name",
  [sym_field_name] = "field_name",
  [sym_properties] = "properties",
  [sym_property] = "property",
  [sym_fg] = "fg",
  [sym_bg] = "bg",
  [sym_attr1] = "attr1",
  [sym_attr2] = "attr2",
  [sym_attrs] = "attrs",
  [sym_attr_or] = "attr_or",
  [sym_attr] = "attr",
  [sym_ansi_color] = "ansi_color",
  [sym_color_name] = "color_name",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_selectors_repeat1] = "selectors_repeat1",
  [aux_sym_selector_repeat1] = "selector_repeat1",
  [aux_sym_properties_repeat1] = "properties_repeat1",
  [aux_sym_attr1_repeat1] = "attr1_repeat1",
  [aux_sym_attrs_repeat1] = "attrs_repeat1",
};

static TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_GT] = anon_sym_GT,
  [anon_sym_PLUS] = anon_sym_PLUS,
  [anon_sym_DASH] = anon_sym_DASH,
  [anon_sym_DOT] = anon_sym_DOT,
  [aux_sym_symbol_name_token1] = aux_sym_symbol_name_token1,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_fg] = anon_sym_fg,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_bg] = anon_sym_bg,
  [anon_sym_attr] = anon_sym_attr,
  [anon_sym_attribute] = anon_sym_attribute,
  [anon_sym_PIPE] = anon_sym_PIPE,
  [anon_sym_bold] = anon_sym_bold,
  [anon_sym_italic] = anon_sym_italic,
  [anon_sym_underlined] = anon_sym_underlined,
  [anon_sym_underline] = anon_sym_underline,
  [anon_sym_reverse] = anon_sym_reverse,
  [sym_rgb_color] = sym_rgb_color,
  [aux_sym_ansi_color_token1] = aux_sym_ansi_color_token1,
  [aux_sym_ansi_color_token2] = aux_sym_ansi_color_token2,
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
  [sym_source_file] = sym_source_file,
  [sym_selectors] = sym_selectors,
  [sym_selector] = sym_selector,
  [sym_sel_symbol] = sym_sel_symbol,
  [sym_sel_child] = sym_sel_child,
  [sym_sel_next_child] = sym_sel_next_child,
  [sym_sel_prev_child] = sym_sel_prev_child,
  [sym_sel_field] = sym_sel_field,
  [sym_sel_symbol_field] = sym_sel_symbol_field,
  [sym_symbol_name] = sym_symbol_name,
  [sym_field_name] = sym_field_name,
  [sym_properties] = sym_properties,
  [sym_property] = sym_property,
  [sym_fg] = sym_fg,
  [sym_bg] = sym_bg,
  [sym_attr1] = sym_attr1,
  [sym_attr2] = sym_attr2,
  [sym_attrs] = sym_attrs,
  [sym_attr_or] = sym_attr_or,
  [sym_attr] = sym_attr,
  [sym_ansi_color] = sym_ansi_color,
  [sym_color_name] = sym_color_name,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_selectors_repeat1] = aux_sym_selectors_repeat1,
  [aux_sym_selector_repeat1] = aux_sym_selector_repeat1,
  [aux_sym_properties_repeat1] = aux_sym_properties_repeat1,
  [aux_sym_attr1_repeat1] = aux_sym_attr1_repeat1,
  [aux_sym_attrs_repeat1] = aux_sym_attrs_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_symbol_name_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_fg] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_bg] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_attr] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_attribute] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PIPE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_bold] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_italic] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_underlined] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_underline] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_reverse] = {
    .visible = true,
    .named = false,
  },
  [sym_rgb_color] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_ansi_color_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_ansi_color_token2] = {
    .visible = false,
    .named = false,
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
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym_selectors] = {
    .visible = true,
    .named = true,
  },
  [sym_selector] = {
    .visible = true,
    .named = true,
  },
  [sym_sel_symbol] = {
    .visible = true,
    .named = true,
  },
  [sym_sel_child] = {
    .visible = true,
    .named = true,
  },
  [sym_sel_next_child] = {
    .visible = true,
    .named = true,
  },
  [sym_sel_prev_child] = {
    .visible = true,
    .named = true,
  },
  [sym_sel_field] = {
    .visible = true,
    .named = true,
  },
  [sym_sel_symbol_field] = {
    .visible = true,
    .named = true,
  },
  [sym_symbol_name] = {
    .visible = true,
    .named = true,
  },
  [sym_field_name] = {
    .visible = true,
    .named = true,
  },
  [sym_properties] = {
    .visible = true,
    .named = true,
  },
  [sym_property] = {
    .visible = true,
    .named = true,
  },
  [sym_fg] = {
    .visible = true,
    .named = true,
  },
  [sym_bg] = {
    .visible = true,
    .named = true,
  },
  [sym_attr1] = {
    .visible = true,
    .named = true,
  },
  [sym_attr2] = {
    .visible = true,
    .named = true,
  },
  [sym_attrs] = {
    .visible = true,
    .named = true,
  },
  [sym_attr_or] = {
    .visible = true,
    .named = true,
  },
  [sym_attr] = {
    .visible = true,
    .named = true,
  },
  [sym_ansi_color] = {
    .visible = true,
    .named = true,
  },
  [sym_color_name] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_selectors_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_selector_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_properties_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_attr1_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_attrs_repeat1] = {
    .visible = false,
    .named = false,
  },
};

enum {
  field_ansi_color_dec = 1,
  field_ansi_color_hex = 2,
  field_properties = 3,
};

static const char *ts_field_names[] = {
  [0] = NULL,
  [field_ansi_color_dec] = "ansi_color_dec",
  [field_ansi_color_hex] = "ansi_color_hex",
  [field_properties] = "properties",
};

static const TSFieldMapSlice ts_field_map_slices[4] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 1},
  [3] = {.index = 2, .length = 1},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_properties, 1},
  [1] =
    {field_ansi_color_dec, 0},
  [2] =
    {field_ansi_color_hex, 0},
};

static TSSymbol ts_alias_sequences[4][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(144);
      if (lookahead == '#') ADVANCE(141);
      if (lookahead == '+') ADVANCE(147);
      if (lookahead == ',') ADVANCE(145);
      if (lookahead == '-') ADVANCE(148);
      if (lookahead == '.') ADVANCE(149);
      if (lookahead == '0') ADVANCE(165);
      if (lookahead == ':') ADVANCE(154);
      if (lookahead == '>') ADVANCE(146);
      if (lookahead == 'b') ADVANCE(60);
      if (lookahead == 'f') ADVANCE(62);
      if (lookahead == '{') ADVANCE(151);
      if (lookahead == '|') ADVANCE(158);
      if (lookahead == '}') ADVANCE(152);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(166);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(142);
      END_STATE();
    case 1:
      if (lookahead == '#') ADVANCE(141);
      if (lookahead == ',') ADVANCE(145);
      if (lookahead == '0') ADVANCE(165);
      if (lookahead == 'a') ADVANCE(114);
      if (lookahead == 'b') ADVANCE(59);
      if (lookahead == 'c') ADVANCE(131);
      if (lookahead == 'd') ADVANCE(9);
      if (lookahead == 'f') ADVANCE(61);
      if (lookahead == 'g') ADVANCE(108);
      if (lookahead == 'i') ADVANCE(116);
      if (lookahead == 'm') ADVANCE(8);
      if (lookahead == 'r') ADVANCE(29);
      if (lookahead == 'u') ADVANCE(87);
      if (lookahead == 'w') ADVANCE(67);
      if (lookahead == 'y') ADVANCE(39);
      if (lookahead == '|') ADVANCE(158);
      if (lookahead == '}') ADVANCE(152);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(1)
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(166);
      END_STATE();
    case 2:
      if (lookahead == '-') ADVANCE(19);
      if (lookahead == '_') ADVANCE(20);
      if (lookahead == 'b') ADVANCE(76);
      if (lookahead == 'c') ADVANCE(132);
      if (lookahead == 'g') ADVANCE(110);
      if (lookahead == 'm') ADVANCE(15);
      if (lookahead == 'r') ADVANCE(42);
      if (lookahead == 'y') ADVANCE(53);
      END_STATE();
    case 3:
      if (lookahead == 'a') ADVANCE(22);
      if (lookahead == 'u') ADVANCE(31);
      END_STATE();
    case 4:
      if (lookahead == 'a') ADVANCE(188);
      END_STATE();
    case 5:
      if (lookahead == 'a') ADVANCE(189);
      END_STATE();
    case 6:
      if (lookahead == 'a') ADVANCE(190);
      END_STATE();
    case 7:
      if (lookahead == 'a') ADVANCE(191);
      END_STATE();
    case 8:
      if (lookahead == 'a') ADVANCE(63);
      END_STATE();
    case 9:
      if (lookahead == 'a') ADVANCE(105);
      END_STATE();
    case 10:
      if (lookahead == 'a') ADVANCE(88);
      END_STATE();
    case 11:
      if (lookahead == 'a') ADVANCE(75);
      END_STATE();
    case 12:
      if (lookahead == 'a') ADVANCE(90);
      END_STATE();
    case 13:
      if (lookahead == 'a') ADVANCE(91);
      END_STATE();
    case 14:
      if (lookahead == 'a') ADVANCE(92);
      END_STATE();
    case 15:
      if (lookahead == 'a') ADVANCE(64);
      END_STATE();
    case 16:
      if (lookahead == 'a') ADVANCE(65);
      END_STATE();
    case 17:
      if (lookahead == 'a') ADVANCE(66);
      END_STATE();
    case 18:
      if (lookahead == 'b') ADVANCE(126);
      END_STATE();
    case 19:
      if (lookahead == 'b') ADVANCE(80);
      if (lookahead == 'c') ADVANCE(133);
      if (lookahead == 'g') ADVANCE(111);
      if (lookahead == 'm') ADVANCE(16);
      if (lookahead == 'r') ADVANCE(46);
      if (lookahead == 'y') ADVANCE(55);
      END_STATE();
    case 20:
      if (lookahead == 'b') ADVANCE(83);
      if (lookahead == 'c') ADVANCE(134);
      if (lookahead == 'g') ADVANCE(112);
      if (lookahead == 'm') ADVANCE(17);
      if (lookahead == 'r') ADVANCE(49);
      if (lookahead == 'y') ADVANCE(57);
      END_STATE();
    case 21:
      if (lookahead == 'c') ADVANCE(160);
      END_STATE();
    case 22:
      if (lookahead == 'c') ADVANCE(72);
      END_STATE();
    case 23:
      if (lookahead == 'd') ADVANCE(172);
      if (lookahead == 'v') ADVANCE(44);
      END_STATE();
    case 24:
      if (lookahead == 'd') ADVANCE(159);
      END_STATE();
    case 25:
      if (lookahead == 'd') ADVANCE(173);
      END_STATE();
    case 26:
      if (lookahead == 'd') ADVANCE(174);
      END_STATE();
    case 27:
      if (lookahead == 'd') ADVANCE(175);
      END_STATE();
    case 28:
      if (lookahead == 'd') ADVANCE(48);
      END_STATE();
    case 29:
      if (lookahead == 'e') ADVANCE(23);
      END_STATE();
    case 30:
      if (lookahead == 'e') ADVANCE(41);
      END_STATE();
    case 31:
      if (lookahead == 'e') ADVANCE(184);
      END_STATE();
    case 32:
      if (lookahead == 'e') ADVANCE(196);
      END_STATE();
    case 33:
      if (lookahead == 'e') ADVANCE(163);
      END_STATE();
    case 34:
      if (lookahead == 'e') ADVANCE(185);
      END_STATE();
    case 35:
      if (lookahead == 'e') ADVANCE(157);
      END_STATE();
    case 36:
      if (lookahead == 'e') ADVANCE(186);
      END_STATE();
    case 37:
      if (lookahead == 'e') ADVANCE(187);
      END_STATE();
    case 38:
      if (lookahead == 'e') ADVANCE(162);
      END_STATE();
    case 39:
      if (lookahead == 'e') ADVANCE(78);
      END_STATE();
    case 40:
      if (lookahead == 'e') ADVANCE(50);
      END_STATE();
    case 41:
      if (lookahead == 'e') ADVANCE(89);
      if (lookahead == 'y') ADVANCE(197);
      END_STATE();
    case 42:
      if (lookahead == 'e') ADVANCE(25);
      END_STATE();
    case 43:
      if (lookahead == 'e') ADVANCE(51);
      END_STATE();
    case 44:
      if (lookahead == 'e') ADVANCE(107);
      END_STATE();
    case 45:
      if (lookahead == 'e') ADVANCE(97);
      END_STATE();
    case 46:
      if (lookahead == 'e') ADVANCE(26);
      END_STATE();
    case 47:
      if (lookahead == 'e') ADVANCE(52);
      END_STATE();
    case 48:
      if (lookahead == 'e') ADVANCE(109);
      END_STATE();
    case 49:
      if (lookahead == 'e') ADVANCE(27);
      END_STATE();
    case 50:
      if (lookahead == 'e') ADVANCE(93);
      if (lookahead == 'y') ADVANCE(169);
      END_STATE();
    case 51:
      if (lookahead == 'e') ADVANCE(94);
      if (lookahead == 'y') ADVANCE(170);
      END_STATE();
    case 52:
      if (lookahead == 'e') ADVANCE(95);
      if (lookahead == 'y') ADVANCE(171);
      END_STATE();
    case 53:
      if (lookahead == 'e') ADVANCE(81);
      END_STATE();
    case 54:
      if (lookahead == 'e') ADVANCE(98);
      END_STATE();
    case 55:
      if (lookahead == 'e') ADVANCE(84);
      END_STATE();
    case 56:
      if (lookahead == 'e') ADVANCE(99);
      END_STATE();
    case 57:
      if (lookahead == 'e') ADVANCE(86);
      END_STATE();
    case 58:
      if (lookahead == 'e') ADVANCE(100);
      END_STATE();
    case 59:
      if (lookahead == 'g') ADVANCE(155);
      if (lookahead == 'l') ADVANCE(3);
      if (lookahead == 'o') ADVANCE(74);
      END_STATE();
    case 60:
      if (lookahead == 'g') ADVANCE(155);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(150);
      END_STATE();
    case 61:
      if (lookahead == 'g') ADVANCE(153);
      END_STATE();
    case 62:
      if (lookahead == 'g') ADVANCE(153);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(150);
      END_STATE();
    case 63:
      if (lookahead == 'g') ADVANCE(45);
      END_STATE();
    case 64:
      if (lookahead == 'g') ADVANCE(54);
      END_STATE();
    case 65:
      if (lookahead == 'g') ADVANCE(56);
      END_STATE();
    case 66:
      if (lookahead == 'g') ADVANCE(58);
      END_STATE();
    case 67:
      if (lookahead == 'h') ADVANCE(69);
      END_STATE();
    case 68:
      if (lookahead == 'i') ADVANCE(21);
      END_STATE();
    case 69:
      if (lookahead == 'i') ADVANCE(120);
      END_STATE();
    case 70:
      if (lookahead == 'i') ADVANCE(96);
      END_STATE();
    case 71:
      if (lookahead == 'k') ADVANCE(2);
      END_STATE();
    case 72:
      if (lookahead == 'k') ADVANCE(168);
      END_STATE();
    case 73:
      if (lookahead == 'l') ADVANCE(101);
      END_STATE();
    case 74:
      if (lookahead == 'l') ADVANCE(24);
      END_STATE();
    case 75:
      if (lookahead == 'l') ADVANCE(68);
      END_STATE();
    case 76:
      if (lookahead == 'l') ADVANCE(123);
      END_STATE();
    case 77:
      if (lookahead == 'l') ADVANCE(70);
      END_STATE();
    case 78:
      if (lookahead == 'l') ADVANCE(73);
      END_STATE();
    case 79:
      if (lookahead == 'l') ADVANCE(102);
      END_STATE();
    case 80:
      if (lookahead == 'l') ADVANCE(124);
      END_STATE();
    case 81:
      if (lookahead == 'l') ADVANCE(79);
      END_STATE();
    case 82:
      if (lookahead == 'l') ADVANCE(103);
      END_STATE();
    case 83:
      if (lookahead == 'l') ADVANCE(125);
      END_STATE();
    case 84:
      if (lookahead == 'l') ADVANCE(82);
      END_STATE();
    case 85:
      if (lookahead == 'l') ADVANCE(104);
      END_STATE();
    case 86:
      if (lookahead == 'l') ADVANCE(85);
      END_STATE();
    case 87:
      if (lookahead == 'n') ADVANCE(28);
      END_STATE();
    case 88:
      if (lookahead == 'n') ADVANCE(192);
      END_STATE();
    case 89:
      if (lookahead == 'n') ADVANCE(176);
      END_STATE();
    case 90:
      if (lookahead == 'n') ADVANCE(193);
      END_STATE();
    case 91:
      if (lookahead == 'n') ADVANCE(194);
      END_STATE();
    case 92:
      if (lookahead == 'n') ADVANCE(195);
      END_STATE();
    case 93:
      if (lookahead == 'n') ADVANCE(177);
      END_STATE();
    case 94:
      if (lookahead == 'n') ADVANCE(178);
      END_STATE();
    case 95:
      if (lookahead == 'n') ADVANCE(179);
      END_STATE();
    case 96:
      if (lookahead == 'n') ADVANCE(38);
      END_STATE();
    case 97:
      if (lookahead == 'n') ADVANCE(117);
      END_STATE();
    case 98:
      if (lookahead == 'n') ADVANCE(118);
      END_STATE();
    case 99:
      if (lookahead == 'n') ADVANCE(119);
      END_STATE();
    case 100:
      if (lookahead == 'n') ADVANCE(121);
      END_STATE();
    case 101:
      if (lookahead == 'o') ADVANCE(127);
      END_STATE();
    case 102:
      if (lookahead == 'o') ADVANCE(128);
      END_STATE();
    case 103:
      if (lookahead == 'o') ADVANCE(129);
      END_STATE();
    case 104:
      if (lookahead == 'o') ADVANCE(130);
      END_STATE();
    case 105:
      if (lookahead == 'r') ADVANCE(71);
      END_STATE();
    case 106:
      if (lookahead == 'r') ADVANCE(156);
      END_STATE();
    case 107:
      if (lookahead == 'r') ADVANCE(113);
      END_STATE();
    case 108:
      if (lookahead == 'r') ADVANCE(30);
      END_STATE();
    case 109:
      if (lookahead == 'r') ADVANCE(77);
      END_STATE();
    case 110:
      if (lookahead == 'r') ADVANCE(40);
      END_STATE();
    case 111:
      if (lookahead == 'r') ADVANCE(43);
      END_STATE();
    case 112:
      if (lookahead == 'r') ADVANCE(47);
      END_STATE();
    case 113:
      if (lookahead == 's') ADVANCE(33);
      END_STATE();
    case 114:
      if (lookahead == 't') ADVANCE(115);
      END_STATE();
    case 115:
      if (lookahead == 't') ADVANCE(106);
      END_STATE();
    case 116:
      if (lookahead == 't') ADVANCE(11);
      END_STATE();
    case 117:
      if (lookahead == 't') ADVANCE(4);
      END_STATE();
    case 118:
      if (lookahead == 't') ADVANCE(5);
      END_STATE();
    case 119:
      if (lookahead == 't') ADVANCE(6);
      END_STATE();
    case 120:
      if (lookahead == 't') ADVANCE(32);
      END_STATE();
    case 121:
      if (lookahead == 't') ADVANCE(7);
      END_STATE();
    case 122:
      if (lookahead == 't') ADVANCE(35);
      END_STATE();
    case 123:
      if (lookahead == 'u') ADVANCE(34);
      END_STATE();
    case 124:
      if (lookahead == 'u') ADVANCE(36);
      END_STATE();
    case 125:
      if (lookahead == 'u') ADVANCE(37);
      END_STATE();
    case 126:
      if (lookahead == 'u') ADVANCE(122);
      END_STATE();
    case 127:
      if (lookahead == 'w') ADVANCE(180);
      END_STATE();
    case 128:
      if (lookahead == 'w') ADVANCE(181);
      END_STATE();
    case 129:
      if (lookahead == 'w') ADVANCE(182);
      END_STATE();
    case 130:
      if (lookahead == 'w') ADVANCE(183);
      END_STATE();
    case 131:
      if (lookahead == 'y') ADVANCE(10);
      END_STATE();
    case 132:
      if (lookahead == 'y') ADVANCE(12);
      END_STATE();
    case 133:
      if (lookahead == 'y') ADVANCE(13);
      END_STATE();
    case 134:
      if (lookahead == 'y') ADVANCE(14);
      END_STATE();
    case 135:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(167);
      END_STATE();
    case 136:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(164);
      END_STATE();
    case 137:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(136);
      END_STATE();
    case 138:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(137);
      END_STATE();
    case 139:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(138);
      END_STATE();
    case 140:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(139);
      END_STATE();
    case 141:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(140);
      END_STATE();
    case 142:
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(150);
      END_STATE();
    case 143:
      if (eof) ADVANCE(144);
      if (lookahead == '+') ADVANCE(147);
      if (lookahead == ',') ADVANCE(145);
      if (lookahead == '-') ADVANCE(148);
      if (lookahead == '.') ADVANCE(149);
      if (lookahead == '>') ADVANCE(146);
      if (lookahead == '{') ADVANCE(151);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(143)
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(142);
      END_STATE();
    case 144:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 145:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 146:
      ACCEPT_TOKEN(anon_sym_GT);
      END_STATE();
    case 147:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 148:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 149:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 150:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      END_STATE();
    case 151:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 152:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 153:
      ACCEPT_TOKEN(anon_sym_fg);
      END_STATE();
    case 154:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 155:
      ACCEPT_TOKEN(anon_sym_bg);
      END_STATE();
    case 156:
      ACCEPT_TOKEN(anon_sym_attr);
      if (lookahead == 'i') ADVANCE(18);
      END_STATE();
    case 157:
      ACCEPT_TOKEN(anon_sym_attribute);
      END_STATE();
    case 158:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 159:
      ACCEPT_TOKEN(anon_sym_bold);
      END_STATE();
    case 160:
      ACCEPT_TOKEN(anon_sym_italic);
      END_STATE();
    case 161:
      ACCEPT_TOKEN(anon_sym_underlined);
      END_STATE();
    case 162:
      ACCEPT_TOKEN(anon_sym_underline);
      if (lookahead == 'd') ADVANCE(161);
      END_STATE();
    case 163:
      ACCEPT_TOKEN(anon_sym_reverse);
      END_STATE();
    case 164:
      ACCEPT_TOKEN(sym_rgb_color);
      END_STATE();
    case 165:
      ACCEPT_TOKEN(aux_sym_ansi_color_token1);
      if (lookahead == 'x') ADVANCE(135);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(166);
      END_STATE();
    case 166:
      ACCEPT_TOKEN(aux_sym_ansi_color_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(166);
      END_STATE();
    case 167:
      ACCEPT_TOKEN(aux_sym_ansi_color_token2);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(167);
      END_STATE();
    case 168:
      ACCEPT_TOKEN(anon_sym_black);
      END_STATE();
    case 169:
      ACCEPT_TOKEN(anon_sym_darkgrey);
      END_STATE();
    case 170:
      ACCEPT_TOKEN(anon_sym_dark_DASHgrey);
      END_STATE();
    case 171:
      ACCEPT_TOKEN(anon_sym_dark_grey);
      END_STATE();
    case 172:
      ACCEPT_TOKEN(anon_sym_red);
      END_STATE();
    case 173:
      ACCEPT_TOKEN(anon_sym_darkred);
      END_STATE();
    case 174:
      ACCEPT_TOKEN(anon_sym_dark_DASHred);
      END_STATE();
    case 175:
      ACCEPT_TOKEN(anon_sym_dark_red);
      END_STATE();
    case 176:
      ACCEPT_TOKEN(anon_sym_green);
      END_STATE();
    case 177:
      ACCEPT_TOKEN(anon_sym_darkgreen);
      END_STATE();
    case 178:
      ACCEPT_TOKEN(anon_sym_dark_DASHgreen);
      END_STATE();
    case 179:
      ACCEPT_TOKEN(anon_sym_dark_green);
      END_STATE();
    case 180:
      ACCEPT_TOKEN(anon_sym_yellow);
      END_STATE();
    case 181:
      ACCEPT_TOKEN(anon_sym_darkyellow);
      END_STATE();
    case 182:
      ACCEPT_TOKEN(anon_sym_dark_DASHyellow);
      END_STATE();
    case 183:
      ACCEPT_TOKEN(anon_sym_dark_yellow);
      END_STATE();
    case 184:
      ACCEPT_TOKEN(anon_sym_blue);
      END_STATE();
    case 185:
      ACCEPT_TOKEN(anon_sym_darkblue);
      END_STATE();
    case 186:
      ACCEPT_TOKEN(anon_sym_dark_DASHblue);
      END_STATE();
    case 187:
      ACCEPT_TOKEN(anon_sym_dark_blue);
      END_STATE();
    case 188:
      ACCEPT_TOKEN(anon_sym_magenta);
      END_STATE();
    case 189:
      ACCEPT_TOKEN(anon_sym_darkmagenta);
      END_STATE();
    case 190:
      ACCEPT_TOKEN(anon_sym_dark_DASHmagenta);
      END_STATE();
    case 191:
      ACCEPT_TOKEN(anon_sym_dark_magenta);
      END_STATE();
    case 192:
      ACCEPT_TOKEN(anon_sym_cyan);
      END_STATE();
    case 193:
      ACCEPT_TOKEN(anon_sym_darkcyan);
      END_STATE();
    case 194:
      ACCEPT_TOKEN(anon_sym_dark_DASHcyan);
      END_STATE();
    case 195:
      ACCEPT_TOKEN(anon_sym_dark_cyan);
      END_STATE();
    case 196:
      ACCEPT_TOKEN(anon_sym_white);
      END_STATE();
    case 197:
      ACCEPT_TOKEN(anon_sym_grey);
      END_STATE();
    default:
      return false;
  }
}

static TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 143},
  [2] = {.lex_state = 1},
  [3] = {.lex_state = 1},
  [4] = {.lex_state = 143},
  [5] = {.lex_state = 143},
  [6] = {.lex_state = 143},
  [7] = {.lex_state = 143},
  [8] = {.lex_state = 1},
  [9] = {.lex_state = 1},
  [10] = {.lex_state = 143},
  [11] = {.lex_state = 1},
  [12] = {.lex_state = 143},
  [13] = {.lex_state = 1},
  [14] = {.lex_state = 1},
  [15] = {.lex_state = 1},
  [16] = {.lex_state = 1},
  [17] = {.lex_state = 1},
  [18] = {.lex_state = 1},
  [19] = {.lex_state = 143},
  [20] = {.lex_state = 143},
  [21] = {.lex_state = 1},
  [22] = {.lex_state = 1},
  [23] = {.lex_state = 143},
  [24] = {.lex_state = 1},
  [25] = {.lex_state = 143},
  [26] = {.lex_state = 143},
  [27] = {.lex_state = 143},
  [28] = {.lex_state = 143},
  [29] = {.lex_state = 1},
  [30] = {.lex_state = 143},
  [31] = {.lex_state = 143},
  [32] = {.lex_state = 143},
  [33] = {.lex_state = 143},
  [34] = {.lex_state = 143},
  [35] = {.lex_state = 1},
  [36] = {.lex_state = 1},
  [37] = {.lex_state = 143},
  [38] = {.lex_state = 0},
  [39] = {.lex_state = 0},
  [40] = {.lex_state = 143},
  [41] = {.lex_state = 143},
  [42] = {.lex_state = 0},
  [43] = {.lex_state = 0},
  [44] = {.lex_state = 143},
  [45] = {.lex_state = 0},
  [46] = {.lex_state = 143},
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
};

static uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_GT] = ACTIONS(1),
    [anon_sym_PLUS] = ACTIONS(1),
    [anon_sym_DASH] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [aux_sym_symbol_name_token1] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_fg] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_bg] = ACTIONS(1),
    [anon_sym_PIPE] = ACTIONS(1),
    [sym_rgb_color] = ACTIONS(1),
    [aux_sym_ansi_color_token1] = ACTIONS(1),
    [aux_sym_ansi_color_token2] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(58),
    [sym_selectors] = STATE(45),
    [sym_selector] = STATE(39),
    [sym_sel_symbol] = STATE(23),
    [sym_sel_child] = STATE(27),
    [sym_sel_next_child] = STATE(27),
    [sym_sel_prev_child] = STATE(27),
    [sym_sel_field] = STATE(27),
    [sym_sel_symbol_field] = STATE(27),
    [sym_symbol_name] = STATE(28),
    [aux_sym_source_file_repeat1] = STATE(5),
    [aux_sym_selector_repeat1] = STATE(7),
    [ts_builtin_sym_end] = ACTIONS(3),
    [anon_sym_DOT] = ACTIONS(5),
    [aux_sym_symbol_name_token1] = ACTIONS(7),
  },
};

static uint16_t ts_small_parse_table[] = {
  [0] = 5,
    ACTIONS(9), 1,
      sym_rgb_color,
    ACTIONS(11), 1,
      aux_sym_ansi_color_token1,
    ACTIONS(13), 1,
      aux_sym_ansi_color_token2,
    STATE(49), 2,
      sym_ansi_color,
      sym_color_name,
    ACTIONS(15), 30,
      anon_sym_black,
      anon_sym_darkgrey,
      anon_sym_dark_DASHgrey,
      anon_sym_dark_grey,
      anon_sym_red,
      anon_sym_darkred,
      anon_sym_dark_DASHred,
      anon_sym_dark_red,
      anon_sym_green,
      anon_sym_darkgreen,
      anon_sym_dark_DASHgreen,
      anon_sym_dark_green,
      anon_sym_yellow,
      anon_sym_darkyellow,
      anon_sym_dark_DASHyellow,
      anon_sym_dark_yellow,
      anon_sym_blue,
      anon_sym_darkblue,
      anon_sym_dark_DASHblue,
      anon_sym_dark_blue,
      anon_sym_magenta,
      anon_sym_darkmagenta,
      anon_sym_dark_DASHmagenta,
      anon_sym_dark_magenta,
      anon_sym_cyan,
      anon_sym_darkcyan,
      anon_sym_dark_DASHcyan,
      anon_sym_dark_cyan,
      anon_sym_white,
      anon_sym_grey,
  [46] = 5,
    ACTIONS(11), 1,
      aux_sym_ansi_color_token1,
    ACTIONS(13), 1,
      aux_sym_ansi_color_token2,
    ACTIONS(17), 1,
      sym_rgb_color,
    STATE(55), 2,
      sym_ansi_color,
      sym_color_name,
    ACTIONS(15), 30,
      anon_sym_black,
      anon_sym_darkgrey,
      anon_sym_dark_DASHgrey,
      anon_sym_dark_grey,
      anon_sym_red,
      anon_sym_darkred,
      anon_sym_dark_DASHred,
      anon_sym_dark_red,
      anon_sym_green,
      anon_sym_darkgreen,
      anon_sym_dark_DASHgreen,
      anon_sym_dark_green,
      anon_sym_yellow,
      anon_sym_darkyellow,
      anon_sym_dark_DASHyellow,
      anon_sym_dark_yellow,
      anon_sym_blue,
      anon_sym_darkblue,
      anon_sym_dark_DASHblue,
      anon_sym_dark_blue,
      anon_sym_magenta,
      anon_sym_darkmagenta,
      anon_sym_dark_DASHmagenta,
      anon_sym_dark_magenta,
      anon_sym_cyan,
      anon_sym_darkcyan,
      anon_sym_dark_DASHcyan,
      anon_sym_dark_cyan,
      anon_sym_white,
      anon_sym_grey,
  [92] = 10,
    ACTIONS(19), 1,
      ts_builtin_sym_end,
    ACTIONS(21), 1,
      anon_sym_DOT,
    ACTIONS(24), 1,
      aux_sym_symbol_name_token1,
    STATE(4), 1,
      aux_sym_source_file_repeat1,
    STATE(7), 1,
      aux_sym_selector_repeat1,
    STATE(23), 1,
      sym_sel_symbol,
    STATE(28), 1,
      sym_symbol_name,
    STATE(39), 1,
      sym_selector,
    STATE(45), 1,
      sym_selectors,
    STATE(27), 5,
      sym_sel_child,
      sym_sel_next_child,
      sym_sel_prev_child,
      sym_sel_field,
      sym_sel_symbol_field,
  [127] = 10,
    ACTIONS(5), 1,
      anon_sym_DOT,
    ACTIONS(7), 1,
      aux_sym_symbol_name_token1,
    ACTIONS(27), 1,
      ts_builtin_sym_end,
    STATE(4), 1,
      aux_sym_source_file_repeat1,
    STATE(7), 1,
      aux_sym_selector_repeat1,
    STATE(23), 1,
      sym_sel_symbol,
    STATE(28), 1,
      sym_symbol_name,
    STATE(39), 1,
      sym_selector,
    STATE(45), 1,
      sym_selectors,
    STATE(27), 5,
      sym_sel_child,
      sym_sel_next_child,
      sym_sel_prev_child,
      sym_sel_field,
      sym_sel_symbol_field,
  [162] = 7,
    ACTIONS(31), 1,
      anon_sym_DOT,
    ACTIONS(34), 1,
      aux_sym_symbol_name_token1,
    STATE(6), 1,
      aux_sym_selector_repeat1,
    STATE(23), 1,
      sym_sel_symbol,
    STATE(28), 1,
      sym_symbol_name,
    ACTIONS(29), 2,
      anon_sym_COMMA,
      anon_sym_LBRACE,
    STATE(27), 5,
      sym_sel_child,
      sym_sel_next_child,
      sym_sel_prev_child,
      sym_sel_field,
      sym_sel_symbol_field,
  [189] = 7,
    ACTIONS(5), 1,
      anon_sym_DOT,
    ACTIONS(7), 1,
      aux_sym_symbol_name_token1,
    STATE(6), 1,
      aux_sym_selector_repeat1,
    STATE(23), 1,
      sym_sel_symbol,
    STATE(28), 1,
      sym_symbol_name,
    ACTIONS(37), 2,
      anon_sym_COMMA,
      anon_sym_LBRACE,
    STATE(27), 5,
      sym_sel_child,
      sym_sel_next_child,
      sym_sel_prev_child,
      sym_sel_field,
      sym_sel_symbol_field,
  [216] = 8,
    ACTIONS(39), 1,
      anon_sym_RBRACE,
    ACTIONS(41), 1,
      anon_sym_fg,
    ACTIONS(43), 1,
      anon_sym_bg,
    ACTIONS(45), 1,
      anon_sym_attr,
    ACTIONS(47), 1,
      anon_sym_attribute,
    STATE(9), 1,
      aux_sym_properties_repeat1,
    STATE(52), 1,
      sym_property,
    STATE(57), 4,
      sym_fg,
      sym_bg,
      sym_attr1,
      sym_attr2,
  [244] = 8,
    ACTIONS(49), 1,
      anon_sym_RBRACE,
    ACTIONS(51), 1,
      anon_sym_fg,
    ACTIONS(54), 1,
      anon_sym_bg,
    ACTIONS(57), 1,
      anon_sym_attr,
    ACTIONS(60), 1,
      anon_sym_attribute,
    STATE(9), 1,
      aux_sym_properties_repeat1,
    STATE(52), 1,
      sym_property,
    STATE(57), 4,
      sym_fg,
      sym_bg,
      sym_attr1,
      sym_attr2,
  [272] = 7,
    ACTIONS(5), 1,
      anon_sym_DOT,
    ACTIONS(7), 1,
      aux_sym_symbol_name_token1,
    STATE(7), 1,
      aux_sym_selector_repeat1,
    STATE(23), 1,
      sym_sel_symbol,
    STATE(28), 1,
      sym_symbol_name,
    STATE(43), 1,
      sym_selector,
    STATE(27), 5,
      sym_sel_child,
      sym_sel_next_child,
      sym_sel_prev_child,
      sym_sel_field,
      sym_sel_symbol_field,
  [298] = 8,
    ACTIONS(41), 1,
      anon_sym_fg,
    ACTIONS(43), 1,
      anon_sym_bg,
    ACTIONS(45), 1,
      anon_sym_attr,
    ACTIONS(47), 1,
      anon_sym_attribute,
    ACTIONS(63), 1,
      anon_sym_RBRACE,
    STATE(8), 1,
      aux_sym_properties_repeat1,
    STATE(52), 1,
      sym_property,
    STATE(57), 4,
      sym_fg,
      sym_bg,
      sym_attr1,
      sym_attr2,
  [326] = 5,
    ACTIONS(5), 1,
      anon_sym_DOT,
    ACTIONS(7), 1,
      aux_sym_symbol_name_token1,
    STATE(28), 1,
      sym_symbol_name,
    STATE(30), 1,
      sym_sel_symbol,
    STATE(27), 5,
      sym_sel_child,
      sym_sel_next_child,
      sym_sel_prev_child,
      sym_sel_field,
      sym_sel_symbol_field,
  [346] = 5,
    ACTIONS(65), 1,
      anon_sym_COMMA,
    ACTIONS(69), 1,
      anon_sym_underline,
    STATE(17), 1,
      sym_attr,
    STATE(18), 2,
      sym_attrs,
      aux_sym_attr1_repeat1,
    ACTIONS(67), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [366] = 5,
    ACTIONS(69), 1,
      anon_sym_underline,
    ACTIONS(71), 1,
      anon_sym_COMMA,
    STATE(17), 1,
      sym_attr,
    STATE(21), 2,
      sym_attrs,
      aux_sym_attr1_repeat1,
    ACTIONS(67), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [386] = 5,
    ACTIONS(73), 1,
      anon_sym_COMMA,
    ACTIONS(78), 1,
      anon_sym_underline,
    STATE(17), 1,
      sym_attr,
    STATE(15), 2,
      sym_attrs,
      aux_sym_attr1_repeat1,
    ACTIONS(75), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [406] = 4,
    ACTIONS(83), 1,
      anon_sym_PIPE,
    ACTIONS(85), 1,
      anon_sym_underline,
    STATE(22), 2,
      sym_attr_or,
      aux_sym_attrs_repeat1,
    ACTIONS(81), 5,
      anon_sym_COMMA,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [424] = 4,
    ACTIONS(83), 1,
      anon_sym_PIPE,
    ACTIONS(89), 1,
      anon_sym_underline,
    STATE(16), 2,
      sym_attr_or,
      aux_sym_attrs_repeat1,
    ACTIONS(87), 5,
      anon_sym_COMMA,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [442] = 5,
    ACTIONS(69), 1,
      anon_sym_underline,
    ACTIONS(91), 1,
      anon_sym_COMMA,
    STATE(17), 1,
      sym_attr,
    STATE(15), 2,
      sym_attrs,
      aux_sym_attr1_repeat1,
    ACTIONS(67), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [462] = 5,
    ACTIONS(5), 1,
      anon_sym_DOT,
    ACTIONS(7), 1,
      aux_sym_symbol_name_token1,
    STATE(28), 1,
      sym_symbol_name,
    STATE(31), 1,
      sym_sel_symbol,
    STATE(27), 5,
      sym_sel_child,
      sym_sel_next_child,
      sym_sel_prev_child,
      sym_sel_field,
      sym_sel_symbol_field,
  [482] = 5,
    ACTIONS(5), 1,
      anon_sym_DOT,
    ACTIONS(7), 1,
      aux_sym_symbol_name_token1,
    STATE(28), 1,
      sym_symbol_name,
    STATE(32), 1,
      sym_sel_symbol,
    STATE(27), 5,
      sym_sel_child,
      sym_sel_next_child,
      sym_sel_prev_child,
      sym_sel_field,
      sym_sel_symbol_field,
  [502] = 5,
    ACTIONS(69), 1,
      anon_sym_underline,
    ACTIONS(93), 1,
      anon_sym_COMMA,
    STATE(17), 1,
      sym_attr,
    STATE(15), 2,
      sym_attrs,
      aux_sym_attr1_repeat1,
    ACTIONS(67), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [522] = 4,
    ACTIONS(97), 1,
      anon_sym_PIPE,
    ACTIONS(100), 1,
      anon_sym_underline,
    STATE(22), 2,
      sym_attr_or,
      aux_sym_attrs_repeat1,
    ACTIONS(95), 5,
      anon_sym_COMMA,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [540] = 4,
    ACTIONS(104), 1,
      anon_sym_GT,
    ACTIONS(106), 1,
      anon_sym_PLUS,
    ACTIONS(108), 1,
      anon_sym_DASH,
    ACTIONS(102), 4,
      anon_sym_COMMA,
      anon_sym_DOT,
      aux_sym_symbol_name_token1,
      anon_sym_LBRACE,
  [556] = 2,
    ACTIONS(112), 1,
      anon_sym_underline,
    ACTIONS(110), 6,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [568] = 1,
    ACTIONS(114), 7,
      anon_sym_COMMA,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_DOT,
      aux_sym_symbol_name_token1,
      anon_sym_LBRACE,
  [578] = 1,
    ACTIONS(116), 7,
      anon_sym_COMMA,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_DOT,
      aux_sym_symbol_name_token1,
      anon_sym_LBRACE,
  [588] = 1,
    ACTIONS(118), 7,
      anon_sym_COMMA,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_DOT,
      aux_sym_symbol_name_token1,
      anon_sym_LBRACE,
  [598] = 2,
    ACTIONS(120), 1,
      anon_sym_DOT,
    ACTIONS(118), 6,
      anon_sym_COMMA,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_DASH,
      aux_sym_symbol_name_token1,
      anon_sym_LBRACE,
  [610] = 2,
    ACTIONS(124), 1,
      anon_sym_underline,
    ACTIONS(122), 6,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [622] = 1,
    ACTIONS(126), 7,
      anon_sym_COMMA,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_DOT,
      aux_sym_symbol_name_token1,
      anon_sym_LBRACE,
  [632] = 1,
    ACTIONS(128), 7,
      anon_sym_COMMA,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_DOT,
      aux_sym_symbol_name_token1,
      anon_sym_LBRACE,
  [642] = 1,
    ACTIONS(130), 7,
      anon_sym_COMMA,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_DOT,
      aux_sym_symbol_name_token1,
      anon_sym_LBRACE,
  [652] = 1,
    ACTIONS(132), 7,
      anon_sym_COMMA,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_DOT,
      aux_sym_symbol_name_token1,
      anon_sym_LBRACE,
  [662] = 1,
    ACTIONS(134), 7,
      anon_sym_COMMA,
      anon_sym_GT,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_DOT,
      aux_sym_symbol_name_token1,
      anon_sym_LBRACE,
  [672] = 3,
    ACTIONS(69), 1,
      anon_sym_underline,
    STATE(29), 1,
      sym_attr,
    ACTIONS(67), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [685] = 2,
    ACTIONS(136), 1,
      anon_sym_attr,
    ACTIONS(49), 4,
      anon_sym_RBRACE,
      anon_sym_fg,
      anon_sym_bg,
      anon_sym_attribute,
  [695] = 1,
    ACTIONS(138), 3,
      ts_builtin_sym_end,
      anon_sym_DOT,
      aux_sym_symbol_name_token1,
  [701] = 3,
    ACTIONS(140), 1,
      anon_sym_COMMA,
    ACTIONS(143), 1,
      anon_sym_LBRACE,
    STATE(38), 1,
      aux_sym_selectors_repeat1,
  [711] = 3,
    ACTIONS(145), 1,
      anon_sym_COMMA,
    ACTIONS(147), 1,
      anon_sym_LBRACE,
    STATE(42), 1,
      aux_sym_selectors_repeat1,
  [721] = 1,
    ACTIONS(149), 3,
      ts_builtin_sym_end,
      anon_sym_DOT,
      aux_sym_symbol_name_token1,
  [727] = 1,
    ACTIONS(19), 3,
      ts_builtin_sym_end,
      anon_sym_DOT,
      aux_sym_symbol_name_token1,
  [733] = 3,
    ACTIONS(145), 1,
      anon_sym_COMMA,
    ACTIONS(151), 1,
      anon_sym_LBRACE,
    STATE(38), 1,
      aux_sym_selectors_repeat1,
  [743] = 1,
    ACTIONS(143), 2,
      anon_sym_COMMA,
      anon_sym_LBRACE,
  [748] = 2,
    ACTIONS(153), 1,
      aux_sym_symbol_name_token1,
    STATE(25), 1,
      sym_field_name,
  [755] = 2,
    ACTIONS(155), 1,
      anon_sym_LBRACE,
    STATE(41), 1,
      sym_properties,
  [762] = 2,
    ACTIONS(153), 1,
      aux_sym_symbol_name_token1,
    STATE(33), 1,
      sym_field_name,
  [769] = 1,
    ACTIONS(157), 1,
      anon_sym_COLON,
  [773] = 1,
    ACTIONS(159), 1,
      anon_sym_COMMA,
  [777] = 1,
    ACTIONS(161), 1,
      anon_sym_COMMA,
  [781] = 1,
    ACTIONS(163), 1,
      anon_sym_COLON,
  [785] = 1,
    ACTIONS(165), 1,
      anon_sym_COLON,
  [789] = 1,
    ACTIONS(167), 1,
      anon_sym_COMMA,
  [793] = 1,
    ACTIONS(169), 1,
      anon_sym_COMMA,
  [797] = 1,
    ACTIONS(171), 1,
      anon_sym_COMMA,
  [801] = 1,
    ACTIONS(173), 1,
      anon_sym_COMMA,
  [805] = 1,
    ACTIONS(175), 1,
      anon_sym_COLON,
  [809] = 1,
    ACTIONS(177), 1,
      anon_sym_COMMA,
  [813] = 1,
    ACTIONS(179), 1,
      ts_builtin_sym_end,
};

static uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 46,
  [SMALL_STATE(4)] = 92,
  [SMALL_STATE(5)] = 127,
  [SMALL_STATE(6)] = 162,
  [SMALL_STATE(7)] = 189,
  [SMALL_STATE(8)] = 216,
  [SMALL_STATE(9)] = 244,
  [SMALL_STATE(10)] = 272,
  [SMALL_STATE(11)] = 298,
  [SMALL_STATE(12)] = 326,
  [SMALL_STATE(13)] = 346,
  [SMALL_STATE(14)] = 366,
  [SMALL_STATE(15)] = 386,
  [SMALL_STATE(16)] = 406,
  [SMALL_STATE(17)] = 424,
  [SMALL_STATE(18)] = 442,
  [SMALL_STATE(19)] = 462,
  [SMALL_STATE(20)] = 482,
  [SMALL_STATE(21)] = 502,
  [SMALL_STATE(22)] = 522,
  [SMALL_STATE(23)] = 540,
  [SMALL_STATE(24)] = 556,
  [SMALL_STATE(25)] = 568,
  [SMALL_STATE(26)] = 578,
  [SMALL_STATE(27)] = 588,
  [SMALL_STATE(28)] = 598,
  [SMALL_STATE(29)] = 610,
  [SMALL_STATE(30)] = 622,
  [SMALL_STATE(31)] = 632,
  [SMALL_STATE(32)] = 642,
  [SMALL_STATE(33)] = 652,
  [SMALL_STATE(34)] = 662,
  [SMALL_STATE(35)] = 672,
  [SMALL_STATE(36)] = 685,
  [SMALL_STATE(37)] = 695,
  [SMALL_STATE(38)] = 701,
  [SMALL_STATE(39)] = 711,
  [SMALL_STATE(40)] = 721,
  [SMALL_STATE(41)] = 727,
  [SMALL_STATE(42)] = 733,
  [SMALL_STATE(43)] = 743,
  [SMALL_STATE(44)] = 748,
  [SMALL_STATE(45)] = 755,
  [SMALL_STATE(46)] = 762,
  [SMALL_STATE(47)] = 769,
  [SMALL_STATE(48)] = 773,
  [SMALL_STATE(49)] = 777,
  [SMALL_STATE(50)] = 781,
  [SMALL_STATE(51)] = 785,
  [SMALL_STATE(52)] = 789,
  [SMALL_STATE(53)] = 793,
  [SMALL_STATE(54)] = 797,
  [SMALL_STATE(55)] = 801,
  [SMALL_STATE(56)] = 805,
  [SMALL_STATE(57)] = 809,
  [SMALL_STATE(58)] = 813,
};

static TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [11] = {.entry = {.count = 1, .reusable = false}}, SHIFT(54),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [19] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [21] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(44),
  [24] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(34),
  [27] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [29] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_selector_repeat1, 2),
  [31] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_selector_repeat1, 2), SHIFT_REPEAT(44),
  [34] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_selector_repeat1, 2), SHIFT_REPEAT(34),
  [37] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selector, 1),
  [39] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [45] = {.entry = {.count = 1, .reusable = false}}, SHIFT(56),
  [47] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [49] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_properties_repeat1, 2),
  [51] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_properties_repeat1, 2), SHIFT_REPEAT(51),
  [54] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_properties_repeat1, 2), SHIFT_REPEAT(50),
  [57] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_properties_repeat1, 2), SHIFT_REPEAT(56),
  [60] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_properties_repeat1, 2), SHIFT_REPEAT(47),
  [63] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [65] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attr2, 2),
  [67] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [69] = {.entry = {.count = 1, .reusable = false}}, SHIFT(24),
  [71] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attr1, 2),
  [73] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_attr1_repeat1, 2),
  [75] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_attr1_repeat1, 2), SHIFT_REPEAT(24),
  [78] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_attr1_repeat1, 2), SHIFT_REPEAT(24),
  [81] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attrs, 2),
  [83] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [85] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attrs, 2),
  [87] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attrs, 1),
  [89] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attrs, 1),
  [91] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attr2, 3),
  [93] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attr1, 3),
  [95] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_attrs_repeat1, 2),
  [97] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_attrs_repeat1, 2), SHIFT_REPEAT(35),
  [100] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_attrs_repeat1, 2),
  [102] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_selector_repeat1, 1),
  [104] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [106] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [108] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [110] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attr, 1),
  [112] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attr, 1),
  [114] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_field, 2),
  [116] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field_name, 1),
  [118] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_symbol, 1),
  [120] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [122] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attr_or, 2),
  [124] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attr_or, 2),
  [126] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_child, 3),
  [128] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_next_child, 3),
  [130] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_prev_child, 3),
  [132] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_symbol_field, 3),
  [134] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_symbol_name, 1),
  [136] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_properties_repeat1, 2),
  [138] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_properties, 2),
  [140] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_selectors_repeat1, 2), SHIFT_REPEAT(10),
  [143] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_selectors_repeat1, 2),
  [145] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [147] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selectors, 1),
  [149] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_properties, 3, .production_id = 1),
  [151] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selectors, 2),
  [153] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [155] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [157] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [159] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_color_name, 1),
  [161] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_bg, 3),
  [163] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [165] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [167] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [169] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ansi_color, 1, .production_id = 3),
  [171] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ansi_color, 1, .production_id = 2),
  [173] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fg, 3),
  [175] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [177] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property, 1),
  [179] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
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
    .field_names = ts_field_names,
    .field_map_slices = (const TSFieldMapSlice *)ts_field_map_slices,
    .field_map_entries = (const TSFieldMapEntry *)ts_field_map_entries,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .lex_fn = ts_lex,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
