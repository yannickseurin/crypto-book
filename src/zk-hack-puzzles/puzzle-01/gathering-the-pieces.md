## Gathering the Pieces

Recall that the BLS signature of message $m$ is the point on $\GG_1$ defined as $S = x H(m)$, where $x \in \ZZ_r$ is the secret key.
We also established in the previous section that $H(m)$ is given by
\[
 H(m) \defeq \sum_{j=0}^{255} h_j B_j
\]
where the $h_j$'s are the bits of $\mathsf{blakes2s}(m)$.
Hence, the signature of $m$ can be expressed as
\[
 S = x H(m) = \sum_{j=0}^{255} h_j (x B_j).
\]
In other words, $S$ is a formal linear combination (with known coefficients) of secret points $(x B_0, \dots, x B_{255})$.
As we are given 256 signatures, we should be able to forge a new signature with some linear algebra.

In the following, we let $m^*$ be the message for which we want to forge a signature, and $\bfh^* = (h^*_0, \dots, h^*_{255}) \defeq \mathsf{blakes2s}(m^*)$ denote the output of the BLAKE2s hash function applied to $m^*$, seen as a vector of bits.
Hence, the signature $S^*$ that we want to forge is
\[
 S^* = x H(m^\ast) = \sum_{j=0}^{255} h^\ast_j (x B_j).
\]
Similarly, let $m_0, \dots, m_{255}$ denote the 256 messages whose signature is given in the puzzle data and let $\bfh_i = (h_{i,j})_{0 \le j \le 255} \defeq \mathsf{blake2s}(m_i)$ denote the hash of $m_i$ with BLAKE2s.
Then the signature $S_i$ of message $m_i$ is
\[
 S_i = \sum_{j=0}^{255} h_{i,j} (x B_j).
\]

Assume that we can write $\bfh^*$ as a linear combination of vectors $\bfh_0, \dots, \bfh_{255}$, i.e., we can find a vector $\bfc = (c_0, \dots, c_{255}) \in (\ZZ_r)^{256}$ such that
\[
 \bfh^* = \sum_{i=0}^{255} c_i \bfh_i. {{numeq}}{hstar}
\]
Then we can compute $S^*$ as
\[\begin{aligned}
 \sum_{i=0}^{255} c_i S_i & = \sum_{i=0}^{255} \sum_{j=0}^{255} c_i h_{i,j} (x B_j) \\
 & = \sum_{j=0}^{255} \underbrace{\left( \sum_{i=0}^{255} c_i h_{i,j} \right)}_ {h^\ast_j}(x B_{j}) \\
 & = S^\ast.
\end{aligned}\]
How do we compute $\bfc$? Letting $\bfM$ denote the $256 \times 256$ matrix whose rows are $\bfh_0, \dots, \bfh_{255}$, then Eq. {{eqref: hstar}} is equivalent to
\[
 \bfh^* = \bfc \cdot \bfM.
\]
Hence, we need to solve a linear system.

### Implementing the Attack

