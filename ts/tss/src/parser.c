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
#define SYMBOL_COUNT 129
#define ALIAS_COUNT 0
#define TOKEN_COUNT 99
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 10
#define MAX_ALIAS_SEQUENCE_LENGTH 4

enum {
  anon_sym_COLON = 1,
  anon_sym_SEMI = 2,
  sym_comment = 3,
  sym_newline = 4,
  anon_sym_COMMA = 5,
  aux_sym_sel_kind_token1 = 6,
  anon_sym_DOT = 7,
  anon_sym_PLUS = 8,
  anon_sym_TILDE = 9,
  anon_sym_GT = 10,
  anon_sym_LBRACE = 11,
  anon_sym_RBRACE = 12,
  anon_sym_fg = 13,
  anon_sym_bg = 14,
  anon_sym_attr = 15,
  anon_sym_attribute = 16,
  anon_sym_PIPE = 17,
  anon_sym_bold = 18,
  anon_sym_italic = 19,
  anon_sym_underlined = 20,
  anon_sym_underline = 21,
  anon_sym_reverse = 22,
  sym_rgb_color = 23,
  aux_sym_ansi_color_token1 = 24,
  aux_sym_ansi_color_token2 = 25,
  anon_sym_black = 26,
  anon_sym_darkgrey = 27,
  anon_sym_dark_DASHgrey = 28,
  anon_sym_dark_grey = 29,
  anon_sym_red = 30,
  anon_sym_darkred = 31,
  anon_sym_dark_DASHred = 32,
  anon_sym_dark_red = 33,
  anon_sym_green = 34,
  anon_sym_darkgreen = 35,
  anon_sym_dark_DASHgreen = 36,
  anon_sym_dark_green = 37,
  anon_sym_yellow = 38,
  anon_sym_darkyellow = 39,
  anon_sym_dark_DASHyellow = 40,
  anon_sym_dark_yellow = 41,
  anon_sym_blue = 42,
  anon_sym_darkblue = 43,
  anon_sym_dark_DASHblue = 44,
  anon_sym_dark_blue = 45,
  anon_sym_magenta = 46,
  anon_sym_darkmagenta = 47,
  anon_sym_dark_DASHmagenta = 48,
  anon_sym_dark_magenta = 49,
  anon_sym_cyan = 50,
  anon_sym_darkcyan = 51,
  anon_sym_dark_DASHcyan = 52,
  anon_sym_dark_cyan = 53,
  anon_sym_white = 54,
  anon_sym_grey = 55,
  anon_sym_bg_DASHcanvas = 56,
  anon_sym_fg_DASHcanvas = 57,
  anon_sym_canvas = 58,
  anon_sym_comment = 59,
  anon_sym_constant = 60,
  anon_sym_string = 61,
  anon_sym_char = 62,
  anon_sym_number = 63,
  anon_sym_boolean = 64,
  anon_sym_float = 65,
  anon_sym_identifier = 66,
  anon_sym_function = 67,
  anon_sym_statement = 68,
  anon_sym_conditional = 69,
  anon_sym_repeat = 70,
  anon_sym_label = 71,
  anon_sym_operator = 72,
  anon_sym_keyword = 73,
  anon_sym_exception = 74,
  anon_sym_preproc = 75,
  anon_sym_include = 76,
  anon_sym_define = 77,
  anon_sym_macro = 78,
  anon_sym_precondit = 79,
  anon_sym_type = 80,
  anon_sym_storage_DASHclass = 81,
  anon_sym_structure = 82,
  anon_sym_typedef = 83,
  anon_sym_special = 84,
  anon_sym_special_DASHchar = 85,
  anon_sym_tag = 86,
  anon_sym_delimiter = 87,
  anon_sym_special_DASHcomment = 88,
  anon_sym_debug = 89,
  anon_sym_ignore = 90,
  anon_sym_error = 91,
  anon_sym_todo = 92,
  anon_sym_line_DASHnr = 93,
  anon_sym_prompt = 94,
  anon_sym_status_DASHline = 95,
  anon_sym_tab_DASHline = 96,
  anon_sym_tab_DASHoption = 97,
  anon_sym_tab_DASHselect = 98,
  sym_s = 99,
  sym_hl_rule = 100,
  sym_selectors = 101,
  sym_selector = 102,
  sym_sel_symbol = 103,
  sym_sel_kind = 104,
  sym_field_name = 105,
  sym_sel_field = 106,
  sym_sel_kind_field = 107,
  sym_sel_twins = 108,
  sym_sel_siblings = 109,
  sym_sel_child = 110,
  sym_properties = 111,
  sym_property = 112,
  sym_fg = 113,
  sym_bg = 114,
  sym_attrb = 115,
  sym_attribute = 116,
  sym_attrs = 117,
  sym_attr_or = 118,
  sym_attr = 119,
  sym_ansi_color = 120,
  sym_color_name = 121,
  sym_highlight = 122,
  aux_sym_s_repeat1 = 123,
  aux_sym_selectors_repeat1 = 124,
  aux_sym_selector_repeat1 = 125,
  aux_sym_properties_repeat1 = 126,
  aux_sym_attrb_repeat1 = 127,
  aux_sym_attrs_repeat1 = 128,
};

