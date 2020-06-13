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
#define STATE_COUNT 65
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 125
#define ALIAS_COUNT 0
#define TOKEN_COUNT 95
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 10
#define MAX_ALIAS_SEQUENCE_LENGTH 4

enum {
  anon_sym_COLON = 1,
  anon_sym_SEMI = 2,
  anon_sym_COMMA = 3,
  aux_sym_sel_kind_token1 = 4,
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
  sym_s = 95,
  sym_hl_rule = 96,
  sym_selectors = 97,
  sym_selector = 98,
  sym_sel_symbol = 99,
  sym_sel_kind = 100,
  sym_field_name = 101,
  sym_sel_field = 102,
  sym_sel_kind_field = 103,
  sym_sel_twins = 104,
  sym_sel_siblings = 105,
  sym_sel_child = 106,
  sym_properties = 107,
  sym_property = 108,
  sym_fg = 109,
  sym_bg = 110,
  sym_attrb = 111,
  sym_attribute = 112,
  sym_attrs = 113,
  sym_attr_or = 114,
  sym_attr = 115,
  sym_ansi_color = 116,
  sym_color_name = 117,
  sym_highlight = 118,
  aux_sym_s_repeat1 = 119,
  aux_sym_selectors_repeat1 = 120,
  aux_sym_selector_repeat1 = 121,
  aux_sym_properties_repeat1 = 122,
  aux_sym_attrb_repeat1 = 123,
  aux_sym_attrs_repeat1 = 124,
};

static const char *ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_COLON] = ":",
  [anon_sym_SEMI] = ";",
  [anon_sym_COMMA] = ",",
  [aux_sym_sel_kind_token1] = "sel_kind_token1",
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
  [sym_s] = "s",
  [sym_hl_rule] = "hl_rule",
  [sym_selectors] = "selectors",
  [sym_selector] = "selector",
  [sym_sel_symbol] = "sel_symbol",
  [sym_sel_kind] = "sel_kind",
  [sym_field_name] = "field_name",
  [sym_sel_field] = "sel_field",
  [sym_sel_kind_field] = "sel_kind_field",
  [sym_sel_twins] = "sel_twins",
  [sym_sel_siblings] = "sel_siblings",
  [sym_sel_child] = "sel_child",
  [sym_properties] = "properties",
  [sym_property] = "property",
  [sym_fg] = "fg",
  [sym_bg] = "bg",
  [sym_attrb] = "attrb",
  [sym_attribute] = "attribute",
  [sym_attrs] = "attrs",
  [sym_attr_or] = "attr_or",
  [sym_attr] = "attr",
  [sym_ansi_color] = "ansi_color",
  [sym_color_name] = "color_name",
  [sym_highlight] = "highlight",
  [aux_sym_s_repeat1] = "s_repeat1",
  [aux_sym_selectors_repeat1] = "selectors_repeat1",
  [aux_sym_selector_repeat1] = "selector_repeat1",
  [aux_sym_properties_repeat1] = "properties_repeat1",
  [aux_sym_attrb_repeat1] = "attrb_repeat1",
  [aux_sym_attrs_repeat1] = "attrs_repeat1",
};

static TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [aux_sym_sel_kind_token1] = aux_sym_sel_kind_token1,
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
  [sym_s] = sym_s,
  [sym_hl_rule] = sym_hl_rule,
  [sym_selectors] = sym_selectors,
  [sym_selector] = sym_selector,
  [sym_sel_symbol] = sym_sel_symbol,
  [sym_sel_kind] = sym_sel_kind,
  [sym_field_name] = sym_field_name,
  [sym_sel_field] = sym_sel_field,
  [sym_sel_kind_field] = sym_sel_kind_field,
  [sym_sel_twins] = sym_sel_twins,
  [sym_sel_siblings] = sym_sel_siblings,
  [sym_sel_child] = sym_sel_child,
  [sym_properties] = sym_properties,
  [sym_property] = sym_property,
  [sym_fg] = sym_fg,
  [sym_bg] = sym_bg,
  [sym_attrb] = sym_attrb,
  [sym_attribute] = sym_attribute,
  [sym_attrs] = sym_attrs,
  [sym_attr_or] = sym_attr_or,
  [sym_attr] = sym_attr,
  [sym_ansi_color] = sym_ansi_color,
  [sym_color_name] = sym_color_name,
  [sym_highlight] = sym_highlight,
  [aux_sym_s_repeat1] = aux_sym_s_repeat1,
  [aux_sym_selectors_repeat1] = aux_sym_selectors_repeat1,
  [aux_sym_selector_repeat1] = aux_sym_selector_repeat1,
  [aux_sym_properties_repeat1] = aux_sym_properties_repeat1,
  [aux_sym_attrb_repeat1] = aux_sym_attrb_repeat1,
  [aux_sym_attrs_repeat1] = aux_sym_attrs_repeat1,
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
  [anon_sym_SEMI] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_sel_kind_token1] = {
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
  [sym_s] = {
    .visible = true,
    .named = true,
  },
  [sym_hl_rule] = {
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
  [sym_sel_kind] = {
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
  [sym_sel_kind_field] = {
    .visible = true,
    .named = true,
  },
  [sym_sel_twins] = {
    .visible = true,
    .named = true,
  },
  [sym_sel_siblings] = {
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
  [sym_attrb] = {
    .visible = true,
    .named = true,
  },
  [sym_attribute] = {
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
  [aux_sym_s_repeat1] = {
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
  [aux_sym_attrb_repeat1] = {
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
  field_attr = 3,
  field_attribute = 4,
  field_bg = 5,
  field_fg = 6,
  field_properties = 7,
  field_property = 8,
  field_selectors = 9,
  field_style = 10,
};

static const char *ts_field_names[] = {
  [0] = NULL,
  [field_ansi_color_dec] = "ansi_color_dec",
  [field_ansi_color_hex] = "ansi_color_hex",
  [field_attr] = "attr",
  [field_attribute] = "attribute",
  [field_bg] = "bg",
  [field_fg] = "fg",
  [field_properties] = "properties",
  [field_property] = "property",
  [field_selectors] = "selectors",
  [field_style] = "style",
};

static const TSFieldMapSlice ts_field_map_slices[11] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 1},
  [3] = {.index = 2, .length = 1},
  [4] = {.index = 3, .length = 1},
  [5] = {.index = 4, .length = 2},
  [6] = {.index = 6, .length = 1},
  [7] = {.index = 7, .length = 2},
  [8] = {.index = 9, .length = 2},
  [9] = {.index = 11, .length = 1},
  [10] = {.index = 12, .length = 1},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_fg, 0},
  [1] =
    {field_bg, 0},
  [2] =
    {field_attr, 0},
  [3] =
    {field_attribute, 0},
  [4] =
    {field_selectors, 0},
    {field_style, 2},
  [6] =
    {field_property, 0},
  [7] =
    {field_properties, 1},
    {field_property, 1, .inherited = true},
  [9] =
    {field_property, 0, .inherited = true},
    {field_property, 1, .inherited = true},
  [11] =
    {field_ansi_color_dec, 0},
  [12] =
    {field_ansi_color_hex, 0},
};

static TSSymbol ts_alias_sequences[11][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(332);
      if (lookahead == '#') ADVANCE(329);
      if (lookahead == '+') ADVANCE(338);
      if (lookahead == ',') ADVANCE(335);
      if (lookahead == '.') ADVANCE(337);
      if (lookahead == '0') ADVANCE(354);
      if (lookahead == ':') ADVANCE(333);
      if (lookahead == ';') ADVANCE(334);
      if (lookahead == '>') ADVANCE(340);
      if (lookahead == 'a') ADVANCE(288);
      if (lookahead == 'b') ADVANCE(129);
      if (lookahead == 'c') ADVANCE(14);
      if (lookahead == 'd') ADVANCE(17);
      if (lookahead == 'e') ADVANCE(260);
      if (lookahead == 'f') ADVANCE(130);
      if (lookahead == 'g') ADVANCE(261);
      if (lookahead == 'i') ADVANCE(64);
      if (lookahead == 'k') ADVANCE(69);
      if (lookahead == 'l') ADVANCE(6);
      if (lookahead == 'm') ADVANCE(7);
      if (lookahead == 'n') ADVANCE(305);
      if (lookahead == 'o') ADVANCE(241);
      if (lookahead == 'p') ADVANCE(247);
      if (lookahead == 'r') ADVANCE(70);
      if (lookahead == 's') ADVANCE(242);
      if (lookahead == 't') ADVANCE(8);
      if (lookahead == 'u') ADVANCE(203);
      if (lookahead == 'w') ADVANCE(137);
      if (lookahead == 'y') ADVANCE(71);
      if (lookahead == '{') ADVANCE(341);
      if (lookahead == '|') ADVANCE(347);
      if (lookahead == '}') ADVANCE(342);
      if (lookahead == '~') ADVANCE(339);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(355);
      END_STATE();
    case 1:
      if (lookahead == '-') ADVANCE(179);
      END_STATE();
    case 2:
      if (lookahead == '-') ADVANCE(41);
      if (lookahead == '_') ADVANCE(42);
      if (lookahead == 'b') ADVANCE(165);
      if (lookahead == 'c') ADVANCE(320);
      if (lookahead == 'g') ADVANCE(268);
      if (lookahead == 'm') ADVANCE(33);
      if (lookahead == 'r') ADVANCE(94);
      if (lookahead == 'y') ADVANCE(123);
      END_STATE();
    case 3:
      if (lookahead == '-') ADVANCE(213);
      END_STATE();
    case 4:
      if (lookahead == '-') ADVANCE(54);
      END_STATE();
    case 5:
      if (lookahead == '-') ADVANCE(181);
      END_STATE();
    case 6:
      if (lookahead == 'a') ADVANCE(40);
      if (lookahead == 'i') ADVANCE(210);
      END_STATE();
    case 7:
      if (lookahead == 'a') ADVANCE(52);
      END_STATE();
    case 8:
      if (lookahead == 'a') ADVANCE(37);
      if (lookahead == 'o') ADVANCE(63);
      if (lookahead == 'y') ADVANCE(244);
      END_STATE();
    case 9:
      if (lookahead == 'a') ADVANCE(273);
      END_STATE();
    case 10:
      if (lookahead == 'a') ADVANCE(377);
      END_STATE();
    case 11:
      if (lookahead == 'a') ADVANCE(378);
      END_STATE();
    case 12:
      if (lookahead == 'a') ADVANCE(379);
      END_STATE();
    case 13:
      if (lookahead == 'a') ADVANCE(380);
      END_STATE();
    case 14:
      if (lookahead == 'a') ADVANCE(190);
      if (lookahead == 'h') ADVANCE(22);
      if (lookahead == 'o') ADVANCE(182);
      if (lookahead == 'y') ADVANCE(23);
      END_STATE();
    case 15:
      if (lookahead == 'a') ADVANCE(133);
      END_STATE();
    case 16:
      if (lookahead == 'a') ADVANCE(278);
      if (lookahead == 'o') ADVANCE(264);
      if (lookahead == 'r') ADVANCE(143);
      END_STATE();
    case 17:
      if (lookahead == 'a') ADVANCE(248);
      if (lookahead == 'e') ADVANCE(38);
      END_STATE();
    case 18:
      if (lookahead == 'a') ADVANCE(47);
      if (lookahead == 'u') ADVANCE(74);
      END_STATE();
    case 19:
      if (lookahead == 'a') ADVANCE(164);
      END_STATE();
    case 20:
      if (lookahead == 'a') ADVANCE(276);
      END_STATE();
    case 21:
      if (lookahead == 'a') ADVANCE(279);
      END_STATE();
    case 22:
      if (lookahead == 'a') ADVANCE(250);
      END_STATE();
    case 23:
      if (lookahead == 'a') ADVANCE(191);
      END_STATE();
    case 24:
      if (lookahead == 'a') ADVANCE(160);
      END_STATE();
    case 25:
      if (lookahead == 'a') ADVANCE(299);
      END_STATE();
    case 26:
      if (lookahead == 'a') ADVANCE(193);
      END_STATE();
    case 27:
      if (lookahead == 'a') ADVANCE(161);
      END_STATE();
    case 28:
      if (lookahead == 'a') ADVANCE(281);
      END_STATE();
    case 29:
      if (lookahead == 'a') ADVANCE(194);
      END_STATE();
    case 30:
      if (lookahead == 'a') ADVANCE(196);
      END_STATE();
    case 31:
      if (lookahead == 'a') ADVANCE(197);
      END_STATE();
    case 32:
      if (lookahead == 'a') ADVANCE(257);
      END_STATE();
    case 33:
      if (lookahead == 'a') ADVANCE(134);
      END_STATE();
    case 34:
      if (lookahead == 'a') ADVANCE(212);
      END_STATE();
    case 35:
      if (lookahead == 'a') ADVANCE(135);
      END_STATE();
    case 36:
      if (lookahead == 'a') ADVANCE(136);
      END_STATE();
    case 37:
      if (lookahead == 'b') ADVANCE(1);
      if (lookahead == 'g') ADVANCE(415);
      END_STATE();
    case 38:
      if (lookahead == 'b') ADVANCE(306);
      if (lookahead == 'f') ADVANCE(148);
      if (lookahead == 'l') ADVANCE(141);
      END_STATE();
    case 39:
      if (lookahead == 'b') ADVANCE(311);
      END_STATE();
    case 40:
      if (lookahead == 'b') ADVANCE(98);
      END_STATE();
    case 41:
      if (lookahead == 'b') ADVANCE(174);
      if (lookahead == 'c') ADVANCE(321);
      if (lookahead == 'g') ADVANCE(269);
      if (lookahead == 'm') ADVANCE(35);
      if (lookahead == 'r') ADVANCE(100);
      if (lookahead == 'y') ADVANCE(124);
      END_STATE();
    case 42:
      if (lookahead == 'b') ADVANCE(176);
      if (lookahead == 'c') ADVANCE(322);
      if (lookahead == 'g') ADVANCE(270);
      if (lookahead == 'm') ADVANCE(36);
      if (lookahead == 'r') ADVANCE(103);
      if (lookahead == 'y') ADVANCE(125);
      END_STATE();
    case 43:
      if (lookahead == 'b') ADVANCE(106);
      END_STATE();
    case 44:
      if (lookahead == 'c') ADVANCE(349);
      END_STATE();
    case 45:
      if (lookahead == 'c') ADVANCE(404);
      END_STATE();
    case 46:
      if (lookahead == 'c') ADVANCE(138);
      END_STATE();
    case 47:
      if (lookahead == 'c') ADVANCE(158);
      END_STATE();
    case 48:
      if (lookahead == 'c') ADVANCE(162);
      END_STATE();
    case 49:
      if (lookahead == 'c') ADVANCE(147);
      END_STATE();
    case 50:
      if (lookahead == 'c') ADVANCE(240);
      if (lookahead == 'p') ADVANCE(265);
      END_STATE();
    case 51:
      if (lookahead == 'c') ADVANCE(291);
      END_STATE();
    case 52:
      if (lookahead == 'c') ADVANCE(262);
      if (lookahead == 'g') ADVANCE(114);
      END_STATE();
    case 53:
      if (lookahead == 'c') ADVANCE(89);
      END_STATE();
    case 54:
      if (lookahead == 'c') ADVANCE(168);
      END_STATE();
    case 55:
      if (lookahead == 'c') ADVANCE(290);
      END_STATE();
    case 56:
      if (lookahead == 'c') ADVANCE(286);
      END_STATE();
    case 57:
      if (lookahead == 'd') ADVANCE(361);
      if (lookahead == 'p') ADVANCE(116);
      if (lookahead == 'v') ADVANCE(104);
      END_STATE();
    case 58:
      if (lookahead == 'd') ADVANCE(348);
      END_STATE();
    case 59:
      if (lookahead == 'd') ADVANCE(362);
      END_STATE();
    case 60:
      if (lookahead == 'd') ADVANCE(402);
      END_STATE();
    case 61:
      if (lookahead == 'd') ADVANCE(363);
      END_STATE();
    case 62:
      if (lookahead == 'd') ADVANCE(364);
      END_STATE();
    case 63:
      if (lookahead == 'd') ADVANCE(224);
      END_STATE();
    case 64:
      if (lookahead == 'd') ADVANCE(99);
      if (lookahead == 'g') ADVANCE(204);
      if (lookahead == 'n') ADVANCE(48);
      if (lookahead == 't') ADVANCE(19);
      END_STATE();
    case 65:
      if (lookahead == 'd') ADVANCE(79);
      END_STATE();
    case 66:
      if (lookahead == 'd') ADVANCE(156);
      if (lookahead == 's') ADVANCE(300);
      END_STATE();
    case 67:
      if (lookahead == 'd') ADVANCE(145);
      END_STATE();
    case 68:
      if (lookahead == 'd') ADVANCE(105);
      END_STATE();
    case 69:
      if (lookahead == 'e') ADVANCE(319);
      END_STATE();
    case 70:
      if (lookahead == 'e') ADVANCE(57);
      END_STATE();
    case 71:
      if (lookahead == 'e') ADVANCE(166);
      END_STATE();
    case 72:
      if (lookahead == 'e') ADVANCE(102);
      END_STATE();
    case 73:
      if (lookahead == 'e') ADVANCE(50);
      if (lookahead == 'o') ADVANCE(184);
      END_STATE();
    case 74:
      if (lookahead == 'e') ADVANCE(373);
      END_STATE();
    case 75:
      if (lookahead == 'e') ADVANCE(409);
      END_STATE();
    case 76:
      if (lookahead == 'e') ADVANCE(385);
      END_STATE();
    case 77:
      if (lookahead == 'e') ADVANCE(406);
      END_STATE();
    case 78:
      if (lookahead == 'e') ADVANCE(419);
      END_STATE();
    case 79:
      if (lookahead == 'e') ADVANCE(405);
      END_STATE();
    case 80:
      if (lookahead == 'e') ADVANCE(352);
      END_STATE();
    case 81:
      if (lookahead == 'e') ADVANCE(374);
      END_STATE();
    case 82:
      if (lookahead == 'e') ADVANCE(425);
      END_STATE();
    case 83:
      if (lookahead == 'e') ADVANCE(346);
      END_STATE();
    case 84:
      if (lookahead == 'e') ADVANCE(375);
      END_STATE();
    case 85:
      if (lookahead == 'e') ADVANCE(376);
      END_STATE();
    case 86:
      if (lookahead == 'e') ADVANCE(411);
      END_STATE();
    case 87:
      if (lookahead == 'e') ADVANCE(351);
      END_STATE();
    case 88:
      if (lookahead == 'e') ADVANCE(424);
      END_STATE();
    case 89:
      if (lookahead == 'e') ADVANCE(245);
      END_STATE();
    case 90:
      if (lookahead == 'e') ADVANCE(107);
      END_STATE();
    case 91:
      if (lookahead == 'e') ADVANCE(3);
      END_STATE();
    case 92:
      if (lookahead == 'e') ADVANCE(127);
      END_STATE();
    case 93:
      if (lookahead == 'e') ADVANCE(110);
      END_STATE();
    case 94:
      if (lookahead == 'e') ADVANCE(59);
      END_STATE();
    case 95:
      if (lookahead == 'e') ADVANCE(111);
      END_STATE();
    case 96:
      if (lookahead == 'e') ADVANCE(4);
      END_STATE();
    case 97:
      if (lookahead == 'e') ADVANCE(49);
      END_STATE();
    case 98:
      if (lookahead == 'e') ADVANCE(159);
      END_STATE();
    case 99:
      if (lookahead == 'e') ADVANCE(208);
      END_STATE();
    case 100:
      if (lookahead == 'e') ADVANCE(61);
      END_STATE();
    case 101:
      if (lookahead == 'e') ADVANCE(272);
      END_STATE();
    case 102:
      if (lookahead == 'e') ADVANCE(192);
      if (lookahead == 'y') ADVANCE(386);
      END_STATE();
    case 103:
      if (lookahead == 'e') ADVANCE(62);
      END_STATE();
    case 104:
      if (lookahead == 'e') ADVANCE(259);
      END_STATE();
    case 105:
      if (lookahead == 'e') ADVANCE(266);
      END_STATE();
    case 106:
      if (lookahead == 'e') ADVANCE(252);
      END_STATE();
    case 107:
      if (lookahead == 'e') ADVANCE(198);
      if (lookahead == 'y') ADVANCE(358);
      END_STATE();
    case 108:
      if (lookahead == 'e') ADVANCE(255);
      END_STATE();
    case 109:
      if (lookahead == 'e') ADVANCE(256);
      END_STATE();
    case 110:
      if (lookahead == 'e') ADVANCE(200);
      if (lookahead == 'y') ADVANCE(359);
      END_STATE();
    case 111:
      if (lookahead == 'e') ADVANCE(201);
      if (lookahead == 'y') ADVANCE(360);
      END_STATE();
    case 112:
      if (lookahead == 'e') ADVANCE(173);
      END_STATE();
    case 113:
      if (lookahead == 'e') ADVANCE(26);
      END_STATE();
    case 114:
      if (lookahead == 'e') ADVANCE(209);
      END_STATE();
    case 115:
      if (lookahead == 'e') ADVANCE(56);
      END_STATE();
    case 116:
      if (lookahead == 'e') ADVANCE(28);
      END_STATE();
    case 117:
      if (lookahead == 'e') ADVANCE(211);
      END_STATE();
    case 118:
      if (lookahead == 'e') ADVANCE(214);
      END_STATE();
    case 119:
      if (lookahead == 'e') ADVANCE(216);
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
      if (lookahead == 'e') ADVANCE(175);
      END_STATE();
    case 124:
      if (lookahead == 'e') ADVANCE(177);
      END_STATE();
    case 125:
      if (lookahead == 'e') ADVANCE(178);
      END_STATE();
    case 126:
      if (lookahead == 'e') ADVANCE(187);
      if (lookahead == 'u') ADVANCE(275);
      END_STATE();
    case 127:
      if (lookahead == 'f') ADVANCE(412);
      END_STATE();
    case 128:
      if (lookahead == 'f') ADVANCE(155);
      END_STATE();
    case 129:
      if (lookahead == 'g') ADVANCE(344);
      if (lookahead == 'l') ADVANCE(18);
      if (lookahead == 'o') ADVANCE(163);
      END_STATE();
    case 130:
      if (lookahead == 'g') ADVANCE(343);
      if (lookahead == 'l') ADVANCE(229);
      if (lookahead == 'u') ADVANCE(206);
      END_STATE();
    case 131:
      if (lookahead == 'g') ADVANCE(418);
      END_STATE();
    case 132:
      if (lookahead == 'g') ADVANCE(390);
      END_STATE();
    case 133:
      if (lookahead == 'g') ADVANCE(96);
      END_STATE();
    case 134:
      if (lookahead == 'g') ADVANCE(119);
      END_STATE();
    case 135:
      if (lookahead == 'g') ADVANCE(120);
      END_STATE();
    case 136:
      if (lookahead == 'g') ADVANCE(121);
      END_STATE();
    case 137:
      if (lookahead == 'h') ADVANCE(140);
      END_STATE();
    case 138:
      if (lookahead == 'h') ADVANCE(32);
      if (lookahead == 'o') ADVANCE(189);
      END_STATE();
    case 139:
      if (lookahead == 'i') ADVANCE(128);
      END_STATE();
    case 140:
      if (lookahead == 'i') ADVANCE(297);
      END_STATE();
    case 141:
      if (lookahead == 'i') ADVANCE(185);
      END_STATE();
    case 142:
      if (lookahead == 'i') ADVANCE(44);
      END_STATE();
    case 143:
      if (lookahead == 'i') ADVANCE(205);
      if (lookahead == 'u') ADVANCE(55);
      END_STATE();
    case 144:
      if (lookahead == 'i') ADVANCE(233);
      END_STATE();
    case 145:
      if (lookahead == 'i') ADVANCE(284);
      END_STATE();
    case 146:
      if (lookahead == 'i') ADVANCE(301);
      END_STATE();
    case 147:
      if (lookahead == 'i') ADVANCE(24);
      END_STATE();
    case 148:
      if (lookahead == 'i') ADVANCE(215);
      END_STATE();
    case 149:
      if (lookahead == 'i') ADVANCE(235);
      END_STATE();
    case 150:
      if (lookahead == 'i') ADVANCE(220);
      END_STATE();
    case 151:
      if (lookahead == 'i') ADVANCE(237);
      END_STATE();
    case 152:
      if (lookahead == 'i') ADVANCE(221);
      END_STATE();
    case 153:
      if (lookahead == 'i') ADVANCE(238);
      END_STATE();
    case 154:
      if (lookahead == 'i') ADVANCE(222);
      END_STATE();
    case 155:
      if (lookahead == 'i') ADVANCE(109);
      END_STATE();
    case 156:
      if (lookahead == 'i') ADVANCE(302);
      END_STATE();
    case 157:
      if (lookahead == 'k') ADVANCE(2);
      END_STATE();
    case 158:
      if (lookahead == 'k') ADVANCE(357);
      END_STATE();
    case 159:
      if (lookahead == 'l') ADVANCE(400);
      END_STATE();
    case 160:
      if (lookahead == 'l') ADVANCE(413);
      END_STATE();
    case 161:
      if (lookahead == 'l') ADVANCE(398);
      END_STATE();
    case 162:
      if (lookahead == 'l') ADVANCE(310);
      END_STATE();
    case 163:
      if (lookahead == 'l') ADVANCE(58);
      if (lookahead == 'o') ADVANCE(172);
      END_STATE();
    case 164:
      if (lookahead == 'l') ADVANCE(142);
      END_STATE();
    case 165:
      if (lookahead == 'l') ADVANCE(307);
      END_STATE();
    case 166:
      if (lookahead == 'l') ADVANCE(167);
      END_STATE();
    case 167:
      if (lookahead == 'l') ADVANCE(226);
      END_STATE();
    case 168:
      if (lookahead == 'l') ADVANCE(20);
      END_STATE();
    case 169:
      if (lookahead == 'l') ADVANCE(227);
      END_STATE();
    case 170:
      if (lookahead == 'l') ADVANCE(228);
      END_STATE();
    case 171:
      if (lookahead == 'l') ADVANCE(230);
      END_STATE();
    case 172:
      if (lookahead == 'l') ADVANCE(113);
      END_STATE();
    case 173:
      if (lookahead == 'l') ADVANCE(115);
      END_STATE();
    case 174:
      if (lookahead == 'l') ADVANCE(308);
      END_STATE();
    case 175:
      if (lookahead == 'l') ADVANCE(169);
      END_STATE();
    case 176:
      if (lookahead == 'l') ADVANCE(309);
      END_STATE();
    case 177:
      if (lookahead == 'l') ADVANCE(170);
      END_STATE();
    case 178:
      if (lookahead == 'l') ADVANCE(171);
      END_STATE();
    case 179:
      if (lookahead == 'l') ADVANCE(150);
      if (lookahead == 'o') ADVANCE(246);
      if (lookahead == 's') ADVANCE(112);
      END_STATE();
    case 180:
      if (lookahead == 'l') ADVANCE(152);
      END_STATE();
    case 181:
      if (lookahead == 'l') ADVANCE(154);
      END_STATE();
    case 182:
      if (lookahead == 'm') ADVANCE(186);
      if (lookahead == 'n') ADVANCE(66);
      END_STATE();
    case 183:
      if (lookahead == 'm') ADVANCE(43);
      END_STATE();
    case 184:
      if (lookahead == 'm') ADVANCE(243);
      END_STATE();
    case 185:
      if (lookahead == 'm') ADVANCE(146);
      END_STATE();
    case 186:
      if (lookahead == 'm') ADVANCE(117);
      END_STATE();
    case 187:
      if (lookahead == 'm') ADVANCE(118);
      END_STATE();
    case 188:
      if (lookahead == 'm') ADVANCE(122);
      END_STATE();
    case 189:
      if (lookahead == 'm') ADVANCE(188);
      END_STATE();
    case 190:
      if (lookahead == 'n') ADVANCE(313);
      END_STATE();
    case 191:
      if (lookahead == 'n') ADVANCE(381);
      END_STATE();
    case 192:
      if (lookahead == 'n') ADVANCE(365);
      END_STATE();
    case 193:
      if (lookahead == 'n') ADVANCE(393);
      END_STATE();
    case 194:
      if (lookahead == 'n') ADVANCE(382);
      END_STATE();
    case 195:
      if (lookahead == 'n') ADVANCE(396);
      END_STATE();
    case 196:
      if (lookahead == 'n') ADVANCE(383);
      END_STATE();
    case 197:
      if (lookahead == 'n') ADVANCE(384);
      END_STATE();
    case 198:
      if (lookahead == 'n') ADVANCE(366);
      END_STATE();
    case 199:
      if (lookahead == 'n') ADVANCE(403);
      END_STATE();
    case 200:
      if (lookahead == 'n') ADVANCE(367);
      END_STATE();
    case 201:
      if (lookahead == 'n') ADVANCE(368);
      END_STATE();
    case 202:
      if (lookahead == 'n') ADVANCE(426);
      END_STATE();
    case 203:
      if (lookahead == 'n') ADVANCE(68);
      END_STATE();
    case 204:
      if (lookahead == 'n') ADVANCE(239);
      END_STATE();
    case 205:
      if (lookahead == 'n') ADVANCE(132);
      END_STATE();
    case 206:
      if (lookahead == 'n') ADVANCE(51);
      END_STATE();
    case 207:
      if (lookahead == 'n') ADVANCE(67);
      END_STATE();
    case 208:
      if (lookahead == 'n') ADVANCE(292);
      END_STATE();
    case 209:
      if (lookahead == 'n') ADVANCE(293);
      END_STATE();
    case 210:
      if (lookahead == 'n') ADVANCE(91);
      END_STATE();
    case 211:
      if (lookahead == 'n') ADVANCE(282);
      END_STATE();
    case 212:
      if (lookahead == 'n') ADVANCE(283);
      END_STATE();
    case 213:
      if (lookahead == 'n') ADVANCE(253);
      END_STATE();
    case 214:
      if (lookahead == 'n') ADVANCE(285);
      END_STATE();
    case 215:
      if (lookahead == 'n') ADVANCE(77);
      END_STATE();
    case 216:
      if (lookahead == 'n') ADVANCE(294);
      END_STATE();
    case 217:
      if (lookahead == 'n') ADVANCE(295);
      END_STATE();
    case 218:
      if (lookahead == 'n') ADVANCE(296);
      END_STATE();
    case 219:
      if (lookahead == 'n') ADVANCE(287);
      END_STATE();
    case 220:
      if (lookahead == 'n') ADVANCE(82);
      END_STATE();
    case 221:
      if (lookahead == 'n') ADVANCE(87);
      END_STATE();
    case 222:
      if (lookahead == 'n') ADVANCE(88);
      END_STATE();
    case 223:
      if (lookahead == 'n') ADVANCE(27);
      END_STATE();
    case 224:
      if (lookahead == 'o') ADVANCE(421);
      END_STATE();
    case 225:
      if (lookahead == 'o') ADVANCE(407);
      END_STATE();
    case 226:
      if (lookahead == 'o') ADVANCE(314);
      END_STATE();
    case 227:
      if (lookahead == 'o') ADVANCE(315);
      END_STATE();
    case 228:
      if (lookahead == 'o') ADVANCE(316);
      END_STATE();
    case 229:
      if (lookahead == 'o') ADVANCE(21);
      END_STATE();
    case 230:
      if (lookahead == 'o') ADVANCE(317);
      END_STATE();
    case 231:
      if (lookahead == 'o') ADVANCE(45);
      END_STATE();
    case 232:
      if (lookahead == 'o') ADVANCE(251);
      END_STATE();
    case 233:
      if (lookahead == 'o') ADVANCE(195);
      END_STATE();
    case 234:
      if (lookahead == 'o') ADVANCE(263);
      END_STATE();
    case 235:
      if (lookahead == 'o') ADVANCE(223);
      END_STATE();
    case 236:
      if (lookahead == 'o') ADVANCE(254);
      END_STATE();
    case 237:
      if (lookahead == 'o') ADVANCE(199);
      END_STATE();
    case 238:
      if (lookahead == 'o') ADVANCE(202);
      END_STATE();
    case 239:
      if (lookahead == 'o') ADVANCE(267);
      END_STATE();
    case 240:
      if (lookahead == 'o') ADVANCE(207);
      END_STATE();
    case 241:
      if (lookahead == 'p') ADVANCE(101);
      END_STATE();
    case 242:
      if (lookahead == 'p') ADVANCE(97);
      if (lookahead == 't') ADVANCE(16);
      END_STATE();
    case 243:
      if (lookahead == 'p') ADVANCE(280);
      END_STATE();
    case 244:
      if (lookahead == 'p') ADVANCE(75);
      END_STATE();
    case 245:
      if (lookahead == 'p') ADVANCE(303);
      END_STATE();
    case 246:
      if (lookahead == 'p') ADVANCE(304);
      END_STATE();
    case 247:
      if (lookahead == 'r') ADVANCE(73);
      END_STATE();
    case 248:
      if (lookahead == 'r') ADVANCE(157);
      END_STATE();
    case 249:
      if (lookahead == 'r') ADVANCE(345);
      END_STATE();
    case 250:
      if (lookahead == 'r') ADVANCE(391);
      END_STATE();
    case 251:
      if (lookahead == 'r') ADVANCE(420);
      END_STATE();
    case 252:
      if (lookahead == 'r') ADVANCE(392);
      END_STATE();
    case 253:
      if (lookahead == 'r') ADVANCE(422);
      END_STATE();
    case 254:
      if (lookahead == 'r') ADVANCE(401);
      END_STATE();
    case 255:
      if (lookahead == 'r') ADVANCE(416);
      END_STATE();
    case 256:
      if (lookahead == 'r') ADVANCE(395);
      END_STATE();
    case 257:
      if (lookahead == 'r') ADVANCE(414);
      END_STATE();
    case 258:
      if (lookahead == 'r') ADVANCE(232);
      END_STATE();
    case 259:
      if (lookahead == 'r') ADVANCE(277);
      END_STATE();
    case 260:
      if (lookahead == 'r') ADVANCE(258);
      if (lookahead == 'x') ADVANCE(53);
      END_STATE();
    case 261:
      if (lookahead == 'r') ADVANCE(72);
      END_STATE();
    case 262:
      if (lookahead == 'r') ADVANCE(225);
      END_STATE();
    case 263:
      if (lookahead == 'r') ADVANCE(60);
      END_STATE();
    case 264:
      if (lookahead == 'r') ADVANCE(15);
      END_STATE();
    case 265:
      if (lookahead == 'r') ADVANCE(231);
      END_STATE();
    case 266:
      if (lookahead == 'r') ADVANCE(180);
      END_STATE();
    case 267:
      if (lookahead == 'r') ADVANCE(78);
      END_STATE();
    case 268:
      if (lookahead == 'r') ADVANCE(90);
      END_STATE();
    case 269:
      if (lookahead == 'r') ADVANCE(93);
      END_STATE();
    case 270:
      if (lookahead == 'r') ADVANCE(95);
      END_STATE();
    case 271:
      if (lookahead == 'r') ADVANCE(86);
      END_STATE();
    case 272:
      if (lookahead == 'r') ADVANCE(25);
      END_STATE();
    case 273:
      if (lookahead == 's') ADVANCE(387);
      END_STATE();
    case 274:
      if (lookahead == 's') ADVANCE(410);
      END_STATE();
    case 275:
      if (lookahead == 's') ADVANCE(5);
      END_STATE();
    case 276:
      if (lookahead == 's') ADVANCE(274);
      END_STATE();
    case 277:
      if (lookahead == 's') ADVANCE(80);
      END_STATE();
    case 278:
      if (lookahead == 't') ADVANCE(126);
      END_STATE();
    case 279:
      if (lookahead == 't') ADVANCE(394);
      END_STATE();
    case 280:
      if (lookahead == 't') ADVANCE(423);
      END_STATE();
    case 281:
      if (lookahead == 't') ADVANCE(399);
      END_STATE();
    case 282:
      if (lookahead == 't') ADVANCE(388);
      END_STATE();
    case 283:
      if (lookahead == 't') ADVANCE(389);
      END_STATE();
    case 284:
      if (lookahead == 't') ADVANCE(408);
      END_STATE();
    case 285:
      if (lookahead == 't') ADVANCE(397);
      END_STATE();
    case 286:
      if (lookahead == 't') ADVANCE(427);
      END_STATE();
    case 287:
      if (lookahead == 't') ADVANCE(417);
      END_STATE();
    case 288:
      if (lookahead == 't') ADVANCE(289);
      END_STATE();
    case 289:
      if (lookahead == 't') ADVANCE(249);
      END_STATE();
    case 290:
      if (lookahead == 't') ADVANCE(312);
      END_STATE();
    case 291:
      if (lookahead == 't') ADVANCE(144);
      END_STATE();
    case 292:
      if (lookahead == 't') ADVANCE(139);
      END_STATE();
    case 293:
      if (lookahead == 't') ADVANCE(10);
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
      if (lookahead == 't') ADVANCE(76);
      END_STATE();
    case 298:
      if (lookahead == 't') ADVANCE(83);
      END_STATE();
    case 299:
      if (lookahead == 't') ADVANCE(236);
      END_STATE();
    case 300:
      if (lookahead == 't') ADVANCE(34);
      END_STATE();
    case 301:
      if (lookahead == 't') ADVANCE(108);
      END_STATE();
    case 302:
      if (lookahead == 't') ADVANCE(149);
      END_STATE();
    case 303:
      if (lookahead == 't') ADVANCE(151);
      END_STATE();
    case 304:
      if (lookahead == 't') ADVANCE(153);
      END_STATE();
    case 305:
      if (lookahead == 'u') ADVANCE(183);
      END_STATE();
    case 306:
      if (lookahead == 'u') ADVANCE(131);
      END_STATE();
    case 307:
      if (lookahead == 'u') ADVANCE(81);
      END_STATE();
    case 308:
      if (lookahead == 'u') ADVANCE(84);
      END_STATE();
    case 309:
      if (lookahead == 'u') ADVANCE(85);
      END_STATE();
    case 310:
      if (lookahead == 'u') ADVANCE(65);
      END_STATE();
    case 311:
      if (lookahead == 'u') ADVANCE(298);
      END_STATE();
    case 312:
      if (lookahead == 'u') ADVANCE(271);
      END_STATE();
    case 313:
      if (lookahead == 'v') ADVANCE(9);
      END_STATE();
    case 314:
      if (lookahead == 'w') ADVANCE(369);
      END_STATE();
    case 315:
      if (lookahead == 'w') ADVANCE(370);
      END_STATE();
    case 316:
      if (lookahead == 'w') ADVANCE(371);
      END_STATE();
    case 317:
      if (lookahead == 'w') ADVANCE(372);
      END_STATE();
    case 318:
      if (lookahead == 'w') ADVANCE(234);
      END_STATE();
    case 319:
      if (lookahead == 'y') ADVANCE(318);
      END_STATE();
    case 320:
      if (lookahead == 'y') ADVANCE(29);
      END_STATE();
    case 321:
      if (lookahead == 'y') ADVANCE(30);
      END_STATE();
    case 322:
      if (lookahead == 'y') ADVANCE(31);
      END_STATE();
    case 323:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(356);
      END_STATE();
    case 324:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(353);
      END_STATE();
    case 325:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(324);
      END_STATE();
    case 326:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(325);
      END_STATE();
    case 327:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(326);
      END_STATE();
    case 328:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(327);
      END_STATE();
    case 329:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(328);
      END_STATE();
    case 330:
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(336);
      END_STATE();
    case 331:
      if (eof) ADVANCE(332);
      if (lookahead == '+') ADVANCE(338);
      if (lookahead == ',') ADVANCE(335);
      if (lookahead == '.') ADVANCE(337);
      if (lookahead == ':') ADVANCE(333);
      if (lookahead == '>') ADVANCE(340);
      if (lookahead == '~') ADVANCE(339);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(331)
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(330);
      END_STATE();
    case 332:
      ACCEPT_TOKEN(ts_builtin_sym_end);
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
      ACCEPT_TOKEN(aux_sym_sel_kind_token1);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(336);
      END_STATE();
    case 337:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 338:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 339:
      ACCEPT_TOKEN(anon_sym_TILDE);
      END_STATE();
    case 340:
      ACCEPT_TOKEN(anon_sym_GT);
      END_STATE();
    case 341:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 342:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 343:
      ACCEPT_TOKEN(anon_sym_fg);
      END_STATE();
    case 344:
      ACCEPT_TOKEN(anon_sym_bg);
      END_STATE();
    case 345:
      ACCEPT_TOKEN(anon_sym_attr);
      if (lookahead == 'i') ADVANCE(39);
      END_STATE();
    case 346:
      ACCEPT_TOKEN(anon_sym_attribute);
      END_STATE();
    case 347:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 348:
      ACCEPT_TOKEN(anon_sym_bold);
      END_STATE();
    case 349:
      ACCEPT_TOKEN(anon_sym_italic);
      END_STATE();
    case 350:
      ACCEPT_TOKEN(anon_sym_underlined);
      END_STATE();
    case 351:
      ACCEPT_TOKEN(anon_sym_underline);
      if (lookahead == 'd') ADVANCE(350);
      END_STATE();
    case 352:
      ACCEPT_TOKEN(anon_sym_reverse);
      END_STATE();
    case 353:
      ACCEPT_TOKEN(sym_rgb_color);
      END_STATE();
    case 354:
      ACCEPT_TOKEN(aux_sym_ansi_color_token1);
      if (lookahead == 'x') ADVANCE(323);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(355);
      END_STATE();
    case 355:
      ACCEPT_TOKEN(aux_sym_ansi_color_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(355);
      END_STATE();
    case 356:
      ACCEPT_TOKEN(aux_sym_ansi_color_token2);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(356);
      END_STATE();
    case 357:
      ACCEPT_TOKEN(anon_sym_black);
      END_STATE();
    case 358:
      ACCEPT_TOKEN(anon_sym_darkgrey);
      END_STATE();
    case 359:
      ACCEPT_TOKEN(anon_sym_dark_DASHgrey);
      END_STATE();
    case 360:
      ACCEPT_TOKEN(anon_sym_dark_grey);
      END_STATE();
    case 361:
      ACCEPT_TOKEN(anon_sym_red);
      END_STATE();
    case 362:
      ACCEPT_TOKEN(anon_sym_darkred);
      END_STATE();
    case 363:
      ACCEPT_TOKEN(anon_sym_dark_DASHred);
      END_STATE();
    case 364:
      ACCEPT_TOKEN(anon_sym_dark_red);
      END_STATE();
    case 365:
      ACCEPT_TOKEN(anon_sym_green);
      END_STATE();
    case 366:
      ACCEPT_TOKEN(anon_sym_darkgreen);
      END_STATE();
    case 367:
      ACCEPT_TOKEN(anon_sym_dark_DASHgreen);
      END_STATE();
    case 368:
      ACCEPT_TOKEN(anon_sym_dark_green);
      END_STATE();
    case 369:
      ACCEPT_TOKEN(anon_sym_yellow);
      END_STATE();
    case 370:
      ACCEPT_TOKEN(anon_sym_darkyellow);
      END_STATE();
    case 371:
      ACCEPT_TOKEN(anon_sym_dark_DASHyellow);
      END_STATE();
    case 372:
      ACCEPT_TOKEN(anon_sym_dark_yellow);
      END_STATE();
    case 373:
      ACCEPT_TOKEN(anon_sym_blue);
      END_STATE();
    case 374:
      ACCEPT_TOKEN(anon_sym_darkblue);
      END_STATE();
    case 375:
      ACCEPT_TOKEN(anon_sym_dark_DASHblue);
      END_STATE();
    case 376:
      ACCEPT_TOKEN(anon_sym_dark_blue);
      END_STATE();
    case 377:
      ACCEPT_TOKEN(anon_sym_magenta);
      END_STATE();
    case 378:
      ACCEPT_TOKEN(anon_sym_darkmagenta);
      END_STATE();
    case 379:
      ACCEPT_TOKEN(anon_sym_dark_DASHmagenta);
      END_STATE();
    case 380:
      ACCEPT_TOKEN(anon_sym_dark_magenta);
      END_STATE();
    case 381:
      ACCEPT_TOKEN(anon_sym_cyan);
      END_STATE();
    case 382:
      ACCEPT_TOKEN(anon_sym_darkcyan);
      END_STATE();
    case 383:
      ACCEPT_TOKEN(anon_sym_dark_DASHcyan);
      END_STATE();
    case 384:
      ACCEPT_TOKEN(anon_sym_dark_cyan);
      END_STATE();
    case 385:
      ACCEPT_TOKEN(anon_sym_white);
      END_STATE();
    case 386:
      ACCEPT_TOKEN(anon_sym_grey);
      END_STATE();
    case 387:
      ACCEPT_TOKEN(anon_sym_canvas);
      END_STATE();
    case 388:
      ACCEPT_TOKEN(anon_sym_comment);
      END_STATE();
    case 389:
      ACCEPT_TOKEN(anon_sym_constant);
      END_STATE();
    case 390:
      ACCEPT_TOKEN(anon_sym_string);
      END_STATE();
    case 391:
      ACCEPT_TOKEN(anon_sym_char);
      END_STATE();
    case 392:
      ACCEPT_TOKEN(anon_sym_number);
      END_STATE();
    case 393:
      ACCEPT_TOKEN(anon_sym_boolean);
      END_STATE();
    case 394:
      ACCEPT_TOKEN(anon_sym_float);
      END_STATE();
    case 395:
      ACCEPT_TOKEN(anon_sym_identifier);
      END_STATE();
    case 396:
      ACCEPT_TOKEN(anon_sym_function);
      END_STATE();
    case 397:
      ACCEPT_TOKEN(anon_sym_statement);
      END_STATE();
    case 398:
      ACCEPT_TOKEN(anon_sym_conditional);
      END_STATE();
    case 399:
      ACCEPT_TOKEN(anon_sym_repeat);
      END_STATE();
    case 400:
      ACCEPT_TOKEN(anon_sym_label);
      END_STATE();
    case 401:
      ACCEPT_TOKEN(anon_sym_operator);
      END_STATE();
    case 402:
      ACCEPT_TOKEN(anon_sym_keyword);
      END_STATE();
    case 403:
      ACCEPT_TOKEN(anon_sym_exception);
      END_STATE();
    case 404:
      ACCEPT_TOKEN(anon_sym_preproc);
      END_STATE();
    case 405:
      ACCEPT_TOKEN(anon_sym_include);
      END_STATE();
    case 406:
      ACCEPT_TOKEN(anon_sym_define);
      END_STATE();
    case 407:
      ACCEPT_TOKEN(anon_sym_macro);
      END_STATE();
    case 408:
      ACCEPT_TOKEN(anon_sym_precondit);
      END_STATE();
    case 409:
      ACCEPT_TOKEN(anon_sym_type);
      if (lookahead == 'd') ADVANCE(92);
      END_STATE();
    case 410:
      ACCEPT_TOKEN(anon_sym_storage_DASHclass);
      END_STATE();
    case 411:
      ACCEPT_TOKEN(anon_sym_structure);
      END_STATE();
    case 412:
      ACCEPT_TOKEN(anon_sym_typedef);
      END_STATE();
    case 413:
      ACCEPT_TOKEN(anon_sym_special);
      if (lookahead == '-') ADVANCE(46);
      END_STATE();
    case 414:
      ACCEPT_TOKEN(anon_sym_special_DASHchar);
      END_STATE();
    case 415:
      ACCEPT_TOKEN(anon_sym_tag);
      END_STATE();
    case 416:
      ACCEPT_TOKEN(anon_sym_delimiter);
      END_STATE();
    case 417:
      ACCEPT_TOKEN(anon_sym_special_DASHcomment);
      END_STATE();
    case 418:
      ACCEPT_TOKEN(anon_sym_debug);
      END_STATE();
    case 419:
      ACCEPT_TOKEN(anon_sym_ignore);
      END_STATE();
    case 420:
      ACCEPT_TOKEN(anon_sym_error);
      END_STATE();
    case 421:
      ACCEPT_TOKEN(anon_sym_todo);
      END_STATE();
    case 422:
      ACCEPT_TOKEN(anon_sym_line_DASHnr);
      END_STATE();
    case 423:
      ACCEPT_TOKEN(anon_sym_prompt);
      END_STATE();
    case 424:
      ACCEPT_TOKEN(anon_sym_status_DASHline);
      END_STATE();
    case 425:
      ACCEPT_TOKEN(anon_sym_tab_DASHline);
      END_STATE();
    case 426:
      ACCEPT_TOKEN(anon_sym_tab_DASHoption);
      END_STATE();
    case 427:
      ACCEPT_TOKEN(anon_sym_tab_DASHselect);
      END_STATE();
    default:
      return false;
  }
}

static TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 331},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 331},
  [6] = {.lex_state = 331},
  [7] = {.lex_state = 331},
  [8] = {.lex_state = 331},
  [9] = {.lex_state = 331},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 331},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 331},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 331},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 0},
  [22] = {.lex_state = 0},
  [23] = {.lex_state = 0},
  [24] = {.lex_state = 331},
  [25] = {.lex_state = 331},
  [26] = {.lex_state = 0},
  [27] = {.lex_state = 331},
  [28] = {.lex_state = 331},
  [29] = {.lex_state = 331},
  [30] = {.lex_state = 331},
  [31] = {.lex_state = 331},
  [32] = {.lex_state = 331},
  [33] = {.lex_state = 331},
  [34] = {.lex_state = 0},
  [35] = {.lex_state = 331},
  [36] = {.lex_state = 0},
  [37] = {.lex_state = 0},
  [38] = {.lex_state = 0},
  [39] = {.lex_state = 0},
  [40] = {.lex_state = 331},
  [41] = {.lex_state = 0},
  [42] = {.lex_state = 331},
  [43] = {.lex_state = 331},
  [44] = {.lex_state = 0},
  [45] = {.lex_state = 0},
  [46] = {.lex_state = 0},
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
  [59] = {.lex_state = 0},
  [60] = {.lex_state = 0},
  [61] = {.lex_state = 0},
  [62] = {.lex_state = 0},
  [63] = {.lex_state = 0},
  [64] = {.lex_state = 0},
};

static uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
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
    [sym_s] = STATE(64),
    [sym_hl_rule] = STATE(6),
    [sym_selectors] = STATE(63),
    [sym_selector] = STATE(38),
    [sym_sel_symbol] = STATE(27),
    [sym_sel_kind] = STATE(25),
    [sym_sel_field] = STATE(32),
    [sym_sel_kind_field] = STATE(32),
    [sym_sel_twins] = STATE(32),
    [sym_sel_siblings] = STATE(32),
    [sym_sel_child] = STATE(32),
    [aux_sym_s_repeat1] = STATE(6),
    [aux_sym_selector_repeat1] = STATE(8),
    [ts_builtin_sym_end] = ACTIONS(3),
    [aux_sym_sel_kind_token1] = ACTIONS(5),
    [anon_sym_DOT] = ACTIONS(7),
  },
};

static uint16_t ts_small_parse_table[] = {
  [0] = 4,
    ACTIONS(9), 1,
      anon_sym_LBRACE,
    ACTIONS(13), 2,
      anon_sym_type,
      anon_sym_special,
    STATE(58), 2,
      sym_properties,
      sym_highlight,
    ACTIONS(11), 40,
      anon_sym_underlined,
      anon_sym_canvas,
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
  [54] = 5,
    ACTIONS(15), 1,
      sym_rgb_color,
    ACTIONS(17), 1,
      aux_sym_ansi_color_token1,
    ACTIONS(19), 1,
      aux_sym_ansi_color_token2,
    STATE(55), 2,
      sym_ansi_color,
      sym_color_name,
    ACTIONS(21), 30,
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
  [100] = 5,
    ACTIONS(17), 1,
      aux_sym_ansi_color_token1,
    ACTIONS(19), 1,
      aux_sym_ansi_color_token2,
    ACTIONS(23), 1,
      sym_rgb_color,
    STATE(51), 2,
      sym_ansi_color,
      sym_color_name,
    ACTIONS(21), 30,
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
  [146] = 10,
    ACTIONS(25), 1,
      ts_builtin_sym_end,
    ACTIONS(27), 1,
      aux_sym_sel_kind_token1,
    ACTIONS(30), 1,
      anon_sym_DOT,
    STATE(8), 1,
      aux_sym_selector_repeat1,
    STATE(25), 1,
      sym_sel_kind,
    STATE(27), 1,
      sym_sel_symbol,
    STATE(38), 1,
      sym_selector,
    STATE(63), 1,
      sym_selectors,
    STATE(5), 2,
      sym_hl_rule,
      aux_sym_s_repeat1,
    STATE(32), 5,
      sym_sel_field,
      sym_sel_kind_field,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [182] = 10,
    ACTIONS(5), 1,
      aux_sym_sel_kind_token1,
    ACTIONS(7), 1,
      anon_sym_DOT,
    ACTIONS(33), 1,
      ts_builtin_sym_end,
    STATE(8), 1,
      aux_sym_selector_repeat1,
    STATE(25), 1,
      sym_sel_kind,
    STATE(27), 1,
      sym_sel_symbol,
    STATE(38), 1,
      sym_selector,
    STATE(63), 1,
      sym_selectors,
    STATE(5), 2,
      sym_hl_rule,
      aux_sym_s_repeat1,
    STATE(32), 5,
      sym_sel_field,
      sym_sel_kind_field,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [218] = 7,
    ACTIONS(37), 1,
      aux_sym_sel_kind_token1,
    ACTIONS(40), 1,
      anon_sym_DOT,
    STATE(7), 1,
      aux_sym_selector_repeat1,
    STATE(25), 1,
      sym_sel_kind,
    STATE(27), 1,
      sym_sel_symbol,
    ACTIONS(35), 2,
      anon_sym_COLON,
      anon_sym_COMMA,
    STATE(32), 5,
      sym_sel_field,
      sym_sel_kind_field,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [245] = 7,
    ACTIONS(5), 1,
      aux_sym_sel_kind_token1,
    ACTIONS(7), 1,
      anon_sym_DOT,
    STATE(7), 1,
      aux_sym_selector_repeat1,
    STATE(25), 1,
      sym_sel_kind,
    STATE(27), 1,
      sym_sel_symbol,
    ACTIONS(43), 2,
      anon_sym_COLON,
      anon_sym_COMMA,
    STATE(32), 5,
      sym_sel_field,
      sym_sel_kind_field,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [272] = 7,
    ACTIONS(5), 1,
      aux_sym_sel_kind_token1,
    ACTIONS(7), 1,
      anon_sym_DOT,
    STATE(8), 1,
      aux_sym_selector_repeat1,
    STATE(25), 1,
      sym_sel_kind,
    STATE(27), 1,
      sym_sel_symbol,
    STATE(44), 1,
      sym_selector,
    STATE(32), 5,
      sym_sel_field,
      sym_sel_kind_field,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [298] = 11,
    ACTIONS(45), 1,
      anon_sym_RBRACE,
    ACTIONS(47), 1,
      anon_sym_fg,
    ACTIONS(49), 1,
      anon_sym_bg,
    ACTIONS(51), 1,
      anon_sym_attr,
    ACTIONS(53), 1,
      anon_sym_attribute,
    STATE(11), 1,
      aux_sym_properties_repeat1,
    STATE(47), 1,
      sym_property,
    STATE(56), 1,
      sym_bg,
    STATE(57), 1,
      sym_fg,
    STATE(61), 1,
      sym_attribute,
    STATE(62), 1,
      sym_attrb,
  [332] = 11,
    ACTIONS(55), 1,
      anon_sym_RBRACE,
    ACTIONS(57), 1,
      anon_sym_fg,
    ACTIONS(60), 1,
      anon_sym_bg,
    ACTIONS(63), 1,
      anon_sym_attr,
    ACTIONS(66), 1,
      anon_sym_attribute,
    STATE(11), 1,
      aux_sym_properties_repeat1,
    STATE(47), 1,
      sym_property,
    STATE(56), 1,
      sym_bg,
    STATE(57), 1,
      sym_fg,
    STATE(61), 1,
      sym_attribute,
    STATE(62), 1,
      sym_attrb,
  [366] = 11,
    ACTIONS(47), 1,
      anon_sym_fg,
    ACTIONS(49), 1,
      anon_sym_bg,
    ACTIONS(51), 1,
      anon_sym_attr,
    ACTIONS(53), 1,
      anon_sym_attribute,
    ACTIONS(69), 1,
      anon_sym_RBRACE,
    STATE(10), 1,
      aux_sym_properties_repeat1,
    STATE(47), 1,
      sym_property,
    STATE(56), 1,
      sym_bg,
    STATE(57), 1,
      sym_fg,
    STATE(61), 1,
      sym_attribute,
    STATE(62), 1,
      sym_attrb,
  [400] = 5,
    ACTIONS(5), 1,
      aux_sym_sel_kind_token1,
    ACTIONS(7), 1,
      anon_sym_DOT,
    STATE(25), 1,
      sym_sel_kind,
    STATE(29), 1,
      sym_sel_symbol,
    STATE(32), 5,
      sym_sel_field,
      sym_sel_kind_field,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [420] = 5,
    ACTIONS(71), 1,
      anon_sym_COMMA,
    ACTIONS(75), 1,
      anon_sym_underline,
    STATE(23), 1,
      sym_attr,
    STATE(16), 2,
      sym_attrs,
      aux_sym_attrb_repeat1,
    ACTIONS(73), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [440] = 4,
    ACTIONS(79), 1,
      anon_sym_PIPE,
    ACTIONS(82), 1,
      anon_sym_underline,
    STATE(15), 2,
      sym_attr_or,
      aux_sym_attrs_repeat1,
    ACTIONS(77), 5,
      anon_sym_COMMA,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [458] = 5,
    ACTIONS(84), 1,
      anon_sym_COMMA,
    ACTIONS(89), 1,
      anon_sym_underline,
    STATE(23), 1,
      sym_attr,
    STATE(16), 2,
      sym_attrs,
      aux_sym_attrb_repeat1,
    ACTIONS(86), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [478] = 5,
    ACTIONS(5), 1,
      aux_sym_sel_kind_token1,
    ACTIONS(7), 1,
      anon_sym_DOT,
    STATE(25), 1,
      sym_sel_kind,
    STATE(28), 1,
      sym_sel_symbol,
    STATE(32), 5,
      sym_sel_field,
      sym_sel_kind_field,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [498] = 5,
    ACTIONS(75), 1,
      anon_sym_underline,
    ACTIONS(92), 1,
      anon_sym_COMMA,
    STATE(23), 1,
      sym_attr,
    STATE(14), 2,
      sym_attrs,
      aux_sym_attrb_repeat1,
    ACTIONS(73), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [518] = 5,
    ACTIONS(5), 1,
      aux_sym_sel_kind_token1,
    ACTIONS(7), 1,
      anon_sym_DOT,
    STATE(25), 1,
      sym_sel_kind,
    STATE(30), 1,
      sym_sel_symbol,
    STATE(32), 5,
      sym_sel_field,
      sym_sel_kind_field,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [538] = 5,
    ACTIONS(75), 1,
      anon_sym_underline,
    ACTIONS(94), 1,
      anon_sym_COMMA,
    STATE(23), 1,
      sym_attr,
    STATE(16), 2,
      sym_attrs,
      aux_sym_attrb_repeat1,
    ACTIONS(73), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [558] = 5,
    ACTIONS(75), 1,
      anon_sym_underline,
    ACTIONS(96), 1,
      anon_sym_COMMA,
    STATE(23), 1,
      sym_attr,
    STATE(20), 2,
      sym_attrs,
      aux_sym_attrb_repeat1,
    ACTIONS(73), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [578] = 4,
    ACTIONS(100), 1,
      anon_sym_PIPE,
    ACTIONS(102), 1,
      anon_sym_underline,
    STATE(15), 2,
      sym_attr_or,
      aux_sym_attrs_repeat1,
    ACTIONS(98), 5,
      anon_sym_COMMA,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [596] = 4,
    ACTIONS(100), 1,
      anon_sym_PIPE,
    ACTIONS(106), 1,
      anon_sym_underline,
    STATE(22), 2,
      sym_attr_or,
      aux_sym_attrs_repeat1,
    ACTIONS(104), 5,
      anon_sym_COMMA,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [614] = 1,
    ACTIONS(108), 7,
      anon_sym_COLON,
      anon_sym_COMMA,
      aux_sym_sel_kind_token1,
      anon_sym_DOT,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
  [624] = 2,
    ACTIONS(112), 1,
      anon_sym_DOT,
    ACTIONS(110), 6,
      anon_sym_COLON,
      anon_sym_COMMA,
      aux_sym_sel_kind_token1,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
  [636] = 2,
    ACTIONS(116), 1,
      anon_sym_underline,
    ACTIONS(114), 6,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [648] = 4,
    ACTIONS(120), 1,
      anon_sym_PLUS,
    ACTIONS(122), 1,
      anon_sym_TILDE,
    ACTIONS(124), 1,
      anon_sym_GT,
    ACTIONS(118), 4,
      anon_sym_COLON,
      anon_sym_COMMA,
      aux_sym_sel_kind_token1,
      anon_sym_DOT,
  [664] = 1,
    ACTIONS(126), 7,
      anon_sym_COLON,
      anon_sym_COMMA,
      aux_sym_sel_kind_token1,
      anon_sym_DOT,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
  [674] = 1,
    ACTIONS(128), 7,
      anon_sym_COLON,
      anon_sym_COMMA,
      aux_sym_sel_kind_token1,
      anon_sym_DOT,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
  [684] = 1,
    ACTIONS(130), 7,
      anon_sym_COLON,
      anon_sym_COMMA,
      aux_sym_sel_kind_token1,
      anon_sym_DOT,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
  [694] = 1,
    ACTIONS(132), 7,
      anon_sym_COLON,
      anon_sym_COMMA,
      aux_sym_sel_kind_token1,
      anon_sym_DOT,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
  [704] = 1,
    ACTIONS(110), 7,
      anon_sym_COLON,
      anon_sym_COMMA,
      aux_sym_sel_kind_token1,
      anon_sym_DOT,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
  [714] = 1,
    ACTIONS(134), 7,
      anon_sym_COLON,
      anon_sym_COMMA,
      aux_sym_sel_kind_token1,
      anon_sym_DOT,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
  [724] = 2,
    ACTIONS(138), 1,
      anon_sym_underline,
    ACTIONS(136), 6,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [736] = 1,
    ACTIONS(140), 7,
      anon_sym_COLON,
      anon_sym_COMMA,
      aux_sym_sel_kind_token1,
      anon_sym_DOT,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
  [746] = 3,
    ACTIONS(75), 1,
      anon_sym_underline,
    STATE(34), 1,
      sym_attr,
    ACTIONS(73), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [759] = 2,
    ACTIONS(144), 1,
      anon_sym_attr,
    ACTIONS(142), 4,
      anon_sym_RBRACE,
      anon_sym_fg,
      anon_sym_bg,
      anon_sym_attribute,
  [769] = 3,
    ACTIONS(146), 1,
      anon_sym_COLON,
    ACTIONS(148), 1,
      anon_sym_COMMA,
    STATE(39), 1,
      aux_sym_selectors_repeat1,
  [779] = 3,
    ACTIONS(148), 1,
      anon_sym_COMMA,
    ACTIONS(150), 1,
      anon_sym_COLON,
    STATE(41), 1,
      aux_sym_selectors_repeat1,
  [789] = 1,
    ACTIONS(152), 3,
      ts_builtin_sym_end,
      aux_sym_sel_kind_token1,
      anon_sym_DOT,
  [795] = 3,
    ACTIONS(154), 1,
      anon_sym_COLON,
    ACTIONS(156), 1,
      anon_sym_COMMA,
    STATE(41), 1,
      aux_sym_selectors_repeat1,
  [805] = 2,
    ACTIONS(159), 1,
      aux_sym_sel_kind_token1,
    STATE(35), 1,
      sym_field_name,
  [812] = 2,
    ACTIONS(159), 1,
      aux_sym_sel_kind_token1,
    STATE(31), 1,
      sym_field_name,
  [819] = 1,
    ACTIONS(154), 2,
      anon_sym_COLON,
      anon_sym_COMMA,
  [824] = 1,
    ACTIONS(161), 1,
      anon_sym_COLON,
  [828] = 1,
    ACTIONS(163), 1,
      anon_sym_COLON,
  [832] = 1,
    ACTIONS(165), 1,
      anon_sym_COMMA,
  [836] = 1,
    ACTIONS(167), 1,
      anon_sym_COLON,
  [840] = 1,
    ACTIONS(169), 1,
      anon_sym_SEMI,
  [844] = 1,
    ACTIONS(171), 1,
      anon_sym_SEMI,
  [848] = 1,
    ACTIONS(173), 1,
      anon_sym_COMMA,
  [852] = 1,
    ACTIONS(175), 1,
      anon_sym_COMMA,
  [856] = 1,
    ACTIONS(177), 1,
      anon_sym_COMMA,
  [860] = 1,
    ACTIONS(179), 1,
      anon_sym_COMMA,
  [864] = 1,
    ACTIONS(181), 1,
      anon_sym_COMMA,
  [868] = 1,
    ACTIONS(183), 1,
      anon_sym_COMMA,
  [872] = 1,
    ACTIONS(185), 1,
      anon_sym_COMMA,
  [876] = 1,
    ACTIONS(187), 1,
      anon_sym_SEMI,
  [880] = 1,
    ACTIONS(189), 1,
      anon_sym_SEMI,
  [884] = 1,
    ACTIONS(191), 1,
      anon_sym_COLON,
  [888] = 1,
    ACTIONS(193), 1,
      anon_sym_COMMA,
  [892] = 1,
    ACTIONS(195), 1,
      anon_sym_COMMA,
  [896] = 1,
    ACTIONS(197), 1,
      anon_sym_COLON,
  [900] = 1,
    ACTIONS(199), 1,
      ts_builtin_sym_end,
};

static uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 54,
  [SMALL_STATE(4)] = 100,
  [SMALL_STATE(5)] = 146,
  [SMALL_STATE(6)] = 182,
  [SMALL_STATE(7)] = 218,
  [SMALL_STATE(8)] = 245,
  [SMALL_STATE(9)] = 272,
  [SMALL_STATE(10)] = 298,
  [SMALL_STATE(11)] = 332,
  [SMALL_STATE(12)] = 366,
  [SMALL_STATE(13)] = 400,
  [SMALL_STATE(14)] = 420,
  [SMALL_STATE(15)] = 440,
  [SMALL_STATE(16)] = 458,
  [SMALL_STATE(17)] = 478,
  [SMALL_STATE(18)] = 498,
  [SMALL_STATE(19)] = 518,
  [SMALL_STATE(20)] = 538,
  [SMALL_STATE(21)] = 558,
  [SMALL_STATE(22)] = 578,
  [SMALL_STATE(23)] = 596,
  [SMALL_STATE(24)] = 614,
  [SMALL_STATE(25)] = 624,
  [SMALL_STATE(26)] = 636,
  [SMALL_STATE(27)] = 648,
  [SMALL_STATE(28)] = 664,
  [SMALL_STATE(29)] = 674,
  [SMALL_STATE(30)] = 684,
  [SMALL_STATE(31)] = 694,
  [SMALL_STATE(32)] = 704,
  [SMALL_STATE(33)] = 714,
  [SMALL_STATE(34)] = 724,
  [SMALL_STATE(35)] = 736,
  [SMALL_STATE(36)] = 746,
  [SMALL_STATE(37)] = 759,
  [SMALL_STATE(38)] = 769,
  [SMALL_STATE(39)] = 779,
  [SMALL_STATE(40)] = 789,
  [SMALL_STATE(41)] = 795,
  [SMALL_STATE(42)] = 805,
  [SMALL_STATE(43)] = 812,
  [SMALL_STATE(44)] = 819,
  [SMALL_STATE(45)] = 824,
  [SMALL_STATE(46)] = 828,
  [SMALL_STATE(47)] = 832,
  [SMALL_STATE(48)] = 836,
  [SMALL_STATE(49)] = 840,
  [SMALL_STATE(50)] = 844,
  [SMALL_STATE(51)] = 848,
  [SMALL_STATE(52)] = 852,
  [SMALL_STATE(53)] = 856,
  [SMALL_STATE(54)] = 860,
  [SMALL_STATE(55)] = 864,
  [SMALL_STATE(56)] = 868,
  [SMALL_STATE(57)] = 872,
  [SMALL_STATE(58)] = 876,
  [SMALL_STATE(59)] = 880,
  [SMALL_STATE(60)] = 884,
  [SMALL_STATE(61)] = 888,
  [SMALL_STATE(62)] = 892,
  [SMALL_STATE(63)] = 896,
  [SMALL_STATE(64)] = 900,
};

static TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_s, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(59),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [17] = {.entry = {.count = 1, .reusable = false}}, SHIFT(52),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [25] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_s_repeat1, 2),
  [27] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_s_repeat1, 2), SHIFT_REPEAT(33),
  [30] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_s_repeat1, 2), SHIFT_REPEAT(42),
  [33] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_s, 1),
  [35] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_selector_repeat1, 2),
  [37] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_selector_repeat1, 2), SHIFT_REPEAT(33),
  [40] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_selector_repeat1, 2), SHIFT_REPEAT(42),
  [43] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selector, 1),
  [45] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [47] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [49] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [51] = {.entry = {.count = 1, .reusable = false}}, SHIFT(45),
  [53] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [55] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_properties_repeat1, 2, .production_id = 8),
  [57] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_properties_repeat1, 2, .production_id = 8), SHIFT_REPEAT(48),
  [60] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_properties_repeat1, 2, .production_id = 8), SHIFT_REPEAT(60),
  [63] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_properties_repeat1, 2, .production_id = 8), SHIFT_REPEAT(45),
  [66] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_properties_repeat1, 2, .production_id = 8), SHIFT_REPEAT(46),
  [69] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [71] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attrb, 3),
  [73] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [75] = {.entry = {.count = 1, .reusable = false}}, SHIFT(26),
  [77] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_attrs_repeat1, 2),
  [79] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_attrs_repeat1, 2), SHIFT_REPEAT(36),
  [82] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_attrs_repeat1, 2),
  [84] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_attrb_repeat1, 2),
  [86] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_attrb_repeat1, 2), SHIFT_REPEAT(26),
  [89] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_attrb_repeat1, 2), SHIFT_REPEAT(26),
  [92] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attrb, 2),
  [94] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute, 3),
  [96] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute, 2),
  [98] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attrs, 2),
  [100] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [102] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attrs, 2),
  [104] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attrs, 1),
  [106] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attrs, 1),
  [108] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field_name, 1),
  [110] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_symbol, 1),
  [112] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [114] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attr, 1),
  [116] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attr, 1),
  [118] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_selector_repeat1, 1),
  [120] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [122] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [124] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [126] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_twins, 3),
  [128] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_siblings, 3),
  [130] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_child, 3),
  [132] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_kind_field, 3),
  [134] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_kind, 1),
  [136] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attr_or, 2),
  [138] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attr_or, 2),
  [140] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_field, 2),
  [142] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_properties_repeat1, 2, .production_id = 6),
  [144] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_properties_repeat1, 2, .production_id = 6),
  [146] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selectors, 1),
  [148] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [150] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selectors, 2),
  [152] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hl_rule, 4, .production_id = 5),
  [154] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_selectors_repeat1, 2),
  [156] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_selectors_repeat1, 2), SHIFT_REPEAT(9),
  [159] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [161] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [163] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [165] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [167] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [169] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_properties, 3, .production_id = 7),
  [171] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_properties, 2),
  [173] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fg, 3),
  [175] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ansi_color, 1, .production_id = 9),
  [177] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ansi_color, 1, .production_id = 10),
  [179] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_color_name, 1),
  [181] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_bg, 3),
  [183] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property, 1, .production_id = 2),
  [185] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property, 1, .production_id = 1),
  [187] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [189] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_highlight, 1),
  [191] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [193] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property, 1, .production_id = 4),
  [195] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property, 1, .production_id = 3),
  [197] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [199] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
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
