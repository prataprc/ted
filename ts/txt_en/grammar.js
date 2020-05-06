module.exports = grammar({
  name: 'txt_en',

  rules: {
    source_file: $ => repeat(choice($.word, $.wword, $.dot, $.nl)),
    word: $ => /\w+[\x09\x0b\x0c\x0d\x20]*/,
    wword: $ => /[^\s\.]+(\.[^\s\.]+)*[\x09\x0b\x0c\x0d\x20]*/,
    dot: $ => /\.[\x09\x0b\x0c\x0d\x20]*/,
    nl: $ => /(\x0a\x0d)|(\x0d\x0a)|\x0a/,
    ws: $ => /[\x09\x0b\x0c\x0d\x20]+/,
  }
});