To perform the linear algebra and compute $\bfc$, we will use Sage.
For this, we first write $\bfM$ and $\bfh^*$ as arrays in a file *sage/data.sage* that we will load in Sage later on.
Note that we must write the individual bits of the outputs of BLAKE2s.
For this, we use the [`bytes_to_bits`](https://docs.rs/ark-crypto-primitives/0.3.0/src/ark_crypto_primitives/crh/pedersen/mod.rs.html#161-170) function from the `pedersen` module which returns a vector of booleans.
Trying to write this vector yields an array of `true` and `false` strings which is not what we want, so we first need to convert these booleans into bytes.

```rust
{{#rustdoc_include ../../../puzzles/zkhack-bls-pedersen/src/bin/verify-bls-pedersen.rs:write}}
```

Then we move to Sage.
We write a script *sage/lin_algebra.sage* that reads matrix $\bfM$ and vector $\bfh^*$ from file *sage/data.sage* and then we use the [`solve_left`](https://doc.sagemath.org/html/en/reference/matrices/sage/matrix/matrix2.html#sage.matrix.matrix2.Matrix.solve_left) method to solve the linear system.
Note that we must work over the scalar field $\FF_r$ of BLS12-381 and explicitly declare that $\bfM$ and $\bfh^*$ are defined over $\FF_r$.
After that, we write coefficients of solution $\bfc$ returned by Sage in file *sage/coeffs.txt*, one coefficient per line.
Note that $\bfM$ is invertible and the system has a unique solution (which was not a given).
Here is the content of the *sage/lin_algebra.sage* file (the size $r$ of the scalar field of BLS12-381 can be found for example [here](https://docs.rs/ark-bls12-381/0.4.0/ark_bls12_381/)):

```python
{{#include ../../../puzzles/zkhack-bls-pedersen/sage/lin_algebra.sage}}
```

We simply run it with

```console
$ sage ./sage/lin_algebra.sage
```

It remains to import the coefficients in our Rust function and compute $S^*= \sum c_i S_i$.
For this, we read file *sage/coeffs.txt* line by line, obtaining strings that we must convert into elements of the scalar field.
Hence, we bring BLS12-381 scalar field into scope with `use ark_bls12_381::Fr` and look for how to do the conversion.
Initially, I tried to use functions [`from_be_bytes_mod_order`](https://docs.rs/ark-ff/0.3.0/ark_ff/fields/trait.PrimeField.html#method.from_be_bytes_mod_order) and [`from_le_bytes_mod_order`](https://docs.rs/ark-ff/0.3.0/ark_ff/fields/trait.PrimeField.html#method.from_le_bytes_mod_order) of the `PrimeField` trait (after converting strings into slices of bytes using `as_bytes`): the code compiles but the forgery does not verify...
A simpler solution uses the fact that the `PrimeField` trait has supertrait `FromStr`, meaning one can directly convert strings into the `Fr` type using the [`parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse) method.

```rust
{{#rustdoc_include ../../../puzzles/zkhack-bls-pedersen/src/bin/verify-bls-pedersen.rs:read}}
```

We are now ready to compute the forgery.
Scalar multiplication is performed using the `mul` method.
One can choose to work with affine or projective coordinates.
One thing to note is that `mul` applied to a point in affine coordinates [returns a point in projective coordinates](https://docs.rs/ark-ec/0.3.0/ark_ec/trait.AffineCurve.html#tymethod.mul).
In order to add it to an affine point afterwards, it must be converted back into affine form using method `into`.
There is also the option to use the [`multi_scalar_mul`](https://docs.rs/ark-ec/0.3.0/ark_ec/msm/struct.VariableBaseMSM.html#method.multi_scalar_mul) function (replaced by [`msm`](https://docs.rs/ark-ec/0.4.0/ark_ec/scalar_mul/variable_base/trait.VariableBaseMSM.html#method.msm) in version 0.4.0 of the library) implementing multi-scalar multiplication directly.
However, the scalars in vector `coeffs` must first be cast into their "big integer" representation using the [`into_repr`](https://docs.rs/ark-ff/0.3.0/ark_ff/fields/trait.PrimeField.html#tymethod.into_repr) method.
The code below uses the three possibilities.
Note that the `+=` operator does not work for affine representation.

```rust
{{#rustdoc_include ../../../puzzles/zkhack-bls-pedersen/src/bin/verify-bls-pedersen.rs:forge}}
```

The attack requires to run the Rust binary to write the file *sage/data.sage*, then the Sage script, and then the Rust binary again to read *sage/coeffs.txt*. Not great, but I have no idea how to call the Sage script from the Rust `main` function.
Feel free to improve this, for example with a bash script.

### Conclusion

The key takeaway of this puzzle is that ***Pedersen hashing does not behave as a random oracle***. Although it is provably collision-resistant assuming the discrete logarithm problem is hard, it has a rich algebraic structure which makes it unsuitable for cases where a hash function behaving as a random oracle is required, such as BLS signatures.

Which hash-to-curve function should be used to make BLS secure then?
The easy solution is to hash the message together with a counter into the base field of the elliptic curve until the result is the x-coordinate of a point on the curve.{{footnote: Note that hashing into the *scalar* field to get $h(m) \in \FF_r$ and letting $H(m) = h(m) G_1$ is completely insecure: one single signature $S = x H(m) = x (h(m) G_1)$ on some message $m$ reveals $x G_1 = h(m)^{-1} S$, which allows to forge a signature on any other message $m'$ by computing $S' = h(m') (x G_1) = x H(m')$.}}
The drawback is that it is not possible to implement it in constant time and that security of this construction is not known to hold in the strong sense of being indifferentiable from a random oracle [[BCI+10](../../references.md#BCI+10)].
For the specific case of BLS12-381, an efficient solution based on isogenies was recently proposed by Wahby and Boneh [[WB19](../../references.md#WB19)].
See also [RFC 9380](https://www.rfc-editor.org/rfc/rfc9380.html) specifying various hash-to-curve constructions.
