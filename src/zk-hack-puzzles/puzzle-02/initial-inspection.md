## Initial Inspection

On the surface, this puzzle looks like a discrete logarithm problem: we must recover some secret value $s$ given $G_1$, $s G_1$, etc.
Actually, this variant where additional points $s^2 G_1$, $s^3 G_1$, ... $s^q G_1$ are given is sometimes called the $q$-discrete logarithm (or $q$-strong discrete logarithm) problem.
In some cases, this auxiliary information enables to speed up the computation of $s$ using Cheon's algorithm [[Che10](../../references.md#Che10)].
Note also that the assumption that the $q$-discrete log problem is hard implies the soundness of the Groth16 proof system in a restricted model of computation called the algebraic group model [[FKL18](../../references.md#FKL18)].

Let's take a look at the code.
The package directory is organized as follows:

```text
zkhack-trusted-setup
├── Cargo.toml
└── src
    ├── bin
    │   └── verify-trusted-setup.rs
    ├── data.rs
    └── lib.rs
```

As with the previous puzzle, the package has two crates: a library crate with root file *src/lib.rs* which simply declares a module `data` and a string slice with the puzzle description and a binary crate with root file *src/bin/verify-trusted-setup.rs* which contains the following code:

```rust
use ark_bls12_381::Fr;
use ark_ec::AffineCurve;
use prompt::{puzzle, welcome};
use std::str::FromStr;
use trusted_setup::data::puzzle_data;
use trusted_setup::PUZZLE_DESCRIPTION;

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let (_ts1, _ts2) = puzzle_data();

    /* Your solution here! (s in decimal)*/
    let s = Fr::from_str("0").unwrap();

    assert_eq!(_ts1[0].mul(s), _ts1[1]);
    assert_eq!(_ts2[0].mul(s), _ts2[1]);
}
```

We can see that the `main` function simply loads the puzzle data and expects us to replace "0" with the correct value for $s$ so that the two final assertions evaluate to true and the program does not panic anymore.
Note that `ark_bls12_381::Fr`, the scalar field of the [BLS12-381 pairing-friendly elliptic curve](../../mathematical-preliminaries/pairings.md), is brought into scope.

The code for the `data` module in *src/data.rs*, where the `puzzle_data` function is defined, looks like this:

```rust
use ark_bls12_381::{G1Affine, G2Affine};
use ark_serialize::CanonicalDeserialize;
use std::io::Cursor;

pub fn puzzle_data() -> ([G1Affine; 2 * 32 - 1 + 32 + 32], [G2Affine; 32]) {
    // ...
}
```

The `puzzle_data` function returns two arrays: `_ts1` which holds 127 elements of type `G1Affine` (the type representing elements of group $\GG_1$ of BLS12-381 in affine representation) and `_ts2` which holds 32 elements of type `G2Affine` (the type representing elements of group $\GG_2 $ of BLS12-381 in affine representation).
From this and the puzzle description we can infer that `_ts1` holds
\[
 [G_1, sG_1, \dots, s^{62} G_1, \alpha G_1, \alpha s G_1, \dots, \alpha s^{31} G_1, \beta G_1, \beta s G_1, \dots, \beta s^{31} G_1]
\]
while `_ts2` holds
\[
  [G_2, sG_2, \dots, s^{31} G_2].
\]

Importantly, $G_1$ and $G_2$ above do not refer to [the commonly agreed subgroup generators](https://github.com/zcash/librustzcash/blob/6e0364cd42a2b3d2b958a54771ef51a8db79dd29/pairing/src/bls12_381/README.md#generators) of BLS12-381 but to the points `_ts1[0]` and `_ts2[0]` provided by Alice.

Taking a closer look at how these values are defined, we can see that `puzzle_data` calls an associated function named `deserialize_unchecked`.
What is it that this function does not check?

We head up towards the [ark-ec-0.3.0 crate documentation](https://docs.rs/ark-ec/0.3.0/ark_ec/) and search for `deserialize_unchecked`, which leads us [here](https://docs.rs/ark-ec/0.3.0/ark_ec/models/short_weierstrass_jacobian/struct.GroupAffine.html#method.deserialize_unchecked).
This function is part of the `ark_serialize::CanonicalDeserialize` trait.
The documentation simply says:

> `fn deserialize_unchecked<R: Read>(reader: R) -> Result<Self, SerializationError>`
>
> Reads `self` from `reader` without compression, and without performing validity checks. Should be used only when the input is trusted.

Not very informative, so we jump to [the source code](https://docs.rs/ark-ec/0.3.0/src/ark_ec/models/short_weierstrass_jacobian.rs.html#856-862):

```rust
fn deserialize_unchecked<R: Read>(mut reader: R) -> Result<Self, SerializationError> {
    let x: P::BaseField = CanonicalDeserialize::deserialize(&mut reader)?;
    let (y, flags): (P::BaseField, SWFlags) =
        CanonicalDeserializeWithFlags::deserialize_with_flags(&mut reader)?;
    let p = GroupAffine::<P>::new(x, y, flags.is_infinity());
    Ok(p)
}
```

This function simply reads two base field elements $x$ and $y$ from the buffer and creates a new point $(x, y)$ in affine coordinates.
What could go wrong?
If there's a `deserialize_unchecked` function, maybe there's a sibling which actually checks something? Indeed, the function just above in the source code reads:

```rust
fn deserialize_uncompressed<R: Read>(
    reader: R,
) -> Result<Self, ark_serialize::SerializationError> {
    let p = Self::deserialize_unchecked(reader)?;

    if !p.is_in_correct_subgroup_assuming_on_curve() {
        return Err(SerializationError::InvalidData);
    }
    Ok(p)
}
```

(There is also a `deserialize` function which does something similar for points in compressed form, meaning they are encoded with their $x$-coordinate and the sign of $y$.)
Here we are: the crucial property which is not checked by `deserialize_unchecked` is whether the curve point it returns is in the correct subgroup. What does that mean exactly?
