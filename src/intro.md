# Introduction

This book is an ongoing effort to gather some notes about cryptography with a focus on schemes which are relevant to [the decentralized web](https://dci.mit.edu/decentralizedweb) such as multiparty signatures, zero-knowledge proofs, etc.

For now it consists of the following parts:

- ***Mathematics***:
- ***Cryptography***:
- ***Proof Systems***:
- ***ZK Hack Puzzles Walk-through***:

We assume that the reader has some basic knowledge of arithmetic and algebra and of common concepts from cryptography (hash functions, signatures, ...).

Here are a number of freely available textbooks to learn more (we will point to specific sections of them when needed):

- [A Computational Introduction to Number Theory and Algebra](https://shoup.net/ntb/) by Victor Shoup
- the [Boneh-Shoup cryptography textbook](https://toc.cryptobook.us/book.pdf)
- [lecture notes](http://www.cs.umd.edu/~jkatz/gradcrypto2/scribes.html) for an advanced graduate cryptography course by Jonathan Katz
- [The Joy of Cryptography](https://joyofcryptography.com/) by Mike Rosulek
- [Mathematics of Public Key Cryptography](https://www.math.auckland.ac.nz/~sgal018/crypto-book/main.pdf) by Steven Galbraith
- Least Authority's [MoonMath manual](https://leastauthority.com/community-matters/moonmath-manual/)
- [Proofs, Arguments, and Zero-Knowledge](https://people.cs.georgetown.edu/jthaler/ProofsArgsAndZK.pdf) by Justin Thaler

## Mathematical Notation

- Given a set $S$, we let $S^n$ denote the set of strings of length $n$ over $S$, and $S^*$ the set of all strings, i.e., $S^* = \cup_{n \ge 0} S^n$, where $S^0$ denotes the singleton consisting of the empty string; the length of a string $x$ is denoted $|x|$.
- Given a non-empty finite set $S$, the sampling of a variable $s$ according to the uniform distribution is denoted $s \sample S$.
- Unless specified otherwise, groups are denoted additively.
- Main algebraic structures:

| notation | algebraic structure |
|:------:|:------:|
| $\GG$, $\HH$ | arbitrary group |
| $\AA$, $\BB$ | arbitrary ring |
| $\DD$ | integral domain |
| $\FF$ | arbitrary field |
| $\NN$ | natural numbers |
| $\ZZ$ | integers |
| $\QQ$ | rational numbers |
| $\RR$ | real numbers |
| $\CC$ | complex numbers |

Note that all proofs throughout the book are collapsible:

```admonish proof collapsible=true
You can choose to display it or leave it hidden forever.
```

## Acknowledgments

This book is built with [mdBook](https://rust-lang.github.io/mdBook/) using the following preprocessors:

- [mdBook-KaTeX](https://github.com/lzanini/mdbook-katex)
- [mdbook-toc](https://github.com/badboy/mdbook-toc)
- [mdbook-footnote](https://github.com/daviddrysdale/mdbook-footnote)
- [mdbook-mermaid](https://github.com/badboy/mdbook-mermaid)
- [mdbook-admonish](https://github.com/tommilligan/mdbook-admonish)
- [mdbook-numthm](https://github.com/yannickseurin/mdbook-numthm)
- [mdbook-numeq](https://github.com/yannickseurin/mdbook-numeq)
- [mdbook-mathpunc](https://github.com/yannickseurin/mdbook-mathpunc)

If you spot anything off, I'd be happy to get your feedback and acknowledge it here.
