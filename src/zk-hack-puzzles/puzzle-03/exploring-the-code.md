## Exploring the Code

Let us recall the proof system here before digging into the code:

\[
 \def\arraystretch{\myarraystretch}
 \boxed{
 \begin{array}{lcl}
  \text{Prover} & & \text{Verifier} \\
  \hline
  \text{parameters: } (\GG,q,\bfG,H) & & \text{parameters: } (\GG,q,\bfG,H) \\
  \text{instance: } (C_a, \bfb) \in \GG \times (\ZZ_q)^n & & \text{instance: } (C_a, \bfb) \in \GG \times (\ZZ_q)^n \\
  \text{witness: } (\bfa, \alpha) \in \GG^n \times \ZZ_q & & \\
  \text{relation: } C_a = \langle \bfa, \bfG \rangle + \alpha H & & \\
  \hline
  \mathbf{r} \sample (\ZZ_q)^n & & \\
  \rho, \tau, \nu \sample \ZZ_q & & \\
  C_r \defeq \langle \mathbf{r}, \bfG \rangle + \rho H & & \\
  C_1 \defeq \langle \bfa, \bfb \rangle G_1 + \tau H & & \\
  C_2 \defeq \langle \mathbf{r}, \bfb \rangle G_1 + \nu H & & \\
  & \xrightarrow{\displaystyle \ C_r, C_1, C_2 \ } \\
  & & \gamma \sample \ZZ_q \\
  & \xleftarrow{\displaystyle \qquad \gamma \qquad} & \\
  \bfs \defeq \bfa + \gamma \bfr \bmod q & & \\
  u \defeq \alpha + \gamma \rho \bmod q & & \\
  t \defeq \tau + \gamma \nu \bmod q & & \\
  & \xrightarrow{\displaystyle \quad \bfs, u, t \quad} \\
  & & \text{check that} \\
  & & \langle \bfs, \bfG \rangle + u H = C_a + \gamma C_r \\
  & & \langle \bfs, \bfb \rangle G_1 + t H = C_1 + \gamma C_2
 \end{array}
 }
\]

The package directory is organized as follows:

```text
zkhack-double-trouble
├── Cargo.toml
└── src
    ├── bin
    │   └── verify-double-trouble.rs
    ├── inner_product_argument
    │   ├── data_structures.rs
    │   └── utils.rs
    ├── data.rs
    ├── inner_product_argument.rs
    └── lib.rs
```

