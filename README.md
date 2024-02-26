# Crypto Book (Work in Progress)

This repository contains the source files for a cryptography book covering schemes used for decentralized web ("web3").
The latest version of the book is available at <https://yannickseurin.github.io/crypto-book/>.

## Building

The book is built with [mdBook](https://rust-lang.github.io/mdBook/) using the following preprocessors:

- [mdBook-KaTeX](https://github.com/lzanini/mdbook-katex)
- [mdbook-toc](https://github.com/badboy/mdbook-toc)
- [mdbook-footnote](https://github.com/daviddrysdale/mdbook-footnote)
- [mdbook-mermaid](https://github.com/badboy/mdbook-mermaid)
- [mdbook-admonish](https://github.com/tommilligan/mdbook-admonish)
- [mdbook-numthm](https://github.com/yannickseurin/mdbook-numthm)
- [mdbook-numeq](https://github.com/yannickseurin/mdbook-numeq)
- [mdbook-mathpunc](https://github.com/yannickseurin/mdbook-mathpunc)

First clone the repository:

```shell
git clone https://github.com/yannickseurin/crypto-book/
cd crypto-book
```

You must have Rust installed.
Visit [this page](https://doc.rust-lang.org/stable/book/ch01-01-installation.html) for installation instructions.

Then install mdbook and the preprocessors with:

```shell
cargo install mdbook
cargo install mdbook-katex mdbook-toc mdbook-footnote mdbook-mermaid mdbook-admonish mdbook-numthm mdbook-numeq mdbook-mathpunc
```

Then run mdbook-admonish to install required CSS files:

```shell
mdbook-admonish install --css-dir ./assets
mdbook-admonish generate-custom ./assets/mdbook-admonish-custom.css
```

Then run:

```shell
mdbook serve --open
```

This will start a web server with the book.
You'll find the content on <http://localhost:3000>.
You can also use `mdbook build` to create a static version of the book in the `book/` directory.