static const char *ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_COLON] = ":",
  [anon_sym_SEMI] = ";",
  [sym_comment] = "comment",
  [sym_newline] = "newline",
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
  [anon_sym_bg_DASHcanvas] = "bg-canvas",
  [anon_sym_fg_DASHcanvas] = "fg-canvas",
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
  [sym_comment] = sym_comment,
  [sym_newline] = sym_newline,
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
  [anon_sym_bg_DASHcanvas] = anon_sym_bg_DASHcanvas,
  [anon_sym_fg_DASHcanvas] = anon_sym_fg_DASHcanvas,
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
  [sym_comment] = {
    .visible = true,
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
  [anon_sym_bg_DASHcanvas] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_fg_DASHcanvas] = {
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
      if (eof) ADVANCE(353);
      if (lookahead == '\n') ADVANCE(363);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '#') ADVANCE(361);
      if (lookahead == '+') ADVANCE(367);
      if (lookahead == ',') ADVANCE(364);
      if (lookahead == '.') ADVANCE(366);
      if (lookahead == '0') ADVANCE(384);
      if (lookahead == ':') ADVANCE(354);
      if (lookahead == ';') ADVANCE(355);
      if (lookahead == '>') ADVANCE(369);
      if (lookahead == 'a') ADVANCE(312);
      if (lookahead == 'b') ADVANCE(144);
      if (lookahead == 'c') ADVANCE(19);
      if (lookahead == 'd') ADVANCE(23);
      if (lookahead == 'e') ADVANCE(282);
      if (lookahead == 'f') ADVANCE(145);
      if (lookahead == 'g') ADVANCE(283);
      if (lookahead == 'i') ADVANCE(78);
      if (lookahead == 'k') ADVANCE(83);
      if (lookahead == 'l') ADVANCE(11);
      if (lookahead == 'm') ADVANCE(12);
      if (lookahead == 'n') ADVANCE(329);
      if (lookahead == 'o') ADVANCE(263);
      if (lookahead == 'p') ADVANCE(269);
      if (lookahead == 'r') ADVANCE(84);
      if (lookahead == 's') ADVANCE(264);
      if (lookahead == 't') ADVANCE(13);
      if (lookahead == 'u') ADVANCE(223);
      if (lookahead == 'w') ADVANCE(157);
      if (lookahead == 'y') ADVANCE(85);
      if (lookahead == '{') ADVANCE(370);
      if (lookahead == '|') ADVANCE(378);
      if (lookahead == '}') ADVANCE(371);
      if (lookahead == '~') ADVANCE(368);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(0)
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(385);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(363);
      END_STATE();
    case 2:
      if (lookahead == '\n') ADVANCE(363);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '#') ADVANCE(361);
      if (lookahead == '0') ADVANCE(384);
      if (lookahead == 'b') ADVANCE(150);
      if (lookahead == 'c') ADVANCE(346);
      if (lookahead == 'd') ADVANCE(22);
      if (lookahead == 'f') ADVANCE(156);
      if (lookahead == 'g') ADVANCE(283);
      if (lookahead == 'm') ADVANCE(47);
      if (lookahead == 'r') ADVANCE(118);
      if (lookahead == 'w') ADVANCE(157);
      if (lookahead == 'y') ADVANCE(85);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(2)
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(385);
      END_STATE();
    case 3:
      if (lookahead == '\n') ADVANCE(363);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '#') ADVANCE(362);
      if (lookahead == 'a') ADVANCE(312);
      if (lookahead == 'b') ADVANCE(148);
      if (lookahead == 'f') ADVANCE(149);
      if (lookahead == '}') ADVANCE(371);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(3)
      END_STATE();
    case 4:
      if (lookahead == '-') ADVANCE(199);
      END_STATE();
    case 5:
      if (lookahead == '-') ADVANCE(52);
      if (lookahead == '_') ADVANCE(53);
      if (lookahead == 'b') ADVANCE(185);
      if (lookahead == 'c') ADVANCE(347);
      if (lookahead == 'g') ADVANCE(290);
      if (lookahead == 'm') ADVANCE(42);
      if (lookahead == 'r') ADVANCE(108);
      if (lookahead == 'y') ADVANCE(138);
      END_STATE();
    case 6:
      if (lookahead == '-') ADVANCE(61);
      END_STATE();
    case 7:
      if (lookahead == '-') ADVANCE(233);
      END_STATE();
    case 8:
      if (lookahead == '-') ADVANCE(66);
      END_STATE();
    case 9:
      if (lookahead == '-') ADVANCE(69);
      END_STATE();
    case 10:
      if (lookahead == '-') ADVANCE(201);
      END_STATE();
    case 11:
      if (lookahead == 'a') ADVANCE(51);
      if (lookahead == 'i') ADVANCE(230);
      END_STATE();
    case 12:
      if (lookahead == 'a') ADVANCE(64);
      END_STATE();
    case 13:
      if (lookahead == 'a') ADVANCE(48);
      if (lookahead == 'o') ADVANCE(77);
      if (lookahead == 'y') ADVANCE(266);
      END_STATE();
    case 14:
      if (lookahead == 'a') ADVANCE(295);
      END_STATE();
    case 15:
      if (lookahead == 'a') ADVANCE(407);
      END_STATE();
    case 16:
      if (lookahead == 'a') ADVANCE(408);
      END_STATE();
    case 17:
      if (lookahead == 'a') ADVANCE(409);
      END_STATE();
    case 18:
      if (lookahead == 'a') ADVANCE(410);
      END_STATE();
    case 19:
      if (lookahead == 'a') ADVANCE(210);
      if (lookahead == 'h') ADVANCE(29);
      if (lookahead == 'o') ADVANCE(202);
      if (lookahead == 'y') ADVANCE(30);
      END_STATE();
    case 20:
      if (lookahead == 'a') ADVANCE(151);
      END_STATE();
    case 21:
      if (lookahead == 'a') ADVANCE(302);
      if (lookahead == 'o') ADVANCE(286);
      if (lookahead == 'r') ADVANCE(163);
      END_STATE();
    case 22:
      if (lookahead == 'a') ADVANCE(270);
      END_STATE();
    case 23:
      if (lookahead == 'a') ADVANCE(270);
      if (lookahead == 'e') ADVANCE(49);
      END_STATE();
    case 24:
      if (lookahead == 'a') ADVANCE(58);
      if (lookahead == 'u') ADVANCE(88);
      END_STATE();
    case 25:
      if (lookahead == 'a') ADVANCE(184);
      END_STATE();
    case 26:
      if (lookahead == 'a') ADVANCE(296);
      END_STATE();
    case 27:
      if (lookahead == 'a') ADVANCE(297);
      END_STATE();
    case 28:
      if (lookahead == 'a') ADVANCE(303);
      END_STATE();
    case 29:
      if (lookahead == 'a') ADVANCE(272);
      END_STATE();
    case 30:
      if (lookahead == 'a') ADVANCE(211);
      END_STATE();
    case 31:
      if (lookahead == 'a') ADVANCE(300);
      END_STATE();
    case 32:
      if (lookahead == 'a') ADVANCE(180);
      END_STATE();
    case 33:
      if (lookahead == 'a') ADVANCE(323);
      END_STATE();
    case 34:
      if (lookahead == 'a') ADVANCE(213);
      END_STATE();
    case 35:
      if (lookahead == 'a') ADVANCE(181);
      END_STATE();
    case 36:
      if (lookahead == 'a') ADVANCE(305);
      END_STATE();
    case 37:
      if (lookahead == 'a') ADVANCE(214);
      END_STATE();
    case 38:
      if (lookahead == 'a') ADVANCE(216);
      END_STATE();
    case 39:
      if (lookahead == 'a') ADVANCE(217);
      END_STATE();
    case 40:
      if (lookahead == 'a') ADVANCE(279);
      END_STATE();
    case 41:
      if (lookahead == 'a') ADVANCE(240);
      END_STATE();
    case 42:
      if (lookahead == 'a') ADVANCE(153);
      END_STATE();
    case 43:
      if (lookahead == 'a') ADVANCE(242);
      END_STATE();
    case 44:
      if (lookahead == 'a') ADVANCE(232);
      END_STATE();
    case 45:
      if (lookahead == 'a') ADVANCE(154);
      END_STATE();
    case 46:
      if (lookahead == 'a') ADVANCE(155);
      END_STATE();
    case 47:
      if (lookahead == 'a') ADVANCE(152);
      END_STATE();
    case 48:
      if (lookahead == 'b') ADVANCE(4);
      if (lookahead == 'g') ADVANCE(447);
      END_STATE();
    case 49:
      if (lookahead == 'b') ADVANCE(330);
      if (lookahead == 'f') ADVANCE(168);
      if (lookahead == 'l') ADVANCE(161);
      END_STATE();
    case 50:
      if (lookahead == 'b') ADVANCE(335);
      END_STATE();
    case 51:
      if (lookahead == 'b') ADVANCE(112);
      END_STATE();
    case 52:
      if (lookahead == 'b') ADVANCE(194);
      if (lookahead == 'c') ADVANCE(348);
      if (lookahead == 'g') ADVANCE(291);
      if (lookahead == 'm') ADVANCE(45);
      if (lookahead == 'r') ADVANCE(114);
      if (lookahead == 'y') ADVANCE(139);
      END_STATE();
    case 53:
      if (lookahead == 'b') ADVANCE(196);
      if (lookahead == 'c') ADVANCE(349);
      if (lookahead == 'g') ADVANCE(292);
      if (lookahead == 'm') ADVANCE(46);
      if (lookahead == 'r') ADVANCE(117);
      if (lookahead == 'y') ADVANCE(140);
      END_STATE();
    case 54:
      if (lookahead == 'b') ADVANCE(121);
      END_STATE();
    case 55:
      if (lookahead == 'c') ADVANCE(380);
      END_STATE();
    case 56:
      if (lookahead == 'c') ADVANCE(436);
      END_STATE();
    case 57:
      if (lookahead == 'c') ADVANCE(158);
      END_STATE();
    case 58:
      if (lookahead == 'c') ADVANCE(178);
      END_STATE();
    case 59:
      if (lookahead == 'c') ADVANCE(182);
      END_STATE();
    case 60:
      if (lookahead == 'c') ADVANCE(167);
      END_STATE();
    case 61:
      if (lookahead == 'c') ADVANCE(41);
      END_STATE();
    case 62:
      if (lookahead == 'c') ADVANCE(262);
      if (lookahead == 'p') ADVANCE(287);
      END_STATE();
    case 63:
      if (lookahead == 'c') ADVANCE(315);
      END_STATE();
    case 64:
      if (lookahead == 'c') ADVANCE(284);
      if (lookahead == 'g') ADVANCE(129);
      END_STATE();
    case 65:
      if (lookahead == 'c') ADVANCE(103);
      END_STATE();
    case 66:
      if (lookahead == 'c') ADVANCE(191);
      END_STATE();
    case 67:
      if (lookahead == 'c') ADVANCE(314);
      END_STATE();
    case 68:
      if (lookahead == 'c') ADVANCE(310);
      END_STATE();
    case 69:
      if (lookahead == 'c') ADVANCE(43);
      END_STATE();
    case 70:
      if (lookahead == 'd') ADVANCE(391);
      END_STATE();
    case 71:
      if (lookahead == 'd') ADVANCE(391);
      if (lookahead == 'p') ADVANCE(130);
      if (lookahead == 'v') ADVANCE(119);
      END_STATE();
    case 72:
      if (lookahead == 'd') ADVANCE(379);
      END_STATE();
    case 73:
      if (lookahead == 'd') ADVANCE(392);
      END_STATE();
    case 74:
      if (lookahead == 'd') ADVANCE(434);
      END_STATE();
    case 75:
      if (lookahead == 'd') ADVANCE(393);
      END_STATE();
    case 76:
      if (lookahead == 'd') ADVANCE(394);
      END_STATE();
    case 77:
      if (lookahead == 'd') ADVANCE(246);
      END_STATE();
    case 78:
      if (lookahead == 'd') ADVANCE(113);
      if (lookahead == 'g') ADVANCE(224);
      if (lookahead == 'n') ADVANCE(59);
      if (lookahead == 't') ADVANCE(25);
      END_STATE();
    case 79:
      if (lookahead == 'd') ADVANCE(93);
      END_STATE();
    case 80:
      if (lookahead == 'd') ADVANCE(176);
      if (lookahead == 's') ADVANCE(324);
      END_STATE();
    case 81:
      if (lookahead == 'd') ADVANCE(165);
      END_STATE();
    case 82:
      if (lookahead == 'd') ADVANCE(120);
      END_STATE();
    case 83:
      if (lookahead == 'e') ADVANCE(345);
      END_STATE();
    case 84:
      if (lookahead == 'e') ADVANCE(71);
      END_STATE();
    case 85:
      if (lookahead == 'e') ADVANCE(186);
      END_STATE();
    case 86:
      if (lookahead == 'e') ADVANCE(116);
      END_STATE();
    case 87:
      if (lookahead == 'e') ADVANCE(62);
      if (lookahead == 'o') ADVANCE(204);
      END_STATE();
    case 88:
      if (lookahead == 'e') ADVANCE(403);
      END_STATE();
    case 89:
      if (lookahead == 'e') ADVANCE(441);
      END_STATE();
    case 90:
      if (lookahead == 'e') ADVANCE(415);
      END_STATE();
    case 91:
      if (lookahead == 'e') ADVANCE(438);
      END_STATE();
    case 92:
      if (lookahead == 'e') ADVANCE(451);
      END_STATE();
    case 93:
      if (lookahead == 'e') ADVANCE(437);
      END_STATE();
    case 94:
      if (lookahead == 'e') ADVANCE(383);
      END_STATE();
    case 95:
      if (lookahead == 'e') ADVANCE(404);
      END_STATE();
    case 96:
      if (lookahead == 'e') ADVANCE(457);
      END_STATE();
    case 97:
      if (lookahead == 'e') ADVANCE(377);
      END_STATE();
    case 98:
      if (lookahead == 'e') ADVANCE(405);
      END_STATE();
    case 99:
      if (lookahead == 'e') ADVANCE(406);
      END_STATE();
    case 100:
      if (lookahead == 'e') ADVANCE(443);
      END_STATE();
    case 101:
      if (lookahead == 'e') ADVANCE(382);
      END_STATE();
    case 102:
      if (lookahead == 'e') ADVANCE(456);
      END_STATE();
    case 103:
      if (lookahead == 'e') ADVANCE(267);
      END_STATE();
    case 104:
      if (lookahead == 'e') ADVANCE(122);
      END_STATE();
    case 105:
      if (lookahead == 'e') ADVANCE(7);
      END_STATE();
    case 106:
      if (lookahead == 'e') ADVANCE(142);
      END_STATE();
    case 107:
      if (lookahead == 'e') ADVANCE(125);
      END_STATE();
    case 108:
      if (lookahead == 'e') ADVANCE(73);
      END_STATE();
    case 109:
      if (lookahead == 'e') ADVANCE(126);
      END_STATE();
    case 110:
      if (lookahead == 'e') ADVANCE(8);
      END_STATE();
    case 111:
      if (lookahead == 'e') ADVANCE(60);
      END_STATE();
    case 112:
      if (lookahead == 'e') ADVANCE(179);
      END_STATE();
    case 113:
      if (lookahead == 'e') ADVANCE(228);
      END_STATE();
    case 114:
      if (lookahead == 'e') ADVANCE(75);
      END_STATE();
    case 115:
      if (lookahead == 'e') ADVANCE(294);
      END_STATE();
    case 116:
      if (lookahead == 'e') ADVANCE(212);
      if (lookahead == 'y') ADVANCE(416);
      END_STATE();
    case 117:
      if (lookahead == 'e') ADVANCE(76);
      END_STATE();
    case 118:
      if (lookahead == 'e') ADVANCE(70);
      END_STATE();
    case 119:
      if (lookahead == 'e') ADVANCE(281);
      END_STATE();
    case 120:
      if (lookahead == 'e') ADVANCE(288);
      END_STATE();
    case 121:
      if (lookahead == 'e') ADVANCE(274);
      END_STATE();
    case 122:
      if (lookahead == 'e') ADVANCE(218);
      if (lookahead == 'y') ADVANCE(388);
      END_STATE();
    case 123:
      if (lookahead == 'e') ADVANCE(277);
      END_STATE();
    case 124:
      if (lookahead == 'e') ADVANCE(278);
      END_STATE();
    case 125:
      if (lookahead == 'e') ADVANCE(220);
      if (lookahead == 'y') ADVANCE(389);
      END_STATE();
    case 126:
      if (lookahead == 'e') ADVANCE(221);
      if (lookahead == 'y') ADVANCE(390);
      END_STATE();
    case 127:
      if (lookahead == 'e') ADVANCE(193);
      END_STATE();
    case 128:
      if (lookahead == 'e') ADVANCE(34);
      END_STATE();
    case 129:
      if (lookahead == 'e') ADVANCE(229);
      END_STATE();
    case 130:
      if (lookahead == 'e') ADVANCE(36);
      END_STATE();
    case 131:
      if (lookahead == 'e') ADVANCE(68);
      END_STATE();
    case 132:
      if (lookahead == 'e') ADVANCE(231);
      END_STATE();
    case 133:
      if (lookahead == 'e') ADVANCE(234);
      END_STATE();
    case 134:
      if (lookahead == 'e') ADVANCE(241);
      END_STATE();
    case 135:
      if (lookahead == 'e') ADVANCE(244);
      END_STATE();
    case 136:
      if (lookahead == 'e') ADVANCE(245);
      END_STATE();
    case 137:
      if (lookahead == 'e') ADVANCE(236);
      END_STATE();
    case 138:
      if (lookahead == 'e') ADVANCE(195);
      END_STATE();
    case 139:
      if (lookahead == 'e') ADVANCE(197);
      END_STATE();
    case 140:
      if (lookahead == 'e') ADVANCE(198);
      END_STATE();
    case 141:
      if (lookahead == 'e') ADVANCE(207);
      if (lookahead == 'u') ADVANCE(299);
      END_STATE();
    case 142:
      if (lookahead == 'f') ADVANCE(444);
      END_STATE();
    case 143:
      if (lookahead == 'f') ADVANCE(175);
      END_STATE();
    case 144:
      if (lookahead == 'g') ADVANCE(375);
      if (lookahead == 'l') ADVANCE(24);
      if (lookahead == 'o') ADVANCE(183);
      END_STATE();
    case 145:
      if (lookahead == 'g') ADVANCE(373);
      if (lookahead == 'l') ADVANCE(251);
      if (lookahead == 'u') ADVANCE(226);
      END_STATE();
    case 146:
      if (lookahead == 'g') ADVANCE(450);
      END_STATE();
    case 147:
      if (lookahead == 'g') ADVANCE(422);
      END_STATE();
    case 148:
      if (lookahead == 'g') ADVANCE(374);
      END_STATE();
    case 149:
      if (lookahead == 'g') ADVANCE(372);
      END_STATE();
    case 150:
      if (lookahead == 'g') ADVANCE(6);
      if (lookahead == 'l') ADVANCE(24);
      END_STATE();
    case 151:
      if (lookahead == 'g') ADVANCE(110);
      END_STATE();
    case 152:
      if (lookahead == 'g') ADVANCE(129);
      END_STATE();
    case 153:
      if (lookahead == 'g') ADVANCE(134);
      END_STATE();
    case 154:
      if (lookahead == 'g') ADVANCE(135);
      END_STATE();
    case 155:
      if (lookahead == 'g') ADVANCE(136);
      END_STATE();
    case 156:
      if (lookahead == 'g') ADVANCE(9);
      END_STATE();
    case 157:
      if (lookahead == 'h') ADVANCE(160);
      END_STATE();
    case 158:
      if (lookahead == 'h') ADVANCE(40);
      if (lookahead == 'o') ADVANCE(209);
      END_STATE();
    case 159:
      if (lookahead == 'i') ADVANCE(143);
      END_STATE();
    case 160:
      if (lookahead == 'i') ADVANCE(320);
      END_STATE();
    case 161:
      if (lookahead == 'i') ADVANCE(205);
      END_STATE();
    case 162:
      if (lookahead == 'i') ADVANCE(55);
      END_STATE();
    case 163:
      if (lookahead == 'i') ADVANCE(225);
      if (lookahead == 'u') ADVANCE(67);
      END_STATE();
    case 164:
      if (lookahead == 'i') ADVANCE(255);
      END_STATE();
    case 165:
      if (lookahead == 'i') ADVANCE(308);
      END_STATE();
    case 166:
      if (lookahead == 'i') ADVANCE(325);
      END_STATE();
    case 167:
      if (lookahead == 'i') ADVANCE(32);
      END_STATE();
    case 168:
      if (lookahead == 'i') ADVANCE(235);
      END_STATE();
    case 169:
      if (lookahead == 'i') ADVANCE(257);
      END_STATE();
    case 170:
      if (lookahead == 'i') ADVANCE(237);
      END_STATE();
    case 171:
      if (lookahead == 'i') ADVANCE(259);
      END_STATE();
    case 172:
      if (lookahead == 'i') ADVANCE(238);
      END_STATE();
    case 173:
      if (lookahead == 'i') ADVANCE(260);
      END_STATE();
    case 174:
      if (lookahead == 'i') ADVANCE(239);
      END_STATE();
    case 175:
      if (lookahead == 'i') ADVANCE(124);
      END_STATE();
    case 176:
      if (lookahead == 'i') ADVANCE(326);
      END_STATE();
    case 177:
      if (lookahead == 'k') ADVANCE(5);
      END_STATE();
    case 178:
      if (lookahead == 'k') ADVANCE(387);
      END_STATE();
    case 179:
      if (lookahead == 'l') ADVANCE(432);
      END_STATE();
    case 180:
      if (lookahead == 'l') ADVANCE(445);
      END_STATE();
    case 181:
      if (lookahead == 'l') ADVANCE(430);
      END_STATE();
    case 182:
      if (lookahead == 'l') ADVANCE(334);
      END_STATE();
    case 183:
      if (lookahead == 'l') ADVANCE(72);
      if (lookahead == 'o') ADVANCE(192);
      END_STATE();
    case 184:
      if (lookahead == 'l') ADVANCE(162);
      END_STATE();
    case 185:
      if (lookahead == 'l') ADVANCE(331);
      END_STATE();
    case 186:
      if (lookahead == 'l') ADVANCE(187);
      END_STATE();
    case 187:
      if (lookahead == 'l') ADVANCE(248);
      END_STATE();
    case 188:
      if (lookahead == 'l') ADVANCE(249);
      END_STATE();
    case 189:
      if (lookahead == 'l') ADVANCE(250);
      END_STATE();
    case 190:
      if (lookahead == 'l') ADVANCE(252);
      END_STATE();
    case 191:
      if (lookahead == 'l') ADVANCE(31);
      END_STATE();
    case 192:
      if (lookahead == 'l') ADVANCE(128);
      END_STATE();
    case 193:
      if (lookahead == 'l') ADVANCE(131);
      END_STATE();
    case 194:
      if (lookahead == 'l') ADVANCE(332);
      END_STATE();
    case 195:
      if (lookahead == 'l') ADVANCE(188);
      END_STATE();
    case 196:
      if (lookahead == 'l') ADVANCE(333);
      END_STATE();
    case 197:
      if (lookahead == 'l') ADVANCE(189);
      END_STATE();
    case 198:
      if (lookahead == 'l') ADVANCE(190);
      END_STATE();
    case 199:
      if (lookahead == 'l') ADVANCE(170);
      if (lookahead == 'o') ADVANCE(268);
      if (lookahead == 's') ADVANCE(127);
      END_STATE();
    case 200:
      if (lookahead == 'l') ADVANCE(172);
      END_STATE();
    case 201:
      if (lookahead == 'l') ADVANCE(174);
      END_STATE();
    case 202:
      if (lookahead == 'm') ADVANCE(206);
      if (lookahead == 'n') ADVANCE(80);
      END_STATE();
    case 203:
      if (lookahead == 'm') ADVANCE(54);
      END_STATE();
    case 204:
      if (lookahead == 'm') ADVANCE(265);
      END_STATE();
    case 205:
      if (lookahead == 'm') ADVANCE(166);
      END_STATE();
    case 206:
      if (lookahead == 'm') ADVANCE(132);
      END_STATE();
    case 207:
      if (lookahead == 'm') ADVANCE(133);
      END_STATE();
    case 208:
      if (lookahead == 'm') ADVANCE(137);
      END_STATE();
    case 209:
      if (lookahead == 'm') ADVANCE(208);
      END_STATE();
    case 210:
      if (lookahead == 'n') ADVANCE(337);
      END_STATE();
    case 211:
      if (lookahead == 'n') ADVANCE(411);
      END_STATE();
    case 212:
      if (lookahead == 'n') ADVANCE(395);
      END_STATE();
    case 213:
      if (lookahead == 'n') ADVANCE(425);
      END_STATE();
    case 214:
      if (lookahead == 'n') ADVANCE(412);
      END_STATE();
    case 215:
      if (lookahead == 'n') ADVANCE(428);
      END_STATE();
    case 216:
      if (lookahead == 'n') ADVANCE(413);
      END_STATE();
    case 217:
      if (lookahead == 'n') ADVANCE(414);
      END_STATE();
    case 218:
      if (lookahead == 'n') ADVANCE(396);
      END_STATE();
    case 219:
      if (lookahead == 'n') ADVANCE(435);
      END_STATE();
    case 220:
      if (lookahead == 'n') ADVANCE(397);
      END_STATE();
    case 221:
      if (lookahead == 'n') ADVANCE(398);
      END_STATE();
    case 222:
      if (lookahead == 'n') ADVANCE(458);
      END_STATE();
    case 223:
      if (lookahead == 'n') ADVANCE(82);
      END_STATE();
    case 224:
      if (lookahead == 'n') ADVANCE(261);
      END_STATE();
    case 225:
      if (lookahead == 'n') ADVANCE(147);
      END_STATE();
    case 226:
      if (lookahead == 'n') ADVANCE(63);
      END_STATE();
    case 227:
      if (lookahead == 'n') ADVANCE(81);
      END_STATE();
    case 228:
      if (lookahead == 'n') ADVANCE(316);
      END_STATE();
    case 229:
      if (lookahead == 'n') ADVANCE(317);
      END_STATE();
    case 230:
      if (lookahead == 'n') ADVANCE(105);
      END_STATE();
    case 231:
      if (lookahead == 'n') ADVANCE(306);
      END_STATE();
    case 232:
      if (lookahead == 'n') ADVANCE(307);
      END_STATE();
    case 233:
      if (lookahead == 'n') ADVANCE(275);
      END_STATE();
    case 234:
      if (lookahead == 'n') ADVANCE(309);
      END_STATE();
    case 235:
      if (lookahead == 'n') ADVANCE(91);
      END_STATE();
    case 236:
      if (lookahead == 'n') ADVANCE(311);
      END_STATE();
    case 237:
      if (lookahead == 'n') ADVANCE(96);
      END_STATE();
    case 238:
      if (lookahead == 'n') ADVANCE(101);
      END_STATE();
    case 239:
      if (lookahead == 'n') ADVANCE(102);
      END_STATE();
    case 240:
      if (lookahead == 'n') ADVANCE(338);
      END_STATE();
    case 241:
      if (lookahead == 'n') ADVANCE(318);
      END_STATE();
    case 242:
      if (lookahead == 'n') ADVANCE(339);
      END_STATE();
    case 243:
      if (lookahead == 'n') ADVANCE(35);
      END_STATE();
    case 244:
      if (lookahead == 'n') ADVANCE(319);
      END_STATE();
    case 245:
      if (lookahead == 'n') ADVANCE(321);
      END_STATE();
    case 246:
      if (lookahead == 'o') ADVANCE(453);
      END_STATE();
    case 247:
      if (lookahead == 'o') ADVANCE(439);
      END_STATE();
    case 248:
      if (lookahead == 'o') ADVANCE(340);
      END_STATE();
    case 249:
      if (lookahead == 'o') ADVANCE(341);
      END_STATE();
    case 250:
      if (lookahead == 'o') ADVANCE(342);
      END_STATE();
    case 251:
      if (lookahead == 'o') ADVANCE(28);
      END_STATE();
    case 252:
      if (lookahead == 'o') ADVANCE(343);
      END_STATE();
    case 253:
      if (lookahead == 'o') ADVANCE(56);
      END_STATE();
    case 254:
      if (lookahead == 'o') ADVANCE(273);
      END_STATE();
    case 255:
      if (lookahead == 'o') ADVANCE(215);
      END_STATE();
    case 256:
      if (lookahead == 'o') ADVANCE(285);
      END_STATE();
    case 257:
      if (lookahead == 'o') ADVANCE(243);
      END_STATE();
    case 258:
      if (lookahead == 'o') ADVANCE(276);
      END_STATE();
    case 259:
      if (lookahead == 'o') ADVANCE(219);
      END_STATE();
    case 260:
      if (lookahead == 'o') ADVANCE(222);
      END_STATE();
    case 261:
      if (lookahead == 'o') ADVANCE(289);
      END_STATE();
    case 262:
      if (lookahead == 'o') ADVANCE(227);
      END_STATE();
    case 263:
      if (lookahead == 'p') ADVANCE(115);
      END_STATE();
    case 264:
      if (lookahead == 'p') ADVANCE(111);
      if (lookahead == 't') ADVANCE(21);
      END_STATE();
    case 265:
      if (lookahead == 'p') ADVANCE(304);
      END_STATE();
    case 266:
      if (lookahead == 'p') ADVANCE(89);
      END_STATE();
    case 267:
      if (lookahead == 'p') ADVANCE(327);
      END_STATE();
    case 268:
      if (lookahead == 'p') ADVANCE(328);
      END_STATE();
    case 269:
      if (lookahead == 'r') ADVANCE(87);
      END_STATE();
    case 270:
      if (lookahead == 'r') ADVANCE(177);
      END_STATE();
    case 271:
      if (lookahead == 'r') ADVANCE(376);
      END_STATE();
    case 272:
      if (lookahead == 'r') ADVANCE(423);
      END_STATE();
    case 273:
      if (lookahead == 'r') ADVANCE(452);
      END_STATE();
    case 274:
      if (lookahead == 'r') ADVANCE(424);
      END_STATE();
    case 275:
      if (lookahead == 'r') ADVANCE(454);
      END_STATE();
    case 276:
      if (lookahead == 'r') ADVANCE(433);
      END_STATE();
    case 277:
      if (lookahead == 'r') ADVANCE(448);
      END_STATE();
    case 278:
      if (lookahead == 'r') ADVANCE(427);
      END_STATE();
    case 279:
      if (lookahead == 'r') ADVANCE(446);
      END_STATE();
    case 280:
      if (lookahead == 'r') ADVANCE(254);
      END_STATE();
    case 281:
      if (lookahead == 'r') ADVANCE(301);
      END_STATE();
    case 282:
      if (lookahead == 'r') ADVANCE(280);
      if (lookahead == 'x') ADVANCE(65);
      END_STATE();
    case 283:
      if (lookahead == 'r') ADVANCE(86);
      END_STATE();
    case 284:
      if (lookahead == 'r') ADVANCE(247);
      END_STATE();
    case 285:
      if (lookahead == 'r') ADVANCE(74);
      END_STATE();
    case 286:
      if (lookahead == 'r') ADVANCE(20);
      END_STATE();
    case 287:
      if (lookahead == 'r') ADVANCE(253);
      END_STATE();
    case 288:
      if (lookahead == 'r') ADVANCE(200);
      END_STATE();
    case 289:
      if (lookahead == 'r') ADVANCE(92);
      END_STATE();
    case 290:
      if (lookahead == 'r') ADVANCE(104);
      END_STATE();
    case 291:
      if (lookahead == 'r') ADVANCE(107);
      END_STATE();
    case 292:
      if (lookahead == 'r') ADVANCE(109);
      END_STATE();
    case 293:
      if (lookahead == 'r') ADVANCE(100);
      END_STATE();
    case 294:
      if (lookahead == 'r') ADVANCE(33);
      END_STATE();
    case 295:
      if (lookahead == 's') ADVANCE(419);
      END_STATE();
    case 296:
      if (lookahead == 's') ADVANCE(417);
      END_STATE();
    case 297:
      if (lookahead == 's') ADVANCE(418);
      END_STATE();
    case 298:
      if (lookahead == 's') ADVANCE(442);
      END_STATE();
    case 299:
      if (lookahead == 's') ADVANCE(10);
      END_STATE();
    case 300:
      if (lookahead == 's') ADVANCE(298);
      END_STATE();
    case 301:
      if (lookahead == 's') ADVANCE(94);
      END_STATE();
    case 302:
      if (lookahead == 't') ADVANCE(141);
      END_STATE();
    case 303:
      if (lookahead == 't') ADVANCE(426);
      END_STATE();
    case 304:
      if (lookahead == 't') ADVANCE(455);
      END_STATE();
    case 305:
      if (lookahead == 't') ADVANCE(431);
      END_STATE();
    case 306:
      if (lookahead == 't') ADVANCE(420);
      END_STATE();
    case 307:
      if (lookahead == 't') ADVANCE(421);
      END_STATE();
    case 308:
      if (lookahead == 't') ADVANCE(440);
      END_STATE();
    case 309:
      if (lookahead == 't') ADVANCE(429);
      END_STATE();
    case 310:
      if (lookahead == 't') ADVANCE(459);
      END_STATE();
    case 311:
      if (lookahead == 't') ADVANCE(449);
      END_STATE();
    case 312:
      if (lookahead == 't') ADVANCE(313);
      END_STATE();
    case 313:
      if (lookahead == 't') ADVANCE(271);
      END_STATE();
    case 314:
      if (lookahead == 't') ADVANCE(336);
      END_STATE();
    case 315:
      if (lookahead == 't') ADVANCE(164);
      END_STATE();
    case 316:
      if (lookahead == 't') ADVANCE(159);
      END_STATE();
    case 317:
      if (lookahead == 't') ADVANCE(15);
      END_STATE();
    case 318:
      if (lookahead == 't') ADVANCE(16);
      END_STATE();
    case 319:
      if (lookahead == 't') ADVANCE(17);
      END_STATE();
    case 320:
      if (lookahead == 't') ADVANCE(90);
      END_STATE();
    case 321:
      if (lookahead == 't') ADVANCE(18);
      END_STATE();
    case 322:
      if (lookahead == 't') ADVANCE(97);
      END_STATE();
    case 323:
      if (lookahead == 't') ADVANCE(258);
      END_STATE();
    case 324:
      if (lookahead == 't') ADVANCE(44);
      END_STATE();
    case 325:
      if (lookahead == 't') ADVANCE(123);
      END_STATE();
    case 326:
      if (lookahead == 't') ADVANCE(169);
      END_STATE();
    case 327:
      if (lookahead == 't') ADVANCE(171);
      END_STATE();
    case 328:
      if (lookahead == 't') ADVANCE(173);
      END_STATE();
    case 329:
      if (lookahead == 'u') ADVANCE(203);
      END_STATE();
    case 330:
      if (lookahead == 'u') ADVANCE(146);
      END_STATE();
    case 331:
      if (lookahead == 'u') ADVANCE(95);
      END_STATE();
    case 332:
      if (lookahead == 'u') ADVANCE(98);
      END_STATE();
    case 333:
      if (lookahead == 'u') ADVANCE(99);
      END_STATE();
    case 334:
      if (lookahead == 'u') ADVANCE(79);
      END_STATE();
    case 335:
      if (lookahead == 'u') ADVANCE(322);
      END_STATE();
    case 336:
      if (lookahead == 'u') ADVANCE(293);
      END_STATE();
    case 337:
      if (lookahead == 'v') ADVANCE(14);
      END_STATE();
    case 338:
      if (lookahead == 'v') ADVANCE(26);
      END_STATE();
    case 339:
      if (lookahead == 'v') ADVANCE(27);
      END_STATE();
    case 340:
      if (lookahead == 'w') ADVANCE(399);
      END_STATE();
    case 341:
      if (lookahead == 'w') ADVANCE(400);
      END_STATE();
    case 342:
      if (lookahead == 'w') ADVANCE(401);
      END_STATE();
    case 343:
      if (lookahead == 'w') ADVANCE(402);
      END_STATE();
    case 344:
      if (lookahead == 'w') ADVANCE(256);
      END_STATE();
    case 345:
      if (lookahead == 'y') ADVANCE(344);
      END_STATE();
    case 346:
      if (lookahead == 'y') ADVANCE(30);
      END_STATE();
    case 347:
      if (lookahead == 'y') ADVANCE(37);
      END_STATE();
    case 348:
      if (lookahead == 'y') ADVANCE(38);
      END_STATE();
    case 349:
      if (lookahead == 'y') ADVANCE(39);
      END_STATE();
    case 350:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(386);
      END_STATE();
    case 351:
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(365);
      END_STATE();
    case 352:
      if (eof) ADVANCE(353);
      if (lookahead == '\n') ADVANCE(363);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '#') ADVANCE(362);
      if (lookahead == '+') ADVANCE(367);
      if (lookahead == ',') ADVANCE(364);
      if (lookahead == '.') ADVANCE(366);
      if (lookahead == ':') ADVANCE(354);
      if (lookahead == '>') ADVANCE(369);
      if (lookahead == '~') ADVANCE(368);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(352)
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(351);
      END_STATE();
    case 353:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 354:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 355:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 356:
      ACCEPT_TOKEN(sym_comment);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(362);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(362);
      END_STATE();
    case 357:
      ACCEPT_TOKEN(sym_comment);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(356);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(362);
      END_STATE();
    case 358:
      ACCEPT_TOKEN(sym_comment);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(357);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(362);
      END_STATE();
    case 359:
      ACCEPT_TOKEN(sym_comment);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(358);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(362);
      END_STATE();
    case 360:
      ACCEPT_TOKEN(sym_comment);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(359);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(362);
      END_STATE();
    case 361:
      ACCEPT_TOKEN(sym_comment);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(360);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(362);
      END_STATE();
    case 362:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(362);
      END_STATE();
    case 363:
      ACCEPT_TOKEN(sym_newline);
      END_STATE();
    case 364:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 365:
      ACCEPT_TOKEN(aux_sym_sel_kind_token1);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(365);
      END_STATE();
    case 366:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 367:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 368:
      ACCEPT_TOKEN(anon_sym_TILDE);
      END_STATE();
    case 369:
      ACCEPT_TOKEN(anon_sym_GT);
      END_STATE();
    case 370:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 371:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 372:
      ACCEPT_TOKEN(anon_sym_fg);
      END_STATE();
    case 373:
      ACCEPT_TOKEN(anon_sym_fg);
      if (lookahead == '-') ADVANCE(69);
      END_STATE();
    case 374:
      ACCEPT_TOKEN(anon_sym_bg);
      END_STATE();
    case 375:
      ACCEPT_TOKEN(anon_sym_bg);
      if (lookahead == '-') ADVANCE(61);
      END_STATE();
    case 376:
      ACCEPT_TOKEN(anon_sym_attr);
      if (lookahead == 'i') ADVANCE(50);
      END_STATE();
    case 377:
      ACCEPT_TOKEN(anon_sym_attribute);
      END_STATE();
    case 378:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 379:
      ACCEPT_TOKEN(anon_sym_bold);
      END_STATE();
    case 380:
      ACCEPT_TOKEN(anon_sym_italic);
      END_STATE();
    case 381:
      ACCEPT_TOKEN(anon_sym_underlined);
      END_STATE();
    case 382:
      ACCEPT_TOKEN(anon_sym_underline);
      if (lookahead == 'd') ADVANCE(381);
      END_STATE();
    case 383:
      ACCEPT_TOKEN(anon_sym_reverse);
      END_STATE();
    case 384:
      ACCEPT_TOKEN(aux_sym_ansi_color_token1);
      if (lookahead == 'x') ADVANCE(350);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(385);
      END_STATE();
    case 385:
      ACCEPT_TOKEN(aux_sym_ansi_color_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(385);
      END_STATE();
    case 386:
      ACCEPT_TOKEN(aux_sym_ansi_color_token2);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(386);
      END_STATE();
    case 387:
      ACCEPT_TOKEN(anon_sym_black);
      END_STATE();
    case 388:
      ACCEPT_TOKEN(anon_sym_darkgrey);
      END_STATE();
    case 389:
      ACCEPT_TOKEN(anon_sym_dark_DASHgrey);
      END_STATE();
    case 390:
      ACCEPT_TOKEN(anon_sym_dark_grey);
      END_STATE();
    case 391:
      ACCEPT_TOKEN(anon_sym_red);
      END_STATE();
    case 392:
      ACCEPT_TOKEN(anon_sym_darkred);
      END_STATE();
    case 393:
      ACCEPT_TOKEN(anon_sym_dark_DASHred);
      END_STATE();
    case 394:
      ACCEPT_TOKEN(anon_sym_dark_red);
      END_STATE();
    case 395:
      ACCEPT_TOKEN(anon_sym_green);
      END_STATE();
    case 396:
      ACCEPT_TOKEN(anon_sym_darkgreen);
      END_STATE();
    case 397:
      ACCEPT_TOKEN(anon_sym_dark_DASHgreen);
      END_STATE();
    case 398:
      ACCEPT_TOKEN(anon_sym_dark_green);
      END_STATE();
    case 399:
      ACCEPT_TOKEN(anon_sym_yellow);
      END_STATE();
    case 400:
      ACCEPT_TOKEN(anon_sym_darkyellow);
      END_STATE();
    case 401:
      ACCEPT_TOKEN(anon_sym_dark_DASHyellow);
      END_STATE();
    case 402:
      ACCEPT_TOKEN(anon_sym_dark_yellow);
      END_STATE();
    case 403:
      ACCEPT_TOKEN(anon_sym_blue);
      END_STATE();
    case 404:
      ACCEPT_TOKEN(anon_sym_darkblue);
      END_STATE();
    case 405:
      ACCEPT_TOKEN(anon_sym_dark_DASHblue);
      END_STATE();
    case 406:
      ACCEPT_TOKEN(anon_sym_dark_blue);
      END_STATE();
    case 407:
      ACCEPT_TOKEN(anon_sym_magenta);
      END_STATE();
    case 408:
      ACCEPT_TOKEN(anon_sym_darkmagenta);
      END_STATE();
    case 409:
      ACCEPT_TOKEN(anon_sym_dark_DASHmagenta);
      END_STATE();
    case 410:
      ACCEPT_TOKEN(anon_sym_dark_magenta);
      END_STATE();
    case 411:
      ACCEPT_TOKEN(anon_sym_cyan);
      END_STATE();
    case 412:
      ACCEPT_TOKEN(anon_sym_darkcyan);
      END_STATE();
    case 413:
      ACCEPT_TOKEN(anon_sym_dark_DASHcyan);
      END_STATE();
    case 414:
      ACCEPT_TOKEN(anon_sym_dark_cyan);
      END_STATE();
    case 415:
      ACCEPT_TOKEN(anon_sym_white);
      END_STATE();
    case 416:
      ACCEPT_TOKEN(anon_sym_grey);
      END_STATE();
    case 417:
      ACCEPT_TOKEN(anon_sym_bg_DASHcanvas);
      END_STATE();
    case 418:
      ACCEPT_TOKEN(anon_sym_fg_DASHcanvas);
      END_STATE();
    case 419:
      ACCEPT_TOKEN(anon_sym_canvas);
      END_STATE();
    case 420:
      ACCEPT_TOKEN(anon_sym_comment);
      END_STATE();
    case 421:
      ACCEPT_TOKEN(anon_sym_constant);
      END_STATE();
    case 422:
      ACCEPT_TOKEN(anon_sym_string);
      END_STATE();
    case 423:
      ACCEPT_TOKEN(anon_sym_char);
      END_STATE();
    case 424:
      ACCEPT_TOKEN(anon_sym_number);
      END_STATE();
    case 425:
      ACCEPT_TOKEN(anon_sym_boolean);
      END_STATE();
    case 426:
      ACCEPT_TOKEN(anon_sym_float);
      END_STATE();
    case 427:
      ACCEPT_TOKEN(anon_sym_identifier);
      END_STATE();
    case 428:
      ACCEPT_TOKEN(anon_sym_function);
      END_STATE();
    case 429:
      ACCEPT_TOKEN(anon_sym_statement);
      END_STATE();
    case 430:
      ACCEPT_TOKEN(anon_sym_conditional);
      END_STATE();
    case 431:
      ACCEPT_TOKEN(anon_sym_repeat);
      END_STATE();
    case 432:
      ACCEPT_TOKEN(anon_sym_label);
      END_STATE();
    case 433:
      ACCEPT_TOKEN(anon_sym_operator);
      END_STATE();
    case 434:
      ACCEPT_TOKEN(anon_sym_keyword);
      END_STATE();
    case 435:
      ACCEPT_TOKEN(anon_sym_exception);
      END_STATE();
    case 436:
      ACCEPT_TOKEN(anon_sym_preproc);
      END_STATE();
    case 437:
      ACCEPT_TOKEN(anon_sym_include);
      END_STATE();
    case 438:
      ACCEPT_TOKEN(anon_sym_define);
      END_STATE();
    case 439:
      ACCEPT_TOKEN(anon_sym_macro);
      END_STATE();
    case 440:
      ACCEPT_TOKEN(anon_sym_precondit);
      END_STATE();
    case 441:
      ACCEPT_TOKEN(anon_sym_type);
      if (lookahead == 'd') ADVANCE(106);
      END_STATE();
    case 442:
      ACCEPT_TOKEN(anon_sym_storage_DASHclass);
      END_STATE();
    case 443:
      ACCEPT_TOKEN(anon_sym_structure);
      END_STATE();
    case 444:
      ACCEPT_TOKEN(anon_sym_typedef);
      END_STATE();
    case 445:
      ACCEPT_TOKEN(anon_sym_special);
      if (lookahead == '-') ADVANCE(57);
      END_STATE();
    case 446:
      ACCEPT_TOKEN(anon_sym_special_DASHchar);
      END_STATE();
    case 447:
      ACCEPT_TOKEN(anon_sym_tag);
      END_STATE();
    case 448:
      ACCEPT_TOKEN(anon_sym_delimiter);
      END_STATE();
    case 449:
      ACCEPT_TOKEN(anon_sym_special_DASHcomment);
      END_STATE();
    case 450:
      ACCEPT_TOKEN(anon_sym_debug);
      END_STATE();
    case 451:
      ACCEPT_TOKEN(anon_sym_ignore);
      END_STATE();
    case 452:
      ACCEPT_TOKEN(anon_sym_error);
      END_STATE();
    case 453:
      ACCEPT_TOKEN(anon_sym_todo);
      END_STATE();
    case 454:
      ACCEPT_TOKEN(anon_sym_line_DASHnr);
      END_STATE();
    case 455:
      ACCEPT_TOKEN(anon_sym_prompt);
      END_STATE();
    case 456:
      ACCEPT_TOKEN(anon_sym_status_DASHline);
      END_STATE();
    case 457:
      ACCEPT_TOKEN(anon_sym_tab_DASHline);
      END_STATE();
    case 458:
      ACCEPT_TOKEN(anon_sym_tab_DASHoption);
      END_STATE();
    case 459:
      ACCEPT_TOKEN(anon_sym_tab_DASHselect);
      END_STATE();
    default:
      return false;
  }
}

static TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 352},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 2},
  [4] = {.lex_state = 2},
  [5] = {.lex_state = 352},
  [6] = {.lex_state = 352},
  [7] = {.lex_state = 352},
  [8] = {.lex_state = 352},
  [9] = {.lex_state = 352},
  [10] = {.lex_state = 3},
  [11] = {.lex_state = 3},
  [12] = {.lex_state = 3},
  [13] = {.lex_state = 352},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 352},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 352},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 0},
  [22] = {.lex_state = 0},
  [23] = {.lex_state = 0},
  [24] = {.lex_state = 352},
  [25] = {.lex_state = 352},
  [26] = {.lex_state = 0},
  [27] = {.lex_state = 352},
  [28] = {.lex_state = 352},
  [29] = {.lex_state = 352},
  [30] = {.lex_state = 352},
  [31] = {.lex_state = 352},
  [32] = {.lex_state = 352},
  [33] = {.lex_state = 352},
  [34] = {.lex_state = 0},
  [35] = {.lex_state = 352},
  [36] = {.lex_state = 0},
  [37] = {.lex_state = 3},
  [38] = {.lex_state = 0},
  [39] = {.lex_state = 0},
  [40] = {.lex_state = 352},
  [41] = {.lex_state = 0},
  [42] = {.lex_state = 352},
  [43] = {.lex_state = 352},
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
    [sym_comment] = ACTIONS(3),
    [sym_newline] = ACTIONS(3),
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
    [anon_sym_bg_DASHcanvas] = ACTIONS(1),
    [anon_sym_fg_DASHcanvas] = ACTIONS(1),
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
    [ts_builtin_sym_end] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [sym_newline] = ACTIONS(3),
    [aux_sym_sel_kind_token1] = ACTIONS(7),
    [anon_sym_DOT] = ACTIONS(9),
  },
};

