## Exploring the `arkworks` Libraries

As explained in the introduction, the ZK Hack puzzles are a great opportunity to explore the `arkworks` libraries.
Although we have a reasonable understanding of what `verify` does, let us pause a moment and explain how to find its way in all the crates `arkworks` provides.
Say we want to understand what the function `Bls12_381::product_of_pairings` does exactly.
First, we check the path used to bring `Bls12_381` into scope in the *src/bls.rs* module:

```rust
use ark_bls12_381::{Bls12_381, G1Affine, G2Affine};
```

This tells us that we need to look into the `ark-bls12-381` crate.{{footnote: Note that hyphens are not valid characters in Rust identifiers, however it is possible (and idiomatic) to use them in package and crate names. Cargo automatically converts them to underscores. See [here](https://stackoverflow.com/questions/60794905/why-is-changing-hyphenated-crate-names-to-underscored-names-possible-and-what-ar).}}

The first thing to know is that there are two possible places where to look for information: [Docs.rs](https://docs.rs/), the documentation host for Rust crates hosted at [crates.io](https://crates.io), and the [arkworks GitHub repositories](https://github.com/arkworks-rs).{{footnote: If you're curious about what guarantees we have that the source codes on github.com and crates.io are really the same, I recommend [this interesting blog post](https://codeandbitters.com/published-crate-analysis/) by Eric Seppanen.}}

Second, one has to be careful about which version of the crate the puzzle requires.
For this, we must inspect the *Cargo.toml* file which lists the package dependencies:

```toml
[dependencies]
ark-std = "0.3"
ark-ff = "0.3"
ark-ec = "0.3"
ark-serialize = "0.3"
ark-bls12-381 = "0.3"
ark-crypto-primitives = "0.3"
rand = "0.8"
rand_chacha = "0.3"
hex = "0.4"
prompt = { git = "https://github.com/kobigurk/zkhack-prompt" }
blake2s_simd = "0.5.11"
```

We can see that the puzzle requires version 0.3 of the `ark-bls12-381` crate.
Hence, we select to [correct version](https://docs.rs/ark-bls12-381/0.3.0/ark_bls12_381/index.html) of the `ark-bls12-381` crate on [Docs.rs](https://docs.rs/) as starting point of our exploration.

If you prefer to browse libraries on GitHub or locally, be careful to check out the correct commit: the `ark-ec` crate is part of the *algebra* repository, the releases of which are listed [here](https://github.com/arkworks-rs/algebra/releases).

When entering `product_of_pairings` in the search bar on top of the documentation page of the `ark-bls12-381` crate, we don't get any hit.
This probably means that this function is part of a trait that the type `Bls12_381` implements using the default implementation.
Hence, we search for this type instead, which leads us [to its definition](https://docs.rs/ark-bls12-381/0.3.0/ark_bls12_381/type.Bls12_381.html):

```rust
type Bls12_381 = Bls12<Parameters>;
```

Following the link to the definition of the [`Bls12`](https://docs.rs/ark-ec/0.3.0/ark_ec/models/bls12/struct.Bls12.html) type, we see that it is defined in the `models::bls12` submodule of the `ark-ec` crate, which contains all the generic code for curves of the BLS family with embedding degree 12:

```rust
pub struct Bls12<P: Bls12Parameters>(_);
```

This illustrates how `arkworks` uses traits to abstract common behaviour of various curves.
`Bls12` is an empty struct parameterized by a generic type `P` that must satisfy the trait bound [`Bls12Parameters`](https://docs.rs/ark-ec/0.3.0/ark_ec/models/bls12/trait.Bls12Parameters.html).
A specific curve of the BLS-12 family, such as BLS12-381, is then instantiated by defining an empty struct `Parameters` implementing the `Bls12Parameters` trait [in a specific way](https://docs.rs/ark-bls12-381/0.3.0/src/ark_bls12_381/curves/mod.rs.html#20-30).

We can now search for `product_of_pairings` in the `ark-ec` crate.
We are more lucky this time as we [find out](https://docs.rs/ark-ec/0.3.0/ark_ec/trait.PairingEngine.html#method.product_of_pairings) that it is part of the `PairingEngine` trait.
Let's take a look at the code:

```rust
pub trait PairingEngine: Sized + 'static + Copy + Debug + Sync + Send + Eq + PartialEq {
    // ...

    /// Computes a product of pairings.
    #[must_use]
    fn product_of_pairings<'a, I>(i: I) -> Self::Fqk
    where
        I: IntoIterator<Item = &'a (Self::G1Prepared, Self::G2Prepared)>,
    {
        Self::final_exponentiation(&Self::miller_loop(i)).unwrap()
    }

    // ...
}
```

We can see that a default implementation is indeed provided, computing a product of Miller loops followed by a single final exponentiation.
You can keep digging from here and inspect how [`miller_loop`](https://docs.rs/ark-ec/0.3.0/src/ark_ec/models/bls12/mod.rs.html#132-188) and [`final_exponentiation`](https://docs.rs/ark-ec/0.3.0/src/ark_ec/models/bls12/mod.rs.html#190-256) are implemented for the BLS-12 family.

Note that the `product_of_pairings` function [has been replaced](https://github.com/arkworks-rs/algebra/blob/master/CHANGELOG.md?plain=1#L104) in version 0.4.0 of the `ark-ec` crate by the [`multi_pairing`](https://docs.rs/ark-ec/0.4.2/ark_ec/pairing/trait.Pairing.html#method.multi_pairing) function.

Next, we will see what the `into` method applied to curve points does.

### Affine versus Projective Coordinates

The `ark-ec` library allows you to work both with [affine and projective coordinates](../../mathematical-preliminaries/elliptic-curves.md#affine-versus-projective-coordinates) and to easily switch between them.
For short Weierstrass curves, the affine representation corresponds to the [`GroupAffine`](https://docs.rs/ark-ec/0.3.0/ark_ec/models/short_weierstrass_jacobian/struct.GroupAffine.html) struct implementing the [`AffineCurve`](https://docs.rs/ark-ec/0.3.0/ark_ec/trait.AffineCurve.html) trait:

```rust
pub struct GroupAffine<P: Parameters> {
    pub x: P::BaseField,
    pub y: P::BaseField,
    pub infinity: bool,
    // some fields omitted
}
```

The projective representation uses Jacobian (not homogeneous) coordinates and corresponds to the [`GroupProjective`](https://docs.rs/ark-ec/0.3.0/ark_ec/models/short_weierstrass_jacobian/struct.GroupProjective.html) struct implementing the [`ProjectiveCurve`](https://docs.rs/ark-ec/0.3.0/ark_ec/trait.ProjectiveCurve.html) trait:

```rust
pub struct GroupProjective<P: Parameters> {
    pub x: P::BaseField,
    pub y: P::BaseField,
    pub z: P::BaseField,
    // some fields omitted
}
```

Note in particular how the `GroupAffine` struct needs to hold a boolean field `infinity` indicating whether an instance is the point at infinity or not, whereas `GroupProjective` needs not.

The trait `Parameters`, an alias for [`ark_ec::models::SWModelParameters`](https://docs.rs/ark-ec/0.3.0/ark_ec/models/trait.SWModelParameters.html), contains all parameters specifying a prime-order subgroup of an elliptic curve in short Weierstrass form:

```rust
pub trait SWModelParameters: ModelParameters {
    const COEFF_A: Self::BaseField;
    const COEFF_B: Self::BaseField;
    const COFACTOR: &'static [u64];
    const COFACTOR_INV: Self::ScalarField;
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField);
    fn mul_by_a(elem: &Self::BaseField) -> Self::BaseField { ... }
    fn add_b(elem: &Self::BaseField) -> Self::BaseField { ... }
}
```

What about types `G1Affine` and `G2Affine`?
Recall that in order to define a pairing one needs two prime-order subgroups $\GG_1$ and $\GG_2$ of elliptic curves defined on respectively $\FF_p$ and some field extension $\FF_{p^m}$.
Types `G1Affine` and `G2Affine` correspond to the affine representation of these two subgroups and are defined for curves of the BLS-12 family respectively [here](https://docs.rs/ark-ec/0.3.0/ark_ec/models/bls12/g1/type.G1Affine.html) and [there](https://docs.rs/ark-ec/0.3.0/ark_ec/models/bls12/g2/type.G2Affine.html) as:

```rust
type G1Affine<P> = GroupAffine<<P as Bls12Parameters>::G1Parameters>;
type G2Affine<P> = GroupAffine<<P as Bls12Parameters>::G2Parameters>;
```

Type `P` must implement the [`Bls12Parameters`](https://docs.rs/ark-ec/0.3.0/ark_ec/models/bls12/trait.Bls12Parameters.html) trait:

```rust
pub trait Bls12Parameters: 'static {
    type Fp: PrimeField + SquareRootField + Into<<Self::Fp as PrimeField>::BigInt>;
    type Fp2Params: Fp2Parameters<Fp = Self::Fp>;
    type Fp6Params: Fp6Parameters<Fp2Params = Self::Fp2Params>;
    type Fp12Params: Fp12Parameters<Fp6Params = Self::Fp6Params>;
    type G1Parameters: SWModelParameters<BaseField = Self::Fp>;
    type G2Parameters: SWModelParameters<BaseField = Fp2<Self::Fp2Params>, ScalarField = <Self::G1Parameters as ModelParameters>::ScalarField>;

    const X: &'static [u64];
    const X_IS_NEGATIVE: bool;
    const TWIST_TYPE: TwistType;
}
```

As expected, types `G1Parameters` and `G2Parameters` must both implement the `SWModelParameters` trait with a prime base field for `G1Parameters` and a quadratic extension field for `G2Parameters`.

### Conversion

Conversion between affine and projective coordinates is handled by the [`From`](https://doc.rust-lang.org/std/convert/trait.From.html) and [`Into`](https://doc.rust-lang.org/std/convert/trait.Into.html) traits.
These are very general and useful traits that you can read about [here](https://www.lurklurk.org/effective-rust/casts.html) and [here](https://doc.rust-lang.org/rust-by-example/conversion/from_into.html).
They provide respectively an associated function `from` and a method `into`, the latter one being generally derived from the former.

The function `from` for creating a point in projective coordinates from a point in affine coordinates is implemented [here](https://docs.rs/ark-ec/0.3.0/src/ark_ec/models/short_weierstrass_jacobian.rs.html#721-731):

```rust
impl<P: Parameters> From<GroupAffine<P>> for GroupProjective<P> {
    #[inline]
    fn from(p: GroupAffine<P>) -> GroupProjective<P> {
        if p.is_zero() {
            Self::zero()
        } else {
            Self::new(p.x, p.y, P::BaseField::one())
        }
    }
}
```

The converse function, creating a point in affine coordinates from a point in projective coordinates, is implemented [here](https://docs.rs/ark-ec/0.3.0/src/ark_ec/models/short_weierstrass_jacobian.rs.html#734-758):

```rust
impl<P: Parameters> From<GroupProjective<P>> for GroupAffine<P> {
    #[inline]
    fn from(p: GroupProjective<P>) -> GroupAffine<P> {
        if p.is_zero() {
            GroupAffine::zero()
        } else if p.z.is_one() {
            // If Z is one, the point is already normalized.
            GroupAffine::new(p.x, p.y, false)
        } else {
            // Z is nonzero, so it must have an inverse in a field.
            let zinv = p.z.inverse().unwrap();
            let zinv_squared = zinv.square();

            // X/Z^2
            let x = p.x * &zinv_squared;

            // Y/Z^3
            let y = p.y * &(zinv_squared * &zinv);

            GroupAffine::new(x, y, false)
        }
    }
}
```

Note that there are also more explicit [`into_affine`](https://docs.rs/ark-ec/0.3.0/ark_ec/trait.ProjectiveCurve.html#method.into_affine) and [`into_projective`](https://docs.rs/ark-ec/0.3.0/ark_ec/trait.AffineCurve.html#method.into_projective) methods which simply call `into`.

All what we just said was for version 0.3 of the crate.
The structs and traits [have been](https://github.com/arkworks-rs/algebra/blob/master/CHANGELOG.md?plain=1#L77-L78) [renamed](https://github.com/arkworks-rs/algebra/blob/master/CHANGELOG.md?plain=1#L99-L115) in version 0.4 as follows:

- struct `GroupAffine` $\rightarrow$ `Affine`
- trait `AffineCurve` $\rightarrow$ `AffineRepr`
- struct `GroupProjective` $\rightarrow$ `Projective`
- trait `ProjectiveCurve` $\rightarrow$ `CurveGroup: Group`.

Hopefully the code of the `verify` function should now make completely sense.
Note in particular how elliptic curve points are converted from affine coordinates to projective coordinates using method `into` before being passed to `product_of_pairings`.

It's now time to see what the `hash_to_curve` function does exactly.
