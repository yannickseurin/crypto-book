# Puzzle 2: Group Dynamics

- [puzzle page](https://zkhack.dev/events/puzzle2.html)
- [GitHub repository](https://github.com/kobigurk/zkhack-trusted-setup)
- puzzle description:

```text
Alice has computed a trusted setup for a Groth16 proof scheme.
She decided to use a 128-bit long secret, and she swears that she does not know
the secret s needed to get this setup.
The trusted setup is constructed as follows using two additional scalars α and β:
* [s^i] G1 for 0 ⩽ i ⩽ 62,
* [α s^i] G1 for 0 ⩽ i ⩽ 31,
* [β s^i] G1 for 0 ⩽ i ⩽ 31,
* [s^i] G2 for 0 ⩽ i ⩽ 31.

Can you recover the secret anyway?
```

Heads-up: although the puzzle description refers to the Groth16 zk-SNARK [[Gro16](../../references.md#Gro16)], there's no need to know anything about Groth16 to solve the puzzle.
Suffice it to say, Groth16 uses a so-called *structured reference string* (or *common reference string*) generated during a *trusted setup* which has a form similar to the one of the puzzle data, meaning it consists of points on some pairing-friendly pair of curves computed somehow similarly to what is described in the puzzle instructions.
Anyone able to retrieve the secret values $(s, \alpha, \beta)$ (the *simulation trapdoor*, sometimes referred to as "toxic waste" as it must absolutely be discarded after the trusted setup) would be able to break the soundness of the proof system, meaning it could produce valid proofs for false statements.

Another important scheme where such structured parameters show up is the [KZG polynomial commitment scheme](../../cryptographic-notions/polynomial-commitment-schemes.md).

Let's take a look at the code to see what this is about.