static uint16_t ts_small_parse_table[] = {
  [0] = 5,
    ACTIONS(11), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(15), 2,
      anon_sym_type,
      anon_sym_special,
    STATE(58), 2,
      sym_properties,
      sym_highlight,
    ACTIONS(13), 40,
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
  [58] = 6,
    ACTIONS(17), 1,
      sym_rgb_color,
    ACTIONS(19), 1,
      aux_sym_ansi_color_token1,
    ACTIONS(21), 1,
      aux_sym_ansi_color_token2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(55), 2,
      sym_ansi_color,
      sym_color_name,
    ACTIONS(23), 32,
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
      anon_sym_bg_DASHcanvas,
      anon_sym_fg_DASHcanvas,
  [110] = 6,
    ACTIONS(19), 1,
      aux_sym_ansi_color_token1,
    ACTIONS(21), 1,
      aux_sym_ansi_color_token2,
    ACTIONS(25), 1,
      sym_rgb_color,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(51), 2,
      sym_ansi_color,
      sym_color_name,
    ACTIONS(23), 32,
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
      anon_sym_bg_DASHcanvas,
      anon_sym_fg_DASHcanvas,
  [162] = 11,
    ACTIONS(27), 1,
      ts_builtin_sym_end,
    ACTIONS(29), 1,
      aux_sym_sel_kind_token1,
    ACTIONS(32), 1,
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
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(5), 2,
      sym_hl_rule,
      aux_sym_s_repeat1,
    STATE(32), 5,
      sym_sel_field,
      sym_sel_kind_field,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [202] = 11,
    ACTIONS(7), 1,
      aux_sym_sel_kind_token1,
    ACTIONS(9), 1,
      anon_sym_DOT,
    ACTIONS(35), 1,
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
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(5), 2,
      sym_hl_rule,
      aux_sym_s_repeat1,
    STATE(32), 5,
      sym_sel_field,
      sym_sel_kind_field,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [242] = 8,
    ACTIONS(39), 1,
      aux_sym_sel_kind_token1,
    ACTIONS(42), 1,
      anon_sym_DOT,
    STATE(7), 1,
      aux_sym_selector_repeat1,
    STATE(25), 1,
      sym_sel_kind,
    STATE(27), 1,
      sym_sel_symbol,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(37), 2,
      anon_sym_COLON,
      anon_sym_COMMA,
    STATE(32), 5,
      sym_sel_field,
      sym_sel_kind_field,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [273] = 8,
    ACTIONS(7), 1,
      aux_sym_sel_kind_token1,
    ACTIONS(9), 1,
      anon_sym_DOT,
    STATE(7), 1,
      aux_sym_selector_repeat1,
    STATE(25), 1,
      sym_sel_kind,
    STATE(27), 1,
      sym_sel_symbol,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(45), 2,
      anon_sym_COLON,
      anon_sym_COMMA,
    STATE(32), 5,
      sym_sel_field,
      sym_sel_kind_field,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [304] = 8,
    ACTIONS(7), 1,
      aux_sym_sel_kind_token1,
    ACTIONS(9), 1,
      anon_sym_DOT,
    STATE(8), 1,
      aux_sym_selector_repeat1,
    STATE(25), 1,
      sym_sel_kind,
    STATE(27), 1,
      sym_sel_symbol,
    STATE(44), 1,
      sym_selector,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(32), 5,
      sym_sel_field,
      sym_sel_kind_field,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [334] = 12,
    ACTIONS(47), 1,
      anon_sym_RBRACE,
    ACTIONS(49), 1,
      anon_sym_fg,
    ACTIONS(51), 1,
      anon_sym_bg,
    ACTIONS(53), 1,
      anon_sym_attr,
    ACTIONS(55), 1,
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
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [372] = 12,
    ACTIONS(57), 1,
      anon_sym_RBRACE,
    ACTIONS(59), 1,
      anon_sym_fg,
    ACTIONS(62), 1,
      anon_sym_bg,
    ACTIONS(65), 1,
      anon_sym_attr,
    ACTIONS(68), 1,
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
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [410] = 12,
    ACTIONS(49), 1,
      anon_sym_fg,
    ACTIONS(51), 1,
      anon_sym_bg,
    ACTIONS(53), 1,
      anon_sym_attr,
    ACTIONS(55), 1,
      anon_sym_attribute,
    ACTIONS(71), 1,
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
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [448] = 6,
    ACTIONS(7), 1,
      aux_sym_sel_kind_token1,
    ACTIONS(9), 1,
      anon_sym_DOT,
    STATE(25), 1,
      sym_sel_kind,
    STATE(29), 1,
      sym_sel_symbol,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(32), 5,
      sym_sel_field,
      sym_sel_kind_field,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [472] = 6,
    ACTIONS(73), 1,
      anon_sym_COMMA,
    ACTIONS(77), 1,
      anon_sym_underline,
    STATE(23), 1,
      sym_attr,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(16), 2,
      sym_attrs,
      aux_sym_attrb_repeat1,
    ACTIONS(75), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [496] = 5,
    ACTIONS(81), 1,
      anon_sym_PIPE,
    ACTIONS(84), 1,
      anon_sym_underline,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(15), 2,
      sym_attr_or,
      aux_sym_attrs_repeat1,
    ACTIONS(79), 5,
      anon_sym_COMMA,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [518] = 6,
    ACTIONS(86), 1,
      anon_sym_COMMA,
    ACTIONS(91), 1,
      anon_sym_underline,
    STATE(23), 1,
      sym_attr,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(16), 2,
      sym_attrs,
      aux_sym_attrb_repeat1,
    ACTIONS(88), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [542] = 6,
    ACTIONS(7), 1,
      aux_sym_sel_kind_token1,
    ACTIONS(9), 1,
      anon_sym_DOT,
    STATE(25), 1,
      sym_sel_kind,
    STATE(28), 1,
      sym_sel_symbol,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(32), 5,
      sym_sel_field,
      sym_sel_kind_field,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [566] = 6,
    ACTIONS(77), 1,
      anon_sym_underline,
    ACTIONS(94), 1,
      anon_sym_COMMA,
    STATE(23), 1,
      sym_attr,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(14), 2,
      sym_attrs,
      aux_sym_attrb_repeat1,
    ACTIONS(75), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [590] = 6,
    ACTIONS(7), 1,
      aux_sym_sel_kind_token1,
    ACTIONS(9), 1,
      anon_sym_DOT,
    STATE(25), 1,
      sym_sel_kind,
    STATE(30), 1,
      sym_sel_symbol,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(32), 5,
      sym_sel_field,
      sym_sel_kind_field,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [614] = 6,
    ACTIONS(77), 1,
      anon_sym_underline,
    ACTIONS(96), 1,
      anon_sym_COMMA,
    STATE(23), 1,
      sym_attr,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(16), 2,
      sym_attrs,
      aux_sym_attrb_repeat1,
    ACTIONS(75), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [638] = 6,
    ACTIONS(77), 1,
      anon_sym_underline,
    ACTIONS(98), 1,
      anon_sym_COMMA,
    STATE(23), 1,
      sym_attr,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(20), 2,
      sym_attrs,
      aux_sym_attrb_repeat1,
    ACTIONS(75), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [662] = 5,
    ACTIONS(102), 1,
      anon_sym_PIPE,
    ACTIONS(104), 1,
      anon_sym_underline,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(15), 2,
      sym_attr_or,
      aux_sym_attrs_repeat1,
    ACTIONS(100), 5,
      anon_sym_COMMA,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [684] = 5,
    ACTIONS(102), 1,
      anon_sym_PIPE,
    ACTIONS(108), 1,
      anon_sym_underline,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(22), 2,
      sym_attr_or,
      aux_sym_attrs_repeat1,
    ACTIONS(106), 5,
      anon_sym_COMMA,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [706] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(110), 7,
      anon_sym_COLON,
      anon_sym_COMMA,
      aux_sym_sel_kind_token1,
      anon_sym_DOT,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
  [720] = 3,
    ACTIONS(114), 1,
      anon_sym_DOT,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(112), 6,
      anon_sym_COLON,
      anon_sym_COMMA,
      aux_sym_sel_kind_token1,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
  [736] = 3,
    ACTIONS(118), 1,
      anon_sym_underline,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(116), 6,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [752] = 5,
    ACTIONS(122), 1,
      anon_sym_PLUS,
    ACTIONS(124), 1,
      anon_sym_TILDE,
    ACTIONS(126), 1,
      anon_sym_GT,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(120), 4,
      anon_sym_COLON,
      anon_sym_COMMA,
      aux_sym_sel_kind_token1,
      anon_sym_DOT,
  [772] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(128), 7,
      anon_sym_COLON,
      anon_sym_COMMA,
      aux_sym_sel_kind_token1,
      anon_sym_DOT,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
  [786] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(130), 7,
      anon_sym_COLON,
      anon_sym_COMMA,
      aux_sym_sel_kind_token1,
      anon_sym_DOT,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
  [800] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(132), 7,
      anon_sym_COLON,
      anon_sym_COMMA,
      aux_sym_sel_kind_token1,
      anon_sym_DOT,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
  [814] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(134), 7,
      anon_sym_COLON,
      anon_sym_COMMA,
      aux_sym_sel_kind_token1,
      anon_sym_DOT,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
  [828] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(112), 7,
      anon_sym_COLON,
      anon_sym_COMMA,
      aux_sym_sel_kind_token1,
      anon_sym_DOT,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
  [842] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(136), 7,
      anon_sym_COLON,
      anon_sym_COMMA,
      aux_sym_sel_kind_token1,
      anon_sym_DOT,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
  [856] = 3,
    ACTIONS(140), 1,
      anon_sym_underline,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(138), 6,
      anon_sym_COMMA,
      anon_sym_PIPE,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [872] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(142), 7,
      anon_sym_COLON,
      anon_sym_COMMA,
      aux_sym_sel_kind_token1,
      anon_sym_DOT,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
  [886] = 4,
    ACTIONS(77), 1,
      anon_sym_underline,
    STATE(34), 1,
      sym_attr,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(75), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [903] = 3,
    ACTIONS(146), 1,
      anon_sym_attr,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(144), 4,
      anon_sym_RBRACE,
      anon_sym_fg,
      anon_sym_bg,
      anon_sym_attribute,
  [917] = 4,
    ACTIONS(148), 1,
      anon_sym_COLON,
    ACTIONS(150), 1,
      anon_sym_COMMA,
    STATE(39), 1,
      aux_sym_selectors_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [931] = 4,
    ACTIONS(150), 1,
      anon_sym_COMMA,
    ACTIONS(152), 1,
      anon_sym_COLON,
    STATE(41), 1,
      aux_sym_selectors_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [945] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(154), 3,
      ts_builtin_sym_end,
      aux_sym_sel_kind_token1,
      anon_sym_DOT,
  [955] = 4,
    ACTIONS(156), 1,
      anon_sym_COLON,
    ACTIONS(158), 1,
      anon_sym_COMMA,
    STATE(41), 1,
      aux_sym_selectors_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [969] = 3,
    ACTIONS(161), 1,
      aux_sym_sel_kind_token1,
    STATE(35), 1,
      sym_field_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [980] = 3,
    ACTIONS(161), 1,
      aux_sym_sel_kind_token1,
    STATE(31), 1,
      sym_field_name,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [991] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(156), 2,
      anon_sym_COLON,
      anon_sym_COMMA,
  [1000] = 2,
    ACTIONS(163), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1008] = 2,
    ACTIONS(165), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1016] = 2,
    ACTIONS(167), 1,
      anon_sym_COMMA,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1024] = 2,
    ACTIONS(169), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1032] = 2,
    ACTIONS(171), 1,
      anon_sym_SEMI,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1040] = 2,
    ACTIONS(173), 1,
      anon_sym_SEMI,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1048] = 2,
    ACTIONS(175), 1,
      anon_sym_COMMA,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1056] = 2,
    ACTIONS(177), 1,
      anon_sym_COMMA,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1064] = 2,
    ACTIONS(179), 1,
      anon_sym_COMMA,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1072] = 2,
    ACTIONS(181), 1,
      anon_sym_COMMA,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1080] = 2,
    ACTIONS(183), 1,
      anon_sym_COMMA,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1088] = 2,
    ACTIONS(185), 1,
      anon_sym_COMMA,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1096] = 2,
    ACTIONS(187), 1,
      anon_sym_COMMA,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1104] = 2,
    ACTIONS(189), 1,
      anon_sym_SEMI,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1112] = 2,
    ACTIONS(191), 1,
      anon_sym_SEMI,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1120] = 2,
    ACTIONS(193), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1128] = 2,
    ACTIONS(195), 1,
      anon_sym_COMMA,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1136] = 2,
    ACTIONS(197), 1,
      anon_sym_COMMA,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1144] = 2,
    ACTIONS(199), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1152] = 2,
    ACTIONS(201), 1,
      ts_builtin_sym_end,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
};

static uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 58,
  [SMALL_STATE(4)] = 110,
  [SMALL_STATE(5)] = 162,
  [SMALL_STATE(6)] = 202,
  [SMALL_STATE(7)] = 242,
  [SMALL_STATE(8)] = 273,
  [SMALL_STATE(9)] = 304,
  [SMALL_STATE(10)] = 334,
  [SMALL_STATE(11)] = 372,
  [SMALL_STATE(12)] = 410,
  [SMALL_STATE(13)] = 448,
  [SMALL_STATE(14)] = 472,
  [SMALL_STATE(15)] = 496,
  [SMALL_STATE(16)] = 518,
  [SMALL_STATE(17)] = 542,
  [SMALL_STATE(18)] = 566,
  [SMALL_STATE(19)] = 590,
  [SMALL_STATE(20)] = 614,
  [SMALL_STATE(21)] = 638,
  [SMALL_STATE(22)] = 662,
  [SMALL_STATE(23)] = 684,
  [SMALL_STATE(24)] = 706,
  [SMALL_STATE(25)] = 720,
  [SMALL_STATE(26)] = 736,
  [SMALL_STATE(27)] = 752,
  [SMALL_STATE(28)] = 772,
  [SMALL_STATE(29)] = 786,
  [SMALL_STATE(30)] = 800,
  [SMALL_STATE(31)] = 814,
  [SMALL_STATE(32)] = 828,
  [SMALL_STATE(33)] = 842,
  [SMALL_STATE(34)] = 856,
  [SMALL_STATE(35)] = 872,
  [SMALL_STATE(36)] = 886,
  [SMALL_STATE(37)] = 903,
  [SMALL_STATE(38)] = 917,
  [SMALL_STATE(39)] = 931,
  [SMALL_STATE(40)] = 945,
  [SMALL_STATE(41)] = 955,
  [SMALL_STATE(42)] = 969,
  [SMALL_STATE(43)] = 980,
  [SMALL_STATE(44)] = 991,
  [SMALL_STATE(45)] = 1000,
  [SMALL_STATE(46)] = 1008,
  [SMALL_STATE(47)] = 1016,
  [SMALL_STATE(48)] = 1024,
  [SMALL_STATE(49)] = 1032,
  [SMALL_STATE(50)] = 1040,
  [SMALL_STATE(51)] = 1048,
  [SMALL_STATE(52)] = 1056,
  [SMALL_STATE(53)] = 1064,
  [SMALL_STATE(54)] = 1072,
  [SMALL_STATE(55)] = 1080,
  [SMALL_STATE(56)] = 1088,
  [SMALL_STATE(57)] = 1096,
  [SMALL_STATE(58)] = 1104,
  [SMALL_STATE(59)] = 1112,
  [SMALL_STATE(60)] = 1120,
  [SMALL_STATE(61)] = 1128,
  [SMALL_STATE(62)] = 1136,
  [SMALL_STATE(63)] = 1144,
  [SMALL_STATE(64)] = 1152,
};

static TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_s, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(59),
  [17] = {.entry = {.count = 1, .reusable = false}}, SHIFT(55),
  [19] = {.entry = {.count = 1, .reusable = false}}, SHIFT(52),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [25] = {.entry = {.count = 1, .reusable = false}}, SHIFT(51),
  [27] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_s_repeat1, 2),
  [29] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_s_repeat1, 2), SHIFT_REPEAT(33),
  [32] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_s_repeat1, 2), SHIFT_REPEAT(42),
  [35] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_s, 1),
  [37] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_selector_repeat1, 2),
  [39] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_selector_repeat1, 2), SHIFT_REPEAT(33),
  [42] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_selector_repeat1, 2), SHIFT_REPEAT(42),
  [45] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selector, 1),
  [47] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [49] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [51] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [53] = {.entry = {.count = 1, .reusable = false}}, SHIFT(45),
  [55] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [57] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_properties_repeat1, 2, .production_id = 8),
  [59] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_properties_repeat1, 2, .production_id = 8), SHIFT_REPEAT(48),
  [62] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_properties_repeat1, 2, .production_id = 8), SHIFT_REPEAT(60),
  [65] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_properties_repeat1, 2, .production_id = 8), SHIFT_REPEAT(45),
  [68] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_properties_repeat1, 2, .production_id = 8), SHIFT_REPEAT(46),
  [71] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [73] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attrb, 3),
  [75] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [77] = {.entry = {.count = 1, .reusable = false}}, SHIFT(26),
  [79] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_attrs_repeat1, 2),
  [81] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_attrs_repeat1, 2), SHIFT_REPEAT(36),
  [84] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_attrs_repeat1, 2),
  [86] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_attrb_repeat1, 2),
  [88] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_attrb_repeat1, 2), SHIFT_REPEAT(26),
  [91] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_attrb_repeat1, 2), SHIFT_REPEAT(26),
  [94] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attrb, 2),
  [96] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute, 3),
  [98] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute, 2),
  [100] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attrs, 2),
  [102] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [104] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attrs, 2),
  [106] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attrs, 1),
  [108] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attrs, 1),
  [110] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_field_name, 1),
  [112] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_symbol, 1),
  [114] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [116] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attr, 1),
  [118] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attr, 1),
  [120] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_selector_repeat1, 1),
  [122] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [124] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [126] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [128] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_twins, 3),
  [130] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_siblings, 3),
  [132] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_child, 3),
  [134] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_kind_field, 3),
  [136] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_kind, 1),
  [138] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attr_or, 2),
  [140] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attr_or, 2),
  [142] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_field, 2),
  [144] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_properties_repeat1, 2, .production_id = 6),
  [146] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_properties_repeat1, 2, .production_id = 6),
  [148] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selectors, 1),
  [150] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [152] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selectors, 2),
  [154] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hl_rule, 4, .production_id = 5),
  [156] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_selectors_repeat1, 2),
  [158] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_selectors_repeat1, 2), SHIFT_REPEAT(9),
  [161] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [163] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [165] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [167] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [169] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [171] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_properties, 3, .production_id = 7),
  [173] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_properties, 2),
  [175] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fg, 3),
  [177] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ansi_color, 1, .production_id = 9),
  [179] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ansi_color, 1, .production_id = 10),
  [181] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_color_name, 1),
  [183] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_bg, 3),
  [185] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property, 1, .production_id = 2),
  [187] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property, 1, .production_id = 1),
  [189] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [191] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_highlight, 1),
  [193] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [195] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property, 1, .production_id = 4),
  [197] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property, 1, .production_id = 3),
  [199] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [201] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
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
