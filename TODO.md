Stated Goal
===========

* Modern alternative to vim.
* It is modal-editor.
* Vim excels best when it comes to editing plain-text or formatted-text,
  hence ted prefers to stay as close as possible with vim.
* When it comes to documentation, code-navigation, IDE-capabilities and
  other less frequently used features of vim, ted prefers modern opinions
  that make a better use-case.

Visual design
=============

```
        +-------------------------------+
screen  | window 1      | window 2      |
        |               |               |
        |               |               |
        |= status line =|= status line =|
        | window 3                      |
        |                               |
        |                               |
        |==== status line ==============|
        |command line                   |
        +-------------------------------+
```

Nomen-clature
=============

screen; window; status-line; tab; command-line;
buffer; buffer-list;
current-file; alternate-file;
insert-mode; normal-node; visual-mode; ex-mode;
file-type;

Commandline input
=================

`%` -> current-file-name
`#` -> alternate-file-name

:f[file]          -> buffer's file-status.
:f[file]!         -> buffer's full file-status.
:f[file]! {name}  -> set current-buffer's file-name.
:buffers          -> list of buffers.
:files
:ls

:keepalt {cmd}		Execute {cmd} while keeping the current alternate file
			name.  Note that commands invoked indirectly (e.g.,
			with a function) may still set the alternate file
			name.

CTRL-^			Edit the alternate file.  Mostly the alternate file is
			the previously edited file.  This is a quick way to
			toggle between two files.  It is equivalent to ":e #",
			except that it also works when there is no file name.

|zf|	zf	define a fold

Normal-mode input
=================

N   `CTRL-^` -> toggle between current-file and alternate-file.
    `CTRL-G` -> file-status.
  g `CTRL-G` -> cursor status.

Postponed
=========

* All tabs are spaces, defined by `tabstop`.
* Line wraps.
* Visual mode.
* File Backup.
* Virtual edit.

Cmdline-Arguments
=================
