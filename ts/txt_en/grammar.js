module.exports = grammar({
  name: 'txt_en',

  rules: {
    source_file: $ => repeat($.sentence),
    sentence: $ => seq(repeat($.sentence_line), $.sentence_fin),
    sentence_line: $ => seq(repeat($.word), $.nl),
    sentence_fin: $ => seq(repeat($.word), $.dot, $.ws),
    sentence_fin: $ => seq(repeat($.word), $.dot, $.nl),
    word: $ => seq(choice($.word, $.wword), $.ws),
    word: $ => /[^\s\.]+/,
    wword: $ => /[^\s\.]+(\.[^\s\.]+)*/,
    dot: $ => /\./,
    nl: $ => /(\x0a\x0d)|(\x0d\x0a)|\x0a/,
    ws: $ => /[\x09\x0b\x0c\x0d\x20]+/,
  }
});
