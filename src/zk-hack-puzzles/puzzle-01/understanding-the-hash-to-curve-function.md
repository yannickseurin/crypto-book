## Understanding the hash-to-curve Function

It remains to take a look at what the `hash_to_curve` function defined in the *src/hash* module is doing exactly:

```rust
{{#rustdoc_include ../../../puzzles/zkhack-bls-pedersen/src/hash.rs:hash}}
```

This function first initializes the pseudorandom number generator ChaCha20 with a 32-byte seed and feeds this RNG to the `setup` function.
We look up the `setup` function in the `crypto_primitives::crh::pedersen` submodule (how do we know where to look? we check the `use` statement which brings `CRH` into scope at the beginning of the *src/hash.rs* file) and arrive [here](https://docs.rs/ark-crypto-primitives/0.3.0/ark_crypto_primitives/crh/pedersen/struct.CRH.html#method.setup).
Documentation is nonexistent so we jump [to the code](https://docs.rs/ark-crypto-primitives/0.3.0/src/ark_crypto_primitives/crh/pedersen/mod.rs.html#58-68).
Here are the relevant lines:

```rust
pub struct Parameters<C: ProjectiveCurve> {
    pub generators: Vec<Vec<C>>,
}

pub struct CRH<C: ProjectiveCurve, W: Window> {
    group: PhantomData<C>,
    window: PhantomData<W>,
}

impl<C: ProjectiveCurve, W: Window> CRHTrait for CRH<C, W> {
    const INPUT_SIZE_BITS: usize = W::WINDOW_SIZE * W::NUM_WINDOWS;
    type Output = C::Affine;
    type Parameters = Parameters<C>;

    fn setup<R: Rng>(rng: &mut R) -> Result<Self::Parameters, Error> {
        // ...
        let generators = Self::create_generators(rng);
        // ...
        Ok(Self::Parameters { generators })
    }

    // ...
}

impl<C: ProjectiveCurve, W: Window> CRH<C, W> {
    pub fn create_generators<R: Rng>(rng: &mut R) -> Vec<Vec<C>> {
        let mut generators_powers = Vec::new();
        for _ in 0..W::NUM_WINDOWS {
            generators_powers.push(Self::generator_powers(W::WINDOW_SIZE, rng));
        }
        generators_powers
    }

    pub fn generator_powers<R: Rng>(num_powers: usize, rng: &mut R) -> Vec<C> {
        let mut cur_gen_powers = Vec::with_capacity(num_powers);
        let mut base = C::rand(rng);
        for _ in 0..num_powers {
            cur_gen_powers.push(base);
            base.double_in_place();
        }
        cur_gen_powers
    }
}
```

Each invocation of `generator_powers` draws a random group element $B \sample \GG_1$ and returns the vector $(B, 2B, \dots, 2^{w-1} B)$ where $w \cong$ `W::WINDOW_SIZE`.
This function is called $n \cong$ `NUM_WINDOWS` times by `create_generators` which then returns a vector
\[
 \big((B_0,\dots,2^{w-1} B_0), \dots, (B_{n-1},\dots,2^{w-1} B_{n-1})\big)
\]
where $B_0, \dots, B_{n-1}$ are random group elements.
In `hash_to_curve`, this function is called with constants `WINDOW_SIZE = 1` and `NUM_WINDOWS = 256` as defined in the implementation of trait `Window` for struct `ZkHackPedersenWindow`.
Hence, the line

```rust
let parameters = CRH::<G1Projective, ZkHackPedersenWindow>::setup(rng_pedersen).unwrap();
```

defines a `Parameters<G1Projective>` struct whose field `generators` holds a tuple of 256 random group elements $(B_0, \dots, B_{255})$ of type `G1Projective`.

Then, the message is hashed with hash function BLAKE2s and the result is passed to the [`evaluate`](https://docs.rs/ark-crypto-primitives/0.3.0/src/ark_crypto_primitives/crh/pedersen/mod.rs.html#70-120) function, whose core is as follows:

```rust
    fn evaluate(parameters: &Self::Parameters, input: &[u8]) -> Result<Self::Output, Error> {
        // ...

        // Compute sum of h_i^{m_i} for all i.
        let bits = bytes_to_bits(input);
        let result = cfg_chunks!(bits, W::WINDOW_SIZE)
            .zip(&parameters.generators)
            .map(|(bits, generator_powers)| {
                let mut encoded = C::zero();
                for (bit, base) in bits.iter().zip(generator_powers.iter()) {
                    if *bit {
                        encoded += base;
                    }
                }
                encoded
            })
            .sum::<C>();

        // ...

        Ok(result.into())
    }
```

First, the input is converted into a vector of booleans $(b_0, \dots, b_{\ell-1})$ using the [`bytes_to_bits`](https://docs.rs/ark-crypto-primitives/0.3.0/src/ark_crypto_primitives/crh/pedersen/mod.rs.html#161-170) function from the `pedersen` module.
Then, it is split into $n \cong$ `NUM_WINDOWS` chunks of size $w \cong$ `WINDOW_SIZE` and zipped with `parameters.generators` which contains the points
\[
 \big((B_0,\dots,2^{w-1} B_0), \dots, (B_{n-1},\dots,2^{w-1} B_{n-1})\big)
\]
returned by `setup`.
The closure inside `map` takes a chunk of bits $(b_0,\dots,b_{w-1})$ and a vector of points $(B,\dots,2^{w-1} B)$ and returns
\[
 \sum_{i=0}^{w-1} b_i 2^i B = \beta B
\]
where $\beta \defeq \sum_{i=0}^{w-1} b_i 2^i$ is the integer whose bit representation is $(b_0, \dots, b_{w-1})$.
The final value of `result` is the sum over the $n$ windows of the output of this closure, i.e.,
\[
 \sum_{j=0}^{n-1} \sum_{i=0}^{w-1} b_{wj+i} 2^i B_j = \sum_{j=0}^{n-1} \beta_j B_j \label{1} \tag{1}
\]
where $\beta_j = \sum_{i=0}^{w-1} b_{wj+i} 2^i$ is the integer corresponding to the $j$-th chunk of bits of the input.

In the specific case of `hash_to_curve`, we have $w=1$ and $n=256$.
Hence, if we let $(B_0, \dots, B_{255})$ denote the 256 group elements returned by `setup` and $\bfh = (h_0, \dots, h_{255}) \defeq \mathsf{blakes2s}(m)$ denote the output of the BLAKE2s hash function applied to message $m$, seen as a vector of bits, then the `hash_to_curve` function applied to $m$ returns the point on $\GG_1$ defined by
\[
 H(m) \defeq \sum_{j=0}^{255} h_j B_j.
\]

Hence, $H$ can be seen as the composition of BLAKE2s and an instance of [Pedersen hashing](/cryptographic-notions/commitment-schemes.md#commitments-and-hash-functions).
Since both BLAKE2s and Pedersen hashing are collision-resistant (assuming hardness of the discrete logarithm problem for Pedersen hashing), $H$ is collision-resistant as well.
Is it sufficient to make BLS signatures secure though?

Now that we understand all parts of the code, we can get down to solving the puzzle.
