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
#define STATE_COUNT 60
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 122
#define ALIAS_COUNT 0
#define TOKEN_COUNT 93
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 3
#define MAX_ALIAS_SEQUENCE_LENGTH 3

enum {
  anon_sym_COMMA = 1,
  aux_sym_symbol_name_token1 = 2,
  anon_sym_DOT = 3,
  anon_sym_PLUS = 4,
  anon_sym_TILDE = 5,
  anon_sym_GT = 6,
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
  anon_sym_comment = 53,
  anon_sym_constant = 54,
  anon_sym_string = 55,
  anon_sym_char = 56,
  anon_sym_number = 57,
  anon_sym_boolean = 58,
  anon_sym_float = 59,
  anon_sym_identifier = 60,
  anon_sym_function = 61,
  anon_sym_statement = 62,
  anon_sym_conditional = 63,
  anon_sym_repeat = 64,
  anon_sym_label = 65,
  anon_sym_operator = 66,
  anon_sym_keyword = 67,
  anon_sym_exception = 68,
  anon_sym_preproc = 69,
  anon_sym_include = 70,
  anon_sym_define = 71,
  anon_sym_macro = 72,
  anon_sym_precondit = 73,
  anon_sym_type = 74,
  anon_sym_storage_DASHclass = 75,
  anon_sym_structure = 76,
  anon_sym_typedef = 77,
  anon_sym_special = 78,
  anon_sym_special_DASHchar = 79,
  anon_sym_tag = 80,
  anon_sym_delimiter = 81,
  anon_sym_special_DASHcomment = 82,
  anon_sym_debug = 83,
  anon_sym_ignore = 84,
  anon_sym_error = 85,
  anon_sym_todo = 86,
  anon_sym_line_DASHnr = 87,
  anon_sym_prompt = 88,
  anon_sym_status_DASHline = 89,
  anon_sym_tab_DASHline = 90,
  anon_sym_tab_DASHoption = 91,
  anon_sym_tab_DASHselect = 92,
  sym_source_file = 93,
  sym_selectors = 94,
  sym_selector = 95,
  sym_sel_symbol = 96,
  sym_symbol_name = 97,
  sym_field_name = 98,
  sym_sel_field = 99,
  sym_sel_symbol_field = 100,
  sym_sel_next_child = 101,
  sym_sel_prev_child = 102,
  sym_sel_child = 103,
  sym_properties = 104,
  sym_property = 105,
  sym_fg = 106,
  sym_bg = 107,
  sym_attr1 = 108,
  sym_attr2 = 109,
  sym_attrs = 110,
  sym_attr_or = 111,
  sym_attr = 112,
  sym_ansi_color = 113,
  sym_color_name = 114,
  sym_highlight = 115,
  aux_sym_source_file_repeat1 = 116,
  aux_sym_selectors_repeat1 = 117,
  aux_sym_selector_repeat1 = 118,
  aux_sym_properties_repeat1 = 119,
  aux_sym_attr1_repeat1 = 120,
  aux_sym_attrs_repeat1 = 121,
};

