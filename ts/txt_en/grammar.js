module.exports = grammar({
  name: 'txt_en',

  rules: {
    source_file: $ => repeat($.sentence),
    sentence: $ => seq(repeat($.sentence_line), $.sentence_fin),
    sentence_line: $ => seq(repeat(choice($.wordws, $.wwordws)), $.nl),
    sentence_fin: $ => seq(repeat(choice($.wordws, $.wwordws)), $.dot, $.ws),
    sentence_fin: $ => seq(repeat(choice($.wordws, $.wwordws)), $.dot, $.nl),
    wordws: $ => seq($.word, optional($.ws)),
    wwordws: $ => seq($.wword, optional($.ws)),
    word: $ => /\w+/,
    wword: $ => /[^\s\.]+(\.[^\s\.]+)*/,
    dot: $ => /\./,
    nl: $ => /(\x0a\x0d)|(\x0d\x0a)|\x0a/,
    ws: $ => /[\x09\x0b\x0c\x0d\x20]+/,
  }
});
