#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#ifdef _MSC_VER
#pragma optimize("", off)
#elif defined(__clang__)
#pragma clang optimize off
#elif defined(__GNUC__)
#pragma GCC optimize ("O0")
#endif

#define LANGUAGE_VERSION 11
#define STATE_COUNT 3
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 96
#define ALIAS_COUNT 0
#define TOKEN_COUNT 95
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 1

enum {
  aux_sym_comment_token1 = 1,
  anon_sym_COLON = 2,
  anon_sym_SEMI = 3,
  anon_sym_COMMA = 4,
  anon_sym_DOT = 5,
  anon_sym_PLUS = 6,
  anon_sym_TILDE = 7,
  anon_sym_GT = 8,
  anon_sym_LBRACE = 9,
  anon_sym_RBRACE = 10,
  anon_sym_fg = 11,
  anon_sym_bg = 12,
  anon_sym_attr = 13,
  anon_sym_attribute = 14,
  anon_sym_PIPE = 15,
  anon_sym_bold = 16,
  anon_sym_italic = 17,
  anon_sym_underlined = 18,
  anon_sym_underline = 19,
  anon_sym_reverse = 20,
  sym_rgb_color = 21,
  aux_sym_ansi_color_token1 = 22,
  aux_sym_ansi_color_token2 = 23,
  anon_sym_black = 24,
  anon_sym_darkgrey = 25,
  anon_sym_dark_DASHgrey = 26,
  anon_sym_dark_grey = 27,
  anon_sym_red = 28,
  anon_sym_darkred = 29,
  anon_sym_dark_DASHred = 30,
  anon_sym_dark_red = 31,
  anon_sym_green = 32,
  anon_sym_darkgreen = 33,
  anon_sym_dark_DASHgreen = 34,
  anon_sym_dark_green = 35,
  anon_sym_yellow = 36,
  anon_sym_darkyellow = 37,
  anon_sym_dark_DASHyellow = 38,
  anon_sym_dark_yellow = 39,
  anon_sym_blue = 40,
  anon_sym_darkblue = 41,
  anon_sym_dark_DASHblue = 42,
  anon_sym_dark_blue = 43,
  anon_sym_magenta = 44,
  anon_sym_darkmagenta = 45,
  anon_sym_dark_DASHmagenta = 46,
  anon_sym_dark_magenta = 47,
  anon_sym_cyan = 48,
  anon_sym_darkcyan = 49,
  anon_sym_dark_DASHcyan = 50,
  anon_sym_dark_cyan = 51,
  anon_sym_white = 52,
  anon_sym_grey = 53,
  anon_sym_canvas = 54,
  anon_sym_comment = 55,
  anon_sym_constant = 56,
  anon_sym_string = 57,
  anon_sym_char = 58,
  anon_sym_number = 59,
  anon_sym_boolean = 60,
  anon_sym_float = 61,
  anon_sym_identifier = 62,
  anon_sym_function = 63,
  anon_sym_statement = 64,
  anon_sym_conditional = 65,
  anon_sym_repeat = 66,
  anon_sym_label = 67,
  anon_sym_operator = 68,
  anon_sym_keyword = 69,
  anon_sym_exception = 70,
  anon_sym_preproc = 71,
  anon_sym_include = 72,
  anon_sym_define = 73,
  anon_sym_macro = 74,
  anon_sym_precondit = 75,
  anon_sym_type = 76,
  anon_sym_storage_DASHclass = 77,
  anon_sym_structure = 78,
  anon_sym_typedef = 79,
  anon_sym_special = 80,
  anon_sym_special_DASHchar = 81,
  anon_sym_tag = 82,
  anon_sym_delimiter = 83,
  anon_sym_special_DASHcomment = 84,
  anon_sym_debug = 85,
  anon_sym_ignore = 86,
  anon_sym_error = 87,
  anon_sym_todo = 88,
  anon_sym_line_DASHnr = 89,
  anon_sym_prompt = 90,
  anon_sym_status_DASHline = 91,
  anon_sym_tab_DASHline = 92,
  anon_sym_tab_DASHoption = 93,
  anon_sym_tab_DASHselect = 94,
  sym_comment = 95,
};

static const char *ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [aux_sym_comment_token1] = "comment_token1",
  [anon_sym_COLON] = ":",
  [anon_sym_SEMI] = ";",
  [anon_sym_COMMA] = ",",
  [anon_sym_DOT] = ".",
  [anon_sym_PLUS] = "+",
  [anon_sym_TILDE] = "~",
  [anon_sym_GT] = ">",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_fg] = "fg",
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
  [anon_sym_canvas] = "canvas",
  [anon_sym_comment] = "comment",
  [anon_sym_constant] = "constant",
  [anon_sym_string] = "string",
  [anon_sym_char] = "char",
  [anon_sym_number] = "number",
  [anon_sym_boolean] = "boolean",
  [anon_sym_float] = "float",
  [anon_sym_identifier] = "identifier",
  [anon_sym_function] = "function",
  [anon_sym_statement] = "statement",
  [anon_sym_conditional] = "conditional",
  [anon_sym_repeat] = "repeat",
  [anon_sym_label] = "label",
  [anon_sym_operator] = "operator",
  [anon_sym_keyword] = "keyword",
  [anon_sym_exception] = "exception",
  [anon_sym_preproc] = "preproc",
  [anon_sym_include] = "include",
  [anon_sym_define] = "define",
  [anon_sym_macro] = "macro",
  [anon_sym_precondit] = "precondit",
  [anon_sym_type] = "type",
  [anon_sym_storage_DASHclass] = "storage-class",
  [anon_sym_structure] = "structure",
  [anon_sym_typedef] = "typedef",
  [anon_sym_special] = "special",
  [anon_sym_special_DASHchar] = "special-char",
  [anon_sym_tag] = "tag",
  [anon_sym_delimiter] = "delimiter",
  [anon_sym_special_DASHcomment] = "special-comment",
  [anon_sym_debug] = "debug",
  [anon_sym_ignore] = "ignore",
  [anon_sym_error] = "error",
  [anon_sym_todo] = "todo",
  [anon_sym_line_DASHnr] = "line-nr",
  [anon_sym_prompt] = "prompt",
  [anon_sym_status_DASHline] = "status-line",
  [anon_sym_tab_DASHline] = "tab-line",
  [anon_sym_tab_DASHoption] = "tab-option",
  [anon_sym_tab_DASHselect] = "tab-select",
  [sym_comment] = "comment",
};

static TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [aux_sym_comment_token1] = aux_sym_comment_token1,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_PLUS] = anon_sym_PLUS,
  [anon_sym_TILDE] = anon_sym_TILDE,
  [anon_sym_GT] = anon_sym_GT,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_fg] = anon_sym_fg,
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
  [anon_sym_canvas] = anon_sym_canvas,
  [anon_sym_comment] = anon_sym_comment,
  [anon_sym_constant] = anon_sym_constant,
  [anon_sym_string] = anon_sym_string,
  [anon_sym_char] = anon_sym_char,
  [anon_sym_number] = anon_sym_number,
  [anon_sym_boolean] = anon_sym_boolean,
  [anon_sym_float] = anon_sym_float,
  [anon_sym_identifier] = anon_sym_identifier,
  [anon_sym_function] = anon_sym_function,
  [anon_sym_statement] = anon_sym_statement,
  [anon_sym_conditional] = anon_sym_conditional,
  [anon_sym_repeat] = anon_sym_repeat,
  [anon_sym_label] = anon_sym_label,
  [anon_sym_operator] = anon_sym_operator,
  [anon_sym_keyword] = anon_sym_keyword,
  [anon_sym_exception] = anon_sym_exception,
  [anon_sym_preproc] = anon_sym_preproc,
  [anon_sym_include] = anon_sym_include,
  [anon_sym_define] = anon_sym_define,
  [anon_sym_macro] = anon_sym_macro,
  [anon_sym_precondit] = anon_sym_precondit,
  [anon_sym_type] = anon_sym_type,
  [anon_sym_storage_DASHclass] = anon_sym_storage_DASHclass,
  [anon_sym_structure] = anon_sym_structure,
  [anon_sym_typedef] = anon_sym_typedef,
  [anon_sym_special] = anon_sym_special,
  [anon_sym_special_DASHchar] = anon_sym_special_DASHchar,
  [anon_sym_tag] = anon_sym_tag,
  [anon_sym_delimiter] = anon_sym_delimiter,
  [anon_sym_special_DASHcomment] = anon_sym_special_DASHcomment,
  [anon_sym_debug] = anon_sym_debug,
  [anon_sym_ignore] = anon_sym_ignore,
  [anon_sym_error] = anon_sym_error,
  [anon_sym_todo] = anon_sym_todo,
  [anon_sym_line_DASHnr] = anon_sym_line_DASHnr,
  [anon_sym_prompt] = anon_sym_prompt,
  [anon_sym_status_DASHline] = anon_sym_status_DASHline,
  [anon_sym_tab_DASHline] = anon_sym_tab_DASHline,
  [anon_sym_tab_DASHoption] = anon_sym_tab_DASHoption,
  [anon_sym_tab_DASHselect] = anon_sym_tab_DASHselect,
  [sym_comment] = sym_comment,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [aux_sym_comment_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SEMI] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_TILDE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT] = {
    .visible = true,
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
  [anon_sym_canvas] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_comment] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_constant] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_string] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_char] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_number] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_boolean] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_float] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_identifier] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_function] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_statement] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_conditional] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_repeat] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_label] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_operator] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_keyword] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_exception] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_preproc] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_include] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_define] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_macro] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_precondit] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_type] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_storage_DASHclass] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_structure] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_typedef] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_special] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_special_DASHchar] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_tag] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_delimiter] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_special_DASHcomment] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_debug] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ignore] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_error] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_todo] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_line_DASHnr] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_prompt] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_status_DASHline] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_tab_DASHline] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_tab_DASHoption] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_tab_DASHselect] = {
    .visible = true,
    .named = false,
  },
  [sym_comment] = {
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
      if (eof) ADVANCE(325);
      if (lookahead == '#') ADVANCE(331);
      if (lookahead == '+') ADVANCE(337);
      if (lookahead == ',') ADVANCE(335);
      if (lookahead == '.') ADVANCE(336);
      if (lookahead == '0') ADVANCE(352);
      if (lookahead == ':') ADVANCE(333);
      if (lookahead == ';') ADVANCE(334);
      if (lookahead == '>') ADVANCE(339);
      if (lookahead == 'a') ADVANCE(289);
      if (lookahead == 'b') ADVANCE(130);
      if (lookahead == 'c') ADVANCE(15);
      if (lookahead == 'd') ADVANCE(18);
      if (lookahead == 'e') ADVANCE(261);
      if (lookahead == 'f') ADVANCE(131);
      if (lookahead == 'g') ADVANCE(262);
      if (lookahead == 'i') ADVANCE(65);
      if (lookahead == 'k') ADVANCE(70);
      if (lookahead == 'l') ADVANCE(7);
      if (lookahead == 'm') ADVANCE(8);
      if (lookahead == 'n') ADVANCE(306);
      if (lookahead == 'o') ADVANCE(242);
      if (lookahead == 'p') ADVANCE(248);
      if (lookahead == 'r') ADVANCE(71);
      if (lookahead == 's') ADVANCE(243);
      if (lookahead == 't') ADVANCE(9);
      if (lookahead == 'u') ADVANCE(204);
      if (lookahead == 'w') ADVANCE(138);
      if (lookahead == 'y') ADVANCE(72);
      if (lookahead == '{') ADVANCE(340);
      if (lookahead == '|') ADVANCE(346);
      if (lookahead == '}') ADVANCE(341);
      if (lookahead == '~') ADVANCE(338);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(0)
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(353);
      END_STATE();
    case 1:
      if (lookahead == '#') ADVANCE(332);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(1)
      END_STATE();
    case 2:
      if (lookahead == '-') ADVANCE(180);
      END_STATE();
    case 3:
      if (lookahead == '-') ADVANCE(42);
      if (lookahead == '_') ADVANCE(43);
      if (lookahead == 'b') ADVANCE(166);
      if (lookahead == 'c') ADVANCE(321);
      if (lookahead == 'g') ADVANCE(269);
      if (lookahead == 'm') ADVANCE(34);
      if (lookahead == 'r') ADVANCE(95);
      if (lookahead == 'y') ADVANCE(124);
      END_STATE();
    case 4:
      if (lookahead == '-') ADVANCE(214);
      END_STATE();
    case 5:
      if (lookahead == '-') ADVANCE(55);
      END_STATE();
    case 6:
      if (lookahead == '-') ADVANCE(182);
      END_STATE();
    case 7:
      if (lookahead == 'a') ADVANCE(41);
      if (lookahead == 'i') ADVANCE(211);
      END_STATE();
    case 8:
      if (lookahead == 'a') ADVANCE(53);
      END_STATE();
    case 9:
      if (lookahead == 'a') ADVANCE(38);
      if (lookahead == 'o') ADVANCE(64);
      if (lookahead == 'y') ADVANCE(245);
      END_STATE();
    case 10:
      if (lookahead == 'a') ADVANCE(274);
      END_STATE();
    case 11:
      if (lookahead == 'a') ADVANCE(375);
      END_STATE();
    case 12:
      if (lookahead == 'a') ADVANCE(376);
      END_STATE();
    case 13:
      if (lookahead == 'a') ADVANCE(377);
      END_STATE();
    case 14:
      if (lookahead == 'a') ADVANCE(378);
      END_STATE();
    case 15:
      if (lookahead == 'a') ADVANCE(191);
      if (lookahead == 'h') ADVANCE(23);
      if (lookahead == 'o') ADVANCE(183);
      if (lookahead == 'y') ADVANCE(24);
      END_STATE();
    case 16:
      if (lookahead == 'a') ADVANCE(134);
      END_STATE();
    case 17:
      if (lookahead == 'a') ADVANCE(279);
      if (lookahead == 'o') ADVANCE(265);
      if (lookahead == 'r') ADVANCE(144);
      END_STATE();
    case 18:
      if (lookahead == 'a') ADVANCE(249);
      if (lookahead == 'e') ADVANCE(39);
      END_STATE();
    case 19:
      if (lookahead == 'a') ADVANCE(48);
      if (lookahead == 'u') ADVANCE(75);
      END_STATE();
    case 20:
      if (lookahead == 'a') ADVANCE(165);
      END_STATE();
    case 21:
      if (lookahead == 'a') ADVANCE(277);
      END_STATE();
    case 22:
      if (lookahead == 'a') ADVANCE(280);
      END_STATE();
    case 23:
      if (lookahead == 'a') ADVANCE(251);
      END_STATE();
    case 24:
      if (lookahead == 'a') ADVANCE(192);
      END_STATE();
    case 25:
      if (lookahead == 'a') ADVANCE(161);
      END_STATE();
    case 26:
      if (lookahead == 'a') ADVANCE(300);
      END_STATE();
    case 27:
      if (lookahead == 'a') ADVANCE(194);
      END_STATE();
    case 28:
      if (lookahead == 'a') ADVANCE(162);
      END_STATE();
    case 29:
      if (lookahead == 'a') ADVANCE(282);
      END_STATE();
    case 30:
      if (lookahead == 'a') ADVANCE(195);
      END_STATE();
    case 31:
      if (lookahead == 'a') ADVANCE(197);
      END_STATE();
    case 32:
      if (lookahead == 'a') ADVANCE(198);
      END_STATE();
    case 33:
      if (lookahead == 'a') ADVANCE(258);
      END_STATE();
    case 34:
      if (lookahead == 'a') ADVANCE(135);
      END_STATE();
    case 35:
      if (lookahead == 'a') ADVANCE(213);
      END_STATE();
    case 36:
      if (lookahead == 'a') ADVANCE(136);
      END_STATE();
    case 37:
      if (lookahead == 'a') ADVANCE(137);
      END_STATE();
    case 38:
      if (lookahead == 'b') ADVANCE(2);
      if (lookahead == 'g') ADVANCE(413);
      END_STATE();
    case 39:
      if (lookahead == 'b') ADVANCE(307);
      if (lookahead == 'f') ADVANCE(149);
      if (lookahead == 'l') ADVANCE(142);
      END_STATE();
    case 40:
      if (lookahead == 'b') ADVANCE(312);
      END_STATE();
    case 41:
      if (lookahead == 'b') ADVANCE(99);
      END_STATE();
    case 42:
      if (lookahead == 'b') ADVANCE(175);
      if (lookahead == 'c') ADVANCE(322);
      if (lookahead == 'g') ADVANCE(270);
      if (lookahead == 'm') ADVANCE(36);
      if (lookahead == 'r') ADVANCE(101);
      if (lookahead == 'y') ADVANCE(125);
      END_STATE();
    case 43:
      if (lookahead == 'b') ADVANCE(177);
      if (lookahead == 'c') ADVANCE(323);
      if (lookahead == 'g') ADVANCE(271);
      if (lookahead == 'm') ADVANCE(37);
      if (lookahead == 'r') ADVANCE(104);
      if (lookahead == 'y') ADVANCE(126);
      END_STATE();
    case 44:
      if (lookahead == 'b') ADVANCE(107);
      END_STATE();
    case 45:
      if (lookahead == 'c') ADVANCE(348);
      END_STATE();
    case 46:
      if (lookahead == 'c') ADVANCE(402);
      END_STATE();
    case 47:
      if (lookahead == 'c') ADVANCE(139);
      END_STATE();
    case 48:
      if (lookahead == 'c') ADVANCE(159);
      END_STATE();
    case 49:
      if (lookahead == 'c') ADVANCE(163);
      END_STATE();
    case 50:
      if (lookahead == 'c') ADVANCE(148);
      END_STATE();
    case 51:
      if (lookahead == 'c') ADVANCE(241);
      if (lookahead == 'p') ADVANCE(266);
      END_STATE();
    case 52:
      if (lookahead == 'c') ADVANCE(292);
      END_STATE();
    case 53:
      if (lookahead == 'c') ADVANCE(263);
      if (lookahead == 'g') ADVANCE(115);
      END_STATE();
    case 54:
      if (lookahead == 'c') ADVANCE(90);
      END_STATE();
    case 55:
      if (lookahead == 'c') ADVANCE(169);
      END_STATE();
    case 56:
      if (lookahead == 'c') ADVANCE(291);
      END_STATE();
    case 57:
      if (lookahead == 'c') ADVANCE(287);
      END_STATE();
    case 58:
      if (lookahead == 'd') ADVANCE(359);
      if (lookahead == 'p') ADVANCE(117);
      if (lookahead == 'v') ADVANCE(105);
      END_STATE();
    case 59:
      if (lookahead == 'd') ADVANCE(347);
      END_STATE();
    case 60:
      if (lookahead == 'd') ADVANCE(360);
      END_STATE();
    case 61:
      if (lookahead == 'd') ADVANCE(400);
      END_STATE();
    case 62:
      if (lookahead == 'd') ADVANCE(361);
      END_STATE();
    case 63:
      if (lookahead == 'd') ADVANCE(362);
      END_STATE();
    case 64:
      if (lookahead == 'd') ADVANCE(225);
      END_STATE();
    case 65:
      if (lookahead == 'd') ADVANCE(100);
      if (lookahead == 'g') ADVANCE(205);
      if (lookahead == 'n') ADVANCE(49);
      if (lookahead == 't') ADVANCE(20);
      END_STATE();
    case 66:
      if (lookahead == 'd') ADVANCE(80);
      END_STATE();
    case 67:
      if (lookahead == 'd') ADVANCE(157);
      if (lookahead == 's') ADVANCE(301);
      END_STATE();
    case 68:
      if (lookahead == 'd') ADVANCE(146);
      END_STATE();
    case 69:
      if (lookahead == 'd') ADVANCE(106);
      END_STATE();
    case 70:
      if (lookahead == 'e') ADVANCE(320);
      END_STATE();
    case 71:
      if (lookahead == 'e') ADVANCE(58);
      END_STATE();
    case 72:
      if (lookahead == 'e') ADVANCE(167);
      END_STATE();
    case 73:
      if (lookahead == 'e') ADVANCE(103);
      END_STATE();
    case 74:
      if (lookahead == 'e') ADVANCE(51);
      if (lookahead == 'o') ADVANCE(185);
      END_STATE();
    case 75:
      if (lookahead == 'e') ADVANCE(371);
      END_STATE();
    case 76:
      if (lookahead == 'e') ADVANCE(407);
      END_STATE();
    case 77:
      if (lookahead == 'e') ADVANCE(383);
      END_STATE();
    case 78:
      if (lookahead == 'e') ADVANCE(404);
      END_STATE();
    case 79:
      if (lookahead == 'e') ADVANCE(417);
      END_STATE();
    case 80:
      if (lookahead == 'e') ADVANCE(403);
      END_STATE();
    case 81:
      if (lookahead == 'e') ADVANCE(351);
      END_STATE();
    case 82:
      if (lookahead == 'e') ADVANCE(372);
      END_STATE();
    case 83:
      if (lookahead == 'e') ADVANCE(423);
      END_STATE();
    case 84:
      if (lookahead == 'e') ADVANCE(345);
      END_STATE();
    case 85:
      if (lookahead == 'e') ADVANCE(373);
      END_STATE();
    case 86:
      if (lookahead == 'e') ADVANCE(374);
      END_STATE();
    case 87:
      if (lookahead == 'e') ADVANCE(409);
      END_STATE();
    case 88:
      if (lookahead == 'e') ADVANCE(350);
      END_STATE();
    case 89:
      if (lookahead == 'e') ADVANCE(422);
      END_STATE();
    case 90:
      if (lookahead == 'e') ADVANCE(246);
      END_STATE();
    case 91:
      if (lookahead == 'e') ADVANCE(108);
      END_STATE();
    case 92:
      if (lookahead == 'e') ADVANCE(4);
      END_STATE();
    case 93:
      if (lookahead == 'e') ADVANCE(128);
      END_STATE();
    case 94:
      if (lookahead == 'e') ADVANCE(111);
      END_STATE();
    case 95:
      if (lookahead == 'e') ADVANCE(60);
      END_STATE();
    case 96:
      if (lookahead == 'e') ADVANCE(112);
      END_STATE();
    case 97:
      if (lookahead == 'e') ADVANCE(5);
      END_STATE();
    case 98:
      if (lookahead == 'e') ADVANCE(50);
      END_STATE();
    case 99:
      if (lookahead == 'e') ADVANCE(160);
      END_STATE();
    case 100:
      if (lookahead == 'e') ADVANCE(209);
      END_STATE();
    case 101:
      if (lookahead == 'e') ADVANCE(62);
      END_STATE();
    case 102:
      if (lookahead == 'e') ADVANCE(273);
      END_STATE();
    case 103:
      if (lookahead == 'e') ADVANCE(193);
      if (lookahead == 'y') ADVANCE(384);
      END_STATE();
    case 104:
      if (lookahead == 'e') ADVANCE(63);
      END_STATE();
    case 105:
      if (lookahead == 'e') ADVANCE(260);
      END_STATE();
    case 106:
      if (lookahead == 'e') ADVANCE(267);
      END_STATE();
    case 107:
      if (lookahead == 'e') ADVANCE(253);
      END_STATE();
    case 108:
      if (lookahead == 'e') ADVANCE(199);
      if (lookahead == 'y') ADVANCE(356);
      END_STATE();
    case 109:
      if (lookahead == 'e') ADVANCE(256);
      END_STATE();
    case 110:
      if (lookahead == 'e') ADVANCE(257);
      END_STATE();
    case 111:
      if (lookahead == 'e') ADVANCE(201);
      if (lookahead == 'y') ADVANCE(357);
      END_STATE();
    case 112:
      if (lookahead == 'e') ADVANCE(202);
      if (lookahead == 'y') ADVANCE(358);
      END_STATE();
    case 113:
      if (lookahead == 'e') ADVANCE(174);
      END_STATE();
    case 114:
      if (lookahead == 'e') ADVANCE(27);
      END_STATE();
    case 115:
      if (lookahead == 'e') ADVANCE(210);
      END_STATE();
    case 116:
      if (lookahead == 'e') ADVANCE(57);
      END_STATE();
    case 117:
      if (lookahead == 'e') ADVANCE(29);
      END_STATE();
    case 118:
      if (lookahead == 'e') ADVANCE(212);
      END_STATE();
    case 119:
      if (lookahead == 'e') ADVANCE(215);
      END_STATE();
    case 120:
      if (lookahead == 'e') ADVANCE(217);
      END_STATE();
    case 121:
      if (lookahead == 'e') ADVANCE(218);
      END_STATE();
    case 122:
      if (lookahead == 'e') ADVANCE(219);
      END_STATE();
    case 123:
      if (lookahead == 'e') ADVANCE(220);
      END_STATE();
    case 124:
      if (lookahead == 'e') ADVANCE(176);
      END_STATE();
    case 125:
      if (lookahead == 'e') ADVANCE(178);
      END_STATE();
    case 126:
      if (lookahead == 'e') ADVANCE(179);
      END_STATE();
    case 127:
      if (lookahead == 'e') ADVANCE(188);
      if (lookahead == 'u') ADVANCE(276);
      END_STATE();
    case 128:
      if (lookahead == 'f') ADVANCE(410);
      END_STATE();
    case 129:
      if (lookahead == 'f') ADVANCE(156);
      END_STATE();
    case 130:
      if (lookahead == 'g') ADVANCE(343);
      if (lookahead == 'l') ADVANCE(19);
      if (lookahead == 'o') ADVANCE(164);
      END_STATE();
    case 131:
      if (lookahead == 'g') ADVANCE(342);
      if (lookahead == 'l') ADVANCE(230);
      if (lookahead == 'u') ADVANCE(207);
      END_STATE();
    case 132:
      if (lookahead == 'g') ADVANCE(416);
      END_STATE();
    case 133:
      if (lookahead == 'g') ADVANCE(388);
      END_STATE();
    case 134:
      if (lookahead == 'g') ADVANCE(97);
      END_STATE();
    case 135:
      if (lookahead == 'g') ADVANCE(120);
      END_STATE();
    case 136:
      if (lookahead == 'g') ADVANCE(121);
      END_STATE();
    case 137:
      if (lookahead == 'g') ADVANCE(122);
      END_STATE();
    case 138:
      if (lookahead == 'h') ADVANCE(141);
      END_STATE();
    case 139:
      if (lookahead == 'h') ADVANCE(33);
      if (lookahead == 'o') ADVANCE(190);
      END_STATE();
    case 140:
      if (lookahead == 'i') ADVANCE(129);
      END_STATE();
    case 141:
      if (lookahead == 'i') ADVANCE(298);
      END_STATE();
    case 142:
      if (lookahead == 'i') ADVANCE(186);
      END_STATE();
    case 143:
      if (lookahead == 'i') ADVANCE(45);
      END_STATE();
    case 144:
      if (lookahead == 'i') ADVANCE(206);
      if (lookahead == 'u') ADVANCE(56);
      END_STATE();
    case 145:
      if (lookahead == 'i') ADVANCE(234);
      END_STATE();
    case 146:
      if (lookahead == 'i') ADVANCE(285);
      END_STATE();
    case 147:
      if (lookahead == 'i') ADVANCE(302);
      END_STATE();
    case 148:
      if (lookahead == 'i') ADVANCE(25);
      END_STATE();
    case 149:
      if (lookahead == 'i') ADVANCE(216);
      END_STATE();
    case 150:
      if (lookahead == 'i') ADVANCE(236);
      END_STATE();
    case 151:
      if (lookahead == 'i') ADVANCE(221);
      END_STATE();
    case 152:
      if (lookahead == 'i') ADVANCE(238);
      END_STATE();
    case 153:
      if (lookahead == 'i') ADVANCE(222);
      END_STATE();
    case 154:
      if (lookahead == 'i') ADVANCE(239);
      END_STATE();
    case 155:
      if (lookahead == 'i') ADVANCE(223);
      END_STATE();
    case 156:
      if (lookahead == 'i') ADVANCE(110);
      END_STATE();
    case 157:
      if (lookahead == 'i') ADVANCE(303);
      END_STATE();
    case 158:
      if (lookahead == 'k') ADVANCE(3);
      END_STATE();
    case 159:
      if (lookahead == 'k') ADVANCE(355);
      END_STATE();
    case 160:
      if (lookahead == 'l') ADVANCE(398);
      END_STATE();
    case 161:
      if (lookahead == 'l') ADVANCE(411);
      END_STATE();
    case 162:
      if (lookahead == 'l') ADVANCE(396);
      END_STATE();
    case 163:
      if (lookahead == 'l') ADVANCE(311);
      END_STATE();
    case 164:
      if (lookahead == 'l') ADVANCE(59);
      if (lookahead == 'o') ADVANCE(173);
      END_STATE();
    case 165:
      if (lookahead == 'l') ADVANCE(143);
      END_STATE();
    case 166:
      if (lookahead == 'l') ADVANCE(308);
      END_STATE();
    case 167:
      if (lookahead == 'l') ADVANCE(168);
      END_STATE();
    case 168:
      if (lookahead == 'l') ADVANCE(227);
      END_STATE();
    case 169:
      if (lookahead == 'l') ADVANCE(21);
      END_STATE();
    case 170:
      if (lookahead == 'l') ADVANCE(228);
      END_STATE();
    case 171:
      if (lookahead == 'l') ADVANCE(229);
      END_STATE();
    case 172:
      if (lookahead == 'l') ADVANCE(231);
      END_STATE();
    case 173:
      if (lookahead == 'l') ADVANCE(114);
      END_STATE();
    case 174:
      if (lookahead == 'l') ADVANCE(116);
      END_STATE();
    case 175:
      if (lookahead == 'l') ADVANCE(309);
      END_STATE();
    case 176:
      if (lookahead == 'l') ADVANCE(170);
      END_STATE();
    case 177:
      if (lookahead == 'l') ADVANCE(310);
      END_STATE();
    case 178:
      if (lookahead == 'l') ADVANCE(171);
      END_STATE();
    case 179:
      if (lookahead == 'l') ADVANCE(172);
      END_STATE();
    case 180:
      if (lookahead == 'l') ADVANCE(151);
      if (lookahead == 'o') ADVANCE(247);
      if (lookahead == 's') ADVANCE(113);
      END_STATE();
    case 181:
      if (lookahead == 'l') ADVANCE(153);
      END_STATE();
    case 182:
      if (lookahead == 'l') ADVANCE(155);
      END_STATE();
    case 183:
      if (lookahead == 'm') ADVANCE(187);
      if (lookahead == 'n') ADVANCE(67);
      END_STATE();
    case 184:
      if (lookahead == 'm') ADVANCE(44);
      END_STATE();
    case 185:
      if (lookahead == 'm') ADVANCE(244);
      END_STATE();
    case 186:
      if (lookahead == 'm') ADVANCE(147);
      END_STATE();
    case 187:
      if (lookahead == 'm') ADVANCE(118);
      END_STATE();
    case 188:
      if (lookahead == 'm') ADVANCE(119);
      END_STATE();
    case 189:
      if (lookahead == 'm') ADVANCE(123);
      END_STATE();
    case 190:
      if (lookahead == 'm') ADVANCE(189);
      END_STATE();
    case 191:
      if (lookahead == 'n') ADVANCE(314);
      END_STATE();
    case 192:
      if (lookahead == 'n') ADVANCE(379);
      END_STATE();
    case 193:
      if (lookahead == 'n') ADVANCE(363);
      END_STATE();
    case 194:
      if (lookahead == 'n') ADVANCE(391);
      END_STATE();
    case 195:
      if (lookahead == 'n') ADVANCE(380);
      END_STATE();
    case 196:
      if (lookahead == 'n') ADVANCE(394);
      END_STATE();
    case 197:
      if (lookahead == 'n') ADVANCE(381);
      END_STATE();
    case 198:
      if (lookahead == 'n') ADVANCE(382);
      END_STATE();
    case 199:
      if (lookahead == 'n') ADVANCE(364);
      END_STATE();
    case 200:
      if (lookahead == 'n') ADVANCE(401);
      END_STATE();
    case 201:
      if (lookahead == 'n') ADVANCE(365);
      END_STATE();
    case 202:
      if (lookahead == 'n') ADVANCE(366);
      END_STATE();
    case 203:
      if (lookahead == 'n') ADVANCE(424);
      END_STATE();
    case 204:
      if (lookahead == 'n') ADVANCE(69);
      END_STATE();
    case 205:
      if (lookahead == 'n') ADVANCE(240);
      END_STATE();
    case 206:
      if (lookahead == 'n') ADVANCE(133);
      END_STATE();
    case 207:
      if (lookahead == 'n') ADVANCE(52);
      END_STATE();
    case 208:
      if (lookahead == 'n') ADVANCE(68);
      END_STATE();
    case 209:
      if (lookahead == 'n') ADVANCE(293);
      END_STATE();
    case 210:
      if (lookahead == 'n') ADVANCE(294);
      END_STATE();
    case 211:
      if (lookahead == 'n') ADVANCE(92);
      END_STATE();
    case 212:
      if (lookahead == 'n') ADVANCE(283);
      END_STATE();
    case 213:
      if (lookahead == 'n') ADVANCE(284);
      END_STATE();
    case 214:
      if (lookahead == 'n') ADVANCE(254);
      END_STATE();
    case 215:
      if (lookahead == 'n') ADVANCE(286);
      END_STATE();
    case 216:
      if (lookahead == 'n') ADVANCE(78);
      END_STATE();
    case 217:
      if (lookahead == 'n') ADVANCE(295);
      END_STATE();
    case 218:
      if (lookahead == 'n') ADVANCE(296);
      END_STATE();
    case 219:
      if (lookahead == 'n') ADVANCE(297);
      END_STATE();
    case 220:
      if (lookahead == 'n') ADVANCE(288);
      END_STATE();
    case 221:
      if (lookahead == 'n') ADVANCE(83);
      END_STATE();
    case 222:
      if (lookahead == 'n') ADVANCE(88);
      END_STATE();
    case 223:
      if (lookahead == 'n') ADVANCE(89);
      END_STATE();
    case 224:
      if (lookahead == 'n') ADVANCE(28);
      END_STATE();
    case 225:
      if (lookahead == 'o') ADVANCE(419);
      END_STATE();
    case 226:
      if (lookahead == 'o') ADVANCE(405);
      END_STATE();
    case 227:
      if (lookahead == 'o') ADVANCE(315);
      END_STATE();
    case 228:
      if (lookahead == 'o') ADVANCE(316);
      END_STATE();
    case 229:
      if (lookahead == 'o') ADVANCE(317);
      END_STATE();
    case 230:
      if (lookahead == 'o') ADVANCE(22);
      END_STATE();
    case 231:
      if (lookahead == 'o') ADVANCE(318);
      END_STATE();
    case 232:
      if (lookahead == 'o') ADVANCE(46);
      END_STATE();
    case 233:
      if (lookahead == 'o') ADVANCE(252);
      END_STATE();
    case 234:
      if (lookahead == 'o') ADVANCE(196);
      END_STATE();
    case 235:
      if (lookahead == 'o') ADVANCE(264);
      END_STATE();
    case 236:
      if (lookahead == 'o') ADVANCE(224);
      END_STATE();
    case 237:
      if (lookahead == 'o') ADVANCE(255);
      END_STATE();
    case 238:
      if (lookahead == 'o') ADVANCE(200);
      END_STATE();
    case 239:
      if (lookahead == 'o') ADVANCE(203);
      END_STATE();
    case 240:
      if (lookahead == 'o') ADVANCE(268);
      END_STATE();
    case 241:
      if (lookahead == 'o') ADVANCE(208);
      END_STATE();
    case 242:
      if (lookahead == 'p') ADVANCE(102);
      END_STATE();
    case 243:
      if (lookahead == 'p') ADVANCE(98);
      if (lookahead == 't') ADVANCE(17);
      END_STATE();
    case 244:
      if (lookahead == 'p') ADVANCE(281);
      END_STATE();
    case 245:
      if (lookahead == 'p') ADVANCE(76);
      END_STATE();
    case 246:
      if (lookahead == 'p') ADVANCE(304);
      END_STATE();
    case 247:
      if (lookahead == 'p') ADVANCE(305);
      END_STATE();
    case 248:
      if (lookahead == 'r') ADVANCE(74);
      END_STATE();
    case 249:
      if (lookahead == 'r') ADVANCE(158);
      END_STATE();
    case 250:
      if (lookahead == 'r') ADVANCE(344);
      END_STATE();
    case 251:
      if (lookahead == 'r') ADVANCE(389);
      END_STATE();
    case 252:
      if (lookahead == 'r') ADVANCE(418);
      END_STATE();
    case 253:
      if (lookahead == 'r') ADVANCE(390);
      END_STATE();
    case 254:
      if (lookahead == 'r') ADVANCE(420);
      END_STATE();
    case 255:
      if (lookahead == 'r') ADVANCE(399);
      END_STATE();
    case 256:
      if (lookahead == 'r') ADVANCE(414);
      END_STATE();
    case 257:
      if (lookahead == 'r') ADVANCE(393);
      END_STATE();
    case 258:
      if (lookahead == 'r') ADVANCE(412);
      END_STATE();
    case 259:
      if (lookahead == 'r') ADVANCE(233);
      END_STATE();
    case 260:
      if (lookahead == 'r') ADVANCE(278);
      END_STATE();
    case 261:
      if (lookahead == 'r') ADVANCE(259);
      if (lookahead == 'x') ADVANCE(54);
      END_STATE();
    case 262:
      if (lookahead == 'r') ADVANCE(73);
      END_STATE();
    case 263:
      if (lookahead == 'r') ADVANCE(226);
      END_STATE();
    case 264:
      if (lookahead == 'r') ADVANCE(61);
      END_STATE();
    case 265:
      if (lookahead == 'r') ADVANCE(16);
      END_STATE();
    case 266:
      if (lookahead == 'r') ADVANCE(232);
      END_STATE();
    case 267:
      if (lookahead == 'r') ADVANCE(181);
      END_STATE();
    case 268:
      if (lookahead == 'r') ADVANCE(79);
      END_STATE();
    case 269:
      if (lookahead == 'r') ADVANCE(91);
      END_STATE();
    case 270:
      if (lookahead == 'r') ADVANCE(94);
      END_STATE();
    case 271:
      if (lookahead == 'r') ADVANCE(96);
      END_STATE();
    case 272:
      if (lookahead == 'r') ADVANCE(87);
      END_STATE();
    case 273:
      if (lookahead == 'r') ADVANCE(26);
      END_STATE();
    case 274:
      if (lookahead == 's') ADVANCE(385);
      END_STATE();
    case 275:
      if (lookahead == 's') ADVANCE(408);
      END_STATE();
    case 276:
      if (lookahead == 's') ADVANCE(6);
      END_STATE();
    case 277:
      if (lookahead == 's') ADVANCE(275);
      END_STATE();
    case 278:
      if (lookahead == 's') ADVANCE(81);
      END_STATE();
    case 279:
      if (lookahead == 't') ADVANCE(127);
      END_STATE();
    case 280:
      if (lookahead == 't') ADVANCE(392);
      END_STATE();
    case 281:
      if (lookahead == 't') ADVANCE(421);
      END_STATE();
    case 282:
      if (lookahead == 't') ADVANCE(397);
      END_STATE();
    case 283:
      if (lookahead == 't') ADVANCE(386);
      END_STATE();
    case 284:
      if (lookahead == 't') ADVANCE(387);
      END_STATE();
    case 285:
      if (lookahead == 't') ADVANCE(406);
      END_STATE();
    case 286:
      if (lookahead == 't') ADVANCE(395);
      END_STATE();
    case 287:
      if (lookahead == 't') ADVANCE(425);
      END_STATE();
    case 288:
      if (lookahead == 't') ADVANCE(415);
      END_STATE();
    case 289:
      if (lookahead == 't') ADVANCE(290);
      END_STATE();
    case 290:
      if (lookahead == 't') ADVANCE(250);
      END_STATE();
    case 291:
      if (lookahead == 't') ADVANCE(313);
      END_STATE();
    case 292:
      if (lookahead == 't') ADVANCE(145);
      END_STATE();
    case 293:
      if (lookahead == 't') ADVANCE(140);
      END_STATE();
    case 294:
      if (lookahead == 't') ADVANCE(11);
      END_STATE();
    case 295:
      if (lookahead == 't') ADVANCE(12);
      END_STATE();
    case 296:
      if (lookahead == 't') ADVANCE(13);
      END_STATE();
    case 297:
      if (lookahead == 't') ADVANCE(14);
      END_STATE();
    case 298:
      if (lookahead == 't') ADVANCE(77);
      END_STATE();
    case 299:
      if (lookahead == 't') ADVANCE(84);
      END_STATE();
    case 300:
      if (lookahead == 't') ADVANCE(237);
      END_STATE();
    case 301:
      if (lookahead == 't') ADVANCE(35);
      END_STATE();
    case 302:
      if (lookahead == 't') ADVANCE(109);
      END_STATE();
    case 303:
      if (lookahead == 't') ADVANCE(150);
      END_STATE();
    case 304:
      if (lookahead == 't') ADVANCE(152);
      END_STATE();
    case 305:
      if (lookahead == 't') ADVANCE(154);
      END_STATE();
    case 306:
      if (lookahead == 'u') ADVANCE(184);
      END_STATE();
    case 307:
      if (lookahead == 'u') ADVANCE(132);
      END_STATE();
    case 308:
      if (lookahead == 'u') ADVANCE(82);
      END_STATE();
    case 309:
      if (lookahead == 'u') ADVANCE(85);
      END_STATE();
    case 310:
      if (lookahead == 'u') ADVANCE(86);
      END_STATE();
    case 311:
      if (lookahead == 'u') ADVANCE(66);
      END_STATE();
    case 312:
      if (lookahead == 'u') ADVANCE(299);
      END_STATE();
    case 313:
      if (lookahead == 'u') ADVANCE(272);
      END_STATE();
    case 314:
      if (lookahead == 'v') ADVANCE(10);
      END_STATE();
    case 315:
      if (lookahead == 'w') ADVANCE(367);
      END_STATE();
    case 316:
      if (lookahead == 'w') ADVANCE(368);
      END_STATE();
    case 317:
      if (lookahead == 'w') ADVANCE(369);
      END_STATE();
    case 318:
      if (lookahead == 'w') ADVANCE(370);
      END_STATE();
    case 319:
      if (lookahead == 'w') ADVANCE(235);
      END_STATE();
    case 320:
      if (lookahead == 'y') ADVANCE(319);
      END_STATE();
    case 321:
      if (lookahead == 'y') ADVANCE(30);
      END_STATE();
    case 322:
      if (lookahead == 'y') ADVANCE(31);
      END_STATE();
    case 323:
      if (lookahead == 'y') ADVANCE(32);
      END_STATE();
    case 324:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(354);
      END_STATE();
    case 325:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 326:
      ACCEPT_TOKEN(aux_sym_comment_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(332);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(332);
      END_STATE();
    case 327:
      ACCEPT_TOKEN(aux_sym_comment_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(326);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(332);
      END_STATE();
    case 328:
      ACCEPT_TOKEN(aux_sym_comment_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(327);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(332);
      END_STATE();
    case 329:
      ACCEPT_TOKEN(aux_sym_comment_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(328);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(332);
      END_STATE();
    case 330:
      ACCEPT_TOKEN(aux_sym_comment_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(329);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(332);
      END_STATE();
    case 331:
      ACCEPT_TOKEN(aux_sym_comment_token1);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(330);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(332);
      END_STATE();
    case 332:
      ACCEPT_TOKEN(aux_sym_comment_token1);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(332);
      END_STATE();
    case 333:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 334:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 335:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 336:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 337:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 338:
      ACCEPT_TOKEN(anon_sym_TILDE);
      END_STATE();
    case 339:
      ACCEPT_TOKEN(anon_sym_GT);
      END_STATE();
    case 340:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 341:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 342:
      ACCEPT_TOKEN(anon_sym_fg);
      END_STATE();
    case 343:
      ACCEPT_TOKEN(anon_sym_bg);
      END_STATE();
    case 344:
      ACCEPT_TOKEN(anon_sym_attr);
      if (lookahead == 'i') ADVANCE(40);
      END_STATE();
    case 345:
      ACCEPT_TOKEN(anon_sym_attribute);
      END_STATE();
    case 346:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 347:
      ACCEPT_TOKEN(anon_sym_bold);
      END_STATE();
    case 348:
      ACCEPT_TOKEN(anon_sym_italic);
      END_STATE();
    case 349:
      ACCEPT_TOKEN(anon_sym_underlined);
      END_STATE();
    case 350:
      ACCEPT_TOKEN(anon_sym_underline);
      if (lookahead == 'd') ADVANCE(349);
      END_STATE();
    case 351:
      ACCEPT_TOKEN(anon_sym_reverse);
      END_STATE();
    case 352:
      ACCEPT_TOKEN(aux_sym_ansi_color_token1);
      if (lookahead == 'x') ADVANCE(324);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(353);
      END_STATE();
    case 353:
      ACCEPT_TOKEN(aux_sym_ansi_color_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(353);
      END_STATE();
    case 354:
      ACCEPT_TOKEN(aux_sym_ansi_color_token2);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(354);
      END_STATE();
    case 355:
      ACCEPT_TOKEN(anon_sym_black);
      END_STATE();
    case 356:
      ACCEPT_TOKEN(anon_sym_darkgrey);
      END_STATE();
    case 357:
      ACCEPT_TOKEN(anon_sym_dark_DASHgrey);
      END_STATE();
    case 358:
      ACCEPT_TOKEN(anon_sym_dark_grey);
      END_STATE();
    case 359:
      ACCEPT_TOKEN(anon_sym_red);
      END_STATE();
    case 360:
      ACCEPT_TOKEN(anon_sym_darkred);
      END_STATE();
    case 361:
      ACCEPT_TOKEN(anon_sym_dark_DASHred);
      END_STATE();
    case 362:
      ACCEPT_TOKEN(anon_sym_dark_red);
      END_STATE();
    case 363:
      ACCEPT_TOKEN(anon_sym_green);
      END_STATE();
    case 364:
      ACCEPT_TOKEN(anon_sym_darkgreen);
      END_STATE();
    case 365:
      ACCEPT_TOKEN(anon_sym_dark_DASHgreen);
      END_STATE();
    case 366:
      ACCEPT_TOKEN(anon_sym_dark_green);
      END_STATE();
    case 367:
      ACCEPT_TOKEN(anon_sym_yellow);
      END_STATE();
    case 368:
      ACCEPT_TOKEN(anon_sym_darkyellow);
      END_STATE();
    case 369:
      ACCEPT_TOKEN(anon_sym_dark_DASHyellow);
      END_STATE();
    case 370:
      ACCEPT_TOKEN(anon_sym_dark_yellow);
      END_STATE();
    case 371:
      ACCEPT_TOKEN(anon_sym_blue);
      END_STATE();
    case 372:
      ACCEPT_TOKEN(anon_sym_darkblue);
      END_STATE();
    case 373:
      ACCEPT_TOKEN(anon_sym_dark_DASHblue);
      END_STATE();
    case 374:
      ACCEPT_TOKEN(anon_sym_dark_blue);
      END_STATE();
    case 375:
      ACCEPT_TOKEN(anon_sym_magenta);
      END_STATE();
    case 376:
      ACCEPT_TOKEN(anon_sym_darkmagenta);
      END_STATE();
    case 377:
      ACCEPT_TOKEN(anon_sym_dark_DASHmagenta);
      END_STATE();
    case 378:
      ACCEPT_TOKEN(anon_sym_dark_magenta);
      END_STATE();
    case 379:
      ACCEPT_TOKEN(anon_sym_cyan);
      END_STATE();
    case 380:
      ACCEPT_TOKEN(anon_sym_darkcyan);
      END_STATE();
    case 381:
      ACCEPT_TOKEN(anon_sym_dark_DASHcyan);
      END_STATE();
    case 382:
      ACCEPT_TOKEN(anon_sym_dark_cyan);
      END_STATE();
    case 383:
      ACCEPT_TOKEN(anon_sym_white);
      END_STATE();
    case 384:
      ACCEPT_TOKEN(anon_sym_grey);
      END_STATE();
    case 385:
      ACCEPT_TOKEN(anon_sym_canvas);
      END_STATE();
    case 386:
      ACCEPT_TOKEN(anon_sym_comment);
      END_STATE();
    case 387:
      ACCEPT_TOKEN(anon_sym_constant);
      END_STATE();
    case 388:
      ACCEPT_TOKEN(anon_sym_string);
      END_STATE();
    case 389:
      ACCEPT_TOKEN(anon_sym_char);
      END_STATE();
    case 390:
      ACCEPT_TOKEN(anon_sym_number);
      END_STATE();
    case 391:
      ACCEPT_TOKEN(anon_sym_boolean);
      END_STATE();
    case 392:
      ACCEPT_TOKEN(anon_sym_float);
      END_STATE();
    case 393:
      ACCEPT_TOKEN(anon_sym_identifier);
      END_STATE();
    case 394:
      ACCEPT_TOKEN(anon_sym_function);
      END_STATE();
    case 395:
      ACCEPT_TOKEN(anon_sym_statement);
      END_STATE();
    case 396:
      ACCEPT_TOKEN(anon_sym_conditional);
      END_STATE();
    case 397:
      ACCEPT_TOKEN(anon_sym_repeat);
      END_STATE();
    case 398:
      ACCEPT_TOKEN(anon_sym_label);
      END_STATE();
    case 399:
      ACCEPT_TOKEN(anon_sym_operator);
      END_STATE();
    case 400:
      ACCEPT_TOKEN(anon_sym_keyword);
      END_STATE();
    case 401:
      ACCEPT_TOKEN(anon_sym_exception);
      END_STATE();
    case 402:
      ACCEPT_TOKEN(anon_sym_preproc);
      END_STATE();
    case 403:
      ACCEPT_TOKEN(anon_sym_include);
      END_STATE();
    case 404:
      ACCEPT_TOKEN(anon_sym_define);
      END_STATE();
    case 405:
      ACCEPT_TOKEN(anon_sym_macro);
      END_STATE();
    case 406:
      ACCEPT_TOKEN(anon_sym_precondit);
      END_STATE();
    case 407:
      ACCEPT_TOKEN(anon_sym_type);
      if (lookahead == 'd') ADVANCE(93);
      END_STATE();
    case 408:
      ACCEPT_TOKEN(anon_sym_storage_DASHclass);
      END_STATE();
    case 409:
      ACCEPT_TOKEN(anon_sym_structure);
      END_STATE();
    case 410:
      ACCEPT_TOKEN(anon_sym_typedef);
      END_STATE();
    case 411:
      ACCEPT_TOKEN(anon_sym_special);
      if (lookahead == '-') ADVANCE(47);
      END_STATE();
    case 412:
      ACCEPT_TOKEN(anon_sym_special_DASHchar);
      END_STATE();
    case 413:
      ACCEPT_TOKEN(anon_sym_tag);
      END_STATE();
    case 414:
      ACCEPT_TOKEN(anon_sym_delimiter);
      END_STATE();
    case 415:
      ACCEPT_TOKEN(anon_sym_special_DASHcomment);
      END_STATE();
    case 416:
      ACCEPT_TOKEN(anon_sym_debug);
      END_STATE();
    case 417:
      ACCEPT_TOKEN(anon_sym_ignore);
      END_STATE();
    case 418:
      ACCEPT_TOKEN(anon_sym_error);
      END_STATE();
    case 419:
      ACCEPT_TOKEN(anon_sym_todo);
      END_STATE();
    case 420:
      ACCEPT_TOKEN(anon_sym_line_DASHnr);
      END_STATE();
    case 421:
      ACCEPT_TOKEN(anon_sym_prompt);
      END_STATE();
    case 422:
      ACCEPT_TOKEN(anon_sym_status_DASHline);
      END_STATE();
    case 423:
      ACCEPT_TOKEN(anon_sym_tab_DASHline);
      END_STATE();
    case 424:
      ACCEPT_TOKEN(anon_sym_tab_DASHoption);
      END_STATE();
    case 425:
      ACCEPT_TOKEN(anon_sym_tab_DASHselect);
      END_STATE();
    default:
      return false;
  }
}

static TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 1},
  [2] = {(TSStateId)(-1)},
};

static uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [sym_comment] = STATE(0),
    [ts_builtin_sym_end] = ACTIONS(1),
    [aux_sym_comment_token1] = ACTIONS(3),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_PLUS] = ACTIONS(1),
    [anon_sym_TILDE] = ACTIONS(1),
    [anon_sym_GT] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_fg] = ACTIONS(1),
    [anon_sym_bg] = ACTIONS(1),
    [anon_sym_attr] = ACTIONS(1),
    [anon_sym_attribute] = ACTIONS(1),
    [anon_sym_PIPE] = ACTIONS(1),
    [anon_sym_bold] = ACTIONS(1),
    [anon_sym_italic] = ACTIONS(1),
    [anon_sym_underlined] = ACTIONS(1),
    [anon_sym_underline] = ACTIONS(1),
    [anon_sym_reverse] = ACTIONS(1),
    [sym_rgb_color] = ACTIONS(1),
    [aux_sym_ansi_color_token1] = ACTIONS(1),
    [aux_sym_ansi_color_token2] = ACTIONS(1),
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
    [anon_sym_canvas] = ACTIONS(1),
    [anon_sym_comment] = ACTIONS(1),
    [anon_sym_constant] = ACTIONS(1),
    [anon_sym_string] = ACTIONS(1),
    [anon_sym_char] = ACTIONS(1),
    [anon_sym_number] = ACTIONS(1),
    [anon_sym_boolean] = ACTIONS(1),
    [anon_sym_float] = ACTIONS(1),
    [anon_sym_identifier] = ACTIONS(1),
    [anon_sym_function] = ACTIONS(1),
    [anon_sym_statement] = ACTIONS(1),
    [anon_sym_conditional] = ACTIONS(1),
    [anon_sym_repeat] = ACTIONS(1),
    [anon_sym_label] = ACTIONS(1),
    [anon_sym_operator] = ACTIONS(1),
    [anon_sym_keyword] = ACTIONS(1),
    [anon_sym_exception] = ACTIONS(1),
    [anon_sym_preproc] = ACTIONS(1),
    [anon_sym_include] = ACTIONS(1),
    [anon_sym_define] = ACTIONS(1),
    [anon_sym_macro] = ACTIONS(1),
    [anon_sym_precondit] = ACTIONS(1),
    [anon_sym_type] = ACTIONS(1),
    [anon_sym_storage_DASHclass] = ACTIONS(1),
    [anon_sym_structure] = ACTIONS(1),
    [anon_sym_typedef] = ACTIONS(1),
    [anon_sym_special] = ACTIONS(1),
    [anon_sym_special_DASHchar] = ACTIONS(1),
    [anon_sym_tag] = ACTIONS(1),
    [anon_sym_delimiter] = ACTIONS(1),
    [anon_sym_special_DASHcomment] = ACTIONS(1),
    [anon_sym_debug] = ACTIONS(1),
    [anon_sym_ignore] = ACTIONS(1),
    [anon_sym_error] = ACTIONS(1),
    [anon_sym_todo] = ACTIONS(1),
    [anon_sym_line_DASHnr] = ACTIONS(1),
    [anon_sym_prompt] = ACTIONS(1),
    [anon_sym_status_DASHline] = ACTIONS(1),
    [anon_sym_tab_DASHline] = ACTIONS(1),
    [anon_sym_tab_DASHoption] = ACTIONS(1),
    [anon_sym_tab_DASHselect] = ACTIONS(1),
  },
  [1] = {
    [sym_comment] = STATE(1),
    [aux_sym_comment_token1] = ACTIONS(3),
  },
};

static uint16_t ts_small_parse_table[] = {
  [0] = 1,
    ACTIONS(5), 1,
      ts_builtin_sym_end,
};

static uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
};

static TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_comment, 1),
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
