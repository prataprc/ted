Ted's design philosophy sits some-where between the older generation
editors, like vim, emacs, and the new-gen editors like vscode.

Buffer

Like vi/emacs all text content are handled as buffer. The fundamental
aspect of a buffer is that,

* It is a a blob of bytes, that shall be interpreted as String.
* It has a cursor, character-indexed, within the text.

As of this writing `Buffer` type is backed by `Rope` data-structure,
with basic support for multi-version-concurrency-control.

What to expect from a `Buffer` ?

* It holds a reference to the location, from where the original content
  was read. This means, persistence and content-synchronisation are
  expected to be handled by Buffer type.
* Understand the different text-encoding, like Utf8.
* Understand different text-formats like dos, mac, unix. The main
  difference between these formats lies in the way they interpret new-line.
* Undestand lines-breaks (aka newlines) within text-content and hence it
  is both character-aware and line-aware, and supports both cursor index
  and (row, col) index into the text-content. Note that all indexes
  are character-indexed, and not byte-indexed.
* Move the cursor around the buffer.
* Insert, Append, Replace and Delete parts of text.

Window

Unlike vim/emacs, rendering text is entirely separated from buffer
mechanics. Rendering text is tightly coupled with one of the many
Window type, which defines all screen related behaviour.

Text-format

Text-format defines how end-of-line is interpreted in the text. Following
text formats are supported,

* `dos`, a sequence of <CR><NL> or <NL> is considered new-line
* `unix`, <NL> is considered new-line.
* `mac`, <CR> is considered new-line.

In reality, all new-line related behaviour is transparently handled
without user intervention. While the issue of handling mixed representation,
like having a <CR> in other-wise `unix` like new-line, is still open. Is
is possible to have such scenario ?
