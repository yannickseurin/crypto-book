## Code Analysis

The package directory is organized as follows:

```text
puzzle-gamma-ray
├── Cargo.toml
├── leaked_secret.bin
├── leaves.bin
├── proof_keys.bin
└── src
    ├── main.rs
    └── poseidon_parameters.rs
```

Files *leaked_secret.bin*, *leaves.bin*, and *proof_keys.bin* contain raw data that will be used to initialize variables, as we will see.

The *main.rs* file brings a lot of items from various arkworks crates into scope, notably for MNT4-753 and MNT6-753 curves, Groth16 proofs, R1CS arithmetization, etc.
We will come back to this shortly.

The first thing the `main` function does is to define a number of variables for the puzzle, in particular:

- a proving key and a verification key for the Groth16 [[Gro16](../../references.md#Gro16)] proof system over the MNT4-753 curve:

```rust
    let (pk, vk): (
        <Groth16<MNT4_753> as SNARK<MNT4BigFr>>::ProvingKey,
        <Groth16<MNT4_753> as SNARK<MNT4BigFr>>::VerifyingKey,
    ) = from_file("./proof_keys.bin");

```
- a "leaked secret" of type `MNT4BigFr` (the scalar field of the MNT4-753 curve) used by Alice to spend one of her coins:

```rust
    let leaked_secret: MNT4BigFr = from_file("./leaked_secret.bin");
```
- a Merkle tree, with leaf `leaf` at index `i = 2` playing a special role:

```rust
    let leaves: Vec<Vec<MNT4BigFr>> = from_file("./leaves.bin");
    // ...
    let leaf_crh_params = poseidon_parameters::poseidon_parameters();
    let i = 2;
    let two_to_one_crh_params = leaf_crh_params.clone();
    // ...
    let tree = MntMerkleTree::new(
        &leaf_crh_params,
        &two_to_one_crh_params,
        leaves.iter().map(|x| x.as_slice()),
    )
    .unwrap();
    let root = tree.root();
    let leaf = &leaves[i];

```

The hash function used to build the Merkle tree is the SNARK-friendly Poseidon hash function [[GKR+21](../../references.md#GKR+21)] with parameters specified in the *poseidon_parameters.rs* file.
In particular, the underlying field is also the scalar field `MNT4BigFr` of the MNT4-753 curve.
One can also print the leaves of the Merkle tree:

```rust
{{#rustdoc_include ../../../puzzles/puzzle-gamma-ray/src/main.rs:print}}
```

There are four leaves, each consisting of a single `MNT4BigFr` element.
At this point it's not clear what these leaves represent but we will clarify this in a moment.

Then, a Merkle proof (a proof that a specific leaf contains a specific element) is computed for the leaf at index `i = 2`:

```rust
    let tree_proof = tree.generate_proof(i).unwrap();
```

If you're unfamiliar with how ZCash works, the state of the chain is encoded in a Merkle tree where each leaf represents a coin. Attached to this leaf is a public key and a *nullifier* (originally called *coin serial number* in the ZeroCash paper [[BCG+14](../../references.md#BCG+14)]) whose role is to prevent double spends: when a coin is spent, the corresponding nullifier is revealed and recorded and the protocol later ensures that any transaction using the same nullifier (and hence trying to spend the same coin) is invalid.
Note in particular that leaves of the Merkle tree do not represent UTXOs but rather all coins that ever existed, spent or unspent.
For more details about how nullifiers work, [this blog post](https://electriccoin.co/blog/zcash-private-transactions/) by Ariel Gabizon explains it very well.

Here, we can see that the nullifier is computed as the hash of the secret allowing to spend a coin:

```rust
    let nullifier = <LeafH as CRHScheme>::evaluate(&leaf_crh_params, vec![leaked_secret]).unwrap();
```

In order to spend the coin represented by leaf at index `i = 2`, Alice needs to provide a Groth16 proof that her transaction is valid:

```rust
    let c = SpendCircuit {
        leaf_params: leaf_crh_params.clone(),
        two_to_one_params: two_to_one_crh_params.clone(),
        root: root.clone(),
        proof: tree_proof.clone(),
        nullifier: nullifier.clone(),
        secret: leaked_secret.clone(),
    };

    let proof = Groth16::<MNT4_753>::prove(&pk, c.clone(), rng).unwrap();

    assert!(Groth16::<MNT4_753>::verify(&vk, &vec![root, nullifier], &proof).unwrap());
```

We will get into what `SpendCircuit` is shortly, but before that, let's take a look at the part where we need to work to solve the puzzle:

```rust
    /* Enter your solution here */

    let nullifier_hack = MNT4BigFr::from(0);
    let secret_hack = MNT4BigFr::from(0);

    /* End of solution */

    assert_ne!(nullifier, nullifier_hack);

    let c2 = SpendCircuit {
        leaf_params: leaf_crh_params.clone(),
        two_to_one_params: two_to_one_crh_params.clone(),
        root: root.clone(),
        proof: tree_proof.clone(),
        nullifier: nullifier_hack.clone(),
        secret: secret_hack.clone(),
    };

    let proof = Groth16::<MNT4_753>::prove(&pk, c2.clone(), rng).unwrap();

    assert!(Groth16::<MNT4_753>::verify(&vk, &vec![root, nullifier_hack], &proof).unwrap());
```

As we can see, we must find another nullifier `nullifier_hack` (different from `nullifier`) and another secret `secret_hack` allowing to spend the same coin again (this is the same coin because the second Groth16 proof uses the same Merkle root `root` and the same Merkle proof `tree_proof` as the first Groth16 proof).

Next, let us unravel what the spending circuit does.