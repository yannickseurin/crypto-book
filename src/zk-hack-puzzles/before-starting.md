# Before Starting

```
    ______ _   __  _   _            _
    |___  /| | / / | | | |          | |
       / / | |/ /  | |_| | __ _  ___| | __
      / /  |    \  |  _  |/ _` |/ __| |/ /
    ./ /___| |\  \ | | | | (_| | (__|   <
    \_____/\_| \_/ \_| |_/\__,_|\___|_|\_\
```

Over the last months, [ZK Hack](https://zkhack.dev/) published a series of cryptographic puzzles.
This is a fun way to learn about advanced cryptographic schemes such as BLS signatures, KZG polynomial commitments, proof systems and more, to improve its skills in Rust and Sage, and to delve into the `arkworks` libraries suite.

There are many great write-ups already available for every puzzle.
The goal of this walk-through is to give an in-depth analysis of the cryptography underlying each puzzle.

Needless to say, all the solutions given here are only one of the many ways to solve the puzzles.
We encourage the reader to regularly pause and try to come with its own solution!

The full code of the solutions is available at <https://github.com/yannickseurin/crypto-book/tree/main/puzzles>.

## Prerequisites

All puzzles use Rust and require some familiarity with this language.
Visit [this page](https://doc.rust-lang.org/stable/book/ch01-01-installation.html) for installation instructions and go through the first sections of [the Rust Book](https://doc.rust-lang.org/stable/book/) if you're new to Rust.

We will also rely on the [Sage](https://www.sagemath.org/) mathematics software system to solve some of the puzzles.
See [here](https://doc.sagemath.org/html/en/installation/index.html) for installation instructions.

The solutions will be given for Linux but should be easily adaptable to other operating systems.

## Getting Started

Each puzzle consists of a Rust package hosted on GitHub.
To get started, one first need to clone the project and run it, which displays the puzzle instructions.
E.g., for the first puzzle, one proceeds as follows:

```console
$ git clone https://github.com/kobigurk/zkhack-bls-pedersen
$ cd zkhack-bls-pedersen
$ cargo run --release
```

This displays the puzzle description.
Understanding the organization of the project's code requires some basic knowledge of Rust concepts of packages, crates, and modules.
[Section 7 of the Rust book](https://doc.rust-lang.org/stable/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) contains all you need to know.

## Rust Conventions and Tips

- Paths to files are given relatively to the puzzle directory.
- Most Rust snippets have an eyeball icon which will toggle the visibility of hidden lines.
- The puzzles are based on version 0.3 of the `arkworks` libraries, but version 0.4 has been released meanwhile with a handful of breaking changes; we will strive to indicate those affecting the relevant part of the crates.
- We often switch between mathematical notation and Rust variables. We write $v \cong$ `var` or `var` $\cong v$ to identify the mathematical variable $v$ and the Rust variable `var`.
