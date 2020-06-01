module.exports = grammar({
  name: 'tss',

  rules: {
    source_file: $ => repeat(seq($.selectors, $.properties)),

    selectors: $ => seq($.selector, repeat(seq(',', $.selector))),
    selector: $ => repeat1($.sel_symbol),
    sel_symbol: $ => choice(
        $.symbol_name,
        $.sel_field,
        $.sel_symbol_field,
        $.sel_next_child,
        $.sel_prev_child,
        $.sel_child,
    ),
    sel_child: $ => prec.left(2, seq($.sel_symbol, '>', $.sel_symbol)),
    sel_next_child: $ => prec.left(2, seq($.sel_symbol, '+', $.sel_symbol)),
    sel_prev_child: $ => prec.left(2, seq($.sel_symbol, '-', $.sel_symbol)),
    sel_field: $ => seq('.', $.field_name),
    sel_symbol_field: $ => prec(2, seq($.symbol_name, '.', $.field_name)),
    symbol_name: $ => /[a-z][0-9a-zA-Z-_]/,
    field_name: $ => /[a-z][0-9a-zA-Z-_]/,

    properties: $ => seq('{', field('properties', repeat(seq($.property, ','))), '}'),
    property: $ => choice($.fg, $.bg, $.attr1, $.attr2),
    fg: $ => seq('fg', ':', choice($.rgb_color, $.ansi_color, $.color_name)),
    bg: $ => seq('bg', ':', choice($.rgb_color, $.ansi_color, $.color_name)),
    attr1: $ => seq('attr', ':', repeat($.attrs)),
    attr2: $ => seq('attribute', ':', repeat($.attrs)),
    attrs : $ => seq($.attr, repeat($.attr_or)),
    attr_or: $ => seq('|', $.attr),
    attr: $ => choice(
        'bold',
        'italic',
        'underlined',
        'underline',
        'reverse',
    ),

    rgb_color: $ => /#[0-9a-fA-F]{6}/,
    ansi_color: $ => choice(
        field('ansi_color_dec', /[0-9]+/),
        field('ansi_color_hex', /0x[0-9a-fA-F]+/),
    ),
    color_name: $ => choice(
        'black',
        'darkgrey',
        'dark-grey',
        'dark_grey',
        'red',
        'darkred',
        'dark-red',
        'dark_red',
        'green',
        'darkgreen',
        'dark-green',
        'dark_green',
        'yellow',
        'darkyellow',
        'dark-yellow',
        'dark_yellow',
        'blue',
        'darkblue',
        'dark-blue',
        'dark_blue',
        'magenta',
        'darkmagenta',
        'dark-magenta',
        'dark_magenta',
        'cyan',
        'darkcyan',
        'dark-cyan',
        'dark_cyan',
        'white',
        'grey',
    ),
  }
});
