module.exports = grammar({
  name: 'code_cmd',

  extras: $ => [/[ \t]/, $.newline],

  rules: {
    s: $ => choice($.set),
    newline: $ => /\r?\n/,
    set: $ => seq('set', $.set_flags),
    set_flags: $ => choice(
        'wrap',
        'nowrap',
    )
  }
});

