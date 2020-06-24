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
#define STATE_COUNT 59
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 125
#define ALIAS_COUNT 0
#define TOKEN_COUNT 99
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 7
#define MAX_ALIAS_SEQUENCE_LENGTH 4

enum {
  anon_sym_COLON = 1,
  anon_sym_SEMI = 2,
  sym_comment = 3,
  sym_newline = 4,
  anon_sym_COMMA = 5,
  sym_sel_kind = 6,
  anon_sym_PLUS = 7,
  anon_sym_TILDE = 8,
  anon_sym_GT = 9,
  anon_sym_LBRACE = 10,
  anon_sym_RBRACE = 11,
  anon_sym_fg = 12,
  anon_sym_bg = 13,
  anon_sym_attr = 14,
  anon_sym_attribute = 15,
  anon_sym_PIPE = 16,
  anon_sym_bold = 17,
  anon_sym_italic = 18,
  anon_sym_underlined = 19,
  anon_sym_underline = 20,
  anon_sym_reverse = 21,
  sym_rgb_color = 22,
  aux_sym_ansi_color_token1 = 23,
  aux_sym_ansi_color_token2 = 24,
  anon_sym_black = 25,
  anon_sym_darkgrey = 26,
  anon_sym_dark_DASHgrey = 27,
  anon_sym_dark_grey = 28,
  anon_sym_red = 29,
  anon_sym_darkred = 30,
  anon_sym_dark_DASHred = 31,
  anon_sym_dark_red = 32,
  anon_sym_green = 33,
  anon_sym_darkgreen = 34,
  anon_sym_dark_DASHgreen = 35,
  anon_sym_dark_green = 36,
  anon_sym_yellow = 37,
  anon_sym_darkyellow = 38,
  anon_sym_dark_DASHyellow = 39,
  anon_sym_dark_yellow = 40,
  anon_sym_blue = 41,
  anon_sym_darkblue = 42,
  anon_sym_dark_DASHblue = 43,
  anon_sym_dark_blue = 44,
  anon_sym_magenta = 45,
  anon_sym_darkmagenta = 46,
  anon_sym_dark_DASHmagenta = 47,
  anon_sym_dark_magenta = 48,
  anon_sym_cyan = 49,
  anon_sym_darkcyan = 50,
  anon_sym_dark_DASHcyan = 51,
  anon_sym_dark_cyan = 52,
  anon_sym_white = 53,
  anon_sym_grey = 54,
  anon_sym_bg_DASHcanvas = 55,
  anon_sym_fg_DASHcanvas = 56,
  anon_sym_canvas = 57,
  anon_sym_comment = 58,
  anon_sym_constant = 59,
  anon_sym_string = 60,
  anon_sym_escape_DASHseq = 61,
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
  [sym_comment] = "comment",
  [sym_newline] = "newline",
  [anon_sym_COMMA] = ",",
  [sym_sel_kind] = "sel_kind",
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
  [anon_sym_escape_DASHseq] = "escape-seq",
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
  [sym_sel_kind] = sym_sel_kind,
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
  [anon_sym_escape_DASHseq] = anon_sym_escape_DASHseq,
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
  [sym_sel_kind] = {
    .visible = true,
    .named = true,
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
  [anon_sym_escape_DASHseq] = {
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
  field_selectors = 7,
};

static const char *ts_field_names[] = {
  [0] = NULL,
  [field_ansi_color_dec] = "ansi_color_dec",
  [field_ansi_color_hex] = "ansi_color_hex",
  [field_attr] = "attr",
  [field_attribute] = "attribute",
  [field_bg] = "bg",
  [field_fg] = "fg",
  [field_selectors] = "selectors",
};

static const TSFieldMapSlice ts_field_map_slices[8] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 1},
  [3] = {.index = 2, .length = 1},
  [4] = {.index = 3, .length = 1},
  [5] = {.index = 4, .length = 1},
  [6] = {.index = 5, .length = 1},
  [7] = {.index = 6, .length = 1},
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
  [5] =
    {field_ansi_color_dec, 0},
  [6] =
    {field_ansi_color_hex, 0},
};

