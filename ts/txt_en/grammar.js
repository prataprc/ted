module.exports = grammar({
  name: 'txt_en',

  rules: {
    source_file: $ => repeat($.sentence),
    sentence: $ => seq(repeat($.sentence_line), $.sentence_fin),
    sentence_line: $ => seq(repeat($.wordws), $.nl),
    sentence_fin: $ => seq(repeat($.wordws), $.dot, $.ws),
    sentence_fin: $ => seq(repeat($.wordws), $.dot, $.nl),
    wordws: $ => seq(choice($.word, $.wword), optional($.ws)),
    word: $ => /[^\s\.]+/,
    wword: $ => /[^\s\.]+(\.[^\s\.]+)*/,
    dot: $ => /\./,
    nl: $ => /(\x0a\x0d)|(\x0d\x0a)|\x0a/,
    ws: $ => /[\x09\x0b\x0c\x0d\x20]+/,
  }
});
