## Initial Inspection

As this is the first puzzle, we will go over the code in details.

The package directory is structured as follows:

```text
zkhack-bls-pedersen
├── Cargo.toml
└── src
    ├── bin
    │   └── verify-bls-pedersen.rs
    ├── bls.rs
    ├── data.rs
    ├── hash.rs
    └── lib.rs
```

It has two crates:

- a library crate with root file *src/lib.rs*,
- a binary crate with root file *src/bin/verify-bls-pedersen.rs*.

Let's take a look at the code inside the file *src/lib.rs.*:

```rust
{{#rustdoc_include ../../../puzzles/zkhack-bls-pedersen/src/lib.rs}}
```

It simply declares three public modules named `bls`, `data`, and `hash` and a string slice named `PUZZLE_DESCRIPTION` with the text which is displayed when running the project.

Let's now have a look at the code in the binary crate's source file *src/bin/verify-bls-pedersen.rs*:

```rust
use bls_pedersen::bls::verify;
use bls_pedersen::data::puzzle_data;
use bls_pedersen::PUZZLE_DESCRIPTION;
use prompt::{puzzle, welcome};

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let (pk, ms, sigs) = puzzle_data();
    for (m, sig) in ms.iter().zip(sigs.iter()) {
        verify(pk, m, *sig);
    }

    /* Your solution here! */
    /*
      let sig = ...;
      let m = your username;
      verify(pk, m, sig);
    */
}
```

It first brings some items from the library crate into scope with the `use` keyword.
Note that when a package contains both a library and a binary crate, any public item from the library crate can be used in the binary crate by starting paths with the name of the package (which is specified in the *Cargo.toml* file; in this case, it is `bls_pedersen`).
It also brings into scope two functions called `welcome` and `puzzle` defined in the [`prompt` external crate](https://github.com/kobigurk/zkhack-prompt/blob/master/src/lib.rs).

The `main` function first calls `welcome` and `puzzle` which respectively display some nice ASCII art and the puzzle description.
It then calls the `puzzle_data` function.
From the path used to bring `puzzle_data` into scope, we know that this function is defined in the `data` module of the library.
Hence, we look into the file *src/data.rs* for its code, which looks like this:

```rust
pub fn puzzle_data() -> (G2Affine, Vec<Vec<u8>>, Vec<G1Affine>) {
    // ...
    (pk, ms, sigs)
}
```

This function returns a tuple made up of a public key `pk` of type `G2Affine`, a vector `ms` of messages of type `Vec<u8>`, and a vector `sigs` of signatures of type `G1Affine`.
We'll come back to types `G1Affine` and `G2Affine` shortly.
The `main` function then runs a loop which checks the validity of all message/signature pairs with respect to public key `pk`:

```rust
    for (m, sig) in ms.iter().zip(sigs.iter()) {
        verify(pk, m, *sig);
    }
```

If you don't understand the syntax of the loop, read about [iterators](https://doc.rust-lang.org/stable/book/ch13-02-iterators.html) and the [`zip` method](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.zip) as this will come up quite often.

The `verify` function is defined in the `bls` module, as indicated by the path used to bring it into scope.
In order to understand what this function does, make sure to read the chapters about [pairings](../../mathematical-preliminaries/pairings.md) and [BLS signatures](../../cryptographic-notions/bls-signatures.md).
Here is the code:

```rust
{{#rustdoc_include ../../../puzzles/zkhack-bls-pedersen/src/bls.rs:verify}}
```

Just from the names of the functions, we can guess that it first hashes the message $m \cong$ `msg` by calling `hash_to_curve`, getting a point $H(m) \cong$ `h` on $\mathbb{G_1}$, and then asserts that the product of two pairings is equal to $1_{\GG_t}$, the identity element of group $\GG_t$.
Ignoring the `into` method for now, we can see that the arguments for the first pairing are the signature $S \cong$ `sig` and the opposite of the generator $G_2$ of group $\GG_2$.
The arguments for the second pairing are the hash $H(m) \cong$ `h` and the public key $X \cong$ `pk`.

Hence, what `verify` is asserting is whether (using our notation from the [BLS signatures](../../cryptographic-notions/bls-signatures.md) chapter)
\[
 e(S, -G_2) \cdot e(H(m), X) = 1_{\GG_t} {{numeq}}{alt_bls_verif}
\]
which is equivalent to the verification equation {{eqref: bls_verif}} we gave when describing BLS since
\[\begin{aligned}
 e(S, -G_2) \cdot e(H(m), X) = 1_{\GG_t} & \Leftrightarrow e(S, G_2)^{-1} \cdot e(H(m), X) = 1_{\GG_t} \\
 & \Leftrightarrow e(H(m), X) = e(S, G_2).
\end{aligned}\]

So `verify` is indeed checking a BLS signature.
Computing a product of pairings can be done more efficiently than computing the pairings one by one (see [here](../../mathematical-preliminaries/pairings.md)), which explains why performing verification using Eq. {{eqref: alt_bls_verif}} is often preferable.


What pairing-friendly curve does the signature scheme use?
The arkworks libraries implement many such curves (see [the list here](https://github.com/arkworks-rs/curves)).
From the *Cargo.toml* file listing dependencies of the package, we can see that it includes the `ark-bls12-381` library crate, where the `Bls12_381` type prefixing the call to `product_of_pairings` is defined.
Hence, the puzzle uses the [BLS12-381 curve](../../mathematical-preliminaries/pairings.md).

Our next task is to understand what the `hash_to_curve` function does exactly.
Before that, we take a moment to explore some of the `arkworks` crates used in the puzzle.