static TSSymbol ts_alias_sequences[8][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(361);
      if (lookahead == '\n') ADVANCE(371);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '#') ADVANCE(369);
      if (lookahead == '+') ADVANCE(374);
      if (lookahead == ',') ADVANCE(372);
      if (lookahead == '0') ADVANCE(391);
      if (lookahead == ':') ADVANCE(362);
      if (lookahead == ';') ADVANCE(363);
      if (lookahead == '>') ADVANCE(376);
      if (lookahead == 'a') ADVANCE(320);
      if (lookahead == 'b') ADVANCE(149);
      if (lookahead == 'c') ADVANCE(20);
      if (lookahead == 'd') ADVANCE(24);
      if (lookahead == 'e') ADVANCE(289);
      if (lookahead == 'f') ADVANCE(150);
      if (lookahead == 'g') ADVANCE(290);
      if (lookahead == 'i') ADVANCE(81);
      if (lookahead == 'k') ADVANCE(86);
      if (lookahead == 'l') ADVANCE(12);
      if (lookahead == 'm') ADVANCE(13);
      if (lookahead == 'n') ADVANCE(337);
      if (lookahead == 'o') ADVANCE(268);
      if (lookahead == 'p') ADVANCE(276);
      if (lookahead == 'r') ADVANCE(87);
      if (lookahead == 's') ADVANCE(269);
      if (lookahead == 't') ADVANCE(14);
      if (lookahead == 'u') ADVANCE(228);
      if (lookahead == 'w') ADVANCE(162);
      if (lookahead == 'y') ADVANCE(88);
      if (lookahead == '{') ADVANCE(377);
      if (lookahead == '|') ADVANCE(385);
      if (lookahead == '}') ADVANCE(378);
      if (lookahead == '~') ADVANCE(375);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(0)
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(392);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(371);
      END_STATE();
    case 2:
      if (lookahead == '\n') ADVANCE(371);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '#') ADVANCE(369);
      if (lookahead == '0') ADVANCE(391);
      if (lookahead == 'b') ADVANCE(155);
      if (lookahead == 'c') ADVANCE(354);
      if (lookahead == 'd') ADVANCE(23);
      if (lookahead == 'f') ADVANCE(161);
      if (lookahead == 'g') ADVANCE(290);
      if (lookahead == 'm') ADVANCE(49);
      if (lookahead == 'r') ADVANCE(123);
      if (lookahead == 'w') ADVANCE(162);
      if (lookahead == 'y') ADVANCE(88);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(2)
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(392);
      END_STATE();
    case 3:
      if (lookahead == '\n') ADVANCE(371);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '#') ADVANCE(370);
      if (lookahead == 'a') ADVANCE(320);
      if (lookahead == 'b') ADVANCE(153);
      if (lookahead == 'f') ADVANCE(154);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(3)
      END_STATE();
    case 4:
      if (lookahead == '-') ADVANCE(204);
      END_STATE();
    case 5:
      if (lookahead == '-') ADVANCE(54);
      if (lookahead == '_') ADVANCE(55);
      if (lookahead == 'b') ADVANCE(190);
      if (lookahead == 'c') ADVANCE(355);
      if (lookahead == 'g') ADVANCE(298);
      if (lookahead == 'm') ADVANCE(43);
      if (lookahead == 'r') ADVANCE(113);
      if (lookahead == 'y') ADVANCE(143);
      END_STATE();
    case 6:
      if (lookahead == '-') ADVANCE(238);
      END_STATE();
    case 7:
      if (lookahead == '-') ADVANCE(67);
      END_STATE();
    case 8:
      if (lookahead == '-') ADVANCE(309);
      END_STATE();
    case 9:
      if (lookahead == '-') ADVANCE(69);
      END_STATE();
    case 10:
      if (lookahead == '-') ADVANCE(72);
      END_STATE();
    case 11:
      if (lookahead == '-') ADVANCE(206);
      END_STATE();
    case 12:
      if (lookahead == 'a') ADVANCE(53);
      if (lookahead == 'i') ADVANCE(235);
      END_STATE();
    case 13:
      if (lookahead == 'a') ADVANCE(66);
      END_STATE();
    case 14:
      if (lookahead == 'a') ADVANCE(50);
      if (lookahead == 'o') ADVANCE(80);
      if (lookahead == 'y') ADVANCE(271);
      END_STATE();
    case 15:
      if (lookahead == 'a') ADVANCE(302);
      END_STATE();
    case 16:
      if (lookahead == 'a') ADVANCE(414);
      END_STATE();
    case 17:
      if (lookahead == 'a') ADVANCE(415);
      END_STATE();
    case 18:
      if (lookahead == 'a') ADVANCE(416);
      END_STATE();
    case 19:
      if (lookahead == 'a') ADVANCE(417);
      END_STATE();
    case 20:
      if (lookahead == 'a') ADVANCE(215);
      if (lookahead == 'h') ADVANCE(30);
      if (lookahead == 'o') ADVANCE(207);
      if (lookahead == 'y') ADVANCE(31);
      END_STATE();
    case 21:
      if (lookahead == 'a') ADVANCE(156);
      END_STATE();
    case 22:
      if (lookahead == 'a') ADVANCE(310);
      if (lookahead == 'o') ADVANCE(296);
      if (lookahead == 'r') ADVANCE(168);
      END_STATE();
    case 23:
      if (lookahead == 'a') ADVANCE(277);
      END_STATE();
    case 24:
      if (lookahead == 'a') ADVANCE(277);
      if (lookahead == 'e') ADVANCE(51);
      END_STATE();
    case 25:
      if (lookahead == 'a') ADVANCE(60);
      if (lookahead == 'u') ADVANCE(91);
      END_STATE();
    case 26:
      if (lookahead == 'a') ADVANCE(189);
      END_STATE();
    case 27:
      if (lookahead == 'a') ADVANCE(303);
      END_STATE();
    case 28:
      if (lookahead == 'a') ADVANCE(304);
      END_STATE();
    case 29:
      if (lookahead == 'a') ADVANCE(311);
      END_STATE();
    case 30:
      if (lookahead == 'a') ADVANCE(279);
      END_STATE();
    case 31:
      if (lookahead == 'a') ADVANCE(216);
      END_STATE();
    case 32:
      if (lookahead == 'a') ADVANCE(307);
      END_STATE();
    case 33:
      if (lookahead == 'a') ADVANCE(185);
      END_STATE();
    case 34:
      if (lookahead == 'a') ADVANCE(331);
      END_STATE();
    case 35:
      if (lookahead == 'a') ADVANCE(218);
      END_STATE();
    case 36:
      if (lookahead == 'a') ADVANCE(186);
      END_STATE();
    case 37:
      if (lookahead == 'a') ADVANCE(313);
      END_STATE();
    case 38:
      if (lookahead == 'a') ADVANCE(219);
      END_STATE();
    case 39:
      if (lookahead == 'a') ADVANCE(221);
      END_STATE();
    case 40:
      if (lookahead == 'a') ADVANCE(222);
      END_STATE();
    case 41:
      if (lookahead == 'a') ADVANCE(286);
      END_STATE();
    case 42:
      if (lookahead == 'a') ADVANCE(245);
      END_STATE();
    case 43:
      if (lookahead == 'a') ADVANCE(158);
      END_STATE();
    case 44:
      if (lookahead == 'a') ADVANCE(272);
      END_STATE();
    case 45:
      if (lookahead == 'a') ADVANCE(247);
      END_STATE();
    case 46:
      if (lookahead == 'a') ADVANCE(237);
      END_STATE();
    case 47:
      if (lookahead == 'a') ADVANCE(159);
      END_STATE();
    case 48:
      if (lookahead == 'a') ADVANCE(160);
      END_STATE();
    case 49:
      if (lookahead == 'a') ADVANCE(157);
      END_STATE();
    case 50:
      if (lookahead == 'b') ADVANCE(4);
      if (lookahead == 'g') ADVANCE(455);
      END_STATE();
    case 51:
      if (lookahead == 'b') ADVANCE(338);
      if (lookahead == 'f') ADVANCE(173);
      if (lookahead == 'l') ADVANCE(166);
      END_STATE();
    case 52:
      if (lookahead == 'b') ADVANCE(343);
      END_STATE();
    case 53:
      if (lookahead == 'b') ADVANCE(117);
      END_STATE();
    case 54:
      if (lookahead == 'b') ADVANCE(199);
      if (lookahead == 'c') ADVANCE(356);
      if (lookahead == 'g') ADVANCE(299);
      if (lookahead == 'm') ADVANCE(47);
      if (lookahead == 'r') ADVANCE(119);
      if (lookahead == 'y') ADVANCE(144);
      END_STATE();
    case 55:
      if (lookahead == 'b') ADVANCE(201);
      if (lookahead == 'c') ADVANCE(357);
      if (lookahead == 'g') ADVANCE(300);
      if (lookahead == 'm') ADVANCE(48);
      if (lookahead == 'r') ADVANCE(122);
      if (lookahead == 'y') ADVANCE(145);
      END_STATE();
    case 56:
      if (lookahead == 'b') ADVANCE(126);
      END_STATE();
    case 57:
      if (lookahead == 'c') ADVANCE(387);
      END_STATE();
    case 58:
      if (lookahead == 'c') ADVANCE(444);
      END_STATE();
    case 59:
      if (lookahead == 'c') ADVANCE(163);
      END_STATE();
    case 60:
      if (lookahead == 'c') ADVANCE(183);
      END_STATE();
    case 61:
      if (lookahead == 'c') ADVANCE(187);
      END_STATE();
    case 62:
      if (lookahead == 'c') ADVANCE(44);
      END_STATE();
    case 63:
      if (lookahead == 'c') ADVANCE(172);
      END_STATE();
    case 64:
      if (lookahead == 'c') ADVANCE(267);
      if (lookahead == 'p') ADVANCE(293);
      END_STATE();
    case 65:
      if (lookahead == 'c') ADVANCE(323);
      END_STATE();
    case 66:
      if (lookahead == 'c') ADVANCE(291);
      if (lookahead == 'g') ADVANCE(134);
      END_STATE();
    case 67:
      if (lookahead == 'c') ADVANCE(42);
      END_STATE();
    case 68:
      if (lookahead == 'c') ADVANCE(107);
      END_STATE();
    case 69:
      if (lookahead == 'c') ADVANCE(197);
      END_STATE();
    case 70:
      if (lookahead == 'c') ADVANCE(322);
      END_STATE();
    case 71:
      if (lookahead == 'c') ADVANCE(318);
      END_STATE();
    case 72:
      if (lookahead == 'c') ADVANCE(45);
      END_STATE();
    case 73:
      if (lookahead == 'd') ADVANCE(398);
      END_STATE();
    case 74:
      if (lookahead == 'd') ADVANCE(398);
      if (lookahead == 'p') ADVANCE(135);
      if (lookahead == 'v') ADVANCE(124);
      END_STATE();
    case 75:
      if (lookahead == 'd') ADVANCE(386);
      END_STATE();
    case 76:
      if (lookahead == 'd') ADVANCE(399);
      END_STATE();
    case 77:
      if (lookahead == 'd') ADVANCE(442);
      END_STATE();
    case 78:
      if (lookahead == 'd') ADVANCE(400);
      END_STATE();
    case 79:
      if (lookahead == 'd') ADVANCE(401);
      END_STATE();
    case 80:
      if (lookahead == 'd') ADVANCE(251);
      END_STATE();
    case 81:
      if (lookahead == 'd') ADVANCE(118);
      if (lookahead == 'g') ADVANCE(229);
      if (lookahead == 'n') ADVANCE(61);
      if (lookahead == 't') ADVANCE(26);
      END_STATE();
    case 82:
      if (lookahead == 'd') ADVANCE(96);
      END_STATE();
    case 83:
      if (lookahead == 'd') ADVANCE(181);
      if (lookahead == 's') ADVANCE(332);
      END_STATE();
    case 84:
      if (lookahead == 'd') ADVANCE(170);
      END_STATE();
    case 85:
      if (lookahead == 'd') ADVANCE(125);
      END_STATE();
    case 86:
      if (lookahead == 'e') ADVANCE(353);
      END_STATE();
    case 87:
      if (lookahead == 'e') ADVANCE(74);
      END_STATE();
    case 88:
      if (lookahead == 'e') ADVANCE(191);
      END_STATE();
    case 89:
      if (lookahead == 'e') ADVANCE(121);
      END_STATE();
    case 90:
      if (lookahead == 'e') ADVANCE(64);
      if (lookahead == 'o') ADVANCE(209);
      END_STATE();
    case 91:
      if (lookahead == 'e') ADVANCE(410);
      END_STATE();
    case 92:
      if (lookahead == 'e') ADVANCE(449);
      END_STATE();
    case 93:
      if (lookahead == 'e') ADVANCE(422);
      END_STATE();
    case 94:
      if (lookahead == 'e') ADVANCE(446);
      END_STATE();
    case 95:
      if (lookahead == 'e') ADVANCE(459);
      END_STATE();
    case 96:
      if (lookahead == 'e') ADVANCE(445);
      END_STATE();
    case 97:
      if (lookahead == 'e') ADVANCE(390);
      END_STATE();
    case 98:
      if (lookahead == 'e') ADVANCE(411);
      END_STATE();
    case 99:
      if (lookahead == 'e') ADVANCE(465);
      END_STATE();
    case 100:
      if (lookahead == 'e') ADVANCE(384);
      END_STATE();
    case 101:
      if (lookahead == 'e') ADVANCE(412);
      END_STATE();
    case 102:
      if (lookahead == 'e') ADVANCE(413);
      END_STATE();
    case 103:
      if (lookahead == 'e') ADVANCE(275);
      END_STATE();
    case 104:
      if (lookahead == 'e') ADVANCE(451);
      END_STATE();
    case 105:
      if (lookahead == 'e') ADVANCE(389);
      END_STATE();
    case 106:
      if (lookahead == 'e') ADVANCE(464);
      END_STATE();
    case 107:
      if (lookahead == 'e') ADVANCE(273);
      END_STATE();
    case 108:
      if (lookahead == 'e') ADVANCE(128);
      END_STATE();
    case 109:
      if (lookahead == 'e') ADVANCE(6);
      END_STATE();
    case 110:
      if (lookahead == 'e') ADVANCE(147);
      END_STATE();
    case 111:
      if (lookahead == 'e') ADVANCE(130);
      END_STATE();
    case 112:
      if (lookahead == 'e') ADVANCE(8);
      END_STATE();
    case 113:
      if (lookahead == 'e') ADVANCE(76);
      END_STATE();
    case 114:
      if (lookahead == 'e') ADVANCE(131);
      END_STATE();
    case 115:
      if (lookahead == 'e') ADVANCE(9);
      END_STATE();
    case 116:
      if (lookahead == 'e') ADVANCE(63);
      END_STATE();
    case 117:
      if (lookahead == 'e') ADVANCE(184);
      END_STATE();
    case 118:
      if (lookahead == 'e') ADVANCE(233);
      END_STATE();
    case 119:
      if (lookahead == 'e') ADVANCE(78);
      END_STATE();
    case 120:
      if (lookahead == 'e') ADVANCE(295);
      END_STATE();
    case 121:
      if (lookahead == 'e') ADVANCE(217);
      if (lookahead == 'y') ADVANCE(423);
      END_STATE();
    case 122:
      if (lookahead == 'e') ADVANCE(79);
      END_STATE();
    case 123:
      if (lookahead == 'e') ADVANCE(73);
      END_STATE();
    case 124:
      if (lookahead == 'e') ADVANCE(288);
      END_STATE();
    case 125:
      if (lookahead == 'e') ADVANCE(294);
      END_STATE();
    case 126:
      if (lookahead == 'e') ADVANCE(281);
      END_STATE();
    case 127:
      if (lookahead == 'e') ADVANCE(284);
      END_STATE();
    case 128:
      if (lookahead == 'e') ADVANCE(223);
      if (lookahead == 'y') ADVANCE(395);
      END_STATE();
    case 129:
      if (lookahead == 'e') ADVANCE(285);
      END_STATE();
    case 130:
      if (lookahead == 'e') ADVANCE(225);
      if (lookahead == 'y') ADVANCE(396);
      END_STATE();
    case 131:
      if (lookahead == 'e') ADVANCE(226);
      if (lookahead == 'y') ADVANCE(397);
      END_STATE();
    case 132:
      if (lookahead == 'e') ADVANCE(198);
      END_STATE();
    case 133:
      if (lookahead == 'e') ADVANCE(35);
      END_STATE();
    case 134:
      if (lookahead == 'e') ADVANCE(234);
      END_STATE();
    case 135:
      if (lookahead == 'e') ADVANCE(37);
      END_STATE();
    case 136:
      if (lookahead == 'e') ADVANCE(71);
      END_STATE();
    case 137:
      if (lookahead == 'e') ADVANCE(236);
      END_STATE();
    case 138:
      if (lookahead == 'e') ADVANCE(239);
      END_STATE();
    case 139:
      if (lookahead == 'e') ADVANCE(246);
      END_STATE();
    case 140:
      if (lookahead == 'e') ADVANCE(249);
      END_STATE();
    case 141:
      if (lookahead == 'e') ADVANCE(250);
      END_STATE();
    case 142:
      if (lookahead == 'e') ADVANCE(241);
      END_STATE();
    case 143:
      if (lookahead == 'e') ADVANCE(200);
      END_STATE();
    case 144:
      if (lookahead == 'e') ADVANCE(202);
      END_STATE();
    case 145:
      if (lookahead == 'e') ADVANCE(203);
      END_STATE();
    case 146:
      if (lookahead == 'e') ADVANCE(212);
      if (lookahead == 'u') ADVANCE(306);
      END_STATE();
    case 147:
      if (lookahead == 'f') ADVANCE(452);
      END_STATE();
    case 148:
      if (lookahead == 'f') ADVANCE(180);
      END_STATE();
    case 149:
      if (lookahead == 'g') ADVANCE(382);
      if (lookahead == 'l') ADVANCE(25);
      if (lookahead == 'o') ADVANCE(188);
      END_STATE();
    case 150:
      if (lookahead == 'g') ADVANCE(380);
      if (lookahead == 'l') ADVANCE(257);
      if (lookahead == 'u') ADVANCE(231);
      END_STATE();
    case 151:
      if (lookahead == 'g') ADVANCE(458);
      END_STATE();
    case 152:
      if (lookahead == 'g') ADVANCE(429);
      END_STATE();
    case 153:
      if (lookahead == 'g') ADVANCE(381);
      END_STATE();
    case 154:
      if (lookahead == 'g') ADVANCE(379);
      END_STATE();
    case 155:
      if (lookahead == 'g') ADVANCE(7);
      if (lookahead == 'l') ADVANCE(25);
      END_STATE();
    case 156:
      if (lookahead == 'g') ADVANCE(115);
      END_STATE();
    case 157:
      if (lookahead == 'g') ADVANCE(134);
      END_STATE();
    case 158:
      if (lookahead == 'g') ADVANCE(139);
      END_STATE();
    case 159:
      if (lookahead == 'g') ADVANCE(140);
      END_STATE();
    case 160:
      if (lookahead == 'g') ADVANCE(141);
      END_STATE();
    case 161:
      if (lookahead == 'g') ADVANCE(10);
      END_STATE();
    case 162:
      if (lookahead == 'h') ADVANCE(165);
      END_STATE();
    case 163:
      if (lookahead == 'h') ADVANCE(41);
      if (lookahead == 'o') ADVANCE(214);
      END_STATE();
    case 164:
      if (lookahead == 'i') ADVANCE(148);
      END_STATE();
    case 165:
      if (lookahead == 'i') ADVANCE(327);
      END_STATE();
    case 166:
      if (lookahead == 'i') ADVANCE(210);
      END_STATE();
    case 167:
      if (lookahead == 'i') ADVANCE(57);
      END_STATE();
    case 168:
      if (lookahead == 'i') ADVANCE(230);
      if (lookahead == 'u') ADVANCE(70);
      END_STATE();
    case 169:
      if (lookahead == 'i') ADVANCE(261);
      END_STATE();
    case 170:
      if (lookahead == 'i') ADVANCE(316);
      END_STATE();
    case 171:
      if (lookahead == 'i') ADVANCE(333);
      END_STATE();
    case 172:
      if (lookahead == 'i') ADVANCE(33);
      END_STATE();
    case 173:
      if (lookahead == 'i') ADVANCE(240);
      END_STATE();
    case 174:
      if (lookahead == 'i') ADVANCE(262);
      END_STATE();
    case 175:
      if (lookahead == 'i') ADVANCE(242);
      END_STATE();
    case 176:
      if (lookahead == 'i') ADVANCE(264);
      END_STATE();
    case 177:
      if (lookahead == 'i') ADVANCE(243);
      END_STATE();
    case 178:
      if (lookahead == 'i') ADVANCE(265);
      END_STATE();
    case 179:
      if (lookahead == 'i') ADVANCE(244);
      END_STATE();
    case 180:
      if (lookahead == 'i') ADVANCE(129);
      END_STATE();
    case 181:
      if (lookahead == 'i') ADVANCE(334);
      END_STATE();
    case 182:
      if (lookahead == 'k') ADVANCE(5);
      END_STATE();
    case 183:
      if (lookahead == 'k') ADVANCE(394);
      END_STATE();
    case 184:
      if (lookahead == 'l') ADVANCE(440);
      END_STATE();
    case 185:
      if (lookahead == 'l') ADVANCE(453);
      END_STATE();
    case 186:
      if (lookahead == 'l') ADVANCE(438);
      END_STATE();
    case 187:
      if (lookahead == 'l') ADVANCE(342);
      END_STATE();
    case 188:
      if (lookahead == 'l') ADVANCE(75);
      if (lookahead == 'o') ADVANCE(196);
      END_STATE();
    case 189:
      if (lookahead == 'l') ADVANCE(167);
      END_STATE();
    case 190:
      if (lookahead == 'l') ADVANCE(339);
      END_STATE();
    case 191:
      if (lookahead == 'l') ADVANCE(192);
      END_STATE();
    case 192:
      if (lookahead == 'l') ADVANCE(253);
      END_STATE();
    case 193:
      if (lookahead == 'l') ADVANCE(254);
      END_STATE();
    case 194:
      if (lookahead == 'l') ADVANCE(255);
      END_STATE();
    case 195:
      if (lookahead == 'l') ADVANCE(256);
      END_STATE();
    case 196:
      if (lookahead == 'l') ADVANCE(133);
      END_STATE();
    case 197:
      if (lookahead == 'l') ADVANCE(32);
      END_STATE();
    case 198:
      if (lookahead == 'l') ADVANCE(136);
      END_STATE();
    case 199:
      if (lookahead == 'l') ADVANCE(340);
      END_STATE();
    case 200:
      if (lookahead == 'l') ADVANCE(193);
      END_STATE();
    case 201:
      if (lookahead == 'l') ADVANCE(341);
      END_STATE();
    case 202:
      if (lookahead == 'l') ADVANCE(194);
      END_STATE();
    case 203:
      if (lookahead == 'l') ADVANCE(195);
      END_STATE();
    case 204:
      if (lookahead == 'l') ADVANCE(175);
      if (lookahead == 'o') ADVANCE(274);
      if (lookahead == 's') ADVANCE(132);
      END_STATE();
    case 205:
      if (lookahead == 'l') ADVANCE(177);
      END_STATE();
    case 206:
      if (lookahead == 'l') ADVANCE(179);
      END_STATE();
    case 207:
      if (lookahead == 'm') ADVANCE(211);
      if (lookahead == 'n') ADVANCE(83);
      END_STATE();
    case 208:
      if (lookahead == 'm') ADVANCE(56);
      END_STATE();
    case 209:
      if (lookahead == 'm') ADVANCE(270);
      END_STATE();
    case 210:
      if (lookahead == 'm') ADVANCE(171);
      END_STATE();
    case 211:
      if (lookahead == 'm') ADVANCE(137);
      END_STATE();
    case 212:
      if (lookahead == 'm') ADVANCE(138);
      END_STATE();
    case 213:
      if (lookahead == 'm') ADVANCE(142);
      END_STATE();
    case 214:
      if (lookahead == 'm') ADVANCE(213);
      END_STATE();
    case 215:
      if (lookahead == 'n') ADVANCE(345);
      END_STATE();
    case 216:
      if (lookahead == 'n') ADVANCE(418);
      END_STATE();
    case 217:
      if (lookahead == 'n') ADVANCE(402);
      END_STATE();
    case 218:
      if (lookahead == 'n') ADVANCE(433);
      END_STATE();
    case 219:
      if (lookahead == 'n') ADVANCE(419);
      END_STATE();
    case 220:
      if (lookahead == 'n') ADVANCE(436);
      END_STATE();
    case 221:
      if (lookahead == 'n') ADVANCE(420);
      END_STATE();
    case 222:
      if (lookahead == 'n') ADVANCE(421);
      END_STATE();
    case 223:
      if (lookahead == 'n') ADVANCE(403);
      END_STATE();
    case 224:
      if (lookahead == 'n') ADVANCE(443);
      END_STATE();
    case 225:
      if (lookahead == 'n') ADVANCE(404);
      END_STATE();
    case 226:
      if (lookahead == 'n') ADVANCE(405);
      END_STATE();
    case 227:
      if (lookahead == 'n') ADVANCE(466);
      END_STATE();
    case 228:
      if (lookahead == 'n') ADVANCE(85);
      END_STATE();
    case 229:
      if (lookahead == 'n') ADVANCE(266);
      END_STATE();
    case 230:
      if (lookahead == 'n') ADVANCE(152);
      END_STATE();
    case 231:
      if (lookahead == 'n') ADVANCE(65);
      END_STATE();
    case 232:
      if (lookahead == 'n') ADVANCE(84);
      END_STATE();
    case 233:
      if (lookahead == 'n') ADVANCE(324);
      END_STATE();
    case 234:
      if (lookahead == 'n') ADVANCE(325);
      END_STATE();
    case 235:
      if (lookahead == 'n') ADVANCE(109);
      END_STATE();
    case 236:
      if (lookahead == 'n') ADVANCE(314);
      END_STATE();
    case 237:
      if (lookahead == 'n') ADVANCE(315);
      END_STATE();
    case 238:
      if (lookahead == 'n') ADVANCE(282);
      END_STATE();
    case 239:
      if (lookahead == 'n') ADVANCE(317);
      END_STATE();
    case 240:
      if (lookahead == 'n') ADVANCE(94);
      END_STATE();
    case 241:
      if (lookahead == 'n') ADVANCE(319);
      END_STATE();
    case 242:
      if (lookahead == 'n') ADVANCE(99);
      END_STATE();
    case 243:
      if (lookahead == 'n') ADVANCE(105);
      END_STATE();
    case 244:
      if (lookahead == 'n') ADVANCE(106);
      END_STATE();
    case 245:
      if (lookahead == 'n') ADVANCE(346);
      END_STATE();
    case 246:
      if (lookahead == 'n') ADVANCE(326);
      END_STATE();
    case 247:
      if (lookahead == 'n') ADVANCE(347);
      END_STATE();
    case 248:
      if (lookahead == 'n') ADVANCE(36);
      END_STATE();
    case 249:
      if (lookahead == 'n') ADVANCE(328);
      END_STATE();
    case 250:
      if (lookahead == 'n') ADVANCE(329);
      END_STATE();
    case 251:
      if (lookahead == 'o') ADVANCE(461);
      END_STATE();
    case 252:
      if (lookahead == 'o') ADVANCE(447);
      END_STATE();
    case 253:
      if (lookahead == 'o') ADVANCE(348);
      END_STATE();
    case 254:
      if (lookahead == 'o') ADVANCE(349);
      END_STATE();
    case 255:
      if (lookahead == 'o') ADVANCE(350);
      END_STATE();
    case 256:
      if (lookahead == 'o') ADVANCE(351);
      END_STATE();
    case 257:
      if (lookahead == 'o') ADVANCE(29);
      END_STATE();
    case 258:
      if (lookahead == 'o') ADVANCE(58);
      END_STATE();
    case 259:
      if (lookahead == 'o') ADVANCE(280);
      END_STATE();
    case 260:
      if (lookahead == 'o') ADVANCE(292);
      END_STATE();
    case 261:
      if (lookahead == 'o') ADVANCE(220);
      END_STATE();
    case 262:
      if (lookahead == 'o') ADVANCE(248);
      END_STATE();
    case 263:
      if (lookahead == 'o') ADVANCE(283);
      END_STATE();
    case 264:
      if (lookahead == 'o') ADVANCE(224);
      END_STATE();
    case 265:
      if (lookahead == 'o') ADVANCE(227);
      END_STATE();
    case 266:
      if (lookahead == 'o') ADVANCE(297);
      END_STATE();
    case 267:
      if (lookahead == 'o') ADVANCE(232);
      END_STATE();
    case 268:
      if (lookahead == 'p') ADVANCE(120);
      END_STATE();
    case 269:
      if (lookahead == 'p') ADVANCE(116);
      if (lookahead == 't') ADVANCE(22);
      END_STATE();
    case 270:
      if (lookahead == 'p') ADVANCE(312);
      END_STATE();
    case 271:
      if (lookahead == 'p') ADVANCE(92);
      END_STATE();
    case 272:
      if (lookahead == 'p') ADVANCE(112);
      END_STATE();
    case 273:
      if (lookahead == 'p') ADVANCE(335);
      END_STATE();
    case 274:
      if (lookahead == 'p') ADVANCE(336);
      END_STATE();
    case 275:
      if (lookahead == 'q') ADVANCE(430);
      END_STATE();
    case 276:
      if (lookahead == 'r') ADVANCE(90);
      END_STATE();
    case 277:
      if (lookahead == 'r') ADVANCE(182);
      END_STATE();
    case 278:
      if (lookahead == 'r') ADVANCE(383);
      END_STATE();
    case 279:
      if (lookahead == 'r') ADVANCE(431);
      END_STATE();
    case 280:
      if (lookahead == 'r') ADVANCE(460);
      END_STATE();
    case 281:
      if (lookahead == 'r') ADVANCE(432);
      END_STATE();
    case 282:
      if (lookahead == 'r') ADVANCE(462);
      END_STATE();
    case 283:
      if (lookahead == 'r') ADVANCE(441);
      END_STATE();
    case 284:
      if (lookahead == 'r') ADVANCE(456);
      END_STATE();
    case 285:
      if (lookahead == 'r') ADVANCE(435);
      END_STATE();
    case 286:
      if (lookahead == 'r') ADVANCE(454);
      END_STATE();
    case 287:
      if (lookahead == 'r') ADVANCE(259);
      END_STATE();
    case 288:
      if (lookahead == 'r') ADVANCE(308);
      END_STATE();
    case 289:
      if (lookahead == 'r') ADVANCE(287);
      if (lookahead == 's') ADVANCE(62);
      if (lookahead == 'x') ADVANCE(68);
      END_STATE();
    case 290:
      if (lookahead == 'r') ADVANCE(89);
      END_STATE();
    case 291:
      if (lookahead == 'r') ADVANCE(252);
      END_STATE();
    case 292:
      if (lookahead == 'r') ADVANCE(77);
      END_STATE();
    case 293:
      if (lookahead == 'r') ADVANCE(258);
      END_STATE();
    case 294:
      if (lookahead == 'r') ADVANCE(205);
      END_STATE();
    case 295:
      if (lookahead == 'r') ADVANCE(34);
      END_STATE();
    case 296:
      if (lookahead == 'r') ADVANCE(21);
      END_STATE();
    case 297:
      if (lookahead == 'r') ADVANCE(95);
      END_STATE();
    case 298:
      if (lookahead == 'r') ADVANCE(108);
      END_STATE();
    case 299:
      if (lookahead == 'r') ADVANCE(111);
      END_STATE();
    case 300:
      if (lookahead == 'r') ADVANCE(114);
      END_STATE();
    case 301:
      if (lookahead == 'r') ADVANCE(104);
      END_STATE();
    case 302:
      if (lookahead == 's') ADVANCE(426);
      END_STATE();
    case 303:
      if (lookahead == 's') ADVANCE(424);
      END_STATE();
    case 304:
      if (lookahead == 's') ADVANCE(425);
      END_STATE();
    case 305:
      if (lookahead == 's') ADVANCE(450);
      END_STATE();
    case 306:
      if (lookahead == 's') ADVANCE(11);
      END_STATE();
    case 307:
      if (lookahead == 's') ADVANCE(305);
      END_STATE();
    case 308:
      if (lookahead == 's') ADVANCE(97);
      END_STATE();
    case 309:
      if (lookahead == 's') ADVANCE(103);
      END_STATE();
    case 310:
      if (lookahead == 't') ADVANCE(146);
      END_STATE();
    case 311:
      if (lookahead == 't') ADVANCE(434);
      END_STATE();
    case 312:
      if (lookahead == 't') ADVANCE(463);
      END_STATE();
    case 313:
      if (lookahead == 't') ADVANCE(439);
      END_STATE();
    case 314:
      if (lookahead == 't') ADVANCE(427);
      END_STATE();
    case 315:
      if (lookahead == 't') ADVANCE(428);
      END_STATE();
    case 316:
      if (lookahead == 't') ADVANCE(448);
      END_STATE();
    case 317:
      if (lookahead == 't') ADVANCE(437);
      END_STATE();
    case 318:
      if (lookahead == 't') ADVANCE(467);
      END_STATE();
    case 319:
      if (lookahead == 't') ADVANCE(457);
      END_STATE();
    case 320:
      if (lookahead == 't') ADVANCE(321);
      END_STATE();
    case 321:
      if (lookahead == 't') ADVANCE(278);
      END_STATE();
    case 322:
      if (lookahead == 't') ADVANCE(344);
      END_STATE();
    case 323:
      if (lookahead == 't') ADVANCE(169);
      END_STATE();
    case 324:
      if (lookahead == 't') ADVANCE(164);
      END_STATE();
    case 325:
      if (lookahead == 't') ADVANCE(16);
      END_STATE();
    case 326:
      if (lookahead == 't') ADVANCE(17);
      END_STATE();
    case 327:
      if (lookahead == 't') ADVANCE(93);
      END_STATE();
    case 328:
      if (lookahead == 't') ADVANCE(18);
      END_STATE();
    case 329:
      if (lookahead == 't') ADVANCE(19);
      END_STATE();
    case 330:
      if (lookahead == 't') ADVANCE(100);
      END_STATE();
    case 331:
      if (lookahead == 't') ADVANCE(263);
      END_STATE();
    case 332:
      if (lookahead == 't') ADVANCE(46);
      END_STATE();
    case 333:
      if (lookahead == 't') ADVANCE(127);
      END_STATE();
    case 334:
      if (lookahead == 't') ADVANCE(174);
      END_STATE();
    case 335:
      if (lookahead == 't') ADVANCE(176);
      END_STATE();
    case 336:
      if (lookahead == 't') ADVANCE(178);
      END_STATE();
    case 337:
      if (lookahead == 'u') ADVANCE(208);
      END_STATE();
    case 338:
      if (lookahead == 'u') ADVANCE(151);
      END_STATE();
    case 339:
      if (lookahead == 'u') ADVANCE(98);
      END_STATE();
    case 340:
      if (lookahead == 'u') ADVANCE(101);
      END_STATE();
    case 341:
      if (lookahead == 'u') ADVANCE(102);
      END_STATE();
    case 342:
      if (lookahead == 'u') ADVANCE(82);
      END_STATE();
    case 343:
      if (lookahead == 'u') ADVANCE(330);
      END_STATE();
    case 344:
      if (lookahead == 'u') ADVANCE(301);
      END_STATE();
    case 345:
      if (lookahead == 'v') ADVANCE(15);
      END_STATE();
    case 346:
      if (lookahead == 'v') ADVANCE(27);
      END_STATE();
    case 347:
      if (lookahead == 'v') ADVANCE(28);
      END_STATE();
    case 348:
      if (lookahead == 'w') ADVANCE(406);
      END_STATE();
    case 349:
      if (lookahead == 'w') ADVANCE(407);
      END_STATE();
    case 350:
      if (lookahead == 'w') ADVANCE(408);
      END_STATE();
    case 351:
      if (lookahead == 'w') ADVANCE(409);
      END_STATE();
    case 352:
      if (lookahead == 'w') ADVANCE(260);
      END_STATE();
    case 353:
      if (lookahead == 'y') ADVANCE(352);
      END_STATE();
    case 354:
      if (lookahead == 'y') ADVANCE(31);
      END_STATE();
    case 355:
      if (lookahead == 'y') ADVANCE(38);
      END_STATE();
    case 356:
      if (lookahead == 'y') ADVANCE(39);
      END_STATE();
    case 357:
      if (lookahead == 'y') ADVANCE(40);
      END_STATE();
    case 358:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(393);
      END_STATE();
    case 359:
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(373);
      END_STATE();
    case 360:
      if (eof) ADVANCE(361);
      if (lookahead == '\n') ADVANCE(371);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '#') ADVANCE(370);
      if (lookahead == '+') ADVANCE(374);
      if (lookahead == ',') ADVANCE(372);
      if (lookahead == ':') ADVANCE(362);
      if (lookahead == '>') ADVANCE(376);
      if (lookahead == '~') ADVANCE(375);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(360)
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(359);
      END_STATE();
    case 361:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 362:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 363:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 364:
      ACCEPT_TOKEN(sym_comment);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(370);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(370);
      END_STATE();
    case 365:
      ACCEPT_TOKEN(sym_comment);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(364);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(370);
      END_STATE();
    case 366:
      ACCEPT_TOKEN(sym_comment);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(365);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(370);
      END_STATE();
    case 367:
      ACCEPT_TOKEN(sym_comment);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(366);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(370);
      END_STATE();
    case 368:
      ACCEPT_TOKEN(sym_comment);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(367);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(370);
      END_STATE();
    case 369:
      ACCEPT_TOKEN(sym_comment);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(368);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(370);
      END_STATE();
    case 370:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(370);
      END_STATE();
    case 371:
      ACCEPT_TOKEN(sym_newline);
      END_STATE();
    case 372:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 373:
      ACCEPT_TOKEN(sym_sel_kind);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(373);
      END_STATE();
    case 374:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 375:
      ACCEPT_TOKEN(anon_sym_TILDE);
      END_STATE();
    case 376:
      ACCEPT_TOKEN(anon_sym_GT);
      END_STATE();
    case 377:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 378:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 379:
      ACCEPT_TOKEN(anon_sym_fg);
      END_STATE();
    case 380:
      ACCEPT_TOKEN(anon_sym_fg);
      if (lookahead == '-') ADVANCE(72);
      END_STATE();
    case 381:
      ACCEPT_TOKEN(anon_sym_bg);
      END_STATE();
    case 382:
      ACCEPT_TOKEN(anon_sym_bg);
      if (lookahead == '-') ADVANCE(67);
      END_STATE();
    case 383:
      ACCEPT_TOKEN(anon_sym_attr);
      if (lookahead == 'i') ADVANCE(52);
      END_STATE();
    case 384:
      ACCEPT_TOKEN(anon_sym_attribute);
      END_STATE();
    case 385:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 386:
      ACCEPT_TOKEN(anon_sym_bold);
      END_STATE();
    case 387:
      ACCEPT_TOKEN(anon_sym_italic);
      END_STATE();
    case 388:
      ACCEPT_TOKEN(anon_sym_underlined);
      END_STATE();
    case 389:
      ACCEPT_TOKEN(anon_sym_underline);
      if (lookahead == 'd') ADVANCE(388);
      END_STATE();
    case 390:
      ACCEPT_TOKEN(anon_sym_reverse);
      END_STATE();
    case 391:
      ACCEPT_TOKEN(aux_sym_ansi_color_token1);
      if (lookahead == 'x') ADVANCE(358);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(392);
      END_STATE();
    case 392:
      ACCEPT_TOKEN(aux_sym_ansi_color_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(392);
      END_STATE();
    case 393:
      ACCEPT_TOKEN(aux_sym_ansi_color_token2);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(393);
      END_STATE();
    case 394:
      ACCEPT_TOKEN(anon_sym_black);
      END_STATE();
    case 395:
      ACCEPT_TOKEN(anon_sym_darkgrey);
      END_STATE();
    case 396:
      ACCEPT_TOKEN(anon_sym_dark_DASHgrey);
      END_STATE();
    case 397:
      ACCEPT_TOKEN(anon_sym_dark_grey);
      END_STATE();
    case 398:
      ACCEPT_TOKEN(anon_sym_red);
      END_STATE();
    case 399:
      ACCEPT_TOKEN(anon_sym_darkred);
      END_STATE();
    case 400:
      ACCEPT_TOKEN(anon_sym_dark_DASHred);
      END_STATE();
    case 401:
      ACCEPT_TOKEN(anon_sym_dark_red);
      END_STATE();
    case 402:
      ACCEPT_TOKEN(anon_sym_green);
      END_STATE();
    case 403:
      ACCEPT_TOKEN(anon_sym_darkgreen);
      END_STATE();
    case 404:
      ACCEPT_TOKEN(anon_sym_dark_DASHgreen);
      END_STATE();
    case 405:
      ACCEPT_TOKEN(anon_sym_dark_green);
      END_STATE();
    case 406:
      ACCEPT_TOKEN(anon_sym_yellow);
      END_STATE();
    case 407:
      ACCEPT_TOKEN(anon_sym_darkyellow);
      END_STATE();
    case 408:
      ACCEPT_TOKEN(anon_sym_dark_DASHyellow);
      END_STATE();
    case 409:
      ACCEPT_TOKEN(anon_sym_dark_yellow);
      END_STATE();
    case 410:
      ACCEPT_TOKEN(anon_sym_blue);
      END_STATE();
    case 411:
      ACCEPT_TOKEN(anon_sym_darkblue);
      END_STATE();
    case 412:
      ACCEPT_TOKEN(anon_sym_dark_DASHblue);
      END_STATE();
    case 413:
      ACCEPT_TOKEN(anon_sym_dark_blue);
      END_STATE();
    case 414:
      ACCEPT_TOKEN(anon_sym_magenta);
      END_STATE();
    case 415:
      ACCEPT_TOKEN(anon_sym_darkmagenta);
      END_STATE();
    case 416:
      ACCEPT_TOKEN(anon_sym_dark_DASHmagenta);
      END_STATE();
    case 417:
      ACCEPT_TOKEN(anon_sym_dark_magenta);
      END_STATE();
    case 418:
      ACCEPT_TOKEN(anon_sym_cyan);
      END_STATE();
    case 419:
      ACCEPT_TOKEN(anon_sym_darkcyan);
      END_STATE();
    case 420:
      ACCEPT_TOKEN(anon_sym_dark_DASHcyan);
      END_STATE();
    case 421:
      ACCEPT_TOKEN(anon_sym_dark_cyan);
      END_STATE();
    case 422:
      ACCEPT_TOKEN(anon_sym_white);
      END_STATE();
    case 423:
      ACCEPT_TOKEN(anon_sym_grey);
      END_STATE();
    case 424:
      ACCEPT_TOKEN(anon_sym_bg_DASHcanvas);
      END_STATE();
    case 425:
      ACCEPT_TOKEN(anon_sym_fg_DASHcanvas);
      END_STATE();
    case 426:
      ACCEPT_TOKEN(anon_sym_canvas);
      END_STATE();
    case 427:
      ACCEPT_TOKEN(anon_sym_comment);
      END_STATE();
    case 428:
      ACCEPT_TOKEN(anon_sym_constant);
      END_STATE();
    case 429:
      ACCEPT_TOKEN(anon_sym_string);
      END_STATE();
    case 430:
      ACCEPT_TOKEN(anon_sym_escape_DASHseq);
      END_STATE();
    case 431:
      ACCEPT_TOKEN(anon_sym_char);
      END_STATE();
    case 432:
      ACCEPT_TOKEN(anon_sym_number);
      END_STATE();
    case 433:
      ACCEPT_TOKEN(anon_sym_boolean);
      END_STATE();
    case 434:
      ACCEPT_TOKEN(anon_sym_float);
      END_STATE();
    case 435:
      ACCEPT_TOKEN(anon_sym_identifier);
      END_STATE();
    case 436:
      ACCEPT_TOKEN(anon_sym_function);
      END_STATE();
    case 437:
      ACCEPT_TOKEN(anon_sym_statement);
      END_STATE();
    case 438:
      ACCEPT_TOKEN(anon_sym_conditional);
      END_STATE();
    case 439:
      ACCEPT_TOKEN(anon_sym_repeat);
      END_STATE();
    case 440:
      ACCEPT_TOKEN(anon_sym_label);
      END_STATE();
    case 441:
      ACCEPT_TOKEN(anon_sym_operator);
      END_STATE();
    case 442:
      ACCEPT_TOKEN(anon_sym_keyword);
      END_STATE();
    case 443:
      ACCEPT_TOKEN(anon_sym_exception);
      END_STATE();
    case 444:
      ACCEPT_TOKEN(anon_sym_preproc);
      END_STATE();
    case 445:
      ACCEPT_TOKEN(anon_sym_include);
      END_STATE();
    case 446:
      ACCEPT_TOKEN(anon_sym_define);
      END_STATE();
    case 447:
      ACCEPT_TOKEN(anon_sym_macro);
      END_STATE();
    case 448:
      ACCEPT_TOKEN(anon_sym_precondit);
      END_STATE();
    case 449:
      ACCEPT_TOKEN(anon_sym_type);
      if (lookahead == 'd') ADVANCE(110);
      END_STATE();
    case 450:
      ACCEPT_TOKEN(anon_sym_storage_DASHclass);
      END_STATE();
    case 451:
      ACCEPT_TOKEN(anon_sym_structure);
      END_STATE();
    case 452:
      ACCEPT_TOKEN(anon_sym_typedef);
      END_STATE();
    case 453:
      ACCEPT_TOKEN(anon_sym_special);
      if (lookahead == '-') ADVANCE(59);
      END_STATE();
    case 454:
      ACCEPT_TOKEN(anon_sym_special_DASHchar);
      END_STATE();
    case 455:
      ACCEPT_TOKEN(anon_sym_tag);
      END_STATE();
    case 456:
      ACCEPT_TOKEN(anon_sym_delimiter);
      END_STATE();
    case 457:
      ACCEPT_TOKEN(anon_sym_special_DASHcomment);
      END_STATE();
    case 458:
      ACCEPT_TOKEN(anon_sym_debug);
      END_STATE();
    case 459:
      ACCEPT_TOKEN(anon_sym_ignore);
      END_STATE();
    case 460:
      ACCEPT_TOKEN(anon_sym_error);
      END_STATE();
    case 461:
      ACCEPT_TOKEN(anon_sym_todo);
      END_STATE();
    case 462:
      ACCEPT_TOKEN(anon_sym_line_DASHnr);
      END_STATE();
    case 463:
      ACCEPT_TOKEN(anon_sym_prompt);
      END_STATE();
    case 464:
      ACCEPT_TOKEN(anon_sym_status_DASHline);
      END_STATE();
    case 465:
      ACCEPT_TOKEN(anon_sym_tab_DASHline);
      END_STATE();
    case 466:
      ACCEPT_TOKEN(anon_sym_tab_DASHoption);
      END_STATE();
    case 467:
      ACCEPT_TOKEN(anon_sym_tab_DASHselect);
      END_STATE();
    default:
      return false;
  }
}

static TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 360},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 2},
  [4] = {.lex_state = 2},
  [5] = {.lex_state = 360},
  [6] = {.lex_state = 360},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 3},
  [16] = {.lex_state = 3},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 360},
  [19] = {.lex_state = 360},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 360},
  [22] = {.lex_state = 360},
  [23] = {.lex_state = 360},
  [24] = {.lex_state = 0},
  [25] = {.lex_state = 360},
  [26] = {.lex_state = 360},
  [27] = {.lex_state = 360},
  [28] = {.lex_state = 360},
  [29] = {.lex_state = 360},
  [30] = {.lex_state = 360},
  [31] = {.lex_state = 0},
  [32] = {.lex_state = 0},
  [33] = {.lex_state = 0},
  [34] = {.lex_state = 0},
  [35] = {.lex_state = 0},
  [36] = {.lex_state = 0},
  [37] = {.lex_state = 0},
  [38] = {.lex_state = 0},
  [39] = {.lex_state = 0},
  [40] = {.lex_state = 0},
  [41] = {.lex_state = 0},
  [42] = {.lex_state = 0},
  [43] = {.lex_state = 0},
  [44] = {.lex_state = 0},
  [45] = {.lex_state = 0},
  [46] = {.lex_state = 0},
  [47] = {.lex_state = 0},
  [48] = {.lex_state = 360},
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
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
    [sym_comment] = ACTIONS(3),
    [sym_newline] = ACTIONS(3),
    [anon_sym_COMMA] = ACTIONS(1),
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
    [anon_sym_escape_DASHseq] = ACTIONS(1),
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
    [sym_s] = STATE(51),
    [sym_hl_rule] = STATE(5),
    [sym_selectors] = STATE(58),
    [sym_selector] = STATE(35),
    [sym_sel_symbol] = STATE(22),
    [sym_sel_twins] = STATE(23),
    [sym_sel_siblings] = STATE(23),
    [sym_sel_child] = STATE(23),
    [aux_sym_s_repeat1] = STATE(5),
    [aux_sym_selector_repeat1] = STATE(18),
    [ts_builtin_sym_end] = ACTIONS(5),
    [sym_comment] = ACTIONS(3),
    [sym_newline] = ACTIONS(3),
    [sym_sel_kind] = ACTIONS(7),
  },
};

