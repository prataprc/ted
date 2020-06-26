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
#define STATE_COUNT 56
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 137
#define ALIAS_COUNT 0
#define TOKEN_COUNT 111
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 3
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
  anon_sym_dim = 21,
  anon_sym_slowblink = 22,
  anon_sym_slow_DASHblink = 23,
  anon_sym_slow_blink = 24,
  anon_sym_rapidblink = 25,
  anon_sym_rapid_DASHblink = 26,
  anon_sym_rapid_blink = 27,
  anon_sym_crossedout = 28,
  anon_sym_crossed_DASHout = 29,
  anon_sym_crossed_out = 30,
  anon_sym_framed = 31,
  anon_sym_encircled = 32,
  anon_sym_reverse = 33,
  sym_rgb_color = 34,
  aux_sym_ansi_color_token1 = 35,
  aux_sym_ansi_color_token2 = 36,
  anon_sym_black = 37,
  anon_sym_darkgrey = 38,
  anon_sym_dark_DASHgrey = 39,
  anon_sym_dark_grey = 40,
  anon_sym_red = 41,
  anon_sym_darkred = 42,
  anon_sym_dark_DASHred = 43,
  anon_sym_dark_red = 44,
  anon_sym_green = 45,
  anon_sym_darkgreen = 46,
  anon_sym_dark_DASHgreen = 47,
  anon_sym_dark_green = 48,
  anon_sym_yellow = 49,
  anon_sym_darkyellow = 50,
  anon_sym_dark_DASHyellow = 51,
  anon_sym_dark_yellow = 52,
  anon_sym_blue = 53,
  anon_sym_darkblue = 54,
  anon_sym_dark_DASHblue = 55,
  anon_sym_dark_blue = 56,
  anon_sym_magenta = 57,
  anon_sym_darkmagenta = 58,
  anon_sym_dark_DASHmagenta = 59,
  anon_sym_dark_magenta = 60,
  anon_sym_cyan = 61,
  anon_sym_darkcyan = 62,
  anon_sym_dark_DASHcyan = 63,
  anon_sym_dark_cyan = 64,
  anon_sym_white = 65,
  anon_sym_grey = 66,
  anon_sym_bg_DASHcanvas = 67,
  anon_sym_fg_DASHcanvas = 68,
  anon_sym_canvas = 69,
  anon_sym_comment = 70,
  anon_sym_constant = 71,
  anon_sym_string = 72,
  anon_sym_escape_DASHseq = 73,
  anon_sym_char = 74,
  anon_sym_number = 75,
  anon_sym_boolean = 76,
  anon_sym_float = 77,
  anon_sym_identifier = 78,
  anon_sym_function = 79,
  anon_sym_statement = 80,
  anon_sym_conditional = 81,
  anon_sym_repeat = 82,
  anon_sym_label = 83,
  anon_sym_operator = 84,
  anon_sym_keyword = 85,
  anon_sym_exception = 86,
  anon_sym_preproc = 87,
  anon_sym_include = 88,
  anon_sym_define = 89,
  anon_sym_macro = 90,
  anon_sym_precondit = 91,
  anon_sym_type = 92,
  anon_sym_storage_DASHclass = 93,
  anon_sym_structure = 94,
  anon_sym_typedef = 95,
  anon_sym_special = 96,
  anon_sym_special_DASHchar = 97,
  anon_sym_tag = 98,
  anon_sym_delimiter = 99,
  anon_sym_special_DASHcomment = 100,
  anon_sym_debug = 101,
  anon_sym_ignore = 102,
  anon_sym_error = 103,
  anon_sym_todo = 104,
  anon_sym_line_DASHnr = 105,
  anon_sym_prompt = 106,
  anon_sym_status_DASHline = 107,
  anon_sym_tab_DASHline = 108,
  anon_sym_tab_DASHoption = 109,
  anon_sym_tab_DASHselect = 110,
  sym_s = 111,
  sym_hl_rule = 112,
  sym_selectors = 113,
  sym_selector = 114,
  sym_sel_symbol = 115,
  sym_sel_twins = 116,
  sym_sel_siblings = 117,
  sym_sel_child = 118,
  sym_properties = 119,
  sym_property = 120,
  sym_fg = 121,
  sym_bg = 122,
  sym_attrb = 123,
  sym_attribute = 124,
  sym_attrs = 125,
  sym_attr_or = 126,
  sym_attr = 127,
  sym_ansi_color = 128,
  sym_color_name = 129,
  sym_highlight = 130,
  aux_sym_s_repeat1 = 131,
  aux_sym_selectors_repeat1 = 132,
  aux_sym_selector_repeat1 = 133,
  aux_sym_properties_repeat1 = 134,
  aux_sym_attrb_repeat1 = 135,
  aux_sym_attrs_repeat1 = 136,
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
  [anon_sym_dim] = "dim",
  [anon_sym_slowblink] = "slowblink",
  [anon_sym_slow_DASHblink] = "slow-blink",
  [anon_sym_slow_blink] = "slow_blink",
  [anon_sym_rapidblink] = "rapidblink",
  [anon_sym_rapid_DASHblink] = "rapid-blink",
  [anon_sym_rapid_blink] = "rapid_blink",
  [anon_sym_crossedout] = "crossedout",
  [anon_sym_crossed_DASHout] = "crossed-out",
  [anon_sym_crossed_out] = "crossed_out",
  [anon_sym_framed] = "framed",
  [anon_sym_encircled] = "encircled",
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
  [anon_sym_dim] = anon_sym_dim,
  [anon_sym_slowblink] = anon_sym_slowblink,
  [anon_sym_slow_DASHblink] = anon_sym_slow_DASHblink,
  [anon_sym_slow_blink] = anon_sym_slow_blink,
  [anon_sym_rapidblink] = anon_sym_rapidblink,
  [anon_sym_rapid_DASHblink] = anon_sym_rapid_DASHblink,
  [anon_sym_rapid_blink] = anon_sym_rapid_blink,
  [anon_sym_crossedout] = anon_sym_crossedout,
  [anon_sym_crossed_DASHout] = anon_sym_crossed_DASHout,
  [anon_sym_crossed_out] = anon_sym_crossed_out,
  [anon_sym_framed] = anon_sym_framed,
  [anon_sym_encircled] = anon_sym_encircled,
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
  [anon_sym_dim] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_slowblink] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_slow_DASHblink] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_slow_blink] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_rapidblink] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_rapid_DASHblink] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_rapid_blink] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_crossedout] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_crossed_DASHout] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_crossed_out] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_framed] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_encircled] = {
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
  field_selectors = 3,
};

static const char *ts_field_names[] = {
  [0] = NULL,
  [field_ansi_color_dec] = "ansi_color_dec",
  [field_ansi_color_hex] = "ansi_color_hex",
  [field_selectors] = "selectors",
};

static const TSFieldMapSlice ts_field_map_slices[4] = {
  [1] = {.index = 0, .length = 1},
  [2] = {.index = 1, .length = 1},
  [3] = {.index = 2, .length = 1},
};

