To clone this project and make it working:

* Install Rust [tool-chain][rust].
* [Clone][git-clone] this repository locally.
* Clone submodules.
  * `$ cd <repo-clone-path>`
  * `$ git submodule init`
  * `$ git submodule update`
* Setup tree-sitter incremental-parser
  * Install `npm` package.
  * `$ npm install tree-sitter-cli`
  * `$ export PATH=$PATH:<repo-clone-path>/node_modules/.bin`

[rust]: https://www.rust-lang.org/tools/install
[git-clone]: https://docs.github.com/en/github/creating-cloning-and-archiving-repositories/cloning-a-repository