The proof system is implemented in the `inner_product_argument` module of the library crate and follows closely the specification from the puzzle's description.
The puzzle uses the [ark-ed-on-bls12-381](https://docs.rs/ark-ed-on-bls12-381/0.3.0/ark_ed_on_bls12_381/) library which implements the so-called [Jubjub curve](https://neuromancer.sk/std/other/JubJub/) developed by the Zcash team.
This curve was designed to have its base field equal to the scalar field of BLS12-381, allowing to efficiently prove statements about cryptographic schemes based on Jubjub (such a Pedersen commitments, Schnorr signatures, etc) using a proof system based on BLS12-381.
The affine group and the scalar field of this curve are brought into scope with

```rust
use ark_ed_on_bls12_381::{EdwardsAffine as GAffine, Fr};
```

Two structures `Instance` and `Witness` corresponding respectively to the instance $(C_a,\bfb)$ and the witness $(\bfa, \alpha)$ are defined directly in *src/inner_product_argument.rs*:

```rust
pub struct Instance {
    pub comm_a: GAffine,
    pub b: Vec<Fr>,
}

pub struct Witness {
    pub a: Vec<Fr>,
    pub comm_a_rand: Fr,
}
```

Four additional structures are defined in *src/inner_product_argument/data_structures.rs*:

- the commitment key `CommitKey` $\cong (\bfG, H)$,
- the proof commitment (first message sent by the prover in the interactive protocol) `ProofCommitment` $\cong (C_r, C_1, C_2)$,
- the proof response (second message sent by the prover in the interactive protocol) `ProofResponse` $\cong (\bfs, u ,t)$,
- and a fourth structure `Proof` which simply combines `ProofCommitment` and `ProofResponse`.

Here is the code defining these four structures:

```rust
pub struct CommitKey {
    pub generators: Vec<GAffine>,
    pub hiding_generator: GAffine,
}

pub struct ProofCommitment {
    pub comm_r: GAffine,
    pub comm_1: GAffine,
    pub comm_2: GAffine,
}

pub struct ProofResponse {
    pub s: Vec<Fr>,
    pub u: Fr,
    pub t: Fr,
}

pub struct Proof {
    pub commitment: ProofCommitment,
    pub response: ProofResponse,
}
```

The rest of the code in the `inner_product_argument` module and sub-modules does not show anything surprising.
As said in the puzzle description, the proof system is made non-interactive using the Fiat-Shamir transform.
Namely, the challenge $\gamma$ is computed by hashing the commitment key, the instance, and the commitment:

```rust
    let challenge = challenge(ck, instance, &commitment);
```

where function `challenge` is defined in *inner_product_argument/utils.rs*:

```rust
pub fn b2s_hash_to_field<C: CanonicalSerialize>(input: &C) -> Fr {
    let bytes = input.hash::<blake2::Blake2s>();
    Fr::from_le_bytes_mod_order(&bytes)
}

pub fn challenge(ck: &CommitKey, instance: &Instance, proof_comm: &ProofCommitment) -> Fr {
    b2s_hash_to_field(&(ck.clone(), instance.clone(), proof_comm.clone()))
}
```

Let's take a look at the binary crate and its `main` function:

```rust
fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let (ck, [instance_and_proof_1, instance_and_proof_2]) = puzzle_data();
    let (instance1, proof1) = instance_and_proof_1;
    let (instance2, proof2) = instance_and_proof_2;
    assert!(verify(&ck, &instance1, &proof1));
    assert!(verify(&ck, &instance2, &proof2));

    let (a, comm_a_rand): (Vec<Fr>, Fr) = {
        // Your solution here!
        todo!()
    };
    assert_eq!(
        ck.commit_with_explicit_randomness(&a, comm_a_rand),
        instance1.comm_a
    );
    assert_eq!(
        ck.commit_with_explicit_randomness(&a, comm_a_rand),
        instance2.comm_a
    );
}
```

The `puzzle_data` function simply deserializes some data and returns a commitment key `ck` and two instance/proof pairs.
We can print the structures we're interested in:

```rust
{{#rustdoc_include ../../../puzzles/zkhack-double-trouble/src/bin/verify-double-trouble.rs:print}}
```

We get:

```text
commitment key:
ck.generators[0] = GroupAffine(x=Fp256 "(198B8C3FC05A64DF64DEC6C0C9CF997E1AFA1DBB5C191ED0DFD5C771467F089D)", y=Fp256 "(5AE26A746DDBBCC5ECC3C970E715AEC48BB2551DD9DDE8AE7A7DA7032E161577)")
ck.generators[1] = GroupAffine(x=Fp256 "(4B92D491ACE817177026CD40C5020D04B30759F240CFDFF8DD795629E3307C5E)", y=Fp256 "(6604E7622969E8E968970E74DB648CF8226DCB0B67FF8E6313A3B9CAD7353701)")
ck.generators[2] = GroupAffine(x=Fp256 "(70F5E9698EF8F51B85D089DBBCC2F25190C905F6113976696F109307D347ACBA)", y=Fp256 "(3BD293E00769FE7963674BFA0745B4FE316AF189856418E679DBEF49B00A9085)")
ck.generators[3] = GroupAffine(x=Fp256 "(3FF005D6FE8FE85EA40051BF5051464AB69F0B587BE571B2800C08F4B93AD452)", y=Fp256 "(445D24D6D0EDEFC80B3785F27613FD072E4E69EBFC6A0B9716036F19E15C6ABF)")
ck.generators[4] = GroupAffine(x=Fp256 "(5174580F00FB73C60E3CA1D0A0EBF328FDAC3A018D15F6ED81B39D833927C079)", y=Fp256 "(26053B0E29A8E735673D8ACC0C0353EA6326DF6F81A2EE0AC65A8C1A853241F5)")
ck.generators[5] = GroupAffine(x=Fp256 "(65B0213A6DE2BC0DDAD9648179372A2B3939B06A1062CF0D58F731ABF7D6B742)", y=Fp256 "(11C6904D697A2638441B3125D0A3316507A03C16D79FCB3F5A19CF3ECFD649E6)")
ck.generators[6] = GroupAffine(x=Fp256 "(50644971731D58AFB54EB92D6F0C7700F06774B2AF10E72B646C0D5A93AA6347)", y=Fp256 "(1C282A3A58C4AD917587EBE68DC84A4CAC0E4A914FA83618439373528617AAAF)")
ck.generators[7] = GroupAffine(x=Fp256 "(3181568CB6D37D00412E3348F2C08C1579DB32492A768F41EAC49D8E7E6F9BBF)", y=Fp256 "(27EDAEA9068A61645B11E8D5C15E9569340E59E3963AE78CAADD2282451F10F4)")
ck.hiding_generator = GroupAffine(x=Fp256 "(3A2ED8E0E81BED90A83FA22E58FA8A0F08752AAB03CD4BA9BB558965B9A57B32)", y=Fp256 "(3C603EF0D0BB80987AD83208034C552F8919C5F8FEACC5404DEBCC16FE3B947F)")

instance 1, C_a:
 GroupAffine(x=Fp256 "(6AE271E04FBB0AE9FB89506FF7180F5C06A8D60F802D934987965F694228BF8A)", y=Fp256 "(2BFBFA9CCF2151F01E71A069366DAD9398960B64684888D1AABB50D4D57BDF32)")

instance 2, C_a:
 GroupAffine(x=Fp256 "(6AE271E04FBB0AE9FB89506FF7180F5C06A8D60F802D934987965F694228BF8A)", y=Fp256 "(2BFBFA9CCF2151F01E71A069366DAD9398960B64684888D1AABB50D4D57BDF32)")

instance 1, b:
instance1.b[0] = Fp256 "(08180E66A534AADEBC88D09E1397DC7C33E2014115EB973B489E7D5CDBF839CD)"
instance1.b[1] = Fp256 "(036AFB822FAC04AC9191CCEEF5BF4E27ADA6DC0440C88ECF3E06DC2FAFB162E6)"
instance1.b[2] = Fp256 "(0DE7FE23DCF79F2A041E2C21876F9B9AEB3F2BC628E07B87F52DF460408334F2)"
instance1.b[3] = Fp256 "(0891BBE1E3DA5717F7ED59288C9F51186E7BBAE018C9DA56F4BC8B4BBBD7457E)"
instance1.b[4] = Fp256 "(05D81F4C416350A3D02B1685176BFE5A98FA15D51C84DBD47680326F9F005E96)"
instance1.b[5] = Fp256 "(06D5E58667508A24F3A3FFBB244575DE29ECB3408D6EBC6D3DCDEFF02AA9453C)"
instance1.b[6] = Fp256 "(06BC47A67C6BD353EE624051B4C4A6A28E7F8CEDB6ED65A007D897AC071CBDCB)"
instance1.b[7] = Fp256 "(0CF6D9D35E0B6F2309568E5BB7C19448D993D2EFFEF7B3D77C137A26C524315A)"

instance 2, b:
instance2.b[0] = Fp256 "(08180E66A534AADEBC88D09E1397DC7C33E2014115EB973B489E7D5CDBF839CD)"
instance2.b[1] = Fp256 "(036AFB822FAC04AC9191CCEEF5BF4E27ADA6DC0440C88ECF3E06DC2FAFB162E6)"
instance2.b[2] = Fp256 "(0DE7FE23DCF79F2A041E2C21876F9B9AEB3F2BC628E07B87F52DF460408334F2)"
instance2.b[3] = Fp256 "(0891BBE1E3DA5717F7ED59288C9F51186E7BBAE018C9DA56F4BC8B4BBBD7457E)"
instance2.b[4] = Fp256 "(05D81F4C416350A3D02B1685176BFE5A98FA15D51C84DBD47680326F9F005E96)"
instance2.b[5] = Fp256 "(06D5E58667508A24F3A3FFBB244575DE29ECB3408D6EBC6D3DCDEFF02AA9453C)"
instance2.b[6] = Fp256 "(06BC47A67C6BD353EE624051B4C4A6A28E7F8CEDB6ED65A007D897AC071CBDCB)"
instance2.b[7] = Fp256 "(0CF6D9D35E0B6F2309568E5BB7C19448D993D2EFFEF7B3D77C137A26C524315A)"
```

We can note a couple of interesting things.
First, points `instance1.comm_a` and `instance2.comm_a` are equal and vectors `instance1.b` and `instance2.b` are equal, meaning the two instances are exactly the same (whereas the puzzle description said that the proofs published by Bob where for different different `b` vectors).
Second, `ck.generators` (vector $\bfG = (G_0, \dots, G_{n-1})$ in the description above) has length $n=8$.
Where does this commitment key come from?
The `CommitKey` structure has an associated function allowing to sample a commitment key:

```rust
impl CommitKey {
    pub fn sample(size: usize) -> Self {
        let mut rng = ChaChaRng::from_seed(*b"zkHack IPA puzzle for 2021-10-26");
        let generators = sample_vector::<GAffine, _>(size, &mut rng)
            .into_iter()
            .map(Into::into)
            .collect();
        let hiding_generator = GProjective::rand(&mut rng).into();
        Self {
            generators,
            hiding_generator,
        }
    }
    // ...
}
```

We can verify that `ck` provided in the puzzle data is indeed the commitment key returned by this function.
These two observations can be checked directly with

```rust
{{#rustdoc_include ../../../puzzles/zkhack-double-trouble/src/bin/verify-double-trouble.rs:check}}
```

(For this, one needs to derive the `PartialEq` trait for structures `CommitKey` and `Instance`).

As we explained in the section about [Pedersen commitments](../../cryptographic-notions/commitment-schemes.md#pedersen-commitments), the knowledge of discrete log relations between the group elements in the commitment key constitutes a trapdoor allowing to break the binding property of the commitment scheme.
However, this does not seem like a promising avenue to solve the puzzle.
On the one hand, this trapdoor does not allow to break the hiding property of the commitment scheme, which is what we would need to recover $\bfa$.
On the other hand, function `sample` does things correctly by sampling uniformly random and independent group elements using a pseudorandom number generator seeded with NUMS string "zkHack IPA puzzle for 2021-10-26".

A side note about the code: in function `sample`, `generators` could be defined more simply as

```rust
        let generators = sample_vector::<GAffine, _>(size, &mut rng);
```

Indeed, `sample_vector::<GAffine, _>(size, &mut rng)` returns an object of type `Vec<GAffine>` so there is no need to apply `into` to each element.
On the other hand, there does not seem to be any good reason for sampling `hiding_generator` as a `GProjective` and then cast it into a `GAffine` using `into`.
