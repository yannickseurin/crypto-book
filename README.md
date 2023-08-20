# Crypto Book (Work in Progress)

This repository contains the source files for a cryptography book covering schemes used for decentralized web ("web3").
The latest version of the book is available at <https://yannickseurin.github.io/crypto-book/>.

## Building

The book is built using the following tools:

- [mdbook](https://github.com/rust-lang/mdBook)
- [mdBook-KaTeX](https://github.com/lzanini/mdbook-katex)
- [mdbook-toc](https://github.com/badboy/mdbook-toc)
- [mdbook-mermaid](https://github.com/badboy/mdbook-mermaid)

First clone the repository:

```shell
git clone https://github.com/yannickseurin/crypto-book/
cd crypto-book
```

You must Rust installed.
Visit [this page](https://doc.rust-lang.org/stable/book/ch01-01-installation.html) for installation instructions.

Then install the tools with:

```shell
cargo install mdbook
cargo install mdbook-katex
cargo install mdbook-toc
cargo install mdbook-mermaid
```

Run

```shell
mdbook serve --open
```

This will start a web server with the book.
You'll find the content on <http://localhost:3000>.
You can also use `mdbook build` to create a static version of the book in the `book/` directory.
