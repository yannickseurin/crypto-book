## Understanding the Spending Circuit

Let us try to understand what the circuit for which Groth16 proofs are generated does.
It is specified by the `generate_constraints` method from the `ConstraintSynthesizer` trait implemented on the `SpendCircuit` struct:

```rust
impl ConstraintSynthesizer<ConstraintF> for SpendCircuit {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {

    // ...

    }
}
```

This method generates R1CS constraints (over the field `ConstraintF = MNT4BigFr`, the scalar field of the MNT4-753 curve) to which the Groth16 proof system is then applied.
There is no need to understand precisely how R1CS arithmetization works exactly, just getting what the circuit does will be enough to solve the puzzle.
For this, having a quick look at the [arkworks R1CS tutorial](https://github.com/arkworks-rs/r1cs-tutorial) can help.
Let's go step by step into the definition of the circuit.
A circuit can have public inputs (the "instance", declared with the `new_input` method of the `AllocVar` trait) and private inputs (the "witness", declared with the `new_witness` method of the `AllocVar` trait): the proof generation function takes public and private inputs and generates a proof that together they "satisfy" the circuit; the verification function takes only the public inputs and the proof and returns 0 or 1 (valid/invalid).
It can also have constants declared with the `new_constant` of the same `AllocVar` trait.

Here, the public inputs consist of the Merkle root and the nullifier:

```rust
        // Allocate Merkle Tree Root
        let root = <LeafHG as CRHSchemeGadget<LeafH, _>>::OutputVar::new_input(
            ark_relations::ns!(cs, "new_digest"),
            || Ok(self.root),
        )?;

        // ...

        let nullifier = <LeafHG as CRHSchemeGadget<LeafH, _>>::OutputVar::new_input(
            ark_relations::ns!(cs, "nullifier"),
            || Ok(self.nullifier),
        )?;
```

The private inputs consist of the secret `secret` and the Merkle proof (the field `proof` of the `SpendCircuit` struct):

```rust
        let secret = FpVar::new_witness(ark_relations::ns!(cs, "secret"), || Ok(self.secret))?;

        // ...

        // Allocate Merkle Tree Path
        let cw: PathVar<MntMerkleTreeParams, ConstraintF, MntMerkleTreeParamsVar> =
            PathVar::new_witness(ark_relations::ns!(cs, "new_witness"), || Ok(&self.proof))?;
```

Then, the `generate_constraints` method calls a number of "gadgets" to implement the logic of the circuit.
First, it checks that the secret is less than the size of the scalar field of the MNT6-763 curve:

```rust
        let secret_bits = secret.to_bits_le()?;
        Boolean::enforce_smaller_or_equal_than_le(&secret_bits, MNT6BigFr::MODULUS)?;
```

It also checks that the hash of the secret is equal to the nullifier passed as input:

```rust
        let nullifier_in_circuit =
            <LeafHG as CRHSchemeGadget<LeafH, _>>::evaluate(&leaf_crh_params_var, &[secret])?;
        nullifier_in_circuit.enforce_equal(&nullifier)?;
```

Then, it computes the public key associated with `secret`:

```rust
        let base = G1Var::new_constant(ark_relations::ns!(cs, "base"), G1Affine::generator())?;
        let pk = base.scalar_mul_le(secret_bits.iter())?.to_affine()?;
```

Note that the `G1Affine` type here represents the group of points of the MNT6-753 curve in short Weierstrass affine representation, meaning `pk` here is a point on this curve, encoded as a pair $(x,y)$ of elements of the base field $\mathbb{F}_{q_6}$ of the MNT6-753 curve, and computed as $P = s G$ where $G$ is the generator of this group corresponding to variable `base` and $s$ corresponds to `secret`.

Finally, the circuit verifies that the Merkle proof passed as private input to the circuit is valid for the root passed as public input and the leaf defined as `pk.x`, the $x$-coordinate of `pk`.

```rust
        // Allocate Leaf
        let leaf_g: Vec<_> = vec![pk.x];

        // ...

        cw.verify_membership(
            &leaf_crh_params_var,
            &two_to_one_crh_params_var,
            &root,
            &leaf_g,
        )?
        .enforce_equal(&Boolean::constant(true))?;
```

Something might seem strange here at first.
Point `pk` lies on the MNT6-753 curve, hence `pk.x` is an element of its base field $\FF_{q_6}$.
Yet we saw previously that leaves of the Merkle tree were defined as elements of the scalar field of the MNT4-753 curve.
In fact, this is fine because MNT4-753 and MNT6-753 form a "cycle of curves", meaning the scalar field of one is the base field of the other.
If $\FF_{q_4}$ and $\FF_{r_4}$ denote respectively the base field and the scalar field of MNT4-753 and $\FF_{q_6}$ and $\FF_{r_6}$ denote the base field and the scalar field of MNT6-753, then forming a cycle means that $q_6 = r_4$ and $q_4 = r_6$.

Cycles of curves were proposed in [[BCTV14](../../references.md#BCTV14)] to solve the "field mismatch" problem when composing SNARKs recursively [[BCCT13](../../references.md#BCCT13)].
For more background, see for example [[AHG23](../../references.md#AHG23)].

This concludes our inspection of the spending circuit.
In short, to spend a coin, Alice must compute a Groth16 proof of satisfiability of the spending circuit using the secret key corresponding to the ($x$-coordinate of the) public key of the leaf representing the coin, a valid Merkle proof for this public key, and the corresponding nullifier, i.e., the hash of the secret.

We are now ready to solve the puzzle.