static const TSFieldMapEntry ts_field_map_entries[] = {
  [0] =
    {field_selectors, 0},
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
      if (eof) ADVANCE(422);
      if (lookahead == '\n') ADVANCE(432);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '#') ADVANCE(430);
      if (lookahead == '+') ADVANCE(435);
      if (lookahead == ',') ADVANCE(433);
      if (lookahead == '0') ADVANCE(464);
      if (lookahead == ':') ADVANCE(423);
      if (lookahead == ';') ADVANCE(424);
      if (lookahead == '>') ADVANCE(437);
      if (lookahead == 'a') ADVANCE(377);
      if (lookahead == 'b') ADVANCE(166);
      if (lookahead == 'c') ADVANCE(23);
      if (lookahead == 'd') ADVANCE(27);
      if (lookahead == 'e') ADVANCE(256);
      if (lookahead == 'f') ADVANCE(167);
      if (lookahead == 'g') ADVANCE(339);
      if (lookahead == 'i') ADVANCE(94);
      if (lookahead == 'k') ADVANCE(101);
      if (lookahead == 'l') ADVANCE(15);
      if (lookahead == 'm') ADVANCE(16);
      if (lookahead == 'n') ADVANCE(395);
      if (lookahead == 'o') ADVANCE(320);
      if (lookahead == 'p') ADVANCE(328);
      if (lookahead == 'r') ADVANCE(22);
      if (lookahead == 's') ADVANCE(219);
      if (lookahead == 't') ADVANCE(17);
      if (lookahead == 'u') ADVANCE(270);
      if (lookahead == 'w') ADVANCE(179);
      if (lookahead == 'y') ADVANCE(102);
      if (lookahead == '{') ADVANCE(438);
      if (lookahead == '|') ADVANCE(446);
      if (lookahead == '}') ADVANCE(439);
      if (lookahead == '~') ADVANCE(436);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(0)
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(465);
      END_STATE();
    case 1:
      if (lookahead == '\n') ADVANCE(432);
      END_STATE();
    case 2:
      if (lookahead == '\n') ADVANCE(432);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '#') ADVANCE(430);
      if (lookahead == '0') ADVANCE(464);
      if (lookahead == 'b') ADVANCE(172);
      if (lookahead == 'c') ADVANCE(415);
      if (lookahead == 'd') ADVANCE(26);
      if (lookahead == 'f') ADVANCE(178);
      if (lookahead == 'g') ADVANCE(339);
      if (lookahead == 'm') ADVANCE(54);
      if (lookahead == 'r') ADVANCE(145);
      if (lookahead == 'w') ADVANCE(179);
      if (lookahead == 'y') ADVANCE(102);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(2)
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(465);
      END_STATE();
    case 3:
      if (lookahead == '\n') ADVANCE(432);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '#') ADVANCE(431);
      if (lookahead == 'a') ADVANCE(377);
      if (lookahead == 'b') ADVANCE(170);
      if (lookahead == 'f') ADVANCE(171);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(3)
      END_STATE();
    case 4:
      if (lookahead == '-') ADVANCE(243);
      END_STATE();
    case 5:
      if (lookahead == '-') ADVANCE(59);
      if (lookahead == '_') ADVANCE(60);
      if (lookahead == 'b') ADVANCE(221);
      if (lookahead == 'c') ADVANCE(416);
      if (lookahead == 'g') ADVANCE(348);
      if (lookahead == 'm') ADVANCE(47);
      if (lookahead == 'r') ADVANCE(134);
      if (lookahead == 'y') ADVANCE(160);
      END_STATE();
    case 6:
      if (lookahead == '-') ADVANCE(62);
      if (lookahead == '_') ADVANCE(63);
      if (lookahead == 'b') ADVANCE(232);
      END_STATE();
    case 7:
      if (lookahead == '-') ADVANCE(78);
      END_STATE();
    case 8:
      if (lookahead == '-') ADVANCE(285);
      END_STATE();
    case 9:
      if (lookahead == '-') ADVANCE(318);
      if (lookahead == '_') ADVANCE(319);
      if (lookahead == 'o') ADVANCE(396);
      END_STATE();
    case 10:
      if (lookahead == '-') ADVANCE(76);
      END_STATE();
    case 11:
      if (lookahead == '-') ADVANCE(363);
      END_STATE();
    case 12:
      if (lookahead == '-') ADVANCE(83);
      END_STATE();
    case 13:
      if (lookahead == '-') ADVANCE(245);
      END_STATE();
    case 14:
      if (lookahead == '-') ADVANCE(64);
      if (lookahead == '_') ADVANCE(65);
      if (lookahead == 'b') ADVANCE(235);
      END_STATE();
    case 15:
      if (lookahead == 'a') ADVANCE(58);
      if (lookahead == 'i') ADVANCE(282);
      END_STATE();
    case 16:
      if (lookahead == 'a') ADVANCE(82);
      END_STATE();
    case 17:
      if (lookahead == 'a') ADVANCE(55);
      if (lookahead == 'o') ADVANCE(95);
      if (lookahead == 'y') ADVANCE(323);
      END_STATE();
    case 18:
      if (lookahead == 'a') ADVANCE(487);
      END_STATE();
    case 19:
      if (lookahead == 'a') ADVANCE(488);
      END_STATE();
    case 20:
      if (lookahead == 'a') ADVANCE(489);
      END_STATE();
    case 21:
      if (lookahead == 'a') ADVANCE(490);
      END_STATE();
    case 22:
      if (lookahead == 'a') ADVANCE(321);
      if (lookahead == 'e') ADVANCE(85);
      END_STATE();
    case 23:
      if (lookahead == 'a') ADVANCE(257);
      if (lookahead == 'h') ADVANCE(33);
      if (lookahead == 'o') ADVANCE(247);
      if (lookahead == 'r') ADVANCE(299);
      if (lookahead == 'y') ADVANCE(34);
      END_STATE();
    case 24:
      if (lookahead == 'a') ADVANCE(173);
      END_STATE();
    case 25:
      if (lookahead == 'a') ADVANCE(364);
      if (lookahead == 'o') ADVANCE(346);
      if (lookahead == 'r') ADVANCE(187);
      END_STATE();
    case 26:
      if (lookahead == 'a') ADVANCE(329);
      END_STATE();
    case 27:
      if (lookahead == 'a') ADVANCE(329);
      if (lookahead == 'e') ADVANCE(56);
      if (lookahead == 'i') ADVANCE(246);
      END_STATE();
    case 28:
      if (lookahead == 'a') ADVANCE(354);
      END_STATE();
    case 29:
      if (lookahead == 'a') ADVANCE(223);
      END_STATE();
    case 30:
      if (lookahead == 'a') ADVANCE(69);
      if (lookahead == 'u') ADVANCE(105);
      END_STATE();
    case 31:
      if (lookahead == 'a') ADVANCE(355);
      END_STATE();
    case 32:
      if (lookahead == 'a') ADVANCE(365);
      END_STATE();
    case 33:
      if (lookahead == 'a') ADVANCE(331);
      END_STATE();
    case 34:
      if (lookahead == 'a') ADVANCE(258);
      END_STATE();
    case 35:
      if (lookahead == 'a') ADVANCE(356);
      END_STATE();
    case 36:
      if (lookahead == 'a') ADVANCE(389);
      END_STATE();
    case 37:
      if (lookahead == 'a') ADVANCE(216);
      END_STATE();
    case 38:
      if (lookahead == 'a') ADVANCE(367);
      END_STATE();
    case 39:
      if (lookahead == 'a') ADVANCE(217);
      END_STATE();
    case 40:
      if (lookahead == 'a') ADVANCE(260);
      END_STATE();
    case 41:
      if (lookahead == 'a') ADVANCE(261);
      END_STATE();
    case 42:
      if (lookahead == 'a') ADVANCE(338);
      END_STATE();
    case 43:
      if (lookahead == 'a') ADVANCE(263);
      END_STATE();
    case 44:
      if (lookahead == 'a') ADVANCE(264);
      END_STATE();
    case 45:
      if (lookahead == 'a') ADVANCE(360);
      END_STATE();
    case 46:
      if (lookahead == 'a') ADVANCE(293);
      END_STATE();
    case 47:
      if (lookahead == 'a') ADVANCE(175);
      END_STATE();
    case 48:
      if (lookahead == 'a') ADVANCE(248);
      END_STATE();
    case 49:
      if (lookahead == 'a') ADVANCE(284);
      END_STATE();
    case 50:
      if (lookahead == 'a') ADVANCE(324);
      END_STATE();
    case 51:
      if (lookahead == 'a') ADVANCE(296);
      END_STATE();
    case 52:
      if (lookahead == 'a') ADVANCE(176);
      END_STATE();
    case 53:
      if (lookahead == 'a') ADVANCE(177);
      END_STATE();
    case 54:
      if (lookahead == 'a') ADVANCE(174);
      END_STATE();
    case 55:
      if (lookahead == 'b') ADVANCE(4);
      if (lookahead == 'g') ADVANCE(528);
      END_STATE();
    case 56:
      if (lookahead == 'b') ADVANCE(394);
      if (lookahead == 'f') ADVANCE(198);
      if (lookahead == 'l') ADVANCE(184);
      END_STATE();
    case 57:
      if (lookahead == 'b') ADVANCE(403);
      END_STATE();
    case 58:
      if (lookahead == 'b') ADVANCE(131);
      END_STATE();
    case 59:
      if (lookahead == 'b') ADVANCE(233);
      if (lookahead == 'c') ADVANCE(417);
      if (lookahead == 'g') ADVANCE(349);
      if (lookahead == 'm') ADVANCE(52);
      if (lookahead == 'r') ADVANCE(138);
      if (lookahead == 'y') ADVANCE(162);
      END_STATE();
    case 60:
      if (lookahead == 'b') ADVANCE(236);
      if (lookahead == 'c') ADVANCE(418);
      if (lookahead == 'g') ADVANCE(350);
      if (lookahead == 'm') ADVANCE(53);
      if (lookahead == 'r') ADVANCE(142);
      if (lookahead == 'y') ADVANCE(163);
      END_STATE();
    case 61:
      if (lookahead == 'b') ADVANCE(144);
      END_STATE();
    case 62:
      if (lookahead == 'b') ADVANCE(238);
      END_STATE();
    case 63:
      if (lookahead == 'b') ADVANCE(240);
      END_STATE();
    case 64:
      if (lookahead == 'b') ADVANCE(241);
      END_STATE();
    case 65:
      if (lookahead == 'b') ADVANCE(242);
      END_STATE();
    case 66:
      if (lookahead == 'c') ADVANCE(448);
      END_STATE();
    case 67:
      if (lookahead == 'c') ADVANCE(517);
      END_STATE();
    case 68:
      if (lookahead == 'c') ADVANCE(180);
      END_STATE();
    case 69:
      if (lookahead == 'c') ADVANCE(208);
      END_STATE();
    case 70:
      if (lookahead == 'c') ADVANCE(218);
      END_STATE();
    case 71:
      if (lookahead == 'c') ADVANCE(185);
      END_STATE();
    case 72:
      if (lookahead == 'c') ADVANCE(50);
      END_STATE();
    case 73:
      if (lookahead == 'c') ADVANCE(380);
      END_STATE();
    case 74:
      if (lookahead == 'c') ADVANCE(189);
      END_STATE();
    case 75:
      if (lookahead == 'c') ADVANCE(124);
      END_STATE();
    case 76:
      if (lookahead == 'c') ADVANCE(46);
      END_STATE();
    case 77:
      if (lookahead == 'c') ADVANCE(317);
      if (lookahead == 'p') ADVANCE(345);
      END_STATE();
    case 78:
      if (lookahead == 'c') ADVANCE(229);
      END_STATE();
    case 79:
      if (lookahead == 'c') ADVANCE(379);
      END_STATE();
    case 80:
      if (lookahead == 'c') ADVANCE(373);
      END_STATE();
    case 81:
      if (lookahead == 'c') ADVANCE(230);
      END_STATE();
    case 82:
      if (lookahead == 'c') ADVANCE(341);
      if (lookahead == 'g') ADVANCE(136);
      END_STATE();
    case 83:
      if (lookahead == 'c') ADVANCE(51);
      END_STATE();
    case 84:
      if (lookahead == 'd') ADVANCE(471);
      END_STATE();
    case 85:
      if (lookahead == 'd') ADVANCE(471);
      if (lookahead == 'p') ADVANCE(140);
      if (lookahead == 'v') ADVANCE(137);
      END_STATE();
    case 86:
      if (lookahead == 'd') ADVANCE(447);
      END_STATE();
    case 87:
      if (lookahead == 'd') ADVANCE(461);
      END_STATE();
    case 88:
      if (lookahead == 'd') ADVANCE(9);
      END_STATE();
    case 89:
      if (lookahead == 'd') ADVANCE(472);
      END_STATE();
    case 90:
      if (lookahead == 'd') ADVANCE(515);
      END_STATE();
    case 91:
      if (lookahead == 'd') ADVANCE(473);
      END_STATE();
    case 92:
      if (lookahead == 'd') ADVANCE(474);
      END_STATE();
    case 93:
      if (lookahead == 'd') ADVANCE(462);
      END_STATE();
    case 94:
      if (lookahead == 'd') ADVANCE(133);
      if (lookahead == 'g') ADVANCE(274);
      if (lookahead == 'n') ADVANCE(70);
      if (lookahead == 't') ADVANCE(29);
      END_STATE();
    case 95:
      if (lookahead == 'd') ADVANCE(300);
      END_STATE();
    case 96:
      if (lookahead == 'd') ADVANCE(110);
      END_STATE();
    case 97:
      if (lookahead == 'd') ADVANCE(206);
      if (lookahead == 's') ADVANCE(388);
      END_STATE();
    case 98:
      if (lookahead == 'd') ADVANCE(190);
      END_STATE();
    case 99:
      if (lookahead == 'd') ADVANCE(139);
      END_STATE();
    case 100:
      if (lookahead == 'd') ADVANCE(14);
      END_STATE();
    case 101:
      if (lookahead == 'e') ADVANCE(414);
      END_STATE();
    case 102:
      if (lookahead == 'e') ADVANCE(222);
      END_STATE();
    case 103:
      if (lookahead == 'e') ADVANCE(135);
      END_STATE();
    case 104:
      if (lookahead == 'e') ADVANCE(77);
      if (lookahead == 'o') ADVANCE(250);
      END_STATE();
    case 105:
      if (lookahead == 'e') ADVANCE(483);
      END_STATE();
    case 106:
      if (lookahead == 'e') ADVANCE(522);
      END_STATE();
    case 107:
      if (lookahead == 'e') ADVANCE(495);
      END_STATE();
    case 108:
      if (lookahead == 'e') ADVANCE(519);
      END_STATE();
    case 109:
      if (lookahead == 'e') ADVANCE(532);
      END_STATE();
    case 110:
      if (lookahead == 'e') ADVANCE(518);
      END_STATE();
    case 111:
      if (lookahead == 'e') ADVANCE(463);
      END_STATE();
    case 112:
      if (lookahead == 'e') ADVANCE(484);
      END_STATE();
    case 113:
      if (lookahead == 'e') ADVANCE(538);
      END_STATE();
    case 114:
      if (lookahead == 'e') ADVANCE(445);
      END_STATE();
    case 115:
      if (lookahead == 'e') ADVANCE(485);
      END_STATE();
    case 116:
      if (lookahead == 'e') ADVANCE(486);
      END_STATE();
    case 117:
      if (lookahead == 'e') ADVANCE(327);
      END_STATE();
    case 118:
      if (lookahead == 'e') ADVANCE(524);
      END_STATE();
    case 119:
      if (lookahead == 'e') ADVANCE(450);
      END_STATE();
    case 120:
      if (lookahead == 'e') ADVANCE(537);
      END_STATE();
    case 121:
      if (lookahead == 'e') ADVANCE(148);
      END_STATE();
    case 122:
      if (lookahead == 'e') ADVANCE(8);
      END_STATE();
    case 123:
      if (lookahead == 'e') ADVANCE(164);
      END_STATE();
    case 124:
      if (lookahead == 'e') ADVANCE(325);
      END_STATE();
    case 125:
      if (lookahead == 'e') ADVANCE(149);
      END_STATE();
    case 126:
      if (lookahead == 'e') ADVANCE(11);
      END_STATE();
    case 127:
      if (lookahead == 'e') ADVANCE(150);
      END_STATE();
    case 128:
      if (lookahead == 'e') ADVANCE(87);
      END_STATE();
    case 129:
      if (lookahead == 'e') ADVANCE(7);
      END_STATE();
    case 130:
      if (lookahead == 'e') ADVANCE(88);
      END_STATE();
    case 131:
      if (lookahead == 'e') ADVANCE(215);
      END_STATE();
    case 132:
      if (lookahead == 'e') ADVANCE(344);
      END_STATE();
    case 133:
      if (lookahead == 'e') ADVANCE(294);
      END_STATE();
    case 134:
      if (lookahead == 'e') ADVANCE(89);
      END_STATE();
    case 135:
      if (lookahead == 'e') ADVANCE(259);
      if (lookahead == 'y') ADVANCE(496);
      END_STATE();
    case 136:
      if (lookahead == 'e') ADVANCE(279);
      END_STATE();
    case 137:
      if (lookahead == 'e') ADVANCE(352);
      END_STATE();
    case 138:
      if (lookahead == 'e') ADVANCE(91);
      END_STATE();
    case 139:
      if (lookahead == 'e') ADVANCE(353);
      END_STATE();
    case 140:
      if (lookahead == 'e') ADVANCE(38);
      END_STATE();
    case 141:
      if (lookahead == 'e') ADVANCE(283);
      END_STATE();
    case 142:
      if (lookahead == 'e') ADVANCE(92);
      END_STATE();
    case 143:
      if (lookahead == 'e') ADVANCE(93);
      END_STATE();
    case 144:
      if (lookahead == 'e') ADVANCE(333);
      END_STATE();
    case 145:
      if (lookahead == 'e') ADVANCE(84);
      END_STATE();
    case 146:
      if (lookahead == 'e') ADVANCE(336);
      END_STATE();
    case 147:
      if (lookahead == 'e') ADVANCE(337);
      END_STATE();
    case 148:
      if (lookahead == 'e') ADVANCE(265);
      if (lookahead == 'y') ADVANCE(468);
      END_STATE();
    case 149:
      if (lookahead == 'e') ADVANCE(267);
      if (lookahead == 'y') ADVANCE(469);
      END_STATE();
    case 150:
      if (lookahead == 'e') ADVANCE(268);
      if (lookahead == 'y') ADVANCE(470);
      END_STATE();
    case 151:
      if (lookahead == 'e') ADVANCE(74);
      END_STATE();
    case 152:
      if (lookahead == 'e') ADVANCE(40);
      END_STATE();
    case 153:
      if (lookahead == 'e') ADVANCE(295);
      END_STATE();
    case 154:
      if (lookahead == 'e') ADVANCE(231);
      END_STATE();
    case 155:
      if (lookahead == 'e') ADVANCE(80);
      END_STATE();
    case 156:
      if (lookahead == 'e') ADVANCE(297);
      END_STATE();
    case 157:
      if (lookahead == 'e') ADVANCE(286);
      END_STATE();
    case 158:
      if (lookahead == 'e') ADVANCE(298);
      END_STATE();
    case 159:
      if (lookahead == 'e') ADVANCE(289);
      END_STATE();
    case 160:
      if (lookahead == 'e') ADVANCE(234);
      END_STATE();
    case 161:
      if (lookahead == 'e') ADVANCE(252);
      if (lookahead == 'u') ADVANCE(359);
      END_STATE();
    case 162:
      if (lookahead == 'e') ADVANCE(237);
      END_STATE();
    case 163:
      if (lookahead == 'e') ADVANCE(239);
      END_STATE();
    case 164:
      if (lookahead == 'f') ADVANCE(525);
      END_STATE();
    case 165:
      if (lookahead == 'f') ADVANCE(205);
      END_STATE();
    case 166:
      if (lookahead == 'g') ADVANCE(443);
      if (lookahead == 'l') ADVANCE(30);
      if (lookahead == 'o') ADVANCE(220);
      END_STATE();
    case 167:
      if (lookahead == 'g') ADVANCE(441);
      if (lookahead == 'l') ADVANCE(307);
      if (lookahead == 'r') ADVANCE(48);
      if (lookahead == 'u') ADVANCE(276);
      END_STATE();
    case 168:
      if (lookahead == 'g') ADVANCE(531);
      END_STATE();
    case 169:
      if (lookahead == 'g') ADVANCE(502);
      END_STATE();
    case 170:
      if (lookahead == 'g') ADVANCE(442);
      END_STATE();
    case 171:
      if (lookahead == 'g') ADVANCE(440);
      END_STATE();
    case 172:
      if (lookahead == 'g') ADVANCE(10);
      if (lookahead == 'l') ADVANCE(30);
      END_STATE();
    case 173:
      if (lookahead == 'g') ADVANCE(129);
      END_STATE();
    case 174:
      if (lookahead == 'g') ADVANCE(136);
      END_STATE();
    case 175:
      if (lookahead == 'g') ADVANCE(153);
      END_STATE();
    case 176:
      if (lookahead == 'g') ADVANCE(156);
      END_STATE();
    case 177:
      if (lookahead == 'g') ADVANCE(158);
      END_STATE();
    case 178:
      if (lookahead == 'g') ADVANCE(12);
      END_STATE();
    case 179:
      if (lookahead == 'h') ADVANCE(182);
      END_STATE();
    case 180:
      if (lookahead == 'h') ADVANCE(42);
      if (lookahead == 'o') ADVANCE(255);
      END_STATE();
    case 181:
      if (lookahead == 'i') ADVANCE(165);
      END_STATE();
    case 182:
      if (lookahead == 'i') ADVANCE(383);
      END_STATE();
    case 183:
      if (lookahead == 'i') ADVANCE(100);
      END_STATE();
    case 184:
      if (lookahead == 'i') ADVANCE(254);
      END_STATE();
    case 185:
      if (lookahead == 'i') ADVANCE(342);
      END_STATE();
    case 186:
      if (lookahead == 'i') ADVANCE(66);
      END_STATE();
    case 187:
      if (lookahead == 'i') ADVANCE(272);
      if (lookahead == 'u') ADVANCE(79);
      END_STATE();
    case 188:
      if (lookahead == 'i') ADVANCE(312);
      END_STATE();
    case 189:
      if (lookahead == 'i') ADVANCE(37);
      END_STATE();
    case 190:
      if (lookahead == 'i') ADVANCE(370);
      END_STATE();
    case 191:
      if (lookahead == 'i') ADVANCE(271);
      END_STATE();
    case 192:
      if (lookahead == 'i') ADVANCE(273);
      END_STATE();
    case 193:
      if (lookahead == 'i') ADVANCE(275);
      END_STATE();
    case 194:
      if (lookahead == 'i') ADVANCE(277);
      END_STATE();
    case 195:
      if (lookahead == 'i') ADVANCE(278);
      END_STATE();
    case 196:
      if (lookahead == 'i') ADVANCE(280);
      END_STATE();
    case 197:
      if (lookahead == 'i') ADVANCE(390);
      END_STATE();
    case 198:
      if (lookahead == 'i') ADVANCE(288);
      END_STATE();
    case 199:
      if (lookahead == 'i') ADVANCE(313);
      END_STATE();
    case 200:
      if (lookahead == 'i') ADVANCE(290);
      END_STATE();
    case 201:
      if (lookahead == 'i') ADVANCE(314);
      END_STATE();
    case 202:
      if (lookahead == 'i') ADVANCE(291);
      END_STATE();
    case 203:
      if (lookahead == 'i') ADVANCE(315);
      END_STATE();
    case 204:
      if (lookahead == 'i') ADVANCE(292);
      END_STATE();
    case 205:
      if (lookahead == 'i') ADVANCE(147);
      END_STATE();
    case 206:
      if (lookahead == 'i') ADVANCE(391);
      END_STATE();
    case 207:
      if (lookahead == 'k') ADVANCE(5);
      END_STATE();
    case 208:
      if (lookahead == 'k') ADVANCE(467);
      END_STATE();
    case 209:
      if (lookahead == 'k') ADVANCE(452);
      END_STATE();
    case 210:
      if (lookahead == 'k') ADVANCE(455);
      END_STATE();
    case 211:
      if (lookahead == 'k') ADVANCE(453);
      END_STATE();
    case 212:
      if (lookahead == 'k') ADVANCE(454);
      END_STATE();
    case 213:
      if (lookahead == 'k') ADVANCE(456);
      END_STATE();
    case 214:
      if (lookahead == 'k') ADVANCE(457);
      END_STATE();
    case 215:
      if (lookahead == 'l') ADVANCE(513);
      END_STATE();
    case 216:
      if (lookahead == 'l') ADVANCE(526);
      END_STATE();
    case 217:
      if (lookahead == 'l') ADVANCE(511);
      END_STATE();
    case 218:
      if (lookahead == 'l') ADVANCE(402);
      END_STATE();
    case 219:
      if (lookahead == 'l') ADVANCE(302);
      if (lookahead == 'p') ADVANCE(151);
      if (lookahead == 't') ADVANCE(25);
      END_STATE();
    case 220:
      if (lookahead == 'l') ADVANCE(86);
      if (lookahead == 'o') ADVANCE(225);
      END_STATE();
    case 221:
      if (lookahead == 'l') ADVANCE(399);
      END_STATE();
    case 222:
      if (lookahead == 'l') ADVANCE(224);
      END_STATE();
    case 223:
      if (lookahead == 'l') ADVANCE(186);
      END_STATE();
    case 224:
      if (lookahead == 'l') ADVANCE(303);
      END_STATE();
    case 225:
      if (lookahead == 'l') ADVANCE(152);
      END_STATE();
    case 226:
      if (lookahead == 'l') ADVANCE(304);
      END_STATE();
    case 227:
      if (lookahead == 'l') ADVANCE(305);
      END_STATE();
    case 228:
      if (lookahead == 'l') ADVANCE(306);
      END_STATE();
    case 229:
      if (lookahead == 'l') ADVANCE(45);
      END_STATE();
    case 230:
      if (lookahead == 'l') ADVANCE(143);
      END_STATE();
    case 231:
      if (lookahead == 'l') ADVANCE(155);
      END_STATE();
    case 232:
      if (lookahead == 'l') ADVANCE(191);
      END_STATE();
    case 233:
      if (lookahead == 'l') ADVANCE(400);
      END_STATE();
    case 234:
      if (lookahead == 'l') ADVANCE(226);
      END_STATE();
    case 235:
      if (lookahead == 'l') ADVANCE(192);
      END_STATE();
    case 236:
      if (lookahead == 'l') ADVANCE(401);
      END_STATE();
    case 237:
      if (lookahead == 'l') ADVANCE(227);
      END_STATE();
    case 238:
      if (lookahead == 'l') ADVANCE(193);
      END_STATE();
    case 239:
      if (lookahead == 'l') ADVANCE(228);
      END_STATE();
    case 240:
      if (lookahead == 'l') ADVANCE(194);
      END_STATE();
    case 241:
      if (lookahead == 'l') ADVANCE(195);
      END_STATE();
    case 242:
      if (lookahead == 'l') ADVANCE(196);
      END_STATE();
    case 243:
      if (lookahead == 'l') ADVANCE(200);
      if (lookahead == 'o') ADVANCE(326);
      if (lookahead == 's') ADVANCE(154);
      END_STATE();
    case 244:
      if (lookahead == 'l') ADVANCE(202);
      END_STATE();
    case 245:
      if (lookahead == 'l') ADVANCE(204);
      END_STATE();
    case 246:
      if (lookahead == 'm') ADVANCE(451);
      END_STATE();
    case 247:
      if (lookahead == 'm') ADVANCE(251);
      if (lookahead == 'n') ADVANCE(97);
      END_STATE();
    case 248:
      if (lookahead == 'm') ADVANCE(128);
      END_STATE();
    case 249:
      if (lookahead == 'm') ADVANCE(61);
      END_STATE();
    case 250:
      if (lookahead == 'm') ADVANCE(322);
      END_STATE();
    case 251:
      if (lookahead == 'm') ADVANCE(141);
      END_STATE();
    case 252:
      if (lookahead == 'm') ADVANCE(157);
      END_STATE();
    case 253:
      if (lookahead == 'm') ADVANCE(159);
      END_STATE();
    case 254:
      if (lookahead == 'm') ADVANCE(197);
      END_STATE();
    case 255:
      if (lookahead == 'm') ADVANCE(253);
      END_STATE();
    case 256:
      if (lookahead == 'n') ADVANCE(71);
      if (lookahead == 'r') ADVANCE(340);
      if (lookahead == 's') ADVANCE(72);
      if (lookahead == 'x') ADVANCE(75);
      END_STATE();
    case 257:
      if (lookahead == 'n') ADVANCE(405);
      END_STATE();
    case 258:
      if (lookahead == 'n') ADVANCE(491);
      END_STATE();
    case 259:
      if (lookahead == 'n') ADVANCE(475);
      END_STATE();
    case 260:
      if (lookahead == 'n') ADVANCE(506);
      END_STATE();
    case 261:
      if (lookahead == 'n') ADVANCE(492);
      END_STATE();
    case 262:
      if (lookahead == 'n') ADVANCE(509);
      END_STATE();
    case 263:
      if (lookahead == 'n') ADVANCE(493);
      END_STATE();
    case 264:
      if (lookahead == 'n') ADVANCE(494);
      END_STATE();
    case 265:
      if (lookahead == 'n') ADVANCE(476);
      END_STATE();
    case 266:
      if (lookahead == 'n') ADVANCE(516);
      END_STATE();
    case 267:
      if (lookahead == 'n') ADVANCE(477);
      END_STATE();
    case 268:
      if (lookahead == 'n') ADVANCE(478);
      END_STATE();
    case 269:
      if (lookahead == 'n') ADVANCE(539);
      END_STATE();
    case 270:
      if (lookahead == 'n') ADVANCE(99);
      END_STATE();
    case 271:
      if (lookahead == 'n') ADVANCE(209);
      END_STATE();
    case 272:
      if (lookahead == 'n') ADVANCE(169);
      END_STATE();
    case 273:
      if (lookahead == 'n') ADVANCE(210);
      END_STATE();
    case 274:
      if (lookahead == 'n') ADVANCE(316);
      END_STATE();
    case 275:
      if (lookahead == 'n') ADVANCE(211);
      END_STATE();
    case 276:
      if (lookahead == 'n') ADVANCE(73);
      END_STATE();
    case 277:
      if (lookahead == 'n') ADVANCE(212);
      END_STATE();
    case 278:
      if (lookahead == 'n') ADVANCE(213);
      END_STATE();
    case 279:
      if (lookahead == 'n') ADVANCE(382);
      END_STATE();
    case 280:
      if (lookahead == 'n') ADVANCE(214);
      END_STATE();
    case 281:
      if (lookahead == 'n') ADVANCE(98);
      END_STATE();
    case 282:
      if (lookahead == 'n') ADVANCE(122);
      END_STATE();
    case 283:
      if (lookahead == 'n') ADVANCE(368);
      END_STATE();
    case 284:
      if (lookahead == 'n') ADVANCE(369);
      END_STATE();
    case 285:
      if (lookahead == 'n') ADVANCE(334);
      END_STATE();
    case 286:
      if (lookahead == 'n') ADVANCE(371);
      END_STATE();
    case 287:
      if (lookahead == 'n') ADVANCE(39);
      END_STATE();
    case 288:
      if (lookahead == 'n') ADVANCE(108);
      END_STATE();
    case 289:
      if (lookahead == 'n') ADVANCE(376);
      END_STATE();
    case 290:
      if (lookahead == 'n') ADVANCE(113);
      END_STATE();
    case 291:
      if (lookahead == 'n') ADVANCE(119);
      END_STATE();
    case 292:
      if (lookahead == 'n') ADVANCE(120);
      END_STATE();
    case 293:
      if (lookahead == 'n') ADVANCE(406);
      END_STATE();
    case 294:
      if (lookahead == 'n') ADVANCE(381);
      END_STATE();
    case 295:
      if (lookahead == 'n') ADVANCE(384);
      END_STATE();
    case 296:
      if (lookahead == 'n') ADVANCE(407);
      END_STATE();
    case 297:
      if (lookahead == 'n') ADVANCE(385);
      END_STATE();
    case 298:
      if (lookahead == 'n') ADVANCE(386);
      END_STATE();
    case 299:
      if (lookahead == 'o') ADVANCE(358);
      END_STATE();
    case 300:
      if (lookahead == 'o') ADVANCE(534);
      END_STATE();
    case 301:
      if (lookahead == 'o') ADVANCE(520);
      END_STATE();
    case 302:
      if (lookahead == 'o') ADVANCE(408);
      END_STATE();
    case 303:
      if (lookahead == 'o') ADVANCE(409);
      END_STATE();
    case 304:
      if (lookahead == 'o') ADVANCE(410);
      END_STATE();
    case 305:
      if (lookahead == 'o') ADVANCE(411);
      END_STATE();
    case 306:
      if (lookahead == 'o') ADVANCE(412);
      END_STATE();
    case 307:
      if (lookahead == 'o') ADVANCE(32);
      END_STATE();
    case 308:
      if (lookahead == 'o') ADVANCE(332);
      END_STATE();
    case 309:
      if (lookahead == 'o') ADVANCE(67);
      END_STATE();
    case 310:
      if (lookahead == 'o') ADVANCE(343);
      END_STATE();
    case 311:
      if (lookahead == 'o') ADVANCE(335);
      END_STATE();
    case 312:
      if (lookahead == 'o') ADVANCE(262);
      END_STATE();
    case 313:
      if (lookahead == 'o') ADVANCE(287);
      END_STATE();
    case 314:
      if (lookahead == 'o') ADVANCE(266);
      END_STATE();
    case 315:
      if (lookahead == 'o') ADVANCE(269);
      END_STATE();
    case 316:
      if (lookahead == 'o') ADVANCE(347);
      END_STATE();
    case 317:
      if (lookahead == 'o') ADVANCE(281);
      END_STATE();
    case 318:
      if (lookahead == 'o') ADVANCE(397);
      END_STATE();
    case 319:
      if (lookahead == 'o') ADVANCE(398);
      END_STATE();
    case 320:
      if (lookahead == 'p') ADVANCE(132);
      END_STATE();
    case 321:
      if (lookahead == 'p') ADVANCE(183);
      END_STATE();
    case 322:
      if (lookahead == 'p') ADVANCE(366);
      END_STATE();
    case 323:
      if (lookahead == 'p') ADVANCE(106);
      END_STATE();
    case 324:
      if (lookahead == 'p') ADVANCE(126);
      END_STATE();
    case 325:
      if (lookahead == 'p') ADVANCE(392);
      END_STATE();
    case 326:
      if (lookahead == 'p') ADVANCE(393);
      END_STATE();
    case 327:
      if (lookahead == 'q') ADVANCE(503);
      END_STATE();
    case 328:
      if (lookahead == 'r') ADVANCE(104);
      END_STATE();
    case 329:
      if (lookahead == 'r') ADVANCE(207);
      END_STATE();
    case 330:
      if (lookahead == 'r') ADVANCE(444);
      END_STATE();
    case 331:
      if (lookahead == 'r') ADVANCE(504);
      END_STATE();
    case 332:
      if (lookahead == 'r') ADVANCE(533);
      END_STATE();
    case 333:
      if (lookahead == 'r') ADVANCE(505);
      END_STATE();
    case 334:
      if (lookahead == 'r') ADVANCE(535);
      END_STATE();
    case 335:
      if (lookahead == 'r') ADVANCE(514);
      END_STATE();
    case 336:
      if (lookahead == 'r') ADVANCE(529);
      END_STATE();
    case 337:
      if (lookahead == 'r') ADVANCE(508);
      END_STATE();
    case 338:
      if (lookahead == 'r') ADVANCE(527);
      END_STATE();
    case 339:
      if (lookahead == 'r') ADVANCE(103);
      END_STATE();
    case 340:
      if (lookahead == 'r') ADVANCE(308);
      END_STATE();
    case 341:
      if (lookahead == 'r') ADVANCE(301);
      END_STATE();
    case 342:
      if (lookahead == 'r') ADVANCE(81);
      END_STATE();
    case 343:
      if (lookahead == 'r') ADVANCE(90);
      END_STATE();
    case 344:
      if (lookahead == 'r') ADVANCE(36);
      END_STATE();
    case 345:
      if (lookahead == 'r') ADVANCE(309);
      END_STATE();
    case 346:
      if (lookahead == 'r') ADVANCE(24);
      END_STATE();
    case 347:
      if (lookahead == 'r') ADVANCE(109);
      END_STATE();
    case 348:
      if (lookahead == 'r') ADVANCE(121);
      END_STATE();
    case 349:
      if (lookahead == 'r') ADVANCE(125);
      END_STATE();
    case 350:
      if (lookahead == 'r') ADVANCE(127);
      END_STATE();
    case 351:
      if (lookahead == 'r') ADVANCE(118);
      END_STATE();
    case 352:
      if (lookahead == 'r') ADVANCE(362);
      END_STATE();
    case 353:
      if (lookahead == 'r') ADVANCE(244);
      END_STATE();
    case 354:
      if (lookahead == 's') ADVANCE(499);
      END_STATE();
    case 355:
      if (lookahead == 's') ADVANCE(497);
      END_STATE();
    case 356:
      if (lookahead == 's') ADVANCE(498);
      END_STATE();
    case 357:
      if (lookahead == 's') ADVANCE(523);
      END_STATE();
    case 358:
      if (lookahead == 's') ADVANCE(361);
      END_STATE();
    case 359:
      if (lookahead == 's') ADVANCE(13);
      END_STATE();
    case 360:
      if (lookahead == 's') ADVANCE(357);
      END_STATE();
    case 361:
      if (lookahead == 's') ADVANCE(130);
      END_STATE();
    case 362:
      if (lookahead == 's') ADVANCE(111);
      END_STATE();
    case 363:
      if (lookahead == 's') ADVANCE(117);
      END_STATE();
    case 364:
      if (lookahead == 't') ADVANCE(161);
      END_STATE();
    case 365:
      if (lookahead == 't') ADVANCE(507);
      END_STATE();
    case 366:
      if (lookahead == 't') ADVANCE(536);
      END_STATE();
    case 367:
      if (lookahead == 't') ADVANCE(512);
      END_STATE();
    case 368:
      if (lookahead == 't') ADVANCE(500);
      END_STATE();
    case 369:
      if (lookahead == 't') ADVANCE(501);
      END_STATE();
    case 370:
      if (lookahead == 't') ADVANCE(521);
      END_STATE();
    case 371:
      if (lookahead == 't') ADVANCE(510);
      END_STATE();
    case 372:
      if (lookahead == 't') ADVANCE(458);
      END_STATE();
    case 373:
      if (lookahead == 't') ADVANCE(540);
      END_STATE();
    case 374:
      if (lookahead == 't') ADVANCE(459);
      END_STATE();
    case 375:
      if (lookahead == 't') ADVANCE(460);
      END_STATE();
    case 376:
      if (lookahead == 't') ADVANCE(530);
      END_STATE();
    case 377:
      if (lookahead == 't') ADVANCE(378);
      END_STATE();
    case 378:
      if (lookahead == 't') ADVANCE(330);
      END_STATE();
    case 379:
      if (lookahead == 't') ADVANCE(404);
      END_STATE();
    case 380:
      if (lookahead == 't') ADVANCE(188);
      END_STATE();
    case 381:
      if (lookahead == 't') ADVANCE(181);
      END_STATE();
    case 382:
      if (lookahead == 't') ADVANCE(18);
      END_STATE();
    case 383:
      if (lookahead == 't') ADVANCE(107);
      END_STATE();
    case 384:
      if (lookahead == 't') ADVANCE(19);
      END_STATE();
    case 385:
      if (lookahead == 't') ADVANCE(20);
      END_STATE();
    case 386:
      if (lookahead == 't') ADVANCE(21);
      END_STATE();
    case 387:
      if (lookahead == 't') ADVANCE(114);
      END_STATE();
    case 388:
      if (lookahead == 't') ADVANCE(49);
      END_STATE();
    case 389:
      if (lookahead == 't') ADVANCE(311);
      END_STATE();
    case 390:
      if (lookahead == 't') ADVANCE(146);
      END_STATE();
    case 391:
      if (lookahead == 't') ADVANCE(199);
      END_STATE();
    case 392:
      if (lookahead == 't') ADVANCE(201);
      END_STATE();
    case 393:
      if (lookahead == 't') ADVANCE(203);
      END_STATE();
    case 394:
      if (lookahead == 'u') ADVANCE(168);
      END_STATE();
    case 395:
      if (lookahead == 'u') ADVANCE(249);
      END_STATE();
    case 396:
      if (lookahead == 'u') ADVANCE(372);
      END_STATE();
    case 397:
      if (lookahead == 'u') ADVANCE(374);
      END_STATE();
    case 398:
      if (lookahead == 'u') ADVANCE(375);
      END_STATE();
    case 399:
      if (lookahead == 'u') ADVANCE(112);
      END_STATE();
    case 400:
      if (lookahead == 'u') ADVANCE(115);
      END_STATE();
    case 401:
      if (lookahead == 'u') ADVANCE(116);
      END_STATE();
    case 402:
      if (lookahead == 'u') ADVANCE(96);
      END_STATE();
    case 403:
      if (lookahead == 'u') ADVANCE(387);
      END_STATE();
    case 404:
      if (lookahead == 'u') ADVANCE(351);
      END_STATE();
    case 405:
      if (lookahead == 'v') ADVANCE(28);
      END_STATE();
    case 406:
      if (lookahead == 'v') ADVANCE(31);
      END_STATE();
    case 407:
      if (lookahead == 'v') ADVANCE(35);
      END_STATE();
    case 408:
      if (lookahead == 'w') ADVANCE(6);
      END_STATE();
    case 409:
      if (lookahead == 'w') ADVANCE(479);
      END_STATE();
    case 410:
      if (lookahead == 'w') ADVANCE(480);
      END_STATE();
    case 411:
      if (lookahead == 'w') ADVANCE(481);
      END_STATE();
    case 412:
      if (lookahead == 'w') ADVANCE(482);
      END_STATE();
    case 413:
      if (lookahead == 'w') ADVANCE(310);
      END_STATE();
    case 414:
      if (lookahead == 'y') ADVANCE(413);
      END_STATE();
    case 415:
      if (lookahead == 'y') ADVANCE(34);
      END_STATE();
    case 416:
      if (lookahead == 'y') ADVANCE(41);
      END_STATE();
    case 417:
      if (lookahead == 'y') ADVANCE(43);
      END_STATE();
    case 418:
      if (lookahead == 'y') ADVANCE(44);
      END_STATE();
    case 419:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(466);
      END_STATE();
    case 420:
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(434);
      END_STATE();
    case 421:
      if (eof) ADVANCE(422);
      if (lookahead == '\n') ADVANCE(432);
      if (lookahead == '\r') ADVANCE(1);
      if (lookahead == '#') ADVANCE(431);
      if (lookahead == '+') ADVANCE(435);
      if (lookahead == ',') ADVANCE(433);
      if (lookahead == ':') ADVANCE(423);
      if (lookahead == '>') ADVANCE(437);
      if (lookahead == '~') ADVANCE(436);
      if (lookahead == '\t' ||
          lookahead == ' ') SKIP(421)
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(420);
      END_STATE();
    case 422:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 423:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 424:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 425:
      ACCEPT_TOKEN(sym_comment);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(431);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(431);
      END_STATE();
    case 426:
      ACCEPT_TOKEN(sym_comment);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(425);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(431);
      END_STATE();
    case 427:
      ACCEPT_TOKEN(sym_comment);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(426);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(431);
      END_STATE();
    case 428:
      ACCEPT_TOKEN(sym_comment);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(427);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(431);
      END_STATE();
    case 429:
      ACCEPT_TOKEN(sym_comment);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(428);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(431);
      END_STATE();
    case 430:
      ACCEPT_TOKEN(sym_comment);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(429);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(431);
      END_STATE();
    case 431:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(431);
      END_STATE();
    case 432:
      ACCEPT_TOKEN(sym_newline);
      END_STATE();
    case 433:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 434:
      ACCEPT_TOKEN(sym_sel_kind);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(434);
      END_STATE();
    case 435:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 436:
      ACCEPT_TOKEN(anon_sym_TILDE);
      END_STATE();
    case 437:
      ACCEPT_TOKEN(anon_sym_GT);
      END_STATE();
    case 438:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 439:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 440:
      ACCEPT_TOKEN(anon_sym_fg);
      END_STATE();
    case 441:
      ACCEPT_TOKEN(anon_sym_fg);
      if (lookahead == '-') ADVANCE(83);
      END_STATE();
    case 442:
      ACCEPT_TOKEN(anon_sym_bg);
      END_STATE();
    case 443:
      ACCEPT_TOKEN(anon_sym_bg);
      if (lookahead == '-') ADVANCE(76);
      END_STATE();
    case 444:
      ACCEPT_TOKEN(anon_sym_attr);
      if (lookahead == 'i') ADVANCE(57);
      END_STATE();
    case 445:
      ACCEPT_TOKEN(anon_sym_attribute);
      END_STATE();
    case 446:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 447:
      ACCEPT_TOKEN(anon_sym_bold);
      END_STATE();
    case 448:
      ACCEPT_TOKEN(anon_sym_italic);
      END_STATE();
    case 449:
      ACCEPT_TOKEN(anon_sym_underlined);
      END_STATE();
    case 450:
      ACCEPT_TOKEN(anon_sym_underline);
      if (lookahead == 'd') ADVANCE(449);
      END_STATE();
    case 451:
      ACCEPT_TOKEN(anon_sym_dim);
      END_STATE();
    case 452:
      ACCEPT_TOKEN(anon_sym_slowblink);
      END_STATE();
    case 453:
      ACCEPT_TOKEN(anon_sym_slow_DASHblink);
      END_STATE();
    case 454:
      ACCEPT_TOKEN(anon_sym_slow_blink);
      END_STATE();
    case 455:
      ACCEPT_TOKEN(anon_sym_rapidblink);
      END_STATE();
    case 456:
      ACCEPT_TOKEN(anon_sym_rapid_DASHblink);
      END_STATE();
    case 457:
      ACCEPT_TOKEN(anon_sym_rapid_blink);
      END_STATE();
    case 458:
      ACCEPT_TOKEN(anon_sym_crossedout);
      END_STATE();
    case 459:
      ACCEPT_TOKEN(anon_sym_crossed_DASHout);
      END_STATE();
    case 460:
      ACCEPT_TOKEN(anon_sym_crossed_out);
      END_STATE();
    case 461:
      ACCEPT_TOKEN(anon_sym_framed);
      END_STATE();
    case 462:
      ACCEPT_TOKEN(anon_sym_encircled);
      END_STATE();
    case 463:
      ACCEPT_TOKEN(anon_sym_reverse);
      END_STATE();
    case 464:
      ACCEPT_TOKEN(aux_sym_ansi_color_token1);
      if (lookahead == 'x') ADVANCE(419);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(465);
      END_STATE();
    case 465:
      ACCEPT_TOKEN(aux_sym_ansi_color_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(465);
      END_STATE();
    case 466:
      ACCEPT_TOKEN(aux_sym_ansi_color_token2);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(466);
      END_STATE();
    case 467:
      ACCEPT_TOKEN(anon_sym_black);
      END_STATE();
    case 468:
      ACCEPT_TOKEN(anon_sym_darkgrey);
      END_STATE();
    case 469:
      ACCEPT_TOKEN(anon_sym_dark_DASHgrey);
      END_STATE();
    case 470:
      ACCEPT_TOKEN(anon_sym_dark_grey);
      END_STATE();
    case 471:
      ACCEPT_TOKEN(anon_sym_red);
      END_STATE();
    case 472:
      ACCEPT_TOKEN(anon_sym_darkred);
      END_STATE();
    case 473:
      ACCEPT_TOKEN(anon_sym_dark_DASHred);
      END_STATE();
    case 474:
      ACCEPT_TOKEN(anon_sym_dark_red);
      END_STATE();
    case 475:
      ACCEPT_TOKEN(anon_sym_green);
      END_STATE();
    case 476:
      ACCEPT_TOKEN(anon_sym_darkgreen);
      END_STATE();
    case 477:
      ACCEPT_TOKEN(anon_sym_dark_DASHgreen);
      END_STATE();
    case 478:
      ACCEPT_TOKEN(anon_sym_dark_green);
      END_STATE();
    case 479:
      ACCEPT_TOKEN(anon_sym_yellow);
      END_STATE();
    case 480:
      ACCEPT_TOKEN(anon_sym_darkyellow);
      END_STATE();
    case 481:
      ACCEPT_TOKEN(anon_sym_dark_DASHyellow);
      END_STATE();
    case 482:
      ACCEPT_TOKEN(anon_sym_dark_yellow);
      END_STATE();
    case 483:
      ACCEPT_TOKEN(anon_sym_blue);
      END_STATE();
    case 484:
      ACCEPT_TOKEN(anon_sym_darkblue);
      END_STATE();
    case 485:
      ACCEPT_TOKEN(anon_sym_dark_DASHblue);
      END_STATE();
    case 486:
      ACCEPT_TOKEN(anon_sym_dark_blue);
      END_STATE();
    case 487:
      ACCEPT_TOKEN(anon_sym_magenta);
      END_STATE();
    case 488:
      ACCEPT_TOKEN(anon_sym_darkmagenta);
      END_STATE();
    case 489:
      ACCEPT_TOKEN(anon_sym_dark_DASHmagenta);
      END_STATE();
    case 490:
      ACCEPT_TOKEN(anon_sym_dark_magenta);
      END_STATE();
    case 491:
      ACCEPT_TOKEN(anon_sym_cyan);
      END_STATE();
    case 492:
      ACCEPT_TOKEN(anon_sym_darkcyan);
      END_STATE();
    case 493:
      ACCEPT_TOKEN(anon_sym_dark_DASHcyan);
      END_STATE();
    case 494:
      ACCEPT_TOKEN(anon_sym_dark_cyan);
      END_STATE();
    case 495:
      ACCEPT_TOKEN(anon_sym_white);
      END_STATE();
    case 496:
      ACCEPT_TOKEN(anon_sym_grey);
      END_STATE();
    case 497:
      ACCEPT_TOKEN(anon_sym_bg_DASHcanvas);
      END_STATE();
    case 498:
      ACCEPT_TOKEN(anon_sym_fg_DASHcanvas);
      END_STATE();
    case 499:
      ACCEPT_TOKEN(anon_sym_canvas);
      END_STATE();
    case 500:
      ACCEPT_TOKEN(anon_sym_comment);
      END_STATE();
    case 501:
      ACCEPT_TOKEN(anon_sym_constant);
      END_STATE();
    case 502:
      ACCEPT_TOKEN(anon_sym_string);
      END_STATE();
    case 503:
      ACCEPT_TOKEN(anon_sym_escape_DASHseq);
      END_STATE();
    case 504:
      ACCEPT_TOKEN(anon_sym_char);
      END_STATE();
    case 505:
      ACCEPT_TOKEN(anon_sym_number);
      END_STATE();
    case 506:
      ACCEPT_TOKEN(anon_sym_boolean);
      END_STATE();
    case 507:
      ACCEPT_TOKEN(anon_sym_float);
      END_STATE();
    case 508:
      ACCEPT_TOKEN(anon_sym_identifier);
      END_STATE();
    case 509:
      ACCEPT_TOKEN(anon_sym_function);
      END_STATE();
    case 510:
      ACCEPT_TOKEN(anon_sym_statement);
      END_STATE();
    case 511:
      ACCEPT_TOKEN(anon_sym_conditional);
      END_STATE();
    case 512:
      ACCEPT_TOKEN(anon_sym_repeat);
      END_STATE();
    case 513:
      ACCEPT_TOKEN(anon_sym_label);
      END_STATE();
    case 514:
      ACCEPT_TOKEN(anon_sym_operator);
      END_STATE();
    case 515:
      ACCEPT_TOKEN(anon_sym_keyword);
      END_STATE();
    case 516:
      ACCEPT_TOKEN(anon_sym_exception);
      END_STATE();
    case 517:
      ACCEPT_TOKEN(anon_sym_preproc);
      END_STATE();
    case 518:
      ACCEPT_TOKEN(anon_sym_include);
      END_STATE();
    case 519:
      ACCEPT_TOKEN(anon_sym_define);
      END_STATE();
    case 520:
      ACCEPT_TOKEN(anon_sym_macro);
      END_STATE();
    case 521:
      ACCEPT_TOKEN(anon_sym_precondit);
      END_STATE();
    case 522:
      ACCEPT_TOKEN(anon_sym_type);
      if (lookahead == 'd') ADVANCE(123);
      END_STATE();
    case 523:
      ACCEPT_TOKEN(anon_sym_storage_DASHclass);
      END_STATE();
    case 524:
      ACCEPT_TOKEN(anon_sym_structure);
      END_STATE();
    case 525:
      ACCEPT_TOKEN(anon_sym_typedef);
      END_STATE();
    case 526:
      ACCEPT_TOKEN(anon_sym_special);
      if (lookahead == '-') ADVANCE(68);
      END_STATE();
    case 527:
      ACCEPT_TOKEN(anon_sym_special_DASHchar);
      END_STATE();
    case 528:
      ACCEPT_TOKEN(anon_sym_tag);
      END_STATE();
    case 529:
      ACCEPT_TOKEN(anon_sym_delimiter);
      END_STATE();
    case 530:
      ACCEPT_TOKEN(anon_sym_special_DASHcomment);
      END_STATE();
    case 531:
      ACCEPT_TOKEN(anon_sym_debug);
      END_STATE();
    case 532:
      ACCEPT_TOKEN(anon_sym_ignore);
      END_STATE();
    case 533:
      ACCEPT_TOKEN(anon_sym_error);
      END_STATE();
    case 534:
      ACCEPT_TOKEN(anon_sym_todo);
      END_STATE();
    case 535:
      ACCEPT_TOKEN(anon_sym_line_DASHnr);
      END_STATE();
    case 536:
      ACCEPT_TOKEN(anon_sym_prompt);
      END_STATE();
    case 537:
      ACCEPT_TOKEN(anon_sym_status_DASHline);
      END_STATE();
    case 538:
      ACCEPT_TOKEN(anon_sym_tab_DASHline);
      END_STATE();
    case 539:
      ACCEPT_TOKEN(anon_sym_tab_DASHoption);
      END_STATE();
    case 540:
      ACCEPT_TOKEN(anon_sym_tab_DASHselect);
      END_STATE();
    default:
      return false;
  }
}

static TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 421},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 2},
  [4] = {.lex_state = 2},
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
  [16] = {.lex_state = 421},
  [17] = {.lex_state = 421},
  [18] = {.lex_state = 3},
  [19] = {.lex_state = 3},
  [20] = {.lex_state = 421},
  [21] = {.lex_state = 421},
  [22] = {.lex_state = 421},
  [23] = {.lex_state = 421},
  [24] = {.lex_state = 421},
  [25] = {.lex_state = 421},
  [26] = {.lex_state = 421},
  [27] = {.lex_state = 421},
  [28] = {.lex_state = 421},
  [29] = {.lex_state = 421},
  [30] = {.lex_state = 421},
  [31] = {.lex_state = 0},
  [32] = {.lex_state = 0},
  [33] = {.lex_state = 0},
  [34] = {.lex_state = 0},
  [35] = {.lex_state = 0},
  [36] = {.lex_state = 0},
  [37] = {.lex_state = 0},
  [38] = {.lex_state = 421},
  [39] = {.lex_state = 0},
  [40] = {.lex_state = 0},
  [41] = {.lex_state = 0},
  [42] = {.lex_state = 0},
  [43] = {.lex_state = 0},
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
    [anon_sym_dim] = ACTIONS(1),
    [anon_sym_slowblink] = ACTIONS(1),
    [anon_sym_slow_DASHblink] = ACTIONS(1),
    [anon_sym_slow_blink] = ACTIONS(1),
    [anon_sym_rapidblink] = ACTIONS(1),
    [anon_sym_rapid_DASHblink] = ACTIONS(1),
    [anon_sym_rapid_blink] = ACTIONS(1),
    [anon_sym_crossedout] = ACTIONS(1),
    [anon_sym_crossed_DASHout] = ACTIONS(1),
    [anon_sym_crossed_out] = ACTIONS(1),
    [anon_sym_framed] = ACTIONS(1),
    [anon_sym_encircled] = ACTIONS(1),
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
    [sym_s] = STATE(47),
    [sym_hl_rule] = STATE(17),
    [sym_selectors] = STATE(55),
    [sym_selector] = STATE(34),
    [sym_sel_symbol] = STATE(26),
    [sym_sel_twins] = STATE(27),
    [sym_sel_siblings] = STATE(27),
    [sym_sel_child] = STATE(27),
    [aux_sym_s_repeat1] = STATE(17),
    [aux_sym_selector_repeat1] = STATE(20),
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
    STATE(53), 2,
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
    STATE(43), 2,
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
    STATE(39), 2,
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
  [163] = 6,
    ACTIONS(30), 1,
      anon_sym_underline,
    STATE(10), 1,
      sym_attr,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(25), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
    STATE(5), 2,
      sym_attrs,
      aux_sym_attrb_repeat1,
    ACTIONS(27), 16,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_dim,
      anon_sym_slowblink,
      anon_sym_slow_DASHblink,
      anon_sym_slow_blink,
      anon_sym_rapidblink,
      anon_sym_rapid_DASHblink,
      anon_sym_rapid_blink,
      anon_sym_crossedout,
      anon_sym_crossed_DASHout,
      anon_sym_crossed_out,
      anon_sym_framed,
      anon_sym_encircled,
      anon_sym_reverse,
  [200] = 5,
    ACTIONS(35), 1,
      anon_sym_PIPE,
    ACTIONS(37), 1,
      anon_sym_underline,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(7), 2,
      sym_attr_or,
      aux_sym_attrs_repeat1,
    ACTIONS(33), 18,
      anon_sym_COMMA,
      anon_sym_RBRACE,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_dim,
      anon_sym_slowblink,
      anon_sym_slow_DASHblink,
      anon_sym_slow_blink,
      anon_sym_rapidblink,
      anon_sym_rapid_DASHblink,
      anon_sym_rapid_blink,
      anon_sym_crossedout,
      anon_sym_crossed_DASHout,
      anon_sym_crossed_out,
      anon_sym_framed,
      anon_sym_encircled,
      anon_sym_reverse,
  [235] = 5,
    ACTIONS(41), 1,
      anon_sym_PIPE,
    ACTIONS(44), 1,
      anon_sym_underline,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(7), 2,
      sym_attr_or,
      aux_sym_attrs_repeat1,
    ACTIONS(39), 18,
      anon_sym_COMMA,
      anon_sym_RBRACE,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_dim,
      anon_sym_slowblink,
      anon_sym_slow_DASHblink,
      anon_sym_slow_blink,
      anon_sym_rapidblink,
      anon_sym_rapid_DASHblink,
      anon_sym_rapid_blink,
      anon_sym_crossedout,
      anon_sym_crossed_DASHout,
      anon_sym_crossed_out,
      anon_sym_framed,
      anon_sym_encircled,
      anon_sym_reverse,
  [270] = 6,
    ACTIONS(50), 1,
      anon_sym_underline,
    STATE(10), 1,
      sym_attr,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(46), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
    STATE(12), 2,
      sym_attrs,
      aux_sym_attrb_repeat1,
    ACTIONS(48), 16,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_dim,
      anon_sym_slowblink,
      anon_sym_slow_DASHblink,
      anon_sym_slow_blink,
      anon_sym_rapidblink,
      anon_sym_rapid_DASHblink,
      anon_sym_rapid_blink,
      anon_sym_crossedout,
      anon_sym_crossed_DASHout,
      anon_sym_crossed_out,
      anon_sym_framed,
      anon_sym_encircled,
      anon_sym_reverse,
  [307] = 6,
    ACTIONS(50), 1,
      anon_sym_underline,
    STATE(10), 1,
      sym_attr,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(52), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
    STATE(11), 2,
      sym_attrs,
      aux_sym_attrb_repeat1,
    ACTIONS(48), 16,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_dim,
      anon_sym_slowblink,
      anon_sym_slow_DASHblink,
      anon_sym_slow_blink,
      anon_sym_rapidblink,
      anon_sym_rapid_DASHblink,
      anon_sym_rapid_blink,
      anon_sym_crossedout,
      anon_sym_crossed_DASHout,
      anon_sym_crossed_out,
      anon_sym_framed,
      anon_sym_encircled,
      anon_sym_reverse,
  [344] = 5,
    ACTIONS(35), 1,
      anon_sym_PIPE,
    ACTIONS(56), 1,
      anon_sym_underline,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(6), 2,
      sym_attr_or,
      aux_sym_attrs_repeat1,
    ACTIONS(54), 18,
      anon_sym_COMMA,
      anon_sym_RBRACE,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_dim,
      anon_sym_slowblink,
      anon_sym_slow_DASHblink,
      anon_sym_slow_blink,
      anon_sym_rapidblink,
      anon_sym_rapid_DASHblink,
      anon_sym_rapid_blink,
      anon_sym_crossedout,
      anon_sym_crossed_DASHout,
      anon_sym_crossed_out,
      anon_sym_framed,
      anon_sym_encircled,
      anon_sym_reverse,
  [379] = 6,
    ACTIONS(50), 1,
      anon_sym_underline,
    STATE(10), 1,
      sym_attr,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(58), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
    STATE(5), 2,
      sym_attrs,
      aux_sym_attrb_repeat1,
    ACTIONS(48), 16,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_dim,
      anon_sym_slowblink,
      anon_sym_slow_DASHblink,
      anon_sym_slow_blink,
      anon_sym_rapidblink,
      anon_sym_rapid_DASHblink,
      anon_sym_rapid_blink,
      anon_sym_crossedout,
      anon_sym_crossed_DASHout,
      anon_sym_crossed_out,
      anon_sym_framed,
      anon_sym_encircled,
      anon_sym_reverse,
  [416] = 6,
    ACTIONS(50), 1,
      anon_sym_underline,
    STATE(10), 1,
      sym_attr,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(60), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
    STATE(5), 2,
      sym_attrs,
      aux_sym_attrb_repeat1,
    ACTIONS(48), 16,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_dim,
      anon_sym_slowblink,
      anon_sym_slow_DASHblink,
      anon_sym_slow_blink,
      anon_sym_rapidblink,
      anon_sym_rapid_DASHblink,
      anon_sym_rapid_blink,
      anon_sym_crossedout,
      anon_sym_crossed_DASHout,
      anon_sym_crossed_out,
      anon_sym_framed,
      anon_sym_encircled,
      anon_sym_reverse,
  [453] = 3,
    ACTIONS(64), 1,
      anon_sym_underline,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(62), 19,
      anon_sym_COMMA,
      anon_sym_RBRACE,
      anon_sym_PIPE,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_dim,
      anon_sym_slowblink,
      anon_sym_slow_DASHblink,
      anon_sym_slow_blink,
      anon_sym_rapidblink,
      anon_sym_rapid_DASHblink,
      anon_sym_rapid_blink,
      anon_sym_crossedout,
      anon_sym_crossed_DASHout,
      anon_sym_crossed_out,
      anon_sym_framed,
      anon_sym_encircled,
      anon_sym_reverse,
  [482] = 3,
    ACTIONS(68), 1,
      anon_sym_underline,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(66), 19,
      anon_sym_COMMA,
      anon_sym_RBRACE,
      anon_sym_PIPE,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_dim,
      anon_sym_slowblink,
      anon_sym_slow_DASHblink,
      anon_sym_slow_blink,
      anon_sym_rapidblink,
      anon_sym_rapid_DASHblink,
      anon_sym_rapid_blink,
      anon_sym_crossedout,
      anon_sym_crossed_DASHout,
      anon_sym_crossed_out,
      anon_sym_framed,
      anon_sym_encircled,
      anon_sym_reverse,
  [511] = 4,
    ACTIONS(50), 1,
      anon_sym_underline,
    STATE(13), 1,
      sym_attr,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(48), 16,
      anon_sym_bold,
      anon_sym_italic,
      anon_sym_underlined,
      anon_sym_dim,
      anon_sym_slowblink,
      anon_sym_slow_DASHblink,
      anon_sym_slow_blink,
      anon_sym_rapidblink,
      anon_sym_rapid_DASHblink,
      anon_sym_rapid_blink,
      anon_sym_crossedout,
      anon_sym_crossed_DASHout,
      anon_sym_crossed_out,
      anon_sym_framed,
      anon_sym_encircled,
      anon_sym_reverse,
  [540] = 9,
    ACTIONS(70), 1,
      ts_builtin_sym_end,
    ACTIONS(72), 1,
      sym_sel_kind,
    STATE(20), 1,
      aux_sym_selector_repeat1,
    STATE(26), 1,
      sym_sel_symbol,
    STATE(34), 1,
      sym_selector,
    STATE(55), 1,
      sym_selectors,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(16), 2,
      sym_hl_rule,
      aux_sym_s_repeat1,
    STATE(27), 3,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [572] = 9,
    ACTIONS(7), 1,
      sym_sel_kind,
    ACTIONS(75), 1,
      ts_builtin_sym_end,
    STATE(20), 1,
      aux_sym_selector_repeat1,
    STATE(26), 1,
      sym_sel_symbol,
    STATE(34), 1,
      sym_selector,
    STATE(55), 1,
      sym_selectors,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(16), 2,
      sym_hl_rule,
      aux_sym_s_repeat1,
    STATE(27), 3,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [604] = 7,
    ACTIONS(77), 1,
      anon_sym_fg,
    ACTIONS(79), 1,
      anon_sym_bg,
    ACTIONS(81), 1,
      anon_sym_attr,
    ACTIONS(83), 1,
      anon_sym_attribute,
    STATE(32), 1,
      sym_property,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(42), 4,
      sym_fg,
      sym_bg,
      sym_attrb,
      sym_attribute,
  [630] = 7,
    ACTIONS(77), 1,
      anon_sym_fg,
    ACTIONS(79), 1,
      anon_sym_bg,
    ACTIONS(81), 1,
      anon_sym_attr,
    ACTIONS(83), 1,
      anon_sym_attribute,
    STATE(44), 1,
      sym_property,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(42), 4,
      sym_fg,
      sym_bg,
      sym_attrb,
      sym_attribute,
  [656] = 6,
    ACTIONS(7), 1,
      sym_sel_kind,
    STATE(21), 1,
      aux_sym_selector_repeat1,
    STATE(26), 1,
      sym_sel_symbol,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(85), 2,
      anon_sym_COLON,
      anon_sym_COMMA,
    STATE(27), 3,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [679] = 6,
    ACTIONS(89), 1,
      sym_sel_kind,
    STATE(21), 1,
      aux_sym_selector_repeat1,
    STATE(26), 1,
      sym_sel_symbol,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(87), 2,
      anon_sym_COLON,
      anon_sym_COMMA,
    STATE(27), 3,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [702] = 6,
    ACTIONS(7), 1,
      sym_sel_kind,
    STATE(20), 1,
      aux_sym_selector_repeat1,
    STATE(26), 1,
      sym_sel_symbol,
    STATE(45), 1,
      sym_selector,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(27), 3,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [724] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(92), 6,
      anon_sym_COLON,
      anon_sym_COMMA,
      sym_sel_kind,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
  [737] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(94), 6,
      anon_sym_COLON,
      anon_sym_COMMA,
      sym_sel_kind,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
  [750] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(96), 6,
      anon_sym_COLON,
      anon_sym_COMMA,
      sym_sel_kind,
      anon_sym_PLUS,
      anon_sym_TILDE,
      anon_sym_GT,
  [763] = 5,
    ACTIONS(100), 1,
      anon_sym_PLUS,
    ACTIONS(102), 1,
      anon_sym_TILDE,
    ACTIONS(104), 1,
      anon_sym_GT,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(98), 3,
      anon_sym_COLON,
      anon_sym_COMMA,
      sym_sel_kind,
  [782] = 2,
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
  [795] = 4,
    ACTIONS(7), 1,
      sym_sel_kind,
    STATE(25), 1,
      sym_sel_symbol,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(27), 3,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [811] = 4,
    ACTIONS(7), 1,
      sym_sel_kind,
    STATE(24), 1,
      sym_sel_symbol,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(27), 3,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [827] = 4,
    ACTIONS(7), 1,
      sym_sel_kind,
    STATE(23), 1,
      sym_sel_symbol,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    STATE(27), 3,
      sym_sel_twins,
      sym_sel_siblings,
      sym_sel_child,
  [843] = 4,
    ACTIONS(108), 1,
      anon_sym_COLON,
    ACTIONS(110), 1,
      anon_sym_COMMA,
    STATE(31), 1,
      aux_sym_selectors_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [857] = 4,
    ACTIONS(113), 1,
      anon_sym_COMMA,
    ACTIONS(115), 1,
      anon_sym_RBRACE,
    STATE(35), 1,
      aux_sym_properties_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [871] = 4,
    ACTIONS(117), 1,
      anon_sym_COLON,
    ACTIONS(119), 1,
      anon_sym_COMMA,
    STATE(31), 1,
      aux_sym_selectors_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [885] = 4,
    ACTIONS(119), 1,
      anon_sym_COMMA,
    ACTIONS(121), 1,
      anon_sym_COLON,
    STATE(33), 1,
      aux_sym_selectors_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [899] = 4,
    ACTIONS(113), 1,
      anon_sym_COMMA,
    ACTIONS(123), 1,
      anon_sym_RBRACE,
    STATE(36), 1,
      aux_sym_properties_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [913] = 4,
    ACTIONS(125), 1,
      anon_sym_COMMA,
    ACTIONS(128), 1,
      anon_sym_RBRACE,
    STATE(36), 1,
      aux_sym_properties_repeat1,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [927] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(130), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
  [936] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(132), 2,
      ts_builtin_sym_end,
      sym_sel_kind,
  [945] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(134), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
  [954] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(136), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
  [963] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(138), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
  [972] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(140), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
  [981] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(142), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
  [990] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(128), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
  [999] = 2,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
    ACTIONS(108), 2,
      anon_sym_COLON,
      anon_sym_COMMA,
  [1008] = 2,
    ACTIONS(144), 1,
      anon_sym_SEMI,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1016] = 2,
    ACTIONS(146), 1,
      ts_builtin_sym_end,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1024] = 2,
    ACTIONS(148), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1032] = 2,
    ACTIONS(150), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1040] = 2,
    ACTIONS(152), 1,
      anon_sym_SEMI,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1048] = 2,
    ACTIONS(154), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1056] = 2,
    ACTIONS(156), 1,
      anon_sym_COLON,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1064] = 2,
    ACTIONS(158), 1,
      anon_sym_SEMI,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1072] = 2,
    ACTIONS(160), 1,
      anon_sym_SEMI,
    ACTIONS(3), 2,
      sym_comment,
      sym_newline,
  [1080] = 2,
    ACTIONS(162), 1,
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
  [SMALL_STATE(6)] = 200,
  [SMALL_STATE(7)] = 235,
  [SMALL_STATE(8)] = 270,
  [SMALL_STATE(9)] = 307,
  [SMALL_STATE(10)] = 344,
  [SMALL_STATE(11)] = 379,
  [SMALL_STATE(12)] = 416,
  [SMALL_STATE(13)] = 453,
  [SMALL_STATE(14)] = 482,
  [SMALL_STATE(15)] = 511,
  [SMALL_STATE(16)] = 540,
  [SMALL_STATE(17)] = 572,
  [SMALL_STATE(18)] = 604,
  [SMALL_STATE(19)] = 630,
  [SMALL_STATE(20)] = 656,
  [SMALL_STATE(21)] = 679,
  [SMALL_STATE(22)] = 702,
  [SMALL_STATE(23)] = 724,
  [SMALL_STATE(24)] = 737,
  [SMALL_STATE(25)] = 750,
  [SMALL_STATE(26)] = 763,
  [SMALL_STATE(27)] = 782,
  [SMALL_STATE(28)] = 795,
  [SMALL_STATE(29)] = 811,
  [SMALL_STATE(30)] = 827,
  [SMALL_STATE(31)] = 843,
  [SMALL_STATE(32)] = 857,
  [SMALL_STATE(33)] = 871,
  [SMALL_STATE(34)] = 885,
  [SMALL_STATE(35)] = 899,
  [SMALL_STATE(36)] = 913,
  [SMALL_STATE(37)] = 927,
  [SMALL_STATE(38)] = 936,
  [SMALL_STATE(39)] = 945,
  [SMALL_STATE(40)] = 954,
  [SMALL_STATE(41)] = 963,
  [SMALL_STATE(42)] = 972,
  [SMALL_STATE(43)] = 981,
  [SMALL_STATE(44)] = 990,
  [SMALL_STATE(45)] = 999,
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
};

static TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_s, 0),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(54),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(43),
  [17] = {.entry = {.count = 1, .reusable = false}}, SHIFT(40),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(41),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [23] = {.entry = {.count = 1, .reusable = false}}, SHIFT(39),
  [25] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_attrb_repeat1, 2),
  [27] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_attrb_repeat1, 2), SHIFT_REPEAT(14),
  [30] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_attrb_repeat1, 2), SHIFT_REPEAT(14),
  [33] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attrs, 2),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [37] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attrs, 2),
  [39] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_attrs_repeat1, 2),
  [41] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_attrs_repeat1, 2), SHIFT_REPEAT(15),
  [44] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_attrs_repeat1, 2),
  [46] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute, 2),
  [48] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [50] = {.entry = {.count = 1, .reusable = false}}, SHIFT(14),
  [52] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attrb, 2),
  [54] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attrs, 1),
  [56] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attrs, 1),
  [58] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attrb, 3),
  [60] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute, 3),
  [62] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attr_or, 2),
  [64] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attr_or, 2),
  [66] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attr, 1),
  [68] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attr, 1),
  [70] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_s_repeat1, 2),
  [72] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_s_repeat1, 2), SHIFT_REPEAT(27),
  [75] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_s, 1),
  [77] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [79] = {.entry = {.count = 1, .reusable = true}}, SHIFT(51),
  [81] = {.entry = {.count = 1, .reusable = false}}, SHIFT(49),
  [83] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [85] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selector, 1),
  [87] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_selector_repeat1, 2),
  [89] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_selector_repeat1, 2), SHIFT_REPEAT(27),
  [92] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_twins, 3),
  [94] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_siblings, 3),
  [96] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_child, 3),
  [98] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_selector_repeat1, 1),
  [100] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [102] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [104] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [106] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sel_symbol, 1),
  [108] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_selectors_repeat1, 2),
  [110] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_selectors_repeat1, 2), SHIFT_REPEAT(22),
  [113] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [115] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [117] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selectors, 2),
  [119] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [121] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_selectors, 1),
  [123] = {.entry = {.count = 1, .reusable = true}}, SHIFT(50),
  [125] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_properties_repeat1, 2), SHIFT_REPEAT(19),
  [128] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_properties_repeat1, 2),
  [130] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_color_name, 1),
  [132] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hl_rule, 4, .production_id = 1),
  [134] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_fg, 3),
  [136] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ansi_color, 1, .production_id = 2),
  [138] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ansi_color, 1, .production_id = 3),
  [140] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_property, 1),
  [142] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_bg, 3),
  [144] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_properties, 3),
  [146] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [148] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [150] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [152] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_properties, 4),
  [154] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [156] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [158] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [160] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_highlight, 1),
  [162] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
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
