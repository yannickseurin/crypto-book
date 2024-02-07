# ZK Hack Puzzle 13: Supervillain

- [puzzle page](https://zkhack.dev/zkhackIV/puzzleF2.html)
- [GitHub repository](https://github.com/ZK-Hack/puzzle-supervillain)
- puzzle description:

```text
Bob has been designing a new optimized signature scheme for his L1 based on BLS
signatures. Specifically, he wanted to be able to use the most efficient form
of BLS signature aggregation, where you just add the signatures together rather
than having to delinearize them. In order to do that, he designed a
proof-of-possession scheme based on the B-KEA assumption he found in the the
Sapling security analysis paper by Mary Maller [1]. Based the reasoning in the
Power of Proofs-of-Possession paper [2], he concluded that his scheme would be
secure. After he deployed the protocol, he found it was attacked and there was
a malicious block entered the system, fooling all the light nodes...
```

BLS aggregate signatures, proofs of possession, ...
This should be interesting and quite relevant to Ethereum since the beacon chain uses BLS signature aggregation since the Merge.
Let's take a look at the code.

## Code Analysis

The package directory is organized as follows:

```text
puzzle-supervillain
├── Cargo.toml
├── public_keys.bin
└── src
    └── main.rs
```

The code is pretty simpler than in most other puzzles.
Let's take a look at *main.rs*.
It first brings a number of items from arkworks crates into scope, in particular related to the [BLS12-381](https://hackmd.io/@benjaminion/bls12-381) pairing-friendly curve.
Let us introduce straightaway some (standard) notation that will help us explain what's going on in this puzzle.
In all the following, we will let $\GG_1$ and $\GG_2$ denote the two groups related to this curve, $r$ denote the order of these groups, and $\FF_r$ denote the corresponding scalar field.
Types `G1Affine` and `G2Affine` respectively correspond to points in $\GG_1$ and $\GG_2$ represented in short Weierstrass form.
We will also let $G_1$ denote the generator of $\GG_1$ returned by `G1Affine::generator()` and $e$ the pairing map from $\GG_1 \times \GG_2$ to $\GG_t$ which for any $P \in \GG_1$, $Q \in \GG_2$, and $a,b \in \FF_r$ satisfies
\[
 e(aP,bQ) = e(P,Q)^{ab}.
\]

Function `main` is also quite simple.
First, it creates a vector `public_keys` of public key/proof pairs $(P_i,Q_i) \in \GG_1 \times \GG_2$ by deserializing the data in *public_keys.bin* and checks the proofs (we will come back to what these proofs are and what function `pok_verify` does in a moment, but the idea is that $Q_i$ should prove possession of the secret key corresponding to public key $P_i$):

```rust
    let public_keys: Vec<(G1Affine, G2Affine)> = from_file("public_keys.bin");

    public_keys
        .iter()
        .enumerate()
        .for_each(|(i, (pk, proof))| pok_verify(*pk, i, *proof));
```

There are 9 public key/proofs pair in total.
We can print these public keys and proofs if we want, although there's not much remarkable about them:

```rust
    for (i, (pk, proof)) in public_keys.iter().enumerate() {
        println!("public_keys[{}].pk: {}", i, pk);
        println!("public_keys[{}].proof: {}\n", i, proof);
    }
```

We get:

```text
public_keys[0].pk: (3951285727116295734026345521365512737910419062953537242549018568832618561552329351430853683858605302756892560527243, 2015562491477402081445210194864883205939261701444702459066048593747231321865210770475706036490256666079149530034340)
public_keys[0].proof: (QuadExtField(3882041700531663080715209917545481876729765846180025125888908765171220948117125212571143276457991056473137284787111 + 1050510852775817852847416507597900558865419625189113347525854846152071282929138131519646186856851998395894795581147 * u), QuadExtField(2276155031300751614807654043081790005359418219874201281987179783127300973516686184393036674096389076445085471809656 + 1499576108176939010561117214629143885375859964478472578277936997691719552590218342980721074321136489528861749450818 * u))

[...]

public_keys[8].pk: (1590421703439460875501217084904151928024777767932960691388269493213756601481659194276214126863101251608754666663069, 2514873486426372291261215275870411521130979618244175339961890502447807774325646533262394833397969654866179194151855)
public_keys[8].proof: (QuadExtField(3425299122867009301502774777484371853886695233020764827572267590585668332652640989134711487565285919169053024365378 + 3944021846570525607818281571743626433255634014300232163668270031644045523012218166565184866119660182557753392994734 * u), QuadExtField(109895792935386285998226095339950304065932040948382827892877878577064124319340998704951294008715504779991886183613 + 2408179566338427416508441175406184228438312159836037637845488388439414219265617277506646523139939207977761770383651 * u))
```

Then, function `main` defines the index of an extra key and a message for which we will have to forge a signature and expects us to define three things: a new public key, a new proof and an aggregate signature:

```rust
    let new_key_index = public_keys.len();
    let message = b"YOUR GITHUB USERNAME";

    /* Enter solution here */

    let new_key = G1Affine::zero();
    let new_proof = G2Affine::zero();
    let aggregate_signature = G2Affine::zero();

    /* End of solution */
```

The solution should satisfy two conditions.
First, the new proof should be valid for the new public key, and second, the aggregate signature should be a valid BLS signature with respect to some *aggregate key*:

```rust
    pok_verify(new_key, new_key_index, new_proof);
    let aggregate_key = public_keys
        .iter()
        .fold(G1Projective::from(new_key), |acc, (pk, _)| acc + pk)
        .into_affine();
    bls_verify(aggregate_key, aggregate_signature, message)
```

This aggregate key is defined by adding all public keys $P_0, \dots, P_8$ from the puzzle data to the new key we must specify in our solution.
In other words, letting $P_9$ denote the new public key, the aggregate key (let us denote it $P^*$) is simply the sum of all public keys:
\[
 P^* = \sum_{i=0}^9 P_i.
\]

That's a good start.
In order to progress, let us recall how BLS signatures work.

## BLS Signatures

There are many great online resources about BLS signatures such as [here](https://eth2book.info/capella/part2/building_blocks/signatures/) or the [IETF draft](https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-bls-signature-05).
See also [the corresponding chapter](../../cryptographic-notions/bls-signatures.md) in this book.
In order to make this write-up more self-contained, let us quickly recall how they work.

The signature and verification functions are defined by these few lines of code:

```rust
fn bls_sign(sk: Fr, msg: &[u8]) -> G2Affine {
    hasher().hash(msg).unwrap().mul(sk).into_affine()
}

fn bls_verify(pk: G1Affine, sig: G2Affine, msg: &[u8]) {
    assert!(Bls12_381::multi_pairing(
        &[pk, G1Affine::generator()],
        &[hasher().hash(msg).unwrap().neg(), sig]
    )
    .is_zero());
}
```

Here, public keys are elements from group $\GG_1$ and signatures are elements from $\GG_2$ (this is the choice made in Ethereum but in general, this can be swapped depending on which one should be shorter for a specific use case; elements from $\GG_2$ are roughly twice longer than elements from $\GG_1$ and arithmetic is slower in $\GG_2$ than in $\GG_1$).

Given a secret key $x$, the corresponding public key is $P= x G_1$ (recall that $G_1$ is a commonly agreed generator of $\GG_1$).

In order to sign a message $m$, one first hashes it "into" $\GG_2$ with some hash function $H : \{0,1\}^* \to \GG_2$ and multiply the resulting point by $x$, meaning the signature is
\[
 S = x H(m).
\]

The verification function, given a public key $P \in \GG_1$, a message $m$, and a signature $S \in \GG_2$, asserts whether
\[
 e(P,-H(m))+e(G_1,S) = 0_{\GG_t}
\]
which is equivalent to (but more efficient to compute)
\[
 e(P,H(m)) = e(G_1,S).
\]
This works since for a signature computed correctly one has $S = xH(m)$ and hence
\[
 e(P,H(m)) = e(xG_1,H(m)) = e(G_1,xH(m)) = e(G_1,S).
\]
(Note that $\GG_t$ is denoted additively here, which would make [Vitalik happy](https://twitter.com/VitalikButerin/status/1571522859410808833); multiplicative notation is more common since $\GG_t$ is a multiplicative subgroup of $\FF_{q^{12}}$ where $q$ is the size of the base field of BLS12-381).

Hashing into elliptic curves is delicate: this should be done in a way that does not leak any algebraic relation (such as relative discrete logarithms) between resulting points (more formally, $H$ should behave as a random oracle returning a random point for each input).
[RFC 9380](https://datatracker.ietf.org/doc/html/rfc9380) describes various methods for doing this.

Here, the hash function used for hashing into $\GG_2$ is the so-called Wahby-Boneh map [[WB19](../../references.md#WB19)]:

```rust
fn hasher() -> MapToCurveBasedHasher<G2Projective, DefaultFieldHasher<Sha256, 128>, WBMap<Config>> {
    let wb_to_curve_hasher =
        MapToCurveBasedHasher::<G2Projective, DefaultFieldHasher<Sha256, 128>, WBMap<Config>>::new(
            &[1, 3, 3, 7],
        )
        .unwrap();
    wb_to_curve_hasher
}
```

It's pretty complicated and fortunately there's no need to understand how it works exactly to solve the puzzle.

## Aggregating BLS Signatures

BLS signatures have the nice property that they can be aggregated by simply adding them.
Namely, if we have $n$ signatures $S_0,\dots,S_{n-1}$ corresponding to public key/message pairs $(P_i,m_i)$, we can simply take the sum of all signatures
\[
 S = \sum_{i=0}^{n-1} S_i.
\]
Then, to check that the $n$ messages have been signed, one verifies that
\[
 e(G_1,S) = \sum_{i=0}^{n-1} e(P_i,H(m_i)).
\]
This cuts the cost of verification roughly by half (one only has to compute $n+1$ pairings instead of $2n$ when checking signatures one by one; the cost of point additions to compute the aggregate signature $S$ is negligible compared to the cost of a pairing).

This can be shown to be secure *assuming all messages are distinct* as otherwise a so-called rogue-key attack is possible.
To see why, let us take the simple case of two signers with respective public keys $P_0 = x_0 G_1$ and $P_1= x_1 G_1$ who want to sign the same message $m$.
(A note about the wording: when all signers want to sign a common message, this is more often called a *multi-signature scheme* rather than an aggregate signature scheme).
To be valid, the aggregate signature must satisfy
\[
 e(G_1,S) = e(P_0,H(m)) + e(P_1,H(m)).
\]
Because messages are the same, this can be written
\[
 e(G_1,S) = e(P_0+P_1,H(m)).
\]
Then, assuming signer 0 announced its public key first, signer 1 could just choose its public key as
\[
 P_1 = xG_1 - P_0
\]
for some known secret $x$.
Then, signer 1 can compute an aggregate signature *on its own* for any message $m$ (even messages signer 0 would refuse to sign) simply by computing $S = xH(m)$: this is a valid signature for $m$ under the "aggregate" key $P_0+P_1 = xG_1$.

There are several solutions to thwart this attack:

- one can use "augmented messages", meaning signer $i$ signs $(P_i,m)$ instead of just $m$; this was suggested in the original paper where aggregate BLS signatures were proposed [[BGLS03](../../references.md#BGLS03)] and further formalized in [[BNN07](../../references.md#BNN07)];
- one can use "delinearization", meaning each public key $P_i$ is multiplied by a some random-looking scalar $H'(i,(P_0,\dots,P_{n-1}))$, for some hash function $H'$ with values in $\FF_r$; this was first suggested to solve the corresponding problem for Schnorr multisignatures and later studied for BLS in [[BDN18](../../references.md#BDN18)];
- finally (and this is the solution the puzzle is about) one can use "proofs of possession", as suggested in [[RY07](../../references.md#RY07)] (reference [2] in the puzzle description); this means that each signer must prove that it has access to the secret key corresponding to its public key; this thwarts rogue-key attacks since signer 1 does not know the secret key corresponding to $P_1 = xG_1 - P_0$ and hence cannot provide a proof of possession.

## Proofs of Possession

What is a proof of possession (PoP) exactly? There is actually no clear security definition. [[RY07](../../references.md#RY07)] defines it as follows:

> A POP attests that a party has access to the secret key associated with his/her public key, which is typically accomplished using the functionality of the key pair’s intended scheme. For signature schemes, the simplest POP has a party sign its certificate request message and send both the message and signature to the CA.

Hence, this is somewhat reminiscent of a proof of knowledge, except there is no formal guarantee that there exists an extractor which is capable of extracting the secret key when granted arbitrary access to the party implementing the PoP.
In particular, this makes PoPs more cumbersome to use in security proofs.
In a protocol based on a proof of knowledge, the security proof is typically modular, meaning it only relies on the assumption that the PoK satisfies extractability.
The protocol is then guaranteed to be secure when used with any PoK meeting the definition (which can be proved independently).
On the contrary, since PoPs have no formal security definition, one must provide a new security proof for each PoP scheme one may want to use the protocol with.

It has been proved in [[RY07](../../references.md#RY07)] that BLS multi-signatures are secure when used with the PoP which consists in signing its own public key with the corresponding secret key.
Namely, if my key pair is $(x, P = xG_1)$, then the proof of possession is the point $xH(X)$ on $\GG_2$.
(There's a slight subtlety here: the hash function used to compute PoPs should be different from the one used to actually sign messages; prepending two different constants to the argument of the hash function does the trick.)
In fact, this PoP can even be proved to be a proof of *knowledge* under a very strong assumption called B-KEA: this is shown in the [paper by Maller](https://github.com/zcash/sapling-security-analysis/blob/master/MaryMallerUpdated.pdf) mentioned in the puzzle description (see Lemma 1).

How does the proof used in the puzzle work exactly?
It is defined as follows:

```rust
fn derive_point_for_pok(i: usize) -> G2Affine {
    let rng = &mut ark_std::rand::rngs::StdRng::seed_from_u64(20399u64);
    G2Affine::rand(rng).mul(Fr::from(i as u64 + 1)).into()
}

#[allow(dead_code)]
fn pok_prove(sk: Fr, i: usize) -> G2Affine {
    derive_point_for_pok(i).mul(sk).into()
}

fn pok_verify(pk: G1Affine, i: usize, proof: G2Affine) {
    assert!(Bls12_381::multi_pairing(
        &[pk, G1Affine::generator()],
        &[derive_point_for_pok(i).neg(), proof]
    )
    .is_zero());
}
```

First, a point in $\GG_2$ is computed as a function of the index $i$ of the public key in the vector of public keys.
This point is equal to $(i+1)Q$, where $Q$ is a random point in $\GG_2$ returned by the `rand` function seeded with some fixed string `20399`.
Importantly, the same point $Q$ is used in every proofs.
The proof $Q_i$ of possession of the secret key $x_i$ for public key $P_i = x_i G_1$ is then the point $Q_i$ defined as
\[
 Q_i = x_i(i+1)Q.
\]
Hence, this kind of looks like a BLS signature, except that the point which is multiplied by the secret key $x_i$ is $(i+1)Q$ rather than $H(P_i)$.
Can we exploit this?


## Solving the Puzzle

We are now ready to gather all the pieces and solve the puzzle.
The straightforward idea is to mount a rogue key attack by choosing some "rogue" secret key $x$ and define our new public key $P_9$ as $xG_1$ minus the sum of all other public keys:
\[
 P_9 = xG_1 - \sum_{i=0}^8 P_i.
\]
This way, the aggregate key $P^*$ is simply
\[\begin{aligned}
 P^* & = \sum_{i=0}^9 P_i \\
 & = \sum_{i=0}^8 P_i + P_9 \\
 & = xG_1.
\end{aligned}\]
This means that we know the secret key corresponding to $P^*$ and hence we can forge a valid signature (with respect to $P^\ast$) for any message we want by just computing $xH(m)$.

Are we done, then?
Well, not exactly, as we now have to come with a valid proof of possession of the secret key for $P_9$.
But, we don't know this secret key!
It seems like we have just moved the problem elsewhere.

Let us write formally what the proof for $P_9$ should be.
For $i = 0, \dots, 9$, let $x_i$ be the secret key corresponding to public key $P_i$.
Since $P_9$ can be written
\[
 P_9 = \left( x - \sum_{i=0}^8 x_i \right) G_1,
\]
the secret key corresponding to $P_9$ is
\[
 x_9 = x - \sum_{i=0}^8 x_i.
\]
Recall that the proof for the $i$-th public key is $Q_i = x_i (i+1) Q$.
Hence, the valid proof that we must compute is
\[\begin{aligned}
 Q_9 & = x_9 (10 \cdot Q) \\
 & = 10 \left( x - \sum_{i=0}^8 x_i \right) Q \\
 & = (10x)Q - \sum_{i=0}^8 10 x_i Q.
\end{aligned}\]

We can compute the first term since we know $x$.
On the other hand, we don't know secret keys $x_i$.
But we have access to proofs $Q_i = x_i (i+1) Q$, and this allows us to compute
\[
 x_i Q = ((i+1)^{-1} \bmod r)Q_i.
\]
Hence, we obtain that the proof we're looking for can be computed from the puzzle data as
\[
 Q_9 = (10x)Q - \sum_{i=0}^8 10 ((i+1)^{-1} \bmod r)Q_i.
\]

Here is the code computing the new key, the new proof, and the aggregate signature (the rogue secret is generated pseudorandomly):

```rust
{{#rustdoc_include ../../../puzzles/puzzle-supervillain/src/main.rs:solve}}
```

## Conclusion

The proof of possession used in the puzzle departed from what has been proven secure in the literature: instead of hashing the public key into the group, it used points of the form $(i+1)Q$ for a common random point $Q$.
While this is secure for a single key in isolation, this breaks when multiple public keys are involved as an attacker can use PoPs of other cosigners to maul a PoP for a public key for which it does not know the corresponding secret key, ultimately enabling a rogue key attack.