static uint16_t ts_small_parse_table[] = {
  [0] = 5,
    ACTIONS(9), 1,
      anon_sym_LBRACE,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(13), 2,
      anon_sym_type,
      anon_sym_special,
    STATE(56), 2,
      sym_properties,
      sym_highlight,
    ACTIONS(11), 41,
      anon_sym_underlined,
      anon_sym_canvas,
      anon_sym_comment,
      anon_sym_constant,
      anon_sym_string,
      anon_sym_escape_DASHseq,
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
  [59] = 6,
    ACTIONS(15), 1,
      sym_rgb_color,
    ACTIONS(17), 1,
      aux_sym_ansi_color_token1,
    ACTIONS(19), 1,
      aux_sym_ansi_color_token2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(46), 2,
      sym_ansi_color,
      sym_color_name,
    ACTIONS(21), 32,
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
  [111] = 6,
    ACTIONS(17), 1,
      aux_sym_ansi_color_token1,
    ACTIONS(19), 1,
      aux_sym_ansi_color_token2,
    ACTIONS(23), 1,
      sym_rgb_color,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(37), 2,
      sym_ansi_color,
      sym_color_name,
    ACTIONS(21), 32,
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
  [163] = 9,
    ACTIONS(7), 1,
      sym_sel_kind,
    ACTIONS(25), 1,
      ts_builtin_sym_end,
    STATE(18), 1,
      aux_sym_selector_repeat1,
    STATE(22), 1,
      sym_sel_symbol,
    STATE(35), 1,
      sym_selector,
    STATE(58), 1,
      sym_selectors,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(6), 2,
      sym_hl_rule,
      aux_sym_s_repeat1,
    STATE(23), 3,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [195] = 9,
    ACTIONS(27), 1,
      ts_builtin_sym_end,
    ACTIONS(29), 1,
      sym_sel_kind,
    STATE(18), 1,
      aux_sym_selector_repeat1,
    STATE(22), 1,
      sym_sel_symbol,
    STATE(35), 1,
      sym_selector,
    STATE(58), 1,
      sym_selectors,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(6), 2,
      sym_hl_rule,
      aux_sym_s_repeat1,
    STATE(23), 3,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [227] = 6,
    ACTIONS(37), 1,
      anon_sym_underline,
    STATE(14), 1,
      sym_attr,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(32), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
    STATE(7), 2,
      sym_attrs,
      aux_sym_attrb_repeat1,
    ACTIONS(34), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [252] = 6,
    ACTIONS(44), 1,
      anon_sym_underline,
    STATE(14), 1,
      sym_attr,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(40), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
    STATE(12), 2,
      sym_attrs,
      aux_sym_attrb_repeat1,
    ACTIONS(42), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [277] = 6,
    ACTIONS(44), 1,
      anon_sym_underline,
    STATE(14), 1,
      sym_attr,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(46), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
    STATE(13), 2,
      sym_attrs,
      aux_sym_attrb_repeat1,
    ACTIONS(42), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [302] = 5,
    ACTIONS(50), 1,
      anon_sym_PIPE,
    ACTIONS(53), 1,
      anon_sym_underline,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(10), 2,
      sym_attr_or,
      aux_sym_attrs_repeat1,
    ACTIONS(48), 6,
      anon_sym_COMMA,
      anon_sym_RBRACE,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [325] = 5,
    ACTIONS(57), 1,
      anon_sym_PIPE,
    ACTIONS(59), 1,
      anon_sym_underline,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(10), 2,
      sym_attr_or,
      aux_sym_attrs_repeat1,
    ACTIONS(55), 6,
      anon_sym_COMMA,
      anon_sym_RBRACE,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [348] = 6,
    ACTIONS(44), 1,
      anon_sym_underline,
    STATE(14), 1,
      sym_attr,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(61), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
    STATE(7), 2,
      sym_attrs,
      aux_sym_attrb_repeat1,
    ACTIONS(42), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [373] = 6,
    ACTIONS(44), 1,
      anon_sym_underline,
    STATE(14), 1,
      sym_attr,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(63), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
    STATE(7), 2,
      sym_attrs,
      aux_sym_attrb_repeat1,
    ACTIONS(42), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [398] = 5,
    ACTIONS(57), 1,
      anon_sym_PIPE,
    ACTIONS(67), 1,
      anon_sym_underline,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(11), 2,
      sym_attr_or,
      aux_sym_attrs_repeat1,
    ACTIONS(65), 6,
      anon_sym_COMMA,
      anon_sym_RBRACE,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [421] = 10,
    ACTIONS(69), 1,
      anon_sym_fg,
    ACTIONS(71), 1,
      anon_sym_bg,
    ACTIONS(73), 1,
      anon_sym_attr,
    ACTIONS(75), 1,
      anon_sym_attribute,
    STATE(38), 1,
      sym_fg,
    STATE(39), 1,
      sym_bg,
    STATE(41), 1,
      sym_attrb,
    STATE(42), 1,
      sym_attribute,
    STATE(47), 1,
      sym_property,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [453] = 10,
    ACTIONS(69), 1,
      anon_sym_fg,
    ACTIONS(71), 1,
      anon_sym_bg,
    ACTIONS(73), 1,
      anon_sym_attr,
    ACTIONS(75), 1,
      anon_sym_attribute,
    STATE(31), 1,
      sym_property,
    STATE(38), 1,
      sym_fg,
    STATE(39), 1,
      sym_bg,
    STATE(41), 1,
      sym_attrb,
    STATE(42), 1,
      sym_attribute,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [485] = 3,
    ACTIONS(79), 1,
      anon_sym_underline,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(77), 7,
      anon_sym_COMMA,
      anon_sym_RBRACE,
      anon_sym_PIPE,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [502] = 6,
    ACTIONS(7), 1,
      sym_sel_kind,
    STATE(19), 1,
      aux_sym_selector_repeat1,
    STATE(22), 1,
      sym_sel_symbol,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(81), 2,
      anon_sym_COLON,
      anon_sym_COMMA,
    STATE(23), 3,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [525] = 6,
    ACTIONS(85), 1,
      sym_sel_kind,
    STATE(19), 1,
      aux_sym_selector_repeat1,
    STATE(22), 1,
      sym_sel_symbol,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(83), 2,
      anon_sym_COLON,
      anon_sym_COMMA,
    STATE(23), 3,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [548] = 3,
    ACTIONS(90), 1,
      anon_sym_underline,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(88), 7,
      anon_sym_COMMA,
      anon_sym_RBRACE,
      anon_sym_PIPE,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [565] = 6,
    ACTIONS(7), 1,
      sym_sel_kind,
    STATE(18), 1,
      aux_sym_selector_repeat1,
    STATE(22), 1,
      sym_sel_symbol,
    STATE(40), 1,
      sym_selector,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(23), 3,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [587] = 5,
    ACTIONS(94), 1,
      anon_sym_PLUS,
    ACTIONS(96), 1,
      anon_sym_TILDE,
    ACTIONS(98), 1,
      anon_sym_GT,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(92), 3,
      anon_sym_COLON,
      anon_sym_COMMA,
      sym_sel_kind,
  [606] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(100), 6,
      anon_sym_COLON,
      anon_sym_COMMA,
      sym_sel_kind,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
  [619] = 4,
    ACTIONS(44), 1,
      anon_sym_underline,
    STATE(20), 1,
      sym_attr,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(42), 4,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_reverse,
  [636] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(102), 6,
      anon_sym_COLON,
      anon_sym_COMMA,
      sym_sel_kind,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
  [649] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(104), 6,
      anon_sym_COLON,
      anon_sym_COMMA,
      sym_sel_kind,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
  [662] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(106), 6,
      anon_sym_COLON,
      anon_sym_COMMA,
      sym_sel_kind,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
  [675] = 4,
    ACTIONS(7), 1,
      sym_sel_kind,
    STATE(27), 1,
      sym_sel_symbol,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(23), 3,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [691] = 4,
    ACTIONS(7), 1,
      sym_sel_kind,
    STATE(26), 1,
      sym_sel_symbol,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(23), 3,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [707] = 4,
    ACTIONS(7), 1,
      sym_sel_kind,
    STATE(25), 1,
      sym_sel_symbol,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(23), 3,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [723] = 4,
    ACTIONS(108), 1,
      anon_sym_COMMA,
    ACTIONS(110), 1,
      anon_sym_RBRACE,
    STATE(36), 1,
      aux_sym_properties_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [737] = 4,
    ACTIONS(112), 1,
      anon_sym_COMMA,
    ACTIONS(115), 1,
      anon_sym_RBRACE,
    STATE(32), 1,
      aux_sym_properties_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [751] = 4,
    ACTIONS(117), 1,
      anon_sym_COLON,
    ACTIONS(119), 1,
      anon_sym_COMMA,
    STATE(34), 1,
      aux_sym_selectors_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [765] = 4,
    ACTIONS(121), 1,
      anon_sym_COLON,
    ACTIONS(123), 1,
      anon_sym_COMMA,
    STATE(34), 1,
      aux_sym_selectors_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [779] = 4,
    ACTIONS(119), 1,
      anon_sym_COMMA,
    ACTIONS(126), 1,
      anon_sym_COLON,
    STATE(33), 1,
      aux_sym_selectors_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [793] = 4,
    ACTIONS(108), 1,
      anon_sym_COMMA,
    ACTIONS(128), 1,
      anon_sym_RBRACE,
    STATE(32), 1,
      aux_sym_properties_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [807] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(130), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
  [816] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(132), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
  [825] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(134), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
  [834] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(121), 2,
      anon_sym_COLON,
      anon_sym_COMMA,
  [843] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(136), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
  [852] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(138), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
  [861] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(140), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
  [870] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(142), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
  [879] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(144), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
  [888] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(146), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
  [897] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(115), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
  [906] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(148), 2,
      ts_builtin_sym_end,
      sym_sel_kind,
  [915] = 2,
    ACTIONS(150), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [923] = 2,
    ACTIONS(152), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [931] = 2,
    ACTIONS(154), 1,
      ts_builtin_sym_end,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [939] = 2,
    ACTIONS(156), 1,
      anon_sym_SEMI,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [947] = 2,
    ACTIONS(158), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [955] = 2,
    ACTIONS(160), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [963] = 2,
    ACTIONS(162), 1,
      anon_sym_SEMI,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [971] = 2,
    ACTIONS(164), 1,
      anon_sym_SEMI,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [979] = 2,
    ACTIONS(166), 1,
      anon_sym_SEMI,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [987] = 2,
    ACTIONS(168), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
};

static uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 59,
  [SMALL_STATE(4)] = 111,
  [SMALL_STATE(5)] = 163,
  [SMALL_STATE(6)] = 195,
  [SMALL_STATE(7)] = 227,
  [SMALL_STATE(8)] = 252,
  [SMALL_STATE(9)] = 277,
  [SMALL_STATE(10)] = 302,
  [SMALL_STATE(11)] = 325,
  [SMALL_STATE(12)] = 348,
  [SMALL_STATE(13)] = 373,
  [SMALL_STATE(14)] = 398,
  [SMALL_STATE(15)] = 421,
  [SMALL_STATE(16)] = 453,
  [SMALL_STATE(17)] = 485,
  [SMALL_STATE(18)] = 502,
  [SMALL_STATE(19)] = 525,
  [SMALL_STATE(20)] = 548,
  [SMALL_STATE(21)] = 565,
  [SMALL_STATE(22)] = 587,
  [SMALL_STATE(23)] = 606,
  [SMALL_STATE(24)] = 619,
  [SMALL_STATE(25)] = 636,
  [SMALL_STATE(26)] = 649,
  [SMALL_STATE(27)] = 662,
  [SMALL_STATE(28)] = 675,
  [SMALL_STATE(29)] = 691,
  [SMALL_STATE(30)] = 707,
  [SMALL_STATE(31)] = 723,
  [SMALL_STATE(32)] = 737,
  [SMALL_STATE(33)] = 751,
  [SMALL_STATE(34)] = 765,
  [SMALL_STATE(35)] = 779,
  [SMALL_STATE(36)] = 793,
  [SMALL_STATE(37)] = 807,
  [SMALL_STATE(38)] = 816,
  [SMALL_STATE(39)] = 825,
  [SMALL_STATE(40)] = 834,
  [SMALL_STATE(41)] = 843,
  [SMALL_STATE(42)] = 852,
  [SMALL_STATE(43)] = 861,
  [SMALL_STATE(44)] = 870,
  [SMALL_STATE(45)] = 879,
  [SMALL_STATE(46)] = 888,
  [SMALL_STATE(47)] = 897,
  [SMALL_STATE(48)] = 906,
  [SMALL_STATE(49)] = 915,
  [SMALL_STATE(50)] = 923,
  [SMALL_STATE(51)] = 931,
  [SMALL_STATE(52)] = 939,
  [SMALL_STATE(53)] = 947,
  [SMALL_STATE(54)] = 955,
  [SMALL_STATE(55)] = 963,
  [SMALL_STATE(56)] = 971,
  [SMALL_STATE(57)] = 979,
  [SMALL_STATE(58)] = 987,
};

static TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_s, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(57),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(46),
  [17] = {.entry = {.count = 1, .reusable = false}}, SHIFT(43),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [23] = {.entry = {.count = 1, .reusable = false}}, SHIFT(37),
  [25] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_s, 1),
  [27] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_s_repeat1, 2),
  [29] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_s_repeat1, 2), SHIFT_REPEAT(23),
  [32] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_attrb_repeat1, 2),
  [34] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_attrb_repeat1, 2), SHIFT_REPEAT(17),
  [37] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_attrb_repeat1, 2), SHIFT_REPEAT(17),
  [40] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute, 2),
  [42] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [44] = {.entry = {.count = 1, .reusable = false}}, SHIFT(17),
  [46] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attrb, 2),
  [48] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_attrs_repeat1, 2),
  [50] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_attrs_repeat1, 2), SHIFT_REPEAT(24),
  [53] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_attrs_repeat1, 2),
  [55] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attrs, 2),
  [57] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [59] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attrs, 2),
  [61] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute, 3),
  [63] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attrb, 3),
  [65] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attrs, 1),
  [67] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attrs, 1),
  [69] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [71] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [73] = {.entry = {.count = 1, .reusable = false}}, SHIFT(50),
  [75] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [77] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attr, 1),
  [79] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attr, 1),
  [81] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selector, 1),
  [83] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_selector_repeat1, 2),
  [85] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_selector_repeat1, 2), SHIFT_REPEAT(23),
  [88] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attr_or, 2),
  [90] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attr_or, 2),
  [92] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_selector_repeat1, 1),
  [94] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [96] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [98] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [100] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_symbol, 1),
  [102] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_twins, 3),
  [104] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_siblings, 3),
  [106] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_child, 3),
  [108] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [110] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [112] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_properties_repeat1, 2), SHIFT_REPEAT(15),
  [115] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_properties_repeat1, 2),
  [117] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selectors, 2),
  [119] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [121] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_selectors_repeat1, 2),
  [123] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_selectors_repeat1, 2), SHIFT_REPEAT(21),
  [126] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selectors, 1),
  [128] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [130] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fg, 3),
  [132] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property, 1, .production_id = 1),
  [134] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property, 1, .production_id = 2),
  [136] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property, 1, .production_id = 3),
  [138] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property, 1, .production_id = 4),
  [140] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ansi_color, 1, .production_id = 6),
  [142] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ansi_color, 1, .production_id = 7),
  [144] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_color_name, 1),
  [146] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_bg, 3),
  [148] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hl_rule, 4, .production_id = 5),
  [150] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [152] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [154] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [156] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_properties, 4),
  [158] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [160] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [162] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_properties, 3),
  [164] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [166] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_highlight, 1),
  [168] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
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
