## Solving the Puzzle

Recall that our task is to find another secret/nullifier pair that will satisfy the spending circuit.
We are not allowed to modify the Merkle root nor the Merkle proof, meaning this secret/nullifier pair must correspond to the exact same leaf of the Merkle tree.

But wait, the leaf does not really encode the public key `pk`, only its $x$-coordinate!
This is the key insight to solve the puzzle.
Indeed, for any point $P = (x,y)$ on the curve (different from the point at infinity), there is another point on the curve with the same $x$-coordinate, namely $-P = (x,-y)$.
This point simply correspond to secret $-s \bmod r_6$ where $r_6$ is the size of the scalar field of the MNT6-753 curve since $(-s \bmod r_6) G = - (sG) = -P$.

Hence, all we have to do to solve the puzzle is to take the opposite of `leaked_secret` mod $r_6$ and compute the corresponding nullifier!
There's a catch though: `leaked_secret` is defined as an element in the scalar field of MNT4-753, hence simply defining `secret_hack = - leaked_secret`  won't work as this will compute $-s \bmod r_4$ where $r_4$ is the size of the scalar field of MNT4-753.

There are probably several options here, but a simple one is to cast `leaked_secret` as a big integer first.
For this, we need to add the `num-bigint` crate to the project:

```console
$ cargo add num-bigint
```

Here is the code allowing to solve the puzzle:

```rust
{{#rustdoc_include ../../../puzzles/puzzle-gamma-ray/src/main.rs:solve}}
```

### A Note about Hint 1

The first hint revealed to help solve the puzzle points to Lemma 5.4.7 of the [Zcash specifications](https://zips.z.cash/protocol/protocol.pdf).
It reads:

> Let $P = (u,v) \in \mathbb{J}^{(r)}$. Then $(u,-v) \notin \mathbb{J}^{(r)}$.

Here, $\mathbb{J}^{(r)}$ is (a subgroup of) the Jubjub curve developed by the ZCash team.
Its base field is actually the scalar field of BLS12-381, allowing to efficiently prove algebraic statements about this curve using BLS12-381-based SNARKs.

The reason why the "attack" used to solve the puzzle would not apply with this curve is that it is a twisted Edwards curve rather than a curve in short Weierstrass form.
In particular, Theorem 5.4.8 in the same document states that the function mapping points in $\mathbb{J}^{(r)}$ to their $x$-coordinate is injective, meaning two distinct points have distinct $x$-coordinates.
In this case, it is safe to encode a point by recording only its $x$-coordinate in a leaf.