static const char *ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_COMMA] = ",",
  [aux_sym_symbol_name_token1] = "symbol_name_token1",
  [anon_sym_DOT] = ".",
  [anon_sym_PLUS] = "+",
  [anon_sym_TILDE] = "~",
  [anon_sym_GT] = ">",
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
  [sym_source_file] = "source_file",
  [sym_selectors] = "selectors",
  [sym_selector] = "selector",
  [sym_sel_symbol] = "sel_symbol",
  [sym_symbol_name] = "symbol_name",
  [sym_field_name] = "field_name",
  [sym_sel_field] = "sel_field",
  [sym_sel_symbol_field] = "sel_symbol_field",
  [sym_sel_next_child] = "sel_next_child",
  [sym_sel_prev_child] = "sel_prev_child",
  [sym_sel_child] = "sel_child",
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
  [sym_highlight] = "highlight",
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
  [aux_sym_symbol_name_token1] = aux_sym_symbol_name_token1,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_PLUS] = anon_sym_PLUS,
  [anon_sym_TILDE] = anon_sym_TILDE,
  [anon_sym_GT] = anon_sym_GT,
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
  [sym_source_file] = sym_source_file,
  [sym_selectors] = sym_selectors,
  [sym_selector] = sym_selector,
  [sym_sel_symbol] = sym_sel_symbol,
  [sym_symbol_name] = sym_symbol_name,
  [sym_field_name] = sym_field_name,
  [sym_sel_field] = sym_sel_field,
  [sym_sel_symbol_field] = sym_sel_symbol_field,
  [sym_sel_next_child] = sym_sel_next_child,
  [sym_sel_prev_child] = sym_sel_prev_child,
  [sym_sel_child] = sym_sel_child,
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
  [sym_highlight] = sym_highlight,
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
  [aux_sym_symbol_name_token1] = {
    .visible = false,
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
  [sym_symbol_name] = {
    .visible = true,
    .named = true,
  },
  [sym_field_name] = {
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
  [sym_sel_next_child] = {
    .visible = true,
    .named = true,
  },
  [sym_sel_prev_child] = {
    .visible = true,
    .named = true,
  },
  [sym_sel_child] = {
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
  [sym_highlight] = {
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
      if (eof) ADVANCE(326);
      if (lookahead == '#') ADVANCE(323);
      if (lookahead == '+') ADVANCE(355);
      if (lookahead == ',') ADVANCE(327);
      if (lookahead == '.') ADVANCE(354);
      if (lookahead == '0') ADVANCE(372);
      if (lookahead == ':') ADVANCE(361);
      if (lookahead == '>') ADVANCE(357);
      if (lookahead == 'b') ADVANCE(123);
      if (lookahead == 'c') ADVANCE(133);
      if (lookahead == 'd') ADVANCE(62);
      if (lookahead == 'e') ADVANCE(242);
      if (lookahead == 'f') ADVANCE(125);
      if (lookahead == 'i') ADVANCE(50);
      if (lookahead == 'k') ADVANCE(63);
      if (lookahead == 'l') ADVANCE(8);
      if (lookahead == 'm') ADVANCE(16);
      if (lookahead == 'n') ADVANCE(300);
      if (lookahead == 'o') ADVANCE(237);
      if (lookahead == 'p') ADVANCE(243);
      if (lookahead == 'r') ADVANCE(64);
      if (lookahead == 's') ADVANCE(238);
      if (lookahead == 't') ADVANCE(9);
      if (lookahead == 'u') ADVANCE(200);
      if (lookahead == '{') ADVANCE(358);
      if (lookahead == '|') ADVANCE(365);
      if (lookahead == '}') ADVANCE(359);
      if (lookahead == '~') ADVANCE(356);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(373);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(324);
      END_STATE();
    case 1:
      if (lookahead == '#') ADVANCE(323);
      if (lookahead == ',') ADVANCE(327);
      if (lookahead == '0') ADVANCE(372);
      if (lookahead == 'a') ADVANCE(291);
      if (lookahead == 'b') ADVANCE(122);
      if (lookahead == 'c') ADVANCE(313);
      if (lookahead == 'd') ADVANCE(28);
      if (lookahead == 'f') ADVANCE(124);
      if (lookahead == 'g') ADVANCE(262);
      if (lookahead == 'i') ADVANCE(294);
      if (lookahead == 'm') ADVANCE(31);
      if (lookahead == 'r') ADVANCE(73);
      if (lookahead == 'u') ADVANCE(214);
      if (lookahead == 'w') ADVANCE(135);
      if (lookahead == 'y') ADVANCE(96);
      if (lookahead == '|') ADVANCE(365);
      if (lookahead == '}') ADVANCE(359);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(1)
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(373);
      END_STATE();
    case 2:
      if (lookahead == '+') ADVANCE(355);
      if (lookahead == ',') ADVANCE(327);
      if (lookahead == '.') ADVANCE(354);
      if (lookahead == '>') ADVANCE(357);
      if (lookahead == 'b') ADVANCE(220);
      if (lookahead == 'c') ADVANCE(133);
      if (lookahead == 'd') ADVANCE(62);
      if (lookahead == 'e') ADVANCE(242);
      if (lookahead == 'f') ADVANCE(160);
      if (lookahead == 'i') ADVANCE(50);
      if (lookahead == 'k') ADVANCE(63);
      if (lookahead == 'l') ADVANCE(8);
      if (lookahead == 'm') ADVANCE(16);
      if (lookahead == 'n') ADVANCE(300);
      if (lookahead == 'o') ADVANCE(237);
      if (lookahead == 'p') ADVANCE(243);
      if (lookahead == 'r') ADVANCE(64);
      if (lookahead == 's') ADVANCE(238);
      if (lookahead == 't') ADVANCE(9);
      if (lookahead == 'u') ADVANCE(200);
      if (lookahead == '{') ADVANCE(358);
      if (lookahead == '~') ADVANCE(356);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(2)
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(324);
      END_STATE();
    case 3:
      if (lookahead == '-') ADVANCE(178);
      END_STATE();
    case 4:
      if (lookahead == '-') ADVANCE(38);
      if (lookahead == '_') ADVANCE(39);
      if (lookahead == 'b') ADVANCE(162);
      if (lookahead == 'c') ADVANCE(314);
      if (lookahead == 'g') ADVANCE(263);
      if (lookahead == 'm') ADVANCE(33);
      if (lookahead == 'r') ADVANCE(92);
      if (lookahead == 'y') ADVANCE(107);
      END_STATE();
    case 5:
      if (lookahead == '-') ADVANCE(206);
      END_STATE();
    case 6:
      if (lookahead == '-') ADVANCE(47);
      END_STATE();
    case 7:
      if (lookahead == '-') ADVANCE(180);
      END_STATE();
    case 8:
      if (lookahead == 'a') ADVANCE(333);
      if (lookahead == 'i') ADVANCE(346);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(328);
      END_STATE();
    case 9:
      if (lookahead == 'a') ADVANCE(332);
      if (lookahead == 'o') ADVANCE(337);
      if (lookahead == 'y') ADVANCE(351);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(328);
      END_STATE();
    case 10:
      if (lookahead == 'a') ADVANCE(395);
      END_STATE();
    case 11:
      if (lookahead == 'a') ADVANCE(396);
      END_STATE();
    case 12:
      if (lookahead == 'a') ADVANCE(397);
      END_STATE();
    case 13:
      if (lookahead == 'a') ADVANCE(398);
      END_STATE();
    case 14:
      if (lookahead == 'a') ADVANCE(128);
      END_STATE();
    case 15:
      if (lookahead == 'a') ADVANCE(270);
      END_STATE();
    case 16:
      if (lookahead == 'a') ADVANCE(336);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(328);
      END_STATE();
    case 17:
      if (lookahead == 'a') ADVANCE(273);
      END_STATE();
    case 18:
      if (lookahead == 'a') ADVANCE(158);
      END_STATE();
    case 19:
      if (lookahead == 'a') ADVANCE(296);
      END_STATE();
    case 20:
      if (lookahead == 'a') ADVANCE(43);
      if (lookahead == 'u') ADVANCE(75);
      END_STATE();
    case 21:
      if (lookahead == 'a') ADVANCE(188);
      END_STATE();
    case 22:
      if (lookahead == 'a') ADVANCE(159);
      END_STATE();
    case 23:
      if (lookahead == 'a') ADVANCE(275);
      END_STATE();
    case 24:
      if (lookahead == 'a') ADVANCE(167);
      END_STATE();
    case 25:
      if (lookahead == 'a') ADVANCE(192);
      END_STATE();
    case 26:
      if (lookahead == 'a') ADVANCE(251);
      END_STATE();
    case 27:
      if (lookahead == 'a') ADVANCE(194);
      END_STATE();
    case 28:
      if (lookahead == 'a') ADVANCE(252);
      END_STATE();
    case 29:
      if (lookahead == 'a') ADVANCE(195);
      END_STATE();
    case 30:
      if (lookahead == 'a') ADVANCE(196);
      END_STATE();
    case 31:
      if (lookahead == 'a') ADVANCE(129);
      END_STATE();
    case 32:
      if (lookahead == 'a') ADVANCE(207);
      END_STATE();
    case 33:
      if (lookahead == 'a') ADVANCE(130);
      END_STATE();
    case 34:
      if (lookahead == 'a') ADVANCE(131);
      END_STATE();
    case 35:
      if (lookahead == 'a') ADVANCE(132);
      END_STATE();
    case 36:
      if (lookahead == 'b') ADVANCE(307);
      END_STATE();
    case 37:
      if (lookahead == 'b') ADVANCE(97);
      END_STATE();
    case 38:
      if (lookahead == 'b') ADVANCE(176);
      if (lookahead == 'c') ADVANCE(315);
      if (lookahead == 'g') ADVANCE(264);
      if (lookahead == 'm') ADVANCE(34);
      if (lookahead == 'r') ADVANCE(94);
      if (lookahead == 'y') ADVANCE(110);
      END_STATE();
    case 39:
      if (lookahead == 'b') ADVANCE(177);
      if (lookahead == 'c') ADVANCE(316);
      if (lookahead == 'g') ADVANCE(265);
      if (lookahead == 'm') ADVANCE(35);
      if (lookahead == 'r') ADVANCE(95);
      if (lookahead == 'y') ADVANCE(112);
      END_STATE();
    case 40:
      if (lookahead == 'c') ADVANCE(421);
      END_STATE();
    case 41:
      if (lookahead == 'c') ADVANCE(134);
      END_STATE();
    case 42:
      if (lookahead == 'c') ADVANCE(367);
      END_STATE();
    case 43:
      if (lookahead == 'c') ADVANCE(156);
      END_STATE();
    case 44:
      if (lookahead == 'c') ADVANCE(140);
      END_STATE();
    case 45:
      if (lookahead == 'c') ADVANCE(228);
      if (lookahead == 'p') ADVANCE(259);
      END_STATE();
    case 46:
      if (lookahead == 'c') ADVANCE(284);
      END_STATE();
    case 47:
      if (lookahead == 'c') ADVANCE(169);
      END_STATE();
    case 48:
      if (lookahead == 'c') ADVANCE(282);
      END_STATE();
    case 49:
      if (lookahead == 'c') ADVANCE(280);
      END_STATE();
    case 50:
      if (lookahead == 'd') ADVANCE(339);
      if (lookahead == 'g') ADVANCE(347);
      if (lookahead == 'n') ADVANCE(335);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(328);
      END_STATE();
    case 51:
      if (lookahead == 'd') ADVANCE(419);
      END_STATE();
    case 52:
      if (lookahead == 'd') ADVANCE(368);
      END_STATE();
    case 53:
      if (lookahead == 'd') ADVANCE(379);
      if (lookahead == 'v') ADVANCE(101);
      END_STATE();
    case 54:
      if (lookahead == 'd') ADVANCE(366);
      END_STATE();
    case 55:
      if (lookahead == 'd') ADVANCE(380);
      END_STATE();
    case 56:
      if (lookahead == 'd') ADVANCE(381);
      END_STATE();
    case 57:
      if (lookahead == 'd') ADVANCE(382);
      END_STATE();
    case 58:
      if (lookahead == 'd') ADVANCE(154);
      if (lookahead == 's') ADVANCE(283);
      END_STATE();
    case 59:
      if (lookahead == 'd') ADVANCE(69);
      END_STATE();
    case 60:
      if (lookahead == 'd') ADVANCE(143);
      END_STATE();
    case 61:
      if (lookahead == 'd') ADVANCE(119);
      END_STATE();
    case 62:
      if (lookahead == 'e') ADVANCE(331);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(328);
      END_STATE();
    case 63:
      if (lookahead == 'e') ADVANCE(353);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(328);
      END_STATE();
    case 64:
      if (lookahead == 'e') ADVANCE(350);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(328);
      END_STATE();
    case 65:
      if (lookahead == 'e') ADVANCE(240);
      END_STATE();
    case 66:
      if (lookahead == 'e') ADVANCE(426);
      END_STATE();
    case 67:
      if (lookahead == 'e') ADVANCE(423);
      END_STATE();
    case 68:
      if (lookahead == 'e') ADVANCE(436);
      END_STATE();
    case 69:
      if (lookahead == 'e') ADVANCE(422);
      END_STATE();
    case 70:
      if (lookahead == 'e') ADVANCE(442);
      END_STATE();
    case 71:
      if (lookahead == 'e') ADVANCE(428);
      END_STATE();
    case 72:
      if (lookahead == 'e') ADVANCE(441);
      END_STATE();
    case 73:
      if (lookahead == 'e') ADVANCE(53);
      END_STATE();
    case 74:
      if (lookahead == 'e') ADVANCE(100);
      END_STATE();
    case 75:
      if (lookahead == 'e') ADVANCE(391);
      END_STATE();
    case 76:
      if (lookahead == 'e') ADVANCE(403);
      END_STATE();
    case 77:
      if (lookahead == 'e') ADVANCE(370);
      END_STATE();
    case 78:
      if (lookahead == 'e') ADVANCE(392);
      END_STATE();
    case 79:
      if (lookahead == 'e') ADVANCE(364);
      END_STATE();
    case 80:
      if (lookahead == 'e') ADVANCE(393);
      END_STATE();
    case 81:
      if (lookahead == 'e') ADVANCE(394);
      END_STATE();
    case 82:
      if (lookahead == 'e') ADVANCE(369);
      END_STATE();
    case 83:
      if (lookahead == 'e') ADVANCE(5);
      END_STATE();
    case 84:
      if (lookahead == 'e') ADVANCE(120);
      END_STATE();
    case 85:
      if (lookahead == 'e') ADVANCE(102);
      END_STATE();
    case 86:
      if (lookahead == 'e') ADVANCE(157);
      END_STATE();
    case 87:
      if (lookahead == 'e') ADVANCE(21);
      END_STATE();
    case 88:
      if (lookahead == 'e') ADVANCE(103);
      END_STATE();
    case 89:
      if (lookahead == 'e') ADVANCE(6);
      END_STATE();
    case 90:
      if (lookahead == 'e') ADVANCE(52);
      END_STATE();
    case 91:
      if (lookahead == 'e') ADVANCE(104);
      END_STATE();
    case 92:
      if (lookahead == 'e') ADVANCE(55);
      END_STATE();
    case 93:
      if (lookahead == 'e') ADVANCE(257);
      END_STATE();
    case 94:
      if (lookahead == 'e') ADVANCE(56);
      END_STATE();
    case 95:
      if (lookahead == 'e') ADVANCE(57);
      END_STATE();
    case 96:
      if (lookahead == 'e') ADVANCE(171);
      END_STATE();
    case 97:
      if (lookahead == 'e') ADVANCE(246);
      END_STATE();
    case 98:
      if (lookahead == 'e') ADVANCE(249);
      END_STATE();
    case 99:
      if (lookahead == 'e') ADVANCE(250);
      END_STATE();
    case 100:
      if (lookahead == 'e') ADVANCE(193);
      if (lookahead == 'y') ADVANCE(404);
      END_STATE();
    case 101:
      if (lookahead == 'e') ADVANCE(258);
      END_STATE();
    case 102:
      if (lookahead == 'e') ADVANCE(197);
      if (lookahead == 'y') ADVANCE(376);
      END_STATE();
    case 103:
      if (lookahead == 'e') ADVANCE(198);
      if (lookahead == 'y') ADVANCE(377);
      END_STATE();
    case 104:
      if (lookahead == 'e') ADVANCE(199);
      if (lookahead == 'y') ADVANCE(378);
      END_STATE();
    case 105:
      if (lookahead == 'e') ADVANCE(175);
      END_STATE();
    case 106:
      if (lookahead == 'e') ADVANCE(205);
      END_STATE();
    case 107:
      if (lookahead == 'e') ADVANCE(172);
      END_STATE();
    case 108:
      if (lookahead == 'e') ADVANCE(23);
      END_STATE();
    case 109:
      if (lookahead == 'e') ADVANCE(49);
      END_STATE();
    case 110:
      if (lookahead == 'e') ADVANCE(173);
      END_STATE();
    case 111:
      if (lookahead == 'e') ADVANCE(208);
      END_STATE();
    case 112:
      if (lookahead == 'e') ADVANCE(174);
      END_STATE();
    case 113:
      if (lookahead == 'e') ADVANCE(210);
      END_STATE();
    case 114:
      if (lookahead == 'e') ADVANCE(216);
      END_STATE();
    case 115:
      if (lookahead == 'e') ADVANCE(217);
      END_STATE();
    case 116:
      if (lookahead == 'e') ADVANCE(218);
      END_STATE();
    case 117:
      if (lookahead == 'e') ADVANCE(219);
      END_STATE();
    case 118:
      if (lookahead == 'e') ADVANCE(185);
      if (lookahead == 'u') ADVANCE(269);
      END_STATE();
    case 119:
      if (lookahead == 'e') ADVANCE(267);
      END_STATE();
    case 120:
      if (lookahead == 'f') ADVANCE(429);
      END_STATE();
    case 121:
      if (lookahead == 'f') ADVANCE(151);
      END_STATE();
    case 122:
      if (lookahead == 'g') ADVANCE(362);
      if (lookahead == 'l') ADVANCE(20);
      if (lookahead == 'o') ADVANCE(163);
      END_STATE();
    case 123:
      if (lookahead == 'g') ADVANCE(362);
      if (lookahead == 'o') ADVANCE(348);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(328);
      END_STATE();
    case 124:
      if (lookahead == 'g') ADVANCE(360);
      END_STATE();
    case 125:
      if (lookahead == 'g') ADVANCE(360);
      if (lookahead == 'l') ADVANCE(349);
      if (lookahead == 'u') ADVANCE(345);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(328);
      END_STATE();
    case 126:
      if (lookahead == 'g') ADVANCE(435);
      END_STATE();
    case 127:
      if (lookahead == 'g') ADVANCE(407);
      END_STATE();
    case 128:
      if (lookahead == 'g') ADVANCE(89);
      END_STATE();
    case 129:
      if (lookahead == 'g') ADVANCE(114);
      END_STATE();
    case 130:
      if (lookahead == 'g') ADVANCE(115);
      END_STATE();
    case 131:
      if (lookahead == 'g') ADVANCE(116);
      END_STATE();
    case 132:
      if (lookahead == 'g') ADVANCE(117);
      END_STATE();
    case 133:
      if (lookahead == 'h') ADVANCE(329);
      if (lookahead == 'o') ADVANCE(343);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(328);
      END_STATE();
    case 134:
      if (lookahead == 'h') ADVANCE(26);
      if (lookahead == 'o') ADVANCE(187);
      END_STATE();
    case 135:
      if (lookahead == 'h') ADVANCE(144);
      END_STATE();
    case 136:
      if (lookahead == 'i') ADVANCE(121);
      END_STATE();
    case 137:
      if (lookahead == 'i') ADVANCE(204);
      END_STATE();
    case 138:
      if (lookahead == 'i') ADVANCE(184);
      END_STATE();
    case 139:
      if (lookahead == 'i') ADVANCE(201);
      if (lookahead == 'u') ADVANCE(48);
      END_STATE();
    case 140:
      if (lookahead == 'i') ADVANCE(18);
      END_STATE();
    case 141:
      if (lookahead == 'i') ADVANCE(42);
      END_STATE();
    case 142:
      if (lookahead == 'i') ADVANCE(295);
      END_STATE();
    case 143:
      if (lookahead == 'i') ADVANCE(278);
      END_STATE();
    case 144:
      if (lookahead == 'i') ADVANCE(292);
      END_STATE();
    case 145:
      if (lookahead == 'i') ADVANCE(209);
      END_STATE();
    case 146:
      if (lookahead == 'i') ADVANCE(231);
      END_STATE();
    case 147:
      if (lookahead == 'i') ADVANCE(211);
      END_STATE();
    case 148:
      if (lookahead == 'i') ADVANCE(233);
      END_STATE();
    case 149:
      if (lookahead == 'i') ADVANCE(212);
      END_STATE();
    case 150:
      if (lookahead == 'i') ADVANCE(234);
      END_STATE();
    case 151:
      if (lookahead == 'i') ADVANCE(99);
      END_STATE();
    case 152:
      if (lookahead == 'i') ADVANCE(213);
      END_STATE();
    case 153:
      if (lookahead == 'i') ADVANCE(235);
      END_STATE();
    case 154:
      if (lookahead == 'i') ADVANCE(297);
      END_STATE();
    case 155:
      if (lookahead == 'k') ADVANCE(4);
      END_STATE();
    case 156:
      if (lookahead == 'k') ADVANCE(375);
      END_STATE();
    case 157:
      if (lookahead == 'l') ADVANCE(417);
      END_STATE();
    case 158:
      if (lookahead == 'l') ADVANCE(430);
      END_STATE();
    case 159:
      if (lookahead == 'l') ADVANCE(415);
      END_STATE();
    case 160:
      if (lookahead == 'l') ADVANCE(349);
      if (lookahead == 'u') ADVANCE(345);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(328);
      END_STATE();
    case 161:
      if (lookahead == 'l') ADVANCE(302);
      END_STATE();
    case 162:
      if (lookahead == 'l') ADVANCE(303);
      END_STATE();
    case 163:
      if (lookahead == 'l') ADVANCE(54);
      END_STATE();
    case 164:
      if (lookahead == 'l') ADVANCE(223);
      END_STATE();
    case 165:
      if (lookahead == 'l') ADVANCE(225);
      END_STATE();
    case 166:
      if (lookahead == 'l') ADVANCE(87);
      END_STATE();
    case 167:
      if (lookahead == 'l') ADVANCE(141);
      END_STATE();
    case 168:
      if (lookahead == 'l') ADVANCE(226);
      END_STATE();
    case 169:
      if (lookahead == 'l') ADVANCE(15);
      END_STATE();
    case 170:
      if (lookahead == 'l') ADVANCE(229);
      END_STATE();
    case 171:
      if (lookahead == 'l') ADVANCE(164);
      END_STATE();
    case 172:
      if (lookahead == 'l') ADVANCE(165);
      END_STATE();
    case 173:
      if (lookahead == 'l') ADVANCE(168);
      END_STATE();
    case 174:
      if (lookahead == 'l') ADVANCE(170);
      END_STATE();
    case 175:
      if (lookahead == 'l') ADVANCE(109);
      END_STATE();
    case 176:
      if (lookahead == 'l') ADVANCE(304);
      END_STATE();
    case 177:
      if (lookahead == 'l') ADVANCE(305);
      END_STATE();
    case 178:
      if (lookahead == 'l') ADVANCE(145);
      if (lookahead == 'o') ADVANCE(241);
      if (lookahead == 's') ADVANCE(105);
      END_STATE();
    case 179:
      if (lookahead == 'l') ADVANCE(147);
      END_STATE();
    case 180:
      if (lookahead == 'l') ADVANCE(149);
      END_STATE();
    case 181:
      if (lookahead == 'l') ADVANCE(152);
      END_STATE();
    case 182:
      if (lookahead == 'm') ADVANCE(106);
      END_STATE();
    case 183:
      if (lookahead == 'm') ADVANCE(239);
      END_STATE();
    case 184:
      if (lookahead == 'm') ADVANCE(142);
      END_STATE();
    case 185:
      if (lookahead == 'm') ADVANCE(111);
      END_STATE();
    case 186:
      if (lookahead == 'm') ADVANCE(113);
      END_STATE();
    case 187:
      if (lookahead == 'm') ADVANCE(186);
      END_STATE();
    case 188:
      if (lookahead == 'n') ADVANCE(410);
      END_STATE();
    case 189:
      if (lookahead == 'n') ADVANCE(413);
      END_STATE();
    case 190:
      if (lookahead == 'n') ADVANCE(420);
      END_STATE();
    case 191:
      if (lookahead == 'n') ADVANCE(443);
      END_STATE();
    case 192:
      if (lookahead == 'n') ADVANCE(399);
      END_STATE();
    case 193:
      if (lookahead == 'n') ADVANCE(383);
      END_STATE();
    case 194:
      if (lookahead == 'n') ADVANCE(400);
      END_STATE();
    case 195:
      if (lookahead == 'n') ADVANCE(401);
      END_STATE();
    case 196:
      if (lookahead == 'n') ADVANCE(402);
      END_STATE();
    case 197:
      if (lookahead == 'n') ADVANCE(384);
      END_STATE();
    case 198:
      if (lookahead == 'n') ADVANCE(385);
      END_STATE();
    case 199:
      if (lookahead == 'n') ADVANCE(386);
      END_STATE();
    case 200:
      if (lookahead == 'n') ADVANCE(338);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(328);
      END_STATE();
    case 201:
      if (lookahead == 'n') ADVANCE(127);
      END_STATE();
    case 202:
      if (lookahead == 'n') ADVANCE(60);
      END_STATE();
    case 203:
      if (lookahead == 'n') ADVANCE(22);
      END_STATE();
    case 204:
      if (lookahead == 'n') ADVANCE(67);
      END_STATE();
    case 205:
      if (lookahead == 'n') ADVANCE(276);
      END_STATE();
    case 206:
      if (lookahead == 'n') ADVANCE(247);
      END_STATE();
    case 207:
      if (lookahead == 'n') ADVANCE(277);
      END_STATE();
    case 208:
      if (lookahead == 'n') ADVANCE(279);
      END_STATE();
    case 209:
      if (lookahead == 'n') ADVANCE(70);
      END_STATE();
    case 210:
      if (lookahead == 'n') ADVANCE(281);
      END_STATE();
    case 211:
      if (lookahead == 'n') ADVANCE(90);
      END_STATE();
    case 212:
      if (lookahead == 'n') ADVANCE(72);
      END_STATE();
    case 213:
      if (lookahead == 'n') ADVANCE(82);
      END_STATE();
    case 214:
      if (lookahead == 'n') ADVANCE(61);
      END_STATE();
    case 215:
      if (lookahead == 'n') ADVANCE(285);
      END_STATE();
    case 216:
      if (lookahead == 'n') ADVANCE(286);
      END_STATE();
    case 217:
      if (lookahead == 'n') ADVANCE(287);
      END_STATE();
    case 218:
      if (lookahead == 'n') ADVANCE(288);
      END_STATE();
    case 219:
      if (lookahead == 'n') ADVANCE(289);
      END_STATE();
    case 220:
      if (lookahead == 'o') ADVANCE(348);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(328);
      END_STATE();
    case 221:
      if (lookahead == 'o') ADVANCE(438);
      END_STATE();
    case 222:
      if (lookahead == 'o') ADVANCE(424);
      END_STATE();
    case 223:
      if (lookahead == 'o') ADVANCE(308);
      END_STATE();
    case 224:
      if (lookahead == 'o') ADVANCE(40);
      END_STATE();
    case 225:
      if (lookahead == 'o') ADVANCE(309);
      END_STATE();
    case 226:
      if (lookahead == 'o') ADVANCE(310);
      END_STATE();
    case 227:
      if (lookahead == 'o') ADVANCE(245);
      END_STATE();
    case 228:
      if (lookahead == 'o') ADVANCE(202);
      END_STATE();
    case 229:
      if (lookahead == 'o') ADVANCE(311);
      END_STATE();
    case 230:
      if (lookahead == 'o') ADVANCE(260);
      END_STATE();
    case 231:
      if (lookahead == 'o') ADVANCE(189);
      END_STATE();
    case 232:
      if (lookahead == 'o') ADVANCE(255);
      END_STATE();
    case 233:
      if (lookahead == 'o') ADVANCE(203);
      END_STATE();
    case 234:
      if (lookahead == 'o') ADVANCE(190);
      END_STATE();
    case 235:
      if (lookahead == 'o') ADVANCE(191);
      END_STATE();
    case 236:
      if (lookahead == 'o') ADVANCE(248);
      END_STATE();
    case 237:
      if (lookahead == 'p') ADVANCE(342);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(328);
      END_STATE();
    case 238:
      if (lookahead == 'p') ADVANCE(341);
      if (lookahead == 't') ADVANCE(330);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(328);
      END_STATE();
    case 239:
      if (lookahead == 'p') ADVANCE(274);
      END_STATE();
    case 240:
      if (lookahead == 'p') ADVANCE(298);
      END_STATE();
    case 241:
      if (lookahead == 'p') ADVANCE(299);
      END_STATE();
    case 242:
      if (lookahead == 'r') ADVANCE(352);
      if (lookahead == 'x') ADVANCE(334);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(328);
      END_STATE();
    case 243:
      if (lookahead == 'r') ADVANCE(340);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(328);
      END_STATE();
    case 244:
      if (lookahead == 'r') ADVANCE(408);
      END_STATE();
    case 245:
      if (lookahead == 'r') ADVANCE(437);
      END_STATE();
    case 246:
      if (lookahead == 'r') ADVANCE(409);
      END_STATE();
    case 247:
      if (lookahead == 'r') ADVANCE(439);
      END_STATE();
    case 248:
      if (lookahead == 'r') ADVANCE(418);
      END_STATE();
    case 249:
      if (lookahead == 'r') ADVANCE(433);
      END_STATE();
    case 250:
      if (lookahead == 'r') ADVANCE(412);
      END_STATE();
    case 251:
      if (lookahead == 'r') ADVANCE(431);
      END_STATE();
    case 252:
      if (lookahead == 'r') ADVANCE(155);
      END_STATE();
    case 253:
      if (lookahead == 'r') ADVANCE(363);
      END_STATE();
    case 254:
      if (lookahead == 'r') ADVANCE(14);
      END_STATE();
    case 255:
      if (lookahead == 'r') ADVANCE(51);
      END_STATE();
    case 256:
      if (lookahead == 'r') ADVANCE(222);
      END_STATE();
    case 257:
      if (lookahead == 'r') ADVANCE(179);
      END_STATE();
    case 258:
      if (lookahead == 'r') ADVANCE(271);
      END_STATE();
    case 259:
      if (lookahead == 'r') ADVANCE(224);
      END_STATE();
    case 260:
      if (lookahead == 'r') ADVANCE(68);
      END_STATE();
    case 261:
      if (lookahead == 'r') ADVANCE(71);
      END_STATE();
    case 262:
      if (lookahead == 'r') ADVANCE(74);
      END_STATE();
    case 263:
      if (lookahead == 'r') ADVANCE(85);
      END_STATE();
    case 264:
      if (lookahead == 'r') ADVANCE(88);
      END_STATE();
    case 265:
      if (lookahead == 'r') ADVANCE(91);
      END_STATE();
    case 266:
      if (lookahead == 'r') ADVANCE(19);
      END_STATE();
    case 267:
      if (lookahead == 'r') ADVANCE(181);
      END_STATE();
    case 268:
      if (lookahead == 's') ADVANCE(427);
      END_STATE();
    case 269:
      if (lookahead == 's') ADVANCE(7);
      END_STATE();
    case 270:
      if (lookahead == 's') ADVANCE(268);
      END_STATE();
    case 271:
      if (lookahead == 's') ADVANCE(77);
      END_STATE();
    case 272:
      if (lookahead == 't') ADVANCE(118);
      END_STATE();
    case 273:
      if (lookahead == 't') ADVANCE(411);
      END_STATE();
    case 274:
      if (lookahead == 't') ADVANCE(440);
      END_STATE();
    case 275:
      if (lookahead == 't') ADVANCE(416);
      END_STATE();
    case 276:
      if (lookahead == 't') ADVANCE(405);
      END_STATE();
    case 277:
      if (lookahead == 't') ADVANCE(406);
      END_STATE();
    case 278:
      if (lookahead == 't') ADVANCE(425);
      END_STATE();
    case 279:
      if (lookahead == 't') ADVANCE(414);
      END_STATE();
    case 280:
      if (lookahead == 't') ADVANCE(444);
      END_STATE();
    case 281:
      if (lookahead == 't') ADVANCE(434);
      END_STATE();
    case 282:
      if (lookahead == 't') ADVANCE(306);
      END_STATE();
    case 283:
      if (lookahead == 't') ADVANCE(32);
      END_STATE();
    case 284:
      if (lookahead == 't') ADVANCE(146);
      END_STATE();
    case 285:
      if (lookahead == 't') ADVANCE(136);
      END_STATE();
    case 286:
      if (lookahead == 't') ADVANCE(10);
      END_STATE();
    case 287:
      if (lookahead == 't') ADVANCE(11);
      END_STATE();
    case 288:
      if (lookahead == 't') ADVANCE(12);
      END_STATE();
    case 289:
      if (lookahead == 't') ADVANCE(13);
      END_STATE();
    case 290:
      if (lookahead == 't') ADVANCE(253);
      END_STATE();
    case 291:
      if (lookahead == 't') ADVANCE(290);
      END_STATE();
    case 292:
      if (lookahead == 't') ADVANCE(76);
      END_STATE();
    case 293:
      if (lookahead == 't') ADVANCE(79);
      END_STATE();
    case 294:
      if (lookahead == 't') ADVANCE(24);
      END_STATE();
    case 295:
      if (lookahead == 't') ADVANCE(98);
      END_STATE();
    case 296:
      if (lookahead == 't') ADVANCE(236);
      END_STATE();
    case 297:
      if (lookahead == 't') ADVANCE(148);
      END_STATE();
    case 298:
      if (lookahead == 't') ADVANCE(150);
      END_STATE();
    case 299:
      if (lookahead == 't') ADVANCE(153);
      END_STATE();
    case 300:
      if (lookahead == 'u') ADVANCE(344);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(328);
      END_STATE();
    case 301:
      if (lookahead == 'u') ADVANCE(126);
      END_STATE();
    case 302:
      if (lookahead == 'u') ADVANCE(59);
      END_STATE();
    case 303:
      if (lookahead == 'u') ADVANCE(78);
      END_STATE();
    case 304:
      if (lookahead == 'u') ADVANCE(80);
      END_STATE();
    case 305:
      if (lookahead == 'u') ADVANCE(81);
      END_STATE();
    case 306:
      if (lookahead == 'u') ADVANCE(261);
      END_STATE();
    case 307:
      if (lookahead == 'u') ADVANCE(293);
      END_STATE();
    case 308:
      if (lookahead == 'w') ADVANCE(387);
      END_STATE();
    case 309:
      if (lookahead == 'w') ADVANCE(388);
      END_STATE();
    case 310:
      if (lookahead == 'w') ADVANCE(389);
      END_STATE();
    case 311:
      if (lookahead == 'w') ADVANCE(390);
      END_STATE();
    case 312:
      if (lookahead == 'w') ADVANCE(232);
      END_STATE();
    case 313:
      if (lookahead == 'y') ADVANCE(25);
      END_STATE();
    case 314:
      if (lookahead == 'y') ADVANCE(27);
      END_STATE();
    case 315:
      if (lookahead == 'y') ADVANCE(29);
      END_STATE();
    case 316:
      if (lookahead == 'y') ADVANCE(30);
      END_STATE();
    case 317:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(374);
      END_STATE();
    case 318:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(371);
      END_STATE();
    case 319:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(318);
      END_STATE();
    case 320:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(319);
      END_STATE();
    case 321:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(320);
      END_STATE();
    case 322:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(321);
      END_STATE();
    case 323:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(322);
      END_STATE();
    case 324:
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(328);
      END_STATE();
    case 325:
      if (eof) ADVANCE(326);
      if (lookahead == '.') ADVANCE(354);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(325)
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(324);
      END_STATE();
    case 326:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 327:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 328:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      END_STATE();
    case 329:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      if (lookahead == 'a') ADVANCE(244);
      END_STATE();
    case 330:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      if (lookahead == 'a') ADVANCE(272);
      if (lookahead == 'o') ADVANCE(254);
      if (lookahead == 'r') ADVANCE(139);
      END_STATE();
    case 331:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      if (lookahead == 'b') ADVANCE(301);
      if (lookahead == 'f') ADVANCE(137);
      if (lookahead == 'l') ADVANCE(138);
      END_STATE();
    case 332:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      if (lookahead == 'b') ADVANCE(3);
      if (lookahead == 'g') ADVANCE(432);
      END_STATE();
    case 333:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      if (lookahead == 'b') ADVANCE(86);
      END_STATE();
    case 334:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      if (lookahead == 'c') ADVANCE(65);
      END_STATE();
    case 335:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      if (lookahead == 'c') ADVANCE(161);
      END_STATE();
    case 336:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      if (lookahead == 'c') ADVANCE(256);
      END_STATE();
    case 337:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      if (lookahead == 'd') ADVANCE(221);
      END_STATE();
    case 338:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      if (lookahead == 'd') ADVANCE(93);
      END_STATE();
    case 339:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      if (lookahead == 'e') ADVANCE(215);
      END_STATE();
    case 340:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      if (lookahead == 'e') ADVANCE(45);
      if (lookahead == 'o') ADVANCE(183);
      END_STATE();
    case 341:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      if (lookahead == 'e') ADVANCE(44);
      END_STATE();
    case 342:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      if (lookahead == 'e') ADVANCE(266);
      END_STATE();
    case 343:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      if (lookahead == 'm') ADVANCE(182);
      if (lookahead == 'n') ADVANCE(58);
      END_STATE();
    case 344:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      if (lookahead == 'm') ADVANCE(37);
      END_STATE();
    case 345:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      if (lookahead == 'n') ADVANCE(46);
      END_STATE();
    case 346:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      if (lookahead == 'n') ADVANCE(83);
      END_STATE();
    case 347:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      if (lookahead == 'n') ADVANCE(230);
      END_STATE();
    case 348:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      if (lookahead == 'o') ADVANCE(166);
      END_STATE();
    case 349:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      if (lookahead == 'o') ADVANCE(17);
      END_STATE();
    case 350:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      if (lookahead == 'p') ADVANCE(108);
      END_STATE();
    case 351:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      if (lookahead == 'p') ADVANCE(66);
      END_STATE();
    case 352:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      if (lookahead == 'r') ADVANCE(227);
      END_STATE();
    case 353:
      ACCEPT_TOKEN(aux_sym_symbol_name_token1);
      if (lookahead == 'y') ADVANCE(312);
      END_STATE();
    case 354:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 355:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 356:
      ACCEPT_TOKEN(anon_sym_TILDE);
      END_STATE();
    case 357:
      ACCEPT_TOKEN(anon_sym_GT);
      END_STATE();
    case 358:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 359:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 360:
      ACCEPT_TOKEN(anon_sym_fg);
      END_STATE();
    case 361:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 362:
      ACCEPT_TOKEN(anon_sym_bg);
      END_STATE();
    case 363:
      ACCEPT_TOKEN(anon_sym_attr);
      if (lookahead == 'i') ADVANCE(36);
      END_STATE();
    case 364:
      ACCEPT_TOKEN(anon_sym_attribute);
      END_STATE();
    case 365:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 366:
      ACCEPT_TOKEN(anon_sym_bold);
      END_STATE();
    case 367:
      ACCEPT_TOKEN(anon_sym_italic);
      END_STATE();
    case 368:
      ACCEPT_TOKEN(anon_sym_underlined);
      END_STATE();
    case 369:
      ACCEPT_TOKEN(anon_sym_underline);
      if (lookahead == 'd') ADVANCE(368);
      END_STATE();
    case 370:
      ACCEPT_TOKEN(anon_sym_reverse);
      END_STATE();
    case 371:
      ACCEPT_TOKEN(sym_rgb_color);
      END_STATE();
    case 372:
      ACCEPT_TOKEN(aux_sym_ansi_color_token1);
      if (lookahead == 'x') ADVANCE(317);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(373);
      END_STATE();
    case 373:
      ACCEPT_TOKEN(aux_sym_ansi_color_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(373);
      END_STATE();
    case 374:
      ACCEPT_TOKEN(aux_sym_ansi_color_token2);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(374);
      END_STATE();
    case 375:
      ACCEPT_TOKEN(anon_sym_black);
      END_STATE();
    case 376:
      ACCEPT_TOKEN(anon_sym_darkgrey);
      END_STATE();
    case 377:
      ACCEPT_TOKEN(anon_sym_dark_DASHgrey);
      END_STATE();
    case 378:
      ACCEPT_TOKEN(anon_sym_dark_grey);
      END_STATE();
    case 379:
      ACCEPT_TOKEN(anon_sym_red);
      END_STATE();
    case 380:
      ACCEPT_TOKEN(anon_sym_darkred);
      END_STATE();
    case 381:
      ACCEPT_TOKEN(anon_sym_dark_DASHred);
      END_STATE();
    case 382:
      ACCEPT_TOKEN(anon_sym_dark_red);
      END_STATE();
    case 383:
      ACCEPT_TOKEN(anon_sym_green);
      END_STATE();
    case 384:
      ACCEPT_TOKEN(anon_sym_darkgreen);
      END_STATE();
    case 385:
      ACCEPT_TOKEN(anon_sym_dark_DASHgreen);
      END_STATE();
    case 386:
      ACCEPT_TOKEN(anon_sym_dark_green);
      END_STATE();
    case 387:
      ACCEPT_TOKEN(anon_sym_yellow);
      END_STATE();
    case 388:
      ACCEPT_TOKEN(anon_sym_darkyellow);
      END_STATE();
    case 389:
      ACCEPT_TOKEN(anon_sym_dark_DASHyellow);
      END_STATE();
    case 390:
      ACCEPT_TOKEN(anon_sym_dark_yellow);
      END_STATE();
    case 391:
      ACCEPT_TOKEN(anon_sym_blue);
      END_STATE();
    case 392:
      ACCEPT_TOKEN(anon_sym_darkblue);
      END_STATE();
    case 393:
      ACCEPT_TOKEN(anon_sym_dark_DASHblue);
      END_STATE();
    case 394:
      ACCEPT_TOKEN(anon_sym_dark_blue);
      END_STATE();
    case 395:
      ACCEPT_TOKEN(anon_sym_magenta);
      END_STATE();
    case 396:
      ACCEPT_TOKEN(anon_sym_darkmagenta);
      END_STATE();
    case 397:
      ACCEPT_TOKEN(anon_sym_dark_DASHmagenta);
      END_STATE();
    case 398:
      ACCEPT_TOKEN(anon_sym_dark_magenta);
      END_STATE();
    case 399:
      ACCEPT_TOKEN(anon_sym_cyan);
      END_STATE();
    case 400:
      ACCEPT_TOKEN(anon_sym_darkcyan);
      END_STATE();
    case 401:
      ACCEPT_TOKEN(anon_sym_dark_DASHcyan);
      END_STATE();
    case 402:
      ACCEPT_TOKEN(anon_sym_dark_cyan);
      END_STATE();
    case 403:
      ACCEPT_TOKEN(anon_sym_white);
      END_STATE();
    case 404:
      ACCEPT_TOKEN(anon_sym_grey);
      END_STATE();
    case 405:
      ACCEPT_TOKEN(anon_sym_comment);
      END_STATE();
    case 406:
      ACCEPT_TOKEN(anon_sym_constant);
      END_STATE();
    case 407:
      ACCEPT_TOKEN(anon_sym_string);
      END_STATE();
    case 408:
      ACCEPT_TOKEN(anon_sym_char);
      END_STATE();
    case 409:
      ACCEPT_TOKEN(anon_sym_number);
      END_STATE();
    case 410:
      ACCEPT_TOKEN(anon_sym_boolean);
      END_STATE();
    case 411:
      ACCEPT_TOKEN(anon_sym_float);
      END_STATE();
    case 412:
      ACCEPT_TOKEN(anon_sym_identifier);
      END_STATE();
    case 413:
      ACCEPT_TOKEN(anon_sym_function);
      END_STATE();
    case 414:
      ACCEPT_TOKEN(anon_sym_statement);
      END_STATE();
    case 415:
      ACCEPT_TOKEN(anon_sym_conditional);
      END_STATE();
    case 416:
      ACCEPT_TOKEN(anon_sym_repeat);
      END_STATE();
    case 417:
      ACCEPT_TOKEN(anon_sym_label);
      END_STATE();
    case 418:
      ACCEPT_TOKEN(anon_sym_operator);
      END_STATE();
    case 419:
      ACCEPT_TOKEN(anon_sym_keyword);
      END_STATE();
    case 420:
      ACCEPT_TOKEN(anon_sym_exception);
      END_STATE();
    case 421:
      ACCEPT_TOKEN(anon_sym_preproc);
      END_STATE();
    case 422:
      ACCEPT_TOKEN(anon_sym_include);
      END_STATE();
    case 423:
      ACCEPT_TOKEN(anon_sym_define);
      END_STATE();
    case 424:
      ACCEPT_TOKEN(anon_sym_macro);
      END_STATE();
    case 425:
      ACCEPT_TOKEN(anon_sym_precondit);
      END_STATE();
    case 426:
      ACCEPT_TOKEN(anon_sym_type);
      if (lookahead == 'd') ADVANCE(84);
      END_STATE();
    case 427:
      ACCEPT_TOKEN(anon_sym_storage_DASHclass);
      END_STATE();
    case 428:
      ACCEPT_TOKEN(anon_sym_structure);
      END_STATE();
    case 429:
      ACCEPT_TOKEN(anon_sym_typedef);
      END_STATE();
    case 430:
      ACCEPT_TOKEN(anon_sym_special);
      if (lookahead == '-') ADVANCE(41);
      END_STATE();
    case 431:
      ACCEPT_TOKEN(anon_sym_special_DASHchar);
      END_STATE();
    case 432:
      ACCEPT_TOKEN(anon_sym_tag);
      END_STATE();
    case 433:
      ACCEPT_TOKEN(anon_sym_delimiter);
      END_STATE();
    case 434:
      ACCEPT_TOKEN(anon_sym_special_DASHcomment);
      END_STATE();
    case 435:
      ACCEPT_TOKEN(anon_sym_debug);
      END_STATE();
    case 436:
      ACCEPT_TOKEN(anon_sym_ignore);
      END_STATE();
    case 437:
      ACCEPT_TOKEN(anon_sym_error);
      END_STATE();
    case 438:
      ACCEPT_TOKEN(anon_sym_todo);
      END_STATE();
    case 439:
      ACCEPT_TOKEN(anon_sym_line_DASHnr);
      END_STATE();
    case 440:
      ACCEPT_TOKEN(anon_sym_prompt);
      END_STATE();
    case 441:
      ACCEPT_TOKEN(anon_sym_status_DASHline);
      END_STATE();
    case 442:
      ACCEPT_TOKEN(anon_sym_tab_DASHline);
      END_STATE();
    case 443:
      ACCEPT_TOKEN(anon_sym_tab_DASHoption);
      END_STATE();
    case 444:
      ACCEPT_TOKEN(anon_sym_tab_DASHselect);
      END_STATE();
    default:
      return false;
  }
}

static TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 325},
  [2] = {.lex_state = 2},
  [3] = {.lex_state = 2},
  [4] = {.lex_state = 2},
  [5] = {.lex_state = 2},
  [6] = {.lex_state = 2},
  [7] = {.lex_state = 2},
  [8] = {.lex_state = 2},
  [9] = {.lex_state = 2},
  [10] = {.lex_state = 2},
  [11] = {.lex_state = 2},
  [12] = {.lex_state = 2},
  [13] = {.lex_state = 2},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 1},
  [20] = {.lex_state = 1},
  [21] = {.lex_state = 325},
  [22] = {.lex_state = 325},
  [23] = {.lex_state = 1},
  [24] = {.lex_state = 1},
  [25] = {.lex_state = 325},
  [26] = {.lex_state = 1},
  [27] = {.lex_state = 325},
  [28] = {.lex_state = 1},
  [29] = {.lex_state = 1},
  [30] = {.lex_state = 1},
  [31] = {.lex_state = 1},
  [32] = {.lex_state = 325},
  [33] = {.lex_state = 325},
  [34] = {.lex_state = 1},
  [35] = {.lex_state = 1},
  [36] = {.lex_state = 1},
  [37] = {.lex_state = 1},
  [38] = {.lex_state = 1},
  [39] = {.lex_state = 1},
  [40] = {.lex_state = 1},
  [41] = {.lex_state = 1},
  [42] = {.lex_state = 325},
  [43] = {.lex_state = 325},
  [44] = {.lex_state = 325},
  [45] = {.lex_state = 325},
  [46] = {.lex_state = 325},
  [47] = {.lex_state = 325},
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
};

static uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [aux_sym_symbol_name_token1] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_PLUS] = ACTIONS(1),
    [anon_sym_TILDE] = ACTIONS(1),
    [anon_sym_GT] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_fg] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_bg] = ACTIONS(1),
    [anon_sym_PIPE] = ACTIONS(1),
    [anon_sym_underlined] = ACTIONS(1),
    [sym_rgb_color] = ACTIONS(1),
    [aux_sym_ansi_color_token1] = ACTIONS(1),
    [aux_sym_ansi_color_token2] = ACTIONS(1),
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
    [sym_source_file] = STATE(59),
    [sym_selectors] = STATE(14),
    [sym_selector] = STATE(15),
    [sym_sel_symbol] = STATE(5),
    [sym_symbol_name] = STATE(6),
    [sym_sel_field] = STATE(7),
    [sym_sel_symbol_field] = STATE(7),
    [sym_sel_next_child] = STATE(7),
    [sym_sel_prev_child] = STATE(7),
    [sym_sel_child] = STATE(7),
    [aux_sym_source_file_repeat1] = STATE(21),
    [aux_sym_selector_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(3),
    [aux_sym_symbol_name_token1] = ACTIONS(5),
    [anon_sym_DOT] = ACTIONS(7),
  },
};

static uint16_t ts_small_parse_table[] = {
  [0] = 8,
    ACTIONS(7), 1,
      anon_sym_DOT,
    ACTIONS(11), 1,
      aux_sym_symbol_name_token1,
    STATE(3), 1,
      aux_sym_selector_repeat1,
    STATE(5), 1,
      sym_sel_symbol,
    STATE(6), 1,
      sym_symbol_name,
    ACTIONS(13), 2,
      anon_sym_type,
      anon_sym_special,
    STATE(7), 5,
      sym_sel_field,
      sym_sel_symbol_field,
      sym_sel_next_child,
      sym_sel_prev_child,
      sym_sel_child,
    ACTIONS(9), 41,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_underlined,
      anon_sym_comment,
      anon_sym_constant,
      anon_sym_string,
      anon_sym_char,
      anon_sym_number,
      anon_sym_boolean,
      anon_sym_float,
      anon_sym_identifier,
      anon_sym_function,
      anon_sym_statement,
      anon_sym_conditional,
      anon_sym_repeat,
      anon_sym_label,
      anon_sym_operator,
      anon_sym_keyword,
      anon_sym_exception,
      anon_sym_preproc,
      anon_sym_include,
      anon_sym_define,
      anon_sym_macro,
      anon_sym_precondit,
      anon_sym_storage_DASHclass,
      anon_sym_structure,
      anon_sym_typedef,
      anon_sym_special_DASHchar,
      anon_sym_tag,
      anon_sym_delimiter,
      anon_sym_special_DASHcomment,
      anon_sym_debug,
      anon_sym_ignore,
      anon_sym_error,
      anon_sym_todo,
      anon_sym_line_DASHnr,
      anon_sym_prompt,
      anon_sym_status_DASHline,
      anon_sym_tab_DASHline,
      anon_sym_tab_DASHoption,
      anon_sym_tab_DASHselect,
  [70] = 8,
    ACTIONS(17), 1,
      aux_sym_symbol_name_token1,
    ACTIONS(20), 1,
      anon_sym_DOT,
    STATE(3), 1,
      aux_sym_selector_repeat1,
    STATE(5), 1,
      sym_sel_symbol,
    STATE(6), 1,
      sym_symbol_name,
    ACTIONS(23), 2,
      anon_sym_type,
      anon_sym_special,
    STATE(7), 5,
      sym_sel_field,
      sym_sel_symbol_field,
      sym_sel_next_child,
      sym_sel_prev_child,
      sym_sel_child,
    ACTIONS(15), 41,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_underlined,
      anon_sym_comment,
      anon_sym_constant,
      anon_sym_string,
      anon_sym_char,
      anon_sym_number,
      anon_sym_boolean,
      anon_sym_float,
      anon_sym_identifier,
      anon_sym_function,
      anon_sym_statement,
      anon_sym_conditional,
      anon_sym_repeat,
      anon_sym_label,
      anon_sym_operator,
      anon_sym_keyword,
      anon_sym_exception,
      anon_sym_preproc,
      anon_sym_include,
      anon_sym_define,
      anon_sym_macro,
      anon_sym_precondit,
      anon_sym_storage_DASHclass,
      anon_sym_structure,
      anon_sym_typedef,
      anon_sym_special_DASHchar,
      anon_sym_tag,
      anon_sym_delimiter,
      anon_sym_special_DASHcomment,
      anon_sym_debug,
      anon_sym_ignore,
      anon_sym_error,
      anon_sym_todo,
      anon_sym_line_DASHnr,
      anon_sym_prompt,
      anon_sym_status_DASHline,
      anon_sym_tab_DASHline,
      anon_sym_tab_DASHoption,
      anon_sym_tab_DASHselect,
  [140] = 2,
    ACTIONS(27), 3,
      aux_sym_symbol_name_token1,
      anon_sym_type,
      anon_sym_special,
    ACTIONS(25), 45,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_underlined,
      anon_sym_comment,
      anon_sym_constant,
      anon_sym_string,
      anon_sym_char,
      anon_sym_number,
      anon_sym_boolean,
      anon_sym_float,
      anon_sym_identifier,
      anon_sym_function,
      anon_sym_statement,
      anon_sym_conditional,
      anon_sym_repeat,
      anon_sym_label,
      anon_sym_operator,
      anon_sym_keyword,
      anon_sym_exception,
      anon_sym_preproc,
      anon_sym_include,
      anon_sym_define,
      anon_sym_macro,
      anon_sym_precondit,
      anon_sym_storage_DASHclass,
      anon_sym_structure,
      anon_sym_typedef,
      anon_sym_special_DASHchar,
      anon_sym_tag,
      anon_sym_delimiter,
      anon_sym_special_DASHcomment,
      anon_sym_debug,
      anon_sym_ignore,
      anon_sym_error,
      anon_sym_todo,
      anon_sym_line_DASHnr,
      anon_sym_prompt,
      anon_sym_status_DASHline,
      anon_sym_tab_DASHline,
      anon_sym_tab_DASHoption,
      anon_sym_tab_DASHselect,
  [193] = 5,
    ACTIONS(33), 1,
      anon_sym_PLUS,
    ACTIONS(35), 1,
      anon_sym_TILDE,
    ACTIONS(37), 1,
      anon_sym_GT,
    ACTIONS(31), 3,
      aux_sym_symbol_name_token1,
      anon_sym_type,
      anon_sym_special,
    ACTIONS(29), 42,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_LBRACE,
      anon_sym_underlined,
      anon_sym_comment,
      anon_sym_constant,
      anon_sym_string,
      anon_sym_char,
      anon_sym_number,
      anon_sym_boolean,
      anon_sym_float,
      anon_sym_identifier,
      anon_sym_function,
      anon_sym_statement,
      anon_sym_conditional,
      anon_sym_repeat,
      anon_sym_label,
      anon_sym_operator,
      anon_sym_keyword,
      anon_sym_exception,
      anon_sym_preproc,
      anon_sym_include,
      anon_sym_define,
      anon_sym_macro,
      anon_sym_precondit,
      anon_sym_storage_DASHclass,
      anon_sym_structure,
      anon_sym_typedef,
      anon_sym_special_DASHchar,
      anon_sym_tag,
      anon_sym_delimiter,
      anon_sym_special_DASHcomment,
      anon_sym_debug,
      anon_sym_ignore,
      anon_sym_error,
      anon_sym_todo,
      anon_sym_line_DASHnr,
      anon_sym_prompt,
      anon_sym_status_DASHline,
      anon_sym_tab_DASHline,
      anon_sym_tab_DASHoption,
      anon_sym_tab_DASHselect,
  [252] = 3,
    ACTIONS(43), 1,
      anon_sym_DOT,
    ACTIONS(41), 3,
      aux_sym_symbol_name_token1,
      anon_sym_type,
      anon_sym_special,
    ACTIONS(39), 44,
      anon_sym_COMMA,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_underlined,
      anon_sym_comment,
      anon_sym_constant,
      anon_sym_string,
      anon_sym_char,
      anon_sym_number,
      anon_sym_boolean,
      anon_sym_float,
      anon_sym_identifier,
      anon_sym_function,
      anon_sym_statement,
      anon_sym_conditional,
      anon_sym_repeat,
      anon_sym_label,
      anon_sym_operator,
      anon_sym_keyword,
      anon_sym_exception,
      anon_sym_preproc,
      anon_sym_include,
      anon_sym_define,
      anon_sym_macro,
      anon_sym_precondit,
      anon_sym_storage_DASHclass,
      anon_sym_structure,
      anon_sym_typedef,
      anon_sym_special_DASHchar,
      anon_sym_tag,
      anon_sym_delimiter,
      anon_sym_special_DASHcomment,
      anon_sym_debug,
      anon_sym_ignore,
      anon_sym_error,
      anon_sym_todo,
      anon_sym_line_DASHnr,
      anon_sym_prompt,
      anon_sym_status_DASHline,
      anon_sym_tab_DASHline,
      anon_sym_tab_DASHoption,
      anon_sym_tab_DASHselect,
  [307] = 2,
    ACTIONS(41), 3,
      aux_sym_symbol_name_token1,
      anon_sym_type,
      anon_sym_special,
    ACTIONS(39), 45,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_underlined,
      anon_sym_comment,
      anon_sym_constant,
      anon_sym_string,
      anon_sym_char,
      anon_sym_number,
      anon_sym_boolean,
      anon_sym_float,
      anon_sym_identifier,
      anon_sym_function,
      anon_sym_statement,
      anon_sym_conditional,
      anon_sym_repeat,
      anon_sym_label,
      anon_sym_operator,
      anon_sym_keyword,
      anon_sym_exception,
      anon_sym_preproc,
      anon_sym_include,
      anon_sym_define,
      anon_sym_macro,
      anon_sym_precondit,
      anon_sym_storage_DASHclass,
      anon_sym_structure,
      anon_sym_typedef,
      anon_sym_special_DASHchar,
      anon_sym_tag,
      anon_sym_delimiter,
      anon_sym_special_DASHcomment,
      anon_sym_debug,
      anon_sym_ignore,
      anon_sym_error,
      anon_sym_todo,
      anon_sym_line_DASHnr,
      anon_sym_prompt,
      anon_sym_status_DASHline,
      anon_sym_tab_DASHline,
      anon_sym_tab_DASHoption,
      anon_sym_tab_DASHselect,
  [360] = 2,
    ACTIONS(47), 3,
      aux_sym_symbol_name_token1,
      anon_sym_type,
      anon_sym_special,
    ACTIONS(45), 45,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_underlined,
      anon_sym_comment,
      anon_sym_constant,
      anon_sym_string,
      anon_sym_char,
      anon_sym_number,
      anon_sym_boolean,
      anon_sym_float,
      anon_sym_identifier,
      anon_sym_function,
      anon_sym_statement,
      anon_sym_conditional,
      anon_sym_repeat,
      anon_sym_label,
      anon_sym_operator,
      anon_sym_keyword,
      anon_sym_exception,
      anon_sym_preproc,
      anon_sym_include,
      anon_sym_define,
      anon_sym_macro,
      anon_sym_precondit,
      anon_sym_storage_DASHclass,
      anon_sym_structure,
      anon_sym_typedef,
      anon_sym_special_DASHchar,
      anon_sym_tag,
      anon_sym_delimiter,
      anon_sym_special_DASHcomment,
      anon_sym_debug,
      anon_sym_ignore,
      anon_sym_error,
      anon_sym_todo,
      anon_sym_line_DASHnr,
      anon_sym_prompt,
      anon_sym_status_DASHline,
      anon_sym_tab_DASHline,
      anon_sym_tab_DASHoption,
      anon_sym_tab_DASHselect,
  [413] = 2,
    ACTIONS(51), 3,
      aux_sym_symbol_name_token1,
      anon_sym_type,
      anon_sym_special,
    ACTIONS(49), 45,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_underlined,
      anon_sym_comment,
      anon_sym_constant,
      anon_sym_string,
      anon_sym_char,
      anon_sym_number,
      anon_sym_boolean,
      anon_sym_float,
      anon_sym_identifier,
      anon_sym_function,
      anon_sym_statement,
      anon_sym_conditional,
      anon_sym_repeat,
      anon_sym_label,
      anon_sym_operator,
      anon_sym_keyword,
      anon_sym_exception,
      anon_sym_preproc,
      anon_sym_include,
      anon_sym_define,
      anon_sym_macro,
      anon_sym_precondit,
      anon_sym_storage_DASHclass,
      anon_sym_structure,
      anon_sym_typedef,
      anon_sym_special_DASHchar,
      anon_sym_tag,
      anon_sym_delimiter,
      anon_sym_special_DASHcomment,
      anon_sym_debug,
      anon_sym_ignore,
      anon_sym_error,
      anon_sym_todo,
      anon_sym_line_DASHnr,
      anon_sym_prompt,
      anon_sym_status_DASHline,
      anon_sym_tab_DASHline,
      anon_sym_tab_DASHoption,
      anon_sym_tab_DASHselect,
  [466] = 2,
    ACTIONS(55), 3,
      aux_sym_symbol_name_token1,
      anon_sym_type,
      anon_sym_special,
    ACTIONS(53), 45,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_underlined,
      anon_sym_comment,
      anon_sym_constant,
      anon_sym_string,
      anon_sym_char,
      anon_sym_number,
      anon_sym_boolean,
      anon_sym_float,
      anon_sym_identifier,
      anon_sym_function,
      anon_sym_statement,
      anon_sym_conditional,
      anon_sym_repeat,
      anon_sym_label,
      anon_sym_operator,
      anon_sym_keyword,
      anon_sym_exception,
      anon_sym_preproc,
      anon_sym_include,
      anon_sym_define,
      anon_sym_macro,
      anon_sym_precondit,
      anon_sym_storage_DASHclass,
      anon_sym_structure,
      anon_sym_typedef,
      anon_sym_special_DASHchar,
      anon_sym_tag,
      anon_sym_delimiter,
      anon_sym_special_DASHcomment,
      anon_sym_debug,
      anon_sym_ignore,
      anon_sym_error,
      anon_sym_todo,
      anon_sym_line_DASHnr,
      anon_sym_prompt,
      anon_sym_status_DASHline,
      anon_sym_tab_DASHline,
      anon_sym_tab_DASHoption,
      anon_sym_tab_DASHselect,
  [519] = 2,
    ACTIONS(59), 3,
      aux_sym_symbol_name_token1,
      anon_sym_type,
      anon_sym_special,
    ACTIONS(57), 45,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_underlined,
      anon_sym_comment,
      anon_sym_constant,
      anon_sym_string,
      anon_sym_char,
      anon_sym_number,
      anon_sym_boolean,
      anon_sym_float,
      anon_sym_identifier,
      anon_sym_function,
      anon_sym_statement,
      anon_sym_conditional,
      anon_sym_repeat,
      anon_sym_label,
      anon_sym_operator,
      anon_sym_keyword,
      anon_sym_exception,
      anon_sym_preproc,
      anon_sym_include,
      anon_sym_define,
      anon_sym_macro,
      anon_sym_precondit,
      anon_sym_storage_DASHclass,
      anon_sym_structure,
      anon_sym_typedef,
      anon_sym_special_DASHchar,
      anon_sym_tag,
      anon_sym_delimiter,
      anon_sym_special_DASHcomment,
      anon_sym_debug,
      anon_sym_ignore,
      anon_sym_error,
      anon_sym_todo,
      anon_sym_line_DASHnr,
      anon_sym_prompt,
      anon_sym_status_DASHline,
      anon_sym_tab_DASHline,
      anon_sym_tab_DASHoption,
      anon_sym_tab_DASHselect,
  [572] = 2,
    ACTIONS(63), 3,
      aux_sym_symbol_name_token1,
      anon_sym_type,
      anon_sym_special,
    ACTIONS(61), 45,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_underlined,
      anon_sym_comment,
      anon_sym_constant,
      anon_sym_string,
      anon_sym_char,
      anon_sym_number,
      anon_sym_boolean,
      anon_sym_float,
      anon_sym_identifier,
      anon_sym_function,
      anon_sym_statement,
      anon_sym_conditional,
      anon_sym_repeat,
      anon_sym_label,
      anon_sym_operator,
      anon_sym_keyword,
      anon_sym_exception,
      anon_sym_preproc,
      anon_sym_include,
      anon_sym_define,
      anon_sym_macro,
      anon_sym_precondit,
      anon_sym_storage_DASHclass,
      anon_sym_structure,
      anon_sym_typedef,
      anon_sym_special_DASHchar,
      anon_sym_tag,
      anon_sym_delimiter,
      anon_sym_special_DASHcomment,
      anon_sym_debug,
      anon_sym_ignore,
      anon_sym_error,
      anon_sym_todo,
      anon_sym_line_DASHnr,
      anon_sym_prompt,
      anon_sym_status_DASHline,
      anon_sym_tab_DASHline,
      anon_sym_tab_DASHoption,
      anon_sym_tab_DASHselect,
  [625] = 2,
    ACTIONS(67), 3,
      aux_sym_symbol_name_token1,
      anon_sym_type,
      anon_sym_special,
    ACTIONS(65), 45,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_underlined,
      anon_sym_comment,
      anon_sym_constant,
      anon_sym_string,
      anon_sym_char,
      anon_sym_number,
      anon_sym_boolean,
      anon_sym_float,
      anon_sym_identifier,
      anon_sym_function,
      anon_sym_statement,
      anon_sym_conditional,
      anon_sym_repeat,
      anon_sym_label,
      anon_sym_operator,
      anon_sym_keyword,
      anon_sym_exception,
      anon_sym_preproc,
      anon_sym_include,
      anon_sym_define,
      anon_sym_macro,
      anon_sym_precondit,
      anon_sym_storage_DASHclass,
      anon_sym_structure,
      anon_sym_typedef,
      anon_sym_special_DASHchar,
      anon_sym_tag,
      anon_sym_delimiter,
      anon_sym_special_DASHcomment,
      anon_sym_debug,
      anon_sym_ignore,
      anon_sym_error,
      anon_sym_todo,
      anon_sym_line_DASHnr,
      anon_sym_prompt,
      anon_sym_status_DASHline,
      anon_sym_tab_DASHline,
      anon_sym_tab_DASHoption,
      anon_sym_tab_DASHselect,
  [678] = 4,
    ACTIONS(69), 1,
      anon_sym_LBRACE,
    ACTIONS(73), 2,
      anon_sym_type,
      anon_sym_special,
    STATE(44), 2,
      sym_properties,
      sym_highlight,
    ACTIONS(71), 39,
      anon_sym_underlined,
      anon_sym_comment,
      anon_sym_constant,
      anon_sym_string,
      anon_sym_char,
      anon_sym_number,
      anon_sym_boolean,
      anon_sym_float,
      anon_sym_identifier,
      anon_sym_function,
      anon_sym_statement,
      anon_sym_conditional,
      anon_sym_repeat,
      anon_sym_label,
      anon_sym_operator,
      anon_sym_keyword,
      anon_sym_exception,
      anon_sym_preproc,
      anon_sym_include,
      anon_sym_define,
      anon_sym_macro,
      anon_sym_precondit,
      anon_sym_storage_DASHclass,
      anon_sym_structure,
      anon_sym_typedef,
      anon_sym_special_DASHchar,
      anon_sym_tag,
      anon_sym_delimiter,
      anon_sym_special_DASHcomment,
      anon_sym_debug,
      anon_sym_ignore,
      anon_sym_error,
      anon_sym_todo,
      anon_sym_line_DASHnr,
      anon_sym_prompt,
      anon_sym_status_DASHline,
      anon_sym_tab_DASHline,
      anon_sym_tab_DASHoption,
      anon_sym_tab_DASHselect,
  [731] = 4,
    ACTIONS(75), 1,
      anon_sym_COMMA,
    STATE(17), 1,
      aux_sym_selectors_repeat1,
    ACTIONS(79), 2,
      anon_sym_type,
      anon_sym_special,
    ACTIONS(77), 40,
      anon_sym_LBRACE,
      anon_sym_underlined,
      anon_sym_comment,
      anon_sym_constant,
      anon_sym_string,
      anon_sym_char,
      anon_sym_number,
      anon_sym_boolean,
      anon_sym_float,
      anon_sym_identifier,
      anon_sym_function,
      anon_sym_statement,
      anon_sym_conditional,
      anon_sym_repeat,
      anon_sym_label,
      anon_sym_operator,
      anon_sym_keyword,
      anon_sym_exception,
      anon_sym_preproc,
      anon_sym_include,
      anon_sym_define,
      anon_sym_macro,
      anon_sym_precondit,
      anon_sym_storage_DASHclass,
      anon_sym_structure,
      anon_sym_typedef,
      anon_sym_special_DASHchar,
      anon_sym_tag,
      anon_sym_delimiter,
      anon_sym_special_DASHcomment,
      anon_sym_debug,
      anon_sym_ignore,
      anon_sym_error,
      anon_sym_todo,
      anon_sym_line_DASHnr,
      anon_sym_prompt,
      anon_sym_status_DASHline,
      anon_sym_tab_DASHline,
      anon_sym_tab_DASHoption,
      anon_sym_tab_DASHselect,
  [784] = 4,
    ACTIONS(81), 1,
      anon_sym_COMMA,
    STATE(16), 1,
      aux_sym_selectors_repeat1,
    ACTIONS(86), 2,
      anon_sym_type,
      anon_sym_special,
    ACTIONS(84), 40,
      anon_sym_LBRACE,
      anon_sym_underlined,
      anon_sym_comment,
      anon_sym_constant,
      anon_sym_string,
      anon_sym_char,
      anon_sym_number,
      anon_sym_boolean,
      anon_sym_float,
      anon_sym_identifier,
      anon_sym_function,
      anon_sym_statement,
      anon_sym_conditional,
      anon_sym_repeat,
      anon_sym_label,
      anon_sym_operator,
      anon_sym_keyword,
      anon_sym_exception,
      anon_sym_preproc,
      anon_sym_include,
      anon_sym_define,
      anon_sym_macro,
      anon_sym_precondit,
      anon_sym_storage_DASHclass,
      anon_sym_structure,
      anon_sym_typedef,
      anon_sym_special_DASHchar,
      anon_sym_tag,
      anon_sym_delimiter,
      anon_sym_special_DASHcomment,
      anon_sym_debug,
      anon_sym_ignore,
      anon_sym_error,
      anon_sym_todo,
      anon_sym_line_DASHnr,
      anon_sym_prompt,
      anon_sym_status_DASHline,
      anon_sym_tab_DASHline,
      anon_sym_tab_DASHoption,
      anon_sym_tab_DASHselect,
  [837] = 4,
    ACTIONS(75), 1,
      anon_sym_COMMA,
    STATE(16), 1,
      aux_sym_selectors_repeat1,
    ACTIONS(90), 2,
      anon_sym_type,
      anon_sym_special,
    ACTIONS(88), 40,
      anon_sym_LBRACE,
      anon_sym_underlined,
      anon_sym_comment,
      anon_sym_constant,
      anon_sym_string,
      anon_sym_char,
      anon_sym_number,
      anon_sym_boolean,
      anon_sym_float,
      anon_sym_identifier,
      anon_sym_function,
      anon_sym_statement,
      anon_sym_conditional,
      anon_sym_repeat,
      anon_sym_label,
      anon_sym_operator,
      anon_sym_keyword,
      anon_sym_exception,
      anon_sym_preproc,
      anon_sym_include,
      anon_sym_define,
      anon_sym_macro,
      anon_sym_precondit,
      anon_sym_storage_DASHclass,
      anon_sym_structure,
      anon_sym_typedef,
      anon_sym_special_DASHchar,
      anon_sym_tag,
      anon_sym_delimiter,
      anon_sym_special_DASHcomment,
      anon_sym_debug,
      anon_sym_ignore,
      anon_sym_error,
      anon_sym_todo,
      anon_sym_line_DASHnr,
      anon_sym_prompt,
      anon_sym_status_DASHline,
      anon_sym_tab_DASHline,
      anon_sym_tab_DASHoption,
      anon_sym_tab_DASHselect,
  [890] = 2,
    ACTIONS(86), 2,
      anon_sym_type,
      anon_sym_special,
    ACTIONS(84), 41,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_underlined,
      anon_sym_comment,
      anon_sym_constant,
      anon_sym_string,
      anon_sym_char,
      anon_sym_number,
      anon_sym_boolean,
      anon_sym_float,
      anon_sym_identifier,
      anon_sym_function,
      anon_sym_statement,
      anon_sym_conditional,
      anon_sym_repeat,
      anon_sym_label,
      anon_sym_operator,
      anon_sym_keyword,
      anon_sym_exception,
      anon_sym_preproc,
      anon_sym_include,
      anon_sym_define,
      anon_sym_macro,
      anon_sym_precondit,
      anon_sym_storage_DASHclass,
      anon_sym_structure,
      anon_sym_typedef,
      anon_sym_special_DASHchar,
      anon_sym_tag,
      anon_sym_delimiter,
      anon_sym_special_DASHcomment,
      anon_sym_debug,
      anon_sym_ignore,
      anon_sym_error,
      anon_sym_todo,
      anon_sym_line_DASHnr,
      anon_sym_prompt,
      anon_sym_status_DASHline,
      anon_sym_tab_DASHline,
      anon_sym_tab_DASHoption,
      anon_sym_tab_DASHselect,
  [938] = 5,
    ACTIONS(92), 1,
      sym_rgb_color,
    ACTIONS(94), 1,
      aux_sym_ansi_color_token1,
    ACTIONS(96), 1,
      aux_sym_ansi_color_token2,
    STATE(50), 2,
      sym_ansi_color,
      sym_color_name,
    ACTIONS(98), 30,
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
  [984] = 5,
    ACTIONS(94), 1,
      aux_sym_ansi_color_token1,
    ACTIONS(96), 1,
      aux_sym_ansi_color_token2,
    ACTIONS(100), 1,
      sym_rgb_color,
    STATE(54), 2,
      sym_ansi_color,
      sym_color_name,
    ACTIONS(98), 30,
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
  [1030] = 10,
    ACTIONS(5), 1,
      aux_sym_symbol_name_token1,
    ACTIONS(7), 1,
      anon_sym_DOT,
    ACTIONS(102), 1,
      ts_builtin_sym_end,
    STATE(2), 1,
      aux_sym_selector_repeat1,
    STATE(5), 1,
      sym_sel_symbol,
    STATE(6), 1,
      sym_symbol_name,
    STATE(14), 1,
      sym_selectors,
    STATE(15), 1,
      sym_selector,
    STATE(22), 1,
      aux_sym_source_file_repeat1,
    STATE(7), 5,
      sym_sel_field,
      sym_sel_symbol_field,
      sym_sel_next_child,
      sym_sel_prev_child,
      sym_sel_child,
  [1065] = 10,
    ACTIONS(104), 1,
      ts_builtin_sym_end,
    ACTIONS(106), 1,
      aux_sym_symbol_name_token1,
    ACTIONS(109), 1,
      anon_sym_DOT,
    STATE(2), 1,
      aux_sym_selector_repeat1,
    STATE(5), 1,
      sym_sel_symbol,
    STATE(6), 1,
      sym_symbol_name,
    STATE(14), 1,
      sym_selectors,
    STATE(15), 1,
      sym_selector,
    STATE(22), 1,
      aux_sym_source_file_repeat1,
    STATE(7), 5,
      sym_sel_field,
      sym_sel_symbol_field,
      sym_sel_next_child,
      sym_sel_prev_child,
      sym_sel_child,
  [1100] = 8,
    ACTIONS(112), 1,
      anon_sym_RBRACE,
    ACTIONS(114), 1,
      anon_sym_fg,
    ACTIONS(117), 1,
      anon_sym_bg,
    ACTIONS(120), 1,
      anon_sym_attr,
    ACTIONS(123), 1,
      anon_sym_attribute,
    STATE(23), 1,
      aux_sym_properties_repeat1,
    STATE(49), 1,
      sym_property,
    STATE(48), 4,
      sym_fg,
      sym_bg,
      sym_attr1,
      sym_attr2,
  [1128] = 8,
    ACTIONS(126), 1,
      anon_sym_RBRACE,
    ACTIONS(128), 1,
      anon_sym_fg,
    ACTIONS(130), 1,
      anon_sym_bg,
    ACTIONS(132), 1,
      anon_sym_attr,
    ACTIONS(134), 1,
      anon_sym_attribute,
    STATE(23), 1,
      aux_sym_properties_repeat1,
    STATE(49), 1,
      sym_property,
    STATE(48), 4,
      sym_fg,
      sym_bg,
      sym_attr1,
      sym_attr2,
  [1156] = 7,
    ACTIONS(5), 1,
      aux_sym_symbol_name_token1,
    ACTIONS(7), 1,
      anon_sym_DOT,
    STATE(2), 1,
      aux_sym_selector_repeat1,
    STATE(5), 1,
      sym_sel_symbol,
    STATE(6), 1,
      sym_symbol_name,
    STATE(18), 1,
      sym_selector,
    STATE(7), 5,
      sym_sel_field,
      sym_sel_symbol_field,
      sym_sel_next_child,
      sym_sel_prev_child,
      sym_sel_child,
  [1182] = 8,
    ACTIONS(128), 1,
      anon_sym_fg,
    ACTIONS(130), 1,
      anon_sym_bg,
    ACTIONS(132), 1,
      anon_sym_attr,
    ACTIONS(134), 1,
      anon_sym_attribute,
    ACTIONS(136), 1,
      anon_sym_RBRACE,
    STATE(24), 1,
      aux_sym_properties_repeat1,
    STATE(49), 1,
      sym_property,
    STATE(48), 4,
      sym_fg,
      sym_bg,
      sym_attr1,
      sym_attr2,
  [1210] = 5,
    ACTIONS(5), 1,
      aux_sym_symbol_name_token1,
    ACTIONS(7), 1,
      anon_sym_DOT,
    STATE(6), 1,
      sym_symbol_name,
    STATE(12), 1,
      sym_sel_symbol,
    STATE(7), 5,
      sym_sel_field,
      sym_sel_symbol_field,
      sym_sel_next_child,
      sym_sel_prev_child,
      sym_sel_child,
  [1230] = 5,
    ACTIONS(138), 1,
      anon_sym_COMMA,
    ACTIONS(142), 1,
      anon_sym_underline,
    STATE(30), 1,
      sym_attr,
    STATE(35), 2,
      sym_attrs,
      aux_sym_attr1_repeat1,
    ACTIONS(140), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [1250] = 5,
    ACTIONS(142), 1,
      anon_sym_underline,
    ACTIONS(144), 1,
      anon_sym_COMMA,
    STATE(30), 1,
      sym_attr,
    STATE(35), 2,
      sym_attrs,
      aux_sym_attr1_repeat1,
    ACTIONS(140), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [1270] = 4,
    ACTIONS(148), 1,
      anon_sym_PIPE,
    ACTIONS(150), 1,
      anon_sym_underline,
    STATE(34), 2,
      sym_attr_or,
      aux_sym_attrs_repeat1,
    ACTIONS(146), 5,
      anon_sym_COMMA,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [1288] = 4,
    ACTIONS(154), 1,
      anon_sym_PIPE,
    ACTIONS(157), 1,
      anon_sym_underline,
    STATE(31), 2,
      sym_attr_or,
      aux_sym_attrs_repeat1,
    ACTIONS(152), 5,
      anon_sym_COMMA,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [1306] = 5,
    ACTIONS(5), 1,
      aux_sym_symbol_name_token1,
    ACTIONS(7), 1,
      anon_sym_DOT,
    STATE(4), 1,
      sym_sel_symbol,
    STATE(6), 1,
      sym_symbol_name,
    STATE(7), 5,
      sym_sel_field,
      sym_sel_symbol_field,
      sym_sel_next_child,
      sym_sel_prev_child,
      sym_sel_child,
  [1326] = 5,
    ACTIONS(5), 1,
      aux_sym_symbol_name_token1,
    ACTIONS(7), 1,
      anon_sym_DOT,
    STATE(6), 1,
      sym_symbol_name,
    STATE(13), 1,
      sym_sel_symbol,
    STATE(7), 5,
      sym_sel_field,
      sym_sel_symbol_field,
      sym_sel_next_child,
      sym_sel_prev_child,
      sym_sel_child,
  [1346] = 4,
    ACTIONS(148), 1,
      anon_sym_PIPE,
    ACTIONS(161), 1,
      anon_sym_underline,
    STATE(31), 2,
      sym_attr_or,
      aux_sym_attrs_repeat1,
    ACTIONS(159), 5,
      anon_sym_COMMA,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [1364] = 5,
    ACTIONS(163), 1,
      anon_sym_COMMA,
    ACTIONS(168), 1,
      anon_sym_underline,
    STATE(30), 1,
      sym_attr,
    STATE(35), 2,
      sym_attrs,
      aux_sym_attr1_repeat1,
    ACTIONS(165), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [1384] = 5,
    ACTIONS(142), 1,
      anon_sym_underline,
    ACTIONS(171), 1,
      anon_sym_COMMA,
    STATE(30), 1,
      sym_attr,
    STATE(29), 2,
      sym_attrs,
      aux_sym_attr1_repeat1,
    ACTIONS(140), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [1404] = 5,
    ACTIONS(142), 1,
      anon_sym_underline,
    ACTIONS(173), 1,
      anon_sym_COMMA,
    STATE(30), 1,
      sym_attr,
    STATE(28), 2,
      sym_attrs,
      aux_sym_attr1_repeat1,
    ACTIONS(140), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [1424] = 2,
    ACTIONS(177), 1,
      anon_sym_underline,
    ACTIONS(175), 6,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [1436] = 2,
    ACTIONS(181), 1,
      anon_sym_underline,
    ACTIONS(179), 6,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [1448] = 3,
    ACTIONS(142), 1,
      anon_sym_underline,
    STATE(39), 1,
      sym_attr,
    ACTIONS(140), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [1461] = 2,
    ACTIONS(183), 1,
      anon_sym_attr,
    ACTIONS(112), 4,
      anon_sym_RBRACE,
      anon_sym_fg,
      anon_sym_bg,
      anon_sym_attribute,
  [1471] = 1,
    ACTIONS(185), 3,
      ts_builtin_sym_end,
      aux_sym_symbol_name_token1,
      anon_sym_DOT,
  [1477] = 1,
    ACTIONS(187), 3,
      ts_builtin_sym_end,
      aux_sym_symbol_name_token1,
      anon_sym_DOT,
  [1483] = 1,
    ACTIONS(104), 3,
      ts_builtin_sym_end,
      aux_sym_symbol_name_token1,
      anon_sym_DOT,
  [1489] = 1,
    ACTIONS(189), 3,
      ts_builtin_sym_end,
      aux_sym_symbol_name_token1,
      anon_sym_DOT,
  [1495] = 2,
    ACTIONS(191), 1,
      aux_sym_symbol_name_token1,
    STATE(11), 1,
      sym_field_name,
  [1502] = 2,
    ACTIONS(191), 1,
      aux_sym_symbol_name_token1,
    STATE(8), 1,
      sym_field_name,
  [1509] = 1,
    ACTIONS(193), 1,
      anon_sym_COMMA,
  [1513] = 1,
    ACTIONS(195), 1,
      anon_sym_COMMA,
  [1517] = 1,
    ACTIONS(197), 1,
      anon_sym_COMMA,
  [1521] = 1,
    ACTIONS(199), 1,
      anon_sym_COMMA,
  [1525] = 1,
    ACTIONS(201), 1,
      anon_sym_COMMA,
  [1529] = 1,
    ACTIONS(203), 1,
      anon_sym_COMMA,
  [1533] = 1,
    ACTIONS(205), 1,
      anon_sym_COMMA,
  [1537] = 1,
    ACTIONS(207), 1,
      anon_sym_COLON,
  [1541] = 1,
    ACTIONS(209), 1,
      anon_sym_COLON,
  [1545] = 1,
    ACTIONS(211), 1,
      anon_sym_COLON,
  [1549] = 1,
    ACTIONS(213), 1,
      anon_sym_COLON,
  [1553] = 1,
    ACTIONS(215), 1,
      ts_builtin_sym_end,
};

static uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 70,
  [SMALL_STATE(4)] = 140,
  [SMALL_STATE(5)] = 193,
  [SMALL_STATE(6)] = 252,
  [SMALL_STATE(7)] = 307,
  [SMALL_STATE(8)] = 360,
  [SMALL_STATE(9)] = 413,
  [SMALL_STATE(10)] = 466,
  [SMALL_STATE(11)] = 519,
  [SMALL_STATE(12)] = 572,
  [SMALL_STATE(13)] = 625,
  [SMALL_STATE(14)] = 678,
  [SMALL_STATE(15)] = 731,
  [SMALL_STATE(16)] = 784,
  [SMALL_STATE(17)] = 837,
  [SMALL_STATE(18)] = 890,
  [SMALL_STATE(19)] = 938,
  [SMALL_STATE(20)] = 984,
  [SMALL_STATE(21)] = 1030,
  [SMALL_STATE(22)] = 1065,
  [SMALL_STATE(23)] = 1100,
  [SMALL_STATE(24)] = 1128,
  [SMALL_STATE(25)] = 1156,
  [SMALL_STATE(26)] = 1182,
  [SMALL_STATE(27)] = 1210,
  [SMALL_STATE(28)] = 1230,
  [SMALL_STATE(29)] = 1250,
  [SMALL_STATE(30)] = 1270,
  [SMALL_STATE(31)] = 1288,
  [SMALL_STATE(32)] = 1306,
  [SMALL_STATE(33)] = 1326,
  [SMALL_STATE(34)] = 1346,
  [SMALL_STATE(35)] = 1364,
  [SMALL_STATE(36)] = 1384,
  [SMALL_STATE(37)] = 1404,
  [SMALL_STATE(38)] = 1424,
  [SMALL_STATE(39)] = 1436,
  [SMALL_STATE(40)] = 1448,
  [SMALL_STATE(41)] = 1461,
  [SMALL_STATE(42)] = 1471,
  [SMALL_STATE(43)] = 1477,
  [SMALL_STATE(44)] = 1483,
  [SMALL_STATE(45)] = 1489,
  [SMALL_STATE(46)] = 1495,
  [SMALL_STATE(47)] = 1502,
  [SMALL_STATE(48)] = 1509,
  [SMALL_STATE(49)] = 1513,
  [SMALL_STATE(50)] = 1517,
  [SMALL_STATE(51)] = 1521,
  [SMALL_STATE(52)] = 1525,
  [SMALL_STATE(53)] = 1529,
  [SMALL_STATE(54)] = 1533,
  [SMALL_STATE(55)] = 1537,
  [SMALL_STATE(56)] = 1541,
  [SMALL_STATE(57)] = 1545,
  [SMALL_STATE(58)] = 1549,
  [SMALL_STATE(59)] = 1553,
};

static TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [9] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selector, 1),
  [11] = {.entry = {.count = 1, .reusable = false}}, SHIFT(9),
  [13] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_selector, 1),
  [15] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_selector_repeat1, 2),
  [17] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_selector_repeat1, 2), SHIFT_REPEAT(9),
  [20] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_selector_repeat1, 2), SHIFT_REPEAT(46),
  [23] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_selector_repeat1, 2),
  [25] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_prev_child, 3),
  [27] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_sel_prev_child, 3),
  [29] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_selector_repeat1, 1),
  [31] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_selector_repeat1, 1),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [39] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_symbol, 1),
  [41] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_sel_symbol, 1),
  [43] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [45] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_symbol_field, 3),
  [47] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_sel_symbol_field, 3),
  [49] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_symbol_name, 1),
  [51] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_symbol_name, 1),
  [53] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field_name, 1),
  [55] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_field_name, 1),
  [57] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_field, 2),
  [59] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_sel_field, 2),
  [61] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_child, 3),
  [63] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_sel_child, 3),
  [65] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_next_child, 3),
  [67] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_sel_next_child, 3),
  [69] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [71] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [73] = {.entry = {.count = 1, .reusable = false}}, SHIFT(45),
  [75] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [77] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selectors, 1),
  [79] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_selectors, 1),
  [81] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_selectors_repeat1, 2), SHIFT_REPEAT(25),
  [84] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_selectors_repeat1, 2),
  [86] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_selectors_repeat1, 2),
  [88] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selectors, 2),
  [90] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_selectors, 2),
  [92] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [94] = {.entry = {.count = 1, .reusable = false}}, SHIFT(51),
  [96] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [98] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [100] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [102] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [104] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [106] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(9),
  [109] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(46),
  [112] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_properties_repeat1, 2),
  [114] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_properties_repeat1, 2), SHIFT_REPEAT(58),
  [117] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_properties_repeat1, 2), SHIFT_REPEAT(57),
  [120] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_properties_repeat1, 2), SHIFT_REPEAT(56),
  [123] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_properties_repeat1, 2), SHIFT_REPEAT(55),
  [126] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [128] = {.entry = {.count = 1, .reusable = true}}, SHIFT(58),
  [130] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [132] = {.entry = {.count = 1, .reusable = false}}, SHIFT(56),
  [134] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [136] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [138] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attr2, 3),
  [140] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [142] = {.entry = {.count = 1, .reusable = false}}, SHIFT(38),
  [144] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attr1, 3),
  [146] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attrs, 1),
  [148] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [150] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attrs, 1),
  [152] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_attrs_repeat1, 2),
  [154] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_attrs_repeat1, 2), SHIFT_REPEAT(40),
  [157] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_attrs_repeat1, 2),
  [159] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attrs, 2),
  [161] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attrs, 2),
  [163] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_attr1_repeat1, 2),
  [165] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_attr1_repeat1, 2), SHIFT_REPEAT(38),
  [168] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_attr1_repeat1, 2), SHIFT_REPEAT(38),
  [171] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attr1, 2),
  [173] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attr2, 2),
  [175] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attr, 1),
  [177] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attr, 1),
  [179] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attr_or, 2),
  [181] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attr_or, 2),
  [183] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_properties_repeat1, 2),
  [185] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_properties, 3, .production_id = 1),
  [187] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_properties, 2),
  [189] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_highlight, 1),
  [191] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [193] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property, 1),
  [195] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [197] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fg, 3),
  [199] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ansi_color, 1, .production_id = 2),
  [201] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ansi_color, 1, .production_id = 3),
  [203] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_color_name, 1),
  [205] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_bg, 3),
  [207] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [209] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [211] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [213] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [215] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
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
