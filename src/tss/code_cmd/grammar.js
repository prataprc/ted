module.exports = grammar({
  name: 'code_cmd',

  extras: $ => [/[ \t]/, $.newline],

  rules: {
    s: $ => seq(optional($.range), optional($.cmd)),

    newline: $ => /\r?\n/,

    cmd: $ => choice($.set, $.edit),

    range: $ => seq($.range_start, optional(seq(',', $.range_end))),
    range_start: $ => /([0-9.%]+|(\?[^?]+\?)|'[a-z])[+-]?[0-9]*/,
    range_end: $ => /([0-9.$]+|(\/.*\/)|'[a-z])[+-]?[0-9]*/,

    set: $ => seq('set', $.set_flags),
    set_flags: $ => choice(
        'wrap',
        'nowrap',
    ),

    edit: $ => seq('edit', /.*/),
  }
});

