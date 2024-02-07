# ZK Hack Puzzle 14: Chaos Theory

- [puzzle page](https://zkhack.dev/zkhackIV/puzzleF3.html)
- [GitHub repository](https://github.com/ZK-Hack/puzzle-chaos-theory)
- puzzle description:

```text
Bob designed a new one time scheme, that's based on the tried and true method
of encrypt + sign. He combined ElGamal encryption with BLS signatures in a
clever way, such that you use pairings to verify the encrypted message was
not tampered with. Alice, then, figured out a way to reveal the plaintexts...
```

The puzzle webpage recommends to read background material about [authenticated encryption](https://en.wikipedia.org/wiki/Authenticated_encryption), which usually refers to the combination of symmetric encryption and MACs, which are symmetric-key primitives.
Combining public-key encryption and signatures is more usually called [signcryption](https://en.wikipedia.org/wiki/Signcryption).

## Code Analysis

The package directory is organized as follows:

```text
puzzle-chaos-theory
├── Cargo.toml
├── blob.bin
└── src
    └── main.rs
```

Let us go through the *main.rs* file to understand how Bob designed his signcryption scheme.
It first brings a number of items from arkworks crates into scope, in particular related to the [BLS12-381](https://hackmd.io/@benjaminion/bls12-381) pairing-friendly curve.
Let us introduce straightaway some (standard) notation that will help us explain mathematically how the signcryption scheme works.
In all the following, we will let $\GG_1$ and $\GG_2$ denote the two groups related to the BLS12-381 curve, $r$ denote the order of these groups, and $\FF_r$ denote the corresponding scalar field.
Types `G1Affine` and `G2Affine` respectively correspond to points in $\GG_1$ and $\GG_2$ represented in short Weierstrass affine coordinates.
We will also let $G_1$ denote the generator of $\GG_1$ returned by `G1Affine::generator()` (or `G1Projective::generator()` in projective form) and $e$ the pairing map from $\GG_1 \times \GG_2$ to $\GG_t$ which for any $P \in \GG_1$, $Q \in \GG_2$, and $a,b \in \FF_r$ satisfies

\[
 e(aP,bQ) = e(P,Q)^{ab}.
\]

A `hasher` function is defined, returning an instance of the so-called Wahby-Boneh map [[WB19](../../references.md#WB19)] allowing to hash into $\GG_2$:

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

In all the following we will simply let $H$ denote this hash function.

Then, a tuple struct `ElGamal` holding two `G1Affine` points $(C_1,C_2)$ corresponding to ciphertexts is defined together with a method `hash_to_curve` returning $H(C_1,C_2)$:

```rust
pub struct ElGamal(G1Affine, G1Affine);

impl ElGamal {
    pub fn hash_to_curve(&self) -> G2Affine {
        let mut data = Vec::new();
        self.serialize_uncompressed(&mut data).unwrap();

        hasher().hash(&data).unwrap()
    }
}
```

Messages are simply `G1Affine` points wrapped in a `Message` struct using the [newtype pattern](https://doc.rust-lang.org/book/ch19-04-advanced-types.html#using-the-newtype-pattern-for-type-safety-and-abstraction):

```rust
pub struct Message(G1Affine);
```

Finally, two structs are defined for respectively the sender and receiver:

```rust
struct Sender {
    pub sk: Fr,
    pub pk: G1Affine,
}

pub struct Receiver {
    pk: G1Affine,
}
```

Public keys for both the sender and the receiver are `G1Affine` points.
Note that the receiver has a public key field, but no secret key field (decryption is not implemented).

Then comes the implementation of the encryption and signature by the sender:

```rust
impl Sender {
    pub fn send(&self, m: Message, r: &Receiver) -> ElGamal {
        let c_2: G1Affine = (r.pk.mul(&self.sk) + m.0).into_affine();
        ElGamal(self.pk, c_2)
    }

    pub fn authenticate(&self, c: &ElGamal) -> G2Affine {
        let hash_c = c.hash_to_curve();
        hash_c.mul(&self.sk).into_affine()
    }
}
```

Let us express what it does mathematically.
Let $P_s = x G_1 \in \GG_1$ denote the public key of the sender (`self.pk`) and $x$ denote the corresponding secret key (`self.sk`), $P_r \in \GG_1$ the public key of the receiver (`r.pk`), and $M \in \GG_1$ denote the message (`m`).
Then the ciphertext returned by function `send` is
\[
 (C_1 = P_s, C_2 = x P_r + M).
\]

Hence, this is just ElGamal encryption where $P_s = x G_1$ plays the role of the randomness of the standard ElGamal ciphertext.

The signature returned by function `authenticate` is the point in $\GG_2$ defined as
\[
 S = x H(C_1,C_2).
\]

Hence, this is simply a BLS signature computed on the ciphertext.

Although decryption by the receiver is not implemented, verification of ciphertexts is implemented through a function `check_auth` on the empty struct `Auditor`:

```rust
impl Auditor {
    pub fn check_auth(sender_pk: G1Affine, c: &ElGamal, s: G2Affine) -> bool {
        let lhs = { Bls12_381::pairing(G1Projective::generator(), s) };

        let hash_c = c.hash_to_curve();
        let rhs = { Bls12_381::pairing(sender_pk, hash_c) };

        lhs == rhs
    }
}
```
This simply checks that the BLS signature $S$ is valid for public key $P_s$ and message $(C_1,C_2)$:
\[
 e(G_1, S) \stackrel{?}{=} e(P_s, H(C_1,C_2)).
\]

So to summarize, Bob's signcryption scheme simply encrypts the message using ElGamal and signs the ciphertext using the randomness of the ElGamal ciphertext as secret key for the BLS signature scheme.

## Solving the Puzzle

The `main` function defines a `Blob` instance (by deserializing data in *blob.bin*) containing the sender public key $P_s$, the ciphertext $(C_1,C_2)$, the signature $S$ and the receiver public key $P_r$:

```rust
    let blob = Blob::deserialize_uncompressed(data.as_slice()).unwrap();
```

where `Blob` is defined as

```
pub struct Blob {
    pub sender_pk: G1Affine,
    pub c: ElGamal,
    pub s: G2Affine,
    pub rec_pk: G1Affine,
}
```

It also defines 10 candidate messages:

```rust
    let messages = generate_message_space();
```

where the `generate_message_space` function is defined as

```rust
fn generate_message_space() -> [Message; 10] {
    let g1 = G1Projective::generator();
    let msgs = [
        390183091831u64,
        4987238947234982,
        84327489279482,
        8492374892742,
        5894274824234,
        4982748927426,
        48248927348927427,
        489274982749828,
        99084321987189371,
        8427489729843712893,
    ];
    msgs.iter()
        .map(|&msg_i| Message(g1.mul(Fr::from(msg_i)).into_affine()))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}
```

Hence, the message is not completely arbitrary in $\GG_1$, we know *a priori* that it corresponds to one of the 10 messages in the `messages` vector.

The ciphertext is valid, as checked by the following lines:

```rust
    // ensure that blob is correct
    assert!(Auditor::check_auth(blob.sender_pk, &blob.c, blob.s));
```

ElGamal encryption is IND-CPA secure under the decisional Diffie-Hellman (DDH) assumption (which is believed to hold in group $\GG_1$ of BLS12-381), hence we must find a way to exploit information in the signature.

Since there are only 10 possible messages, if we can find a "test function" which is satisfied only by the real message, then we are done (note that this would not be the case if the space of potential message was too large to exhaustively run the test).

Recall that the message $M$ satisfies $C_2 = xP_r + M$ or equivalently
\[
 xP_r = C_2-M.
\]

Also, the signature is $S= xH(C_1,C_2)$.

In other words, the discrete log of $C_2-M$ in base $P_r$ (in group $\GG_1$) is equal to the discrete log of $S$ in base $H(C_1,C_2)$ (in group $\GG_2$).
But equality of discrete logs is exactly the property that a pairing allows to test!
Here is our test function then: for each potential message, check whether
\[
 e(C_2-M, H(C_1,C_2)) \stackrel{?}{=} e(P_r, S).
\]

Only for the real message will this equation be satisfied since $C_2 - M = xP_r$ implies
\[
 e(C_2-M, H(C_1,C_2)) = e(xP_r, H(C_1,C_2)) = e(P_r, xH(C_1,C_2)) = e(P_r, S).
\]

The attack is straightforward to implement:

```rust
{{#rustdoc_include ../../../puzzles/puzzle-chaos-theory/src/main.rs:solve}}
```

We find that the encrypted message has index 3.

## Conclusion

The main takeaway is that adding a BLS signature using the randomness of the ElGamal ciphertext as secret key for signing allowed a test function to discriminate the real plaintext.

In order to securely combine a public key encryption scheme and a signature scheme, one can use generic composition and simply "encrypt-then-sign", but with *independent* randomness in the encryption part and the signature part.
This means that the ElGmal ciphertext should be
\[
 (C_1 = uG_1, C_2 = uP_r + M)
\]
for some random $u \in \FF_r$ independent from the sender signing key $x$ (and freshly drawn for each ciphertext).
The exact security of this method was studied in [[ADR02](../../references.md#ADR02)] where it was shown that combining an IND-CPA-secure encryption scheme (such as ElGamal encryption) and a EUF-CMA-secure signature scheme (such as BLS) yields a so-called "outsider-secure" signcryption scheme.
Outsider-security means that the sender is protected against forgery as long as the receiver's secret key is not compromised and conversely the confidentiality of messages sent to the receiver is ensured as long as the sender's secret key is not compromised.
By opposition, "insider-security" means that the sender is protected even if the receiver's secret key leaks and vice-versa.

There is actually a more complicated way to combine ElGamal and BLS into a signcryption scheme achieving the stronger notion of insider-security which has been proposed in [[LQ04](../../references.md#LQ04)].

The idea is as follow (note that the paper describes the scheme with a symmetric pairing, we transpose it here for an asymmetric pairing).
As before, let $(x, P_s = xG_1)$ and $(y, P_r = y G_1)$ be the sender and receiver secret/public key pairs.
To encrypt a message $m$, the sender draws $r \in \FF_r$ uniformly at random and computes

\[\begin{aligned}
 U & = r G_1 & \text{(compute nonce)} \\
 V & = x H(m,U,P_r) \in \GG_2 & \text{(sign)} \\
 W & = V \oplus H'(U,P_r,rP_r) & \text{(encrypt sig. with hashed ElGamal)} \\
 Z & = (m \Vert P_s) \oplus H_3(V) & \text{(encrypt message and sender pub key)}.
\end{aligned}\]

The signed ciphertext is $(U,W,Z)$.
