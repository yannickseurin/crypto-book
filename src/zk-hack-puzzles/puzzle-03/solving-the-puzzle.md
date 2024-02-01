## Solving the Puzzle

Where are we left at after this code analysis?
The proof system seems to be well designed, so presumably the problem is with the "proprietary prover" developed by Bob.
Actually, a very well-known implementation vulnerability of sigma protocols is *randomness reuse*.
In the context of discrete-log based signatures such as Schnorr or ECDSA signatures, repeating a nonce allows anyone to compute the private key from just two signatures.
Vulnerable implementations lead, for example, to [the jailbreaking of Sony's Play Station 3](https://fahrplan.events.ccc.de/congress/2010/Fahrplan/events/4087.en.html) and [the theft of some bitcoins from Android wallets](https://arstechnica.com/information-technology/2013/08/google-confirms-critical-android-crypto-flaw-used-in-5700-bitcoin-heist/).
Even if nonces are not repeated, seemingly small biases in nonce randomness [[BH19](../../references.md#BH19)] or partial information leakage (typically through side channels) [[ANT+20](../../references.md#ANT+20)] can be sufficient to retrieve the private key.

For the proof system of this puzzle, note how the [proof that the system is extractable](initial-inspection.md#proof-of-knowledge) exploits the fact that from two accepting transcripts with the same commitments $(C_r,C_1,C_2)$ but different challenges $\gamma$, one can compute a witness $(\bfa, \alpha)$.
This property, which is used in the security proof in a "positive" sense, can actually give rise to a real attack in case a prover reuses the same randomness $(\rho, \tau, \nu)$ (and hence the same commitments) in two runs of the (interactive) protocol with different challenges.
Here, because the Fiat-Shamir transform is used and the challenge is actually computed by hashing the commitment key, the instance, and the commitments, this would actually result in the same challenge and hence exactly the same transcript!
(If you think of the corresponding attack for Schnorr signatures, the challenges are different if the victim signs *different* messages while reusing the same nonce.)
However, one can check that the attack would work if the same randomness was reused for two different instances $(C_a, \bfb)$ and $(C_a, \bfb')$: the challenges obtained via Fiat-Shamir would be different (because $\bfb \neq \bfb'$) but the reasoning of the [extractability proof](initial-inspection.md#proof-of-knowledge) still applies.

Ca we apply this attack here?
Let us display the commitments in the two proofs:

```rust
{{#rustdoc_include ../../../puzzles/zkhack-double-trouble/src/bin/verify-double-trouble.rs:commitments}}
```

We get:

```text
proof1, comm_r:
 GroupAffine(x=Fp256 "(54103849E3BA52CCE4C2C7485134A683257413F5B9A1E0DD8B04FAF09D18EC28)", y=Fp256 "(245981A43B6DB2323AB5DD6B59A72428238E1F7416DA0C30E239D3AD8EFC7CF1)")
proof1, comm_1:
 GroupAffine(x=Fp256 "(0ADC9FA9FE8D825BD5DA31F56D60EEB608EA5C47AF990736C3D27FAC048C11E1)", y=Fp256 "(46202BAFFE0145321B52334023D0A64C70B0283EB02A542C788FDF182C06ED4A)")
proof1, comm_2:
 GroupAffine(x=Fp256 "(19FD6B1FBA846B5212FD91F823E1D3CDD944FE641035B2E459876BB67C2A20F5)", y=Fp256 "(63BE5660CDF347C66B93E5CD5F53BED2AC02172EF99A960D5D13B7BE896952BD)")

proof2, comm_r:
 GroupAffine(x=Fp256 "(10098E91DCAF5036082E598F953E71B128BF1DA198D1CC39364272EE6A0FCD20)", y=Fp256 "(2DD073C47A020602A0CEF1C13E6D1365CB0ADC716935AE1A010E1546DF2BF7A1)")
proof2, comm_1:
 GroupAffine(x=Fp256 "(2F6A95827C2DF00431A43567CE757DCA4FABA1439EE6B09EB0A8CE88DF06B68C)", y=Fp256 "(2F06AC079158FC73402C6C4AF49DA4E9A957283439C4B45C25D116F340107C06)")
proof2, comm_2:
 GroupAffine(x=Fp256 "(110DE1B6E88AABFFAA4ED784B5EEF7BF359D5D02C7EDF745A873ED28221C208B)", y=Fp256 "(2A7594A3D6F65B338A8817D79F5ED22FC2751EBDDD5246A88645D25C8510FD85)")
```

The values of the commitments $(C_r, C_1, C_2)$ in the two proofs provided by the puzzle are different, hence we are not dealing with mere "randomness reuse" here.
The puzzle description gives us a hint:

> he [Bob] developed a proprietary prover that he claims is 2x faster than the standard one described below, but without sacrificing zero-knowledge

So maybe there is a simple relation between the commitments used in the two proofs, allowing to compute several proofs faster?
Indeed, one can check that the commitment $C_r$ in the second proof is twice the one in the first proof (arguably there is some guess work here...):

```rust
{{#rustdoc_include ../../../puzzles/zkhack-double-trouble/src/bin/verify-double-trouble.rs:double}}
```

How can we exploit this fact?

### Coding the Attack

In all the following, we will denote quantities related to the first proof with exponent $(1)$ and quantities related to the second proof with exponent $(2)$, e.g., $C_r^{(1)}$, $C_r^{(2)}$, $\gamma^{(1)}$, $\gamma^{(2)}$, etc.

We have noticed that $C^{(2)}_r = 2 C^{(1)}_r$.
How can we exploit this information?
Presumably, the prover is using $\bfr^{(2)} = 2 \bfr^{(1)}$ and $\rho^{(2)} = 2 \rho^{(1)}$ in the second proof.
Hence, we have the following system of equations:
\[\begin{aligned}
 \bfs^{(1)} & = \bfa + \gamma^{(1)} \bfr^{(1)} \\
 \bfs^{(2)} & = \bfa + 2\gamma^{(2)} \bfr^{(1)}.
\end{aligned}\]

We can get rid of $\bfr^{(1)}$ by multiplying the first equation by $2 \gamma^{(2)}$, the second equation by $\gamma^{(1)}$, and substracting both equations, which yields
\[
 \gamma^{(1)} \bfs^{(2)} - 2 \gamma^{(2)} \bfs^{(1)} = \gamma^{(1)} \bfa - 2 \gamma^{(2)} \bfa
\]
and hence, letting $ k \defeq (\gamma^{(1)} - 2 \gamma^{(2)})^{-1}$,
\[
 \bfa = k (\gamma^{(1)} \bfs^{(2)} - 2 \gamma^{(2)} \bfs^{(1)}).
\]
Similarly, from $\rho^{(2)} = 2 \rho^{(1)}$ we obtain
\[
 \alpha = k (\gamma^{(1)} u^{(2)} - 2 \gamma^{(2)} u^{(1)}).
\]

Here is the code computing $\bfa$ and $\alpha$ and checking that they yield the correct commitment $C_a$:

```rust
{{#rustdoc_include ../../../puzzles/zkhack-double-trouble/src/bin/verify-double-trouble.rs:solve}}
```

### Conclusion

The lesson of this puzzle is that one should never try to optimize a Sigma protocol (or any kind of ZK proof) by compromising on the quality of the randomness used by the prover.
It is not enough for commitments in different runs of a sigma protocol to be different, they must be computed using independent and fresh randomness in each run.
In case the prover does not have access to a reliable source of randomness, one can use proof systems satisfying the stronger *resettable zero-knowledge* notion [[CGGM00](../../references.md#CGGM00)].