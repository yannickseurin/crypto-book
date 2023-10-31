> **Chapter status:** in good shape
>
> **Related puzzles:** [Puzzle 1](../zk-hack-puzzles/puzzle-01/intro.md)
>
> **TODO:**

# Commitment Schemes

This section contains a brief introduction to commitment schemes, focusing on Pedersen commitments.

## Contents

<!-- toc -->

## Generalities

A commitment scheme involves two parties, a *committer* (or prover) and a *verifier*.
It allows the committer to send to the verifier a commitment $C$ to some secret value $m$ and later on to open this commitment to reveal $m$.
The commitment $C$ should not reveal any information about the message $m$ (hiding property) and the committer should not be able to open a commitment in two distinct ways (binding property).

### Syntax

More formally, a commitment scheme consists of three algorithms (the exact syntax can vary slightly in the literature):

- a probabilistic setup algorithm $\setup$ which on input the security parameter $\secparam$ returns public parameters[^par] $par$ which in particular specify a message space $\cM_{par}$ (in the following, we simply denote the message space $\cM$, leaving the dependency on $par$ implicit);
- a probabilistic commitment algorithm $\commit$ which on input parameters $par$ and a message $m \in \cM$ returns a commitment $C$ and a decommitment[^decommit] $D$;
- a deterministic verification algorithm $\verif$ which on input parameters $par$, a commitment $C$, a message $m \in \cM$, and a decommitment $D$, return 1 if the decommitment is valid for $(par,C,m)$ and 0 otherwise.

Quite often, the decommitment $D$ simply consists of the random coins $r$ used by the commitment algorithm, and the verification algorithm simply recomputes the commitment given $m$ and $r$ and compares with $C$.
When this is the case, overloading the notation, we will let $\commit(par, m; r)$ denote the function explicitly taking the random coins $r$ of the commitment algorithm as input and returning the commitment $C$ (letting the decommitment $D = r$ implicit in that case).

Note that what we just defined here is the syntax for a *non-interactive* commitment scheme, where the $\setup$ algorithm is run once and for all and committing consists of a single message sent by the prover to the verifier.
There exists more complex commitment schemes where committing requires some interaction between the prover and the verifier.

Correctness requires that for every security parameter $\secpar$, the following game capturing the nominal execution of algorithms returns true with probability 1:

\[\boxed{\begin{aligned}
 & par \gets \setup(\secparam) \\
 & m \sample \cM \\
 & (C,D) \gets \commit(par, m) \\
 & b \gets \verif(par, C, m, D) \\
 & \pcassert (b=1)
\end{aligned}}\]

### Security

A commitment scheme should satisfy two security properties informally defined as follows:

- *hiding*: the commitment $C$ should not reveal any information about the secret value $m$ to the verifier,
- *binding*: the committer should not be able to open the commitment in two different ways.

Let us formalize these two properties more precisely, starting with hiding, defined by the following game:

\[
 \def\arraystretch{\myarraystretch}
 \boxed{
 \begin{array}{ll}
  \text{\underline{Game HIDING:}} & \qquad \text{\underline{Oracle $\orcl{Commit}(m_0, m_1)$:}} \\
  b \sample \bin & \qquad \pcassert (m_0 \in \cM) \\
  par \gets \setup(\secparam) & \qquad \pcassert (m_1 \in \cM) \\
  b' \gets \adv^{\text{Commit}}(par) & \qquad (C,D) \gets \commit(par,m_b) \\
  \pcassert (b=b') & \qquad \pcreturn C
 \end{array}
 }
\]

In some cases, it might be necessary to check additional conditions and messages $m_0$ and $m_1$ queried to oracle $\orcl{Commit}$ (e.g., when $\cM$ consists of bit strings of various lengths and $\commit$ does not hide the message length, $m_0$ and $m_1$ should have the same length).

Binding is defined by the following game:

\[
 \def\arraystretch{\myarraystretch}
 \boxed{
 \begin{array}{l}
  \text{\underline{Game BINDING:}} & \\
  par \gets \setup(\secparam) \\
  (C,m,D,m',D') \gets \adv(par) \\
  b \gets \verif(par,C,m,D) \\
  b' \gets \verif(par,C,m',D') \\
  \pcassert (m \neq m') \\
  \pcassert (b = 1) \\
  \pcassert (b' = 1)
 \end{array}
 }
\]

For some commitment schemes, one of these two properties holds [statistically](./games-models-and-assumptions.md#advantage) (i.e., cannot be broken with non-negligible advantage even by a computationally unbounded adversary) or even perfectly.
However, a commitment scheme cannot be *both* statistically hiding and statistically binding at the same time.
Hence, at best, a commitment scheme can be either statistically hiding and computationally binding or computationally hiding and statistically binding.

### Homomorphic Commitments

Informally, a commitment scheme is homomorphic if the message space $\cM$ equipped with some binary operation $\star$ forms a [group](../mathematical-preliminaries/groups.md) and given two commitments $C_1$ and $C_2$ to respectively $m_1$ and $m_2$, anyone can compute a commitment $C$ that the committer can open to $m_1 \star m_2$.

More formally, a commitment scheme is homomorphic (with respect to group operation $\star$) if there exists two algorithms $\homcom$ and $\homdecom$ such that

- $\homcom$ takes parameters $par$ and two commitments $C_1$ and $C_2$ and returns a commitment $C$;
- $\homdecom$ takes parameters $par$ and two decommitments $D_1$ and $D_2$ and returns a decommitment $D$;
- for any security parameter $\secpar$, the following game returns true with probability 1:
\[\boxed{\begin{aligned}
 & par \gets \setup(\secparam) \\
 & m_1, m_2 \sample \cM \\
 & (C_1,D_1) \gets \commit(par, m_1) \\
 & (C_2,D_2) \gets \commit(par, m_2) \\
 & C \gets \homcom(par,C_1,C_2) \\
 & D \gets \homdecom(par,D_1,D_2) \\
 & b \gets \verif(par, C, m_1 \star m_2, D) \\
 & \pcassert (b=1)
\end{aligned}}\]

Algorithms $\homcom$ and $\homdecom$ are often quite simple (e.g., when the commitment and decommitment spaces also have a group structure, they simply consist in applying the corresponding group operation to $C_1$ and $C_2$ or $D_1$ and $D_2$ respectively).

## Pedersen Commitments

### Description and Security

The Pedersen commitment scheme, initially introduced in [[Ped91](../references.md#Ped91)], is widely used, in particular to build zero-knowledge proof systems.
It is specified as follows.
Let $\groupsetup$ be a [group setup algorithm](./games-models-and-assumptions.md#group-setup-algorithms).
Then:

- the setup algorithm $\setup$, on input $\secparam$, runs $(\GG,p) \gets \groupsetup(\secparam)$, draws two random generators $G$ and $H$ of $\GG$, and returns parameters $par = (\GG,p,G,H)$; the message space is $\cM = \ZZ_p$;

- the commitment algorithm $\commit$, on input parameters $par = (\GG,p,G,H)$ and a message $m \in \ZZ_p$, draws $r \sample \ZZ_p$ and returns a commitment $C = mG + rH$ and a decommitment $D = r$;

- the verification algorithm $\verif$, on input parameters $par = (\GG,p,G,H)$, a commitment $C \in \GG$, a message $m \in \ZZ_p$, and a decommitment $D = r \in \ZZ_p$, returns 1 if $mG + rH = C$ and 0 otherwise.

**Theorem.**
*The Pedersen commitment scheme is perfectly hiding, computationally binding under the discrete logarithm assumption, and homomorphic with respect to addition over $\ZZ_p$.*

> *Proof.*
Let us sketch the proof of each property:
> - *perfectly hiding*: as $r$ is uniformly random in $\ZZ_p$, for any message $m$, $C$ is uniformly random in $\GG$ and hence does not reveal any information about $m$;
> - *computationally binding*: assume an adversary can output two message/decommitment pairs $(m,r)$ and $(m',r')$ with $m \neq m'$ for the same commitment $C$; then
\[
 (m-m') G = (r'-r) H,
\]
which yields the discrete logarithm of $H$ in base $G$ (note that $m - m' \neq 0$ implies $r'-r \neq 0$ as $G$ and $H$ are generators of $\GG$);
> - *additively homomorphic*: given two commitments $C_1 = m_1 G + r_1 H$ and $C_2 = m_2 G + r_2 H$, anyone can compute $C \defeq C_1 + C_2= (m_1 + m_2) G + (r_1+r_2) H$, and the committer can compute $r_1+r_2$ which is a valid decommitment for $C$ and message $m_1+m_2$.

### A Note About the Setup

Importantly, the setup algorithm should ensure that nobody knows the discrete logarithm of $H$ in base $G$.
In particular, it is not safe to allow the committer to choose the public parameters: if the committer knows the discrete logarithm of $H$ in base $G$, then the scheme is not binding anymore.
Say the committer knows $h$ such that $H = hG$. Then it can send $C = cG$ as commitment; later, it can open this commitment to *any* value $m \in \ZZ_p$ it wants by computing $r = h^{-1}(c-m) \bmod p$ and sending decommitment $D = (m,r)$: it satisfies $mG + rH = (m+rh)G = cG = C$.

For this reason, Pedersen's scheme is sometimes referred to as a *trapdoor* (or *equivocal*) commitment scheme, which can be useful in security proofs but should also make us wary.
However, there are secure ways to select the commitment key without a trusted third party, such as using a hash-to-group function (a.k.a. hash-to-curve function in case $\GG$ is based on an elliptic curve) applied to some NUMS ([nothing-up-my-sleeve](https://en.wikipedia.org/wiki/Nothing-up-my-sleeve_number)) input.
Hence, even though Pedersen commitments do not require a *trusted setup*, one should always *verify* that parameters were correctly generated.
For a real-world example where this trapdoor property could have been used, see Section VI of [[HLPT20](../references.md#HLPT20)] about the Scytl/SwissPost e-voting system.

### Variants

The Pedersen commitment scheme can be generalized to messages which are vectors $\bfm = (m_0, \dots, m_{n-1}) \in (\ZZ_p)^n$: the parameters are extended to $par = (\GG, p, G_0, \dots, G_{n-1}, H)$ where $G_0, \dots, G_{n-1},$ and $H$ are uniformly random and independent generators of $\GG$, and the commitment for message $\bfm = (m_0, \dots, m_{n-1})$ with randomness $r$ is
\[
 C \defeq \sum_{i=0}^{n-1} m_i G_i + r H.
\]
This is usually called the *generalized Pedersen commitment scheme*, or sometimes the *Pedersen vector commitment scheme*, although this is somehow a misnomer as it does not have all the properties required from a *vector commitment scheme* [[CF13](../references.md#CF13)].
As for the basic variant, it can be shown to be perfectly hiding, computationally binding under the DL assumption, and homomorphic with respect to addition over $(\ZZ_p)^n$.

The "random" part of the commitment $rH$ is sometimes omitted, in which case the commitment algorithm becomes deterministic and the commitment is simply
\[
 C \defeq \sum_{i=0}^{n-1} m_i G_i.
\]
In that case, the scheme is still computationally binding under the DL assumption (and even perfectly binding for $n=1$ as the commitment function is bijective), but it is not hiding anymore (given two messages $\bfm_0$ and $\bfm_1$ and a commitment $C$ to $\bfm_b$ for some random bit $b$, one can recover $b$ by simply computing the commitments corresponding to respectively $\bfm_0$ and $\bfm_1$ and comparing with $C$).
For this reason, it is sometimes referred to as the *non-hiding Pedersen commitment scheme*.
It is however *preimage-resistant* under the DL assumption, meaning that given a random commitment $C \in \GG$, it is hard to compute a message $\bfm$ such that $\commit(par,\bfm) = C$.

## ElGamal Commitments

The Pedersen commitments scheme has a relative known as the ElGamal commitment scheme where the commitment key $ck$ is $(\GG,p,G,H)$ as for Pedersen and the commitment for message $m \in \ZZ_p$ with randomness $r \sample \ZZ_p$ is the pair $(C_1, C_2) = (rG, mG + rH)$.
(Note the similarity with ElGamal encryption w.r.t. public key $H$.)
This scheme is perfectly binding, computationally hiding under the DDH assumption, and additively homomorphic.

If the message is encoded as a group element $M \in \GG$ and the commitment computed as $(C_1, C_2) = (rG, M + rH)$, the scheme has a trapdoor property allowing anyone with knowledge of the discrete logarithm $h$ of $H$ in base $G$ to *extract* the message $M$ (by "decrypting" the commitment as in ElGamal encryption, i.e., computing $M = C_2 - h C_1$).

## Commitments and Hash Functions

There is a strong connection between commitment schemes and collision-resistant hash functions.

First, let us consider the following strengthening of the binding property:
We say that a commitment scheme if *strongly binding* if it is hard to find a commitment $C$ and two distinct message-decommitment pairs $(m,D)$ and $(m',D')$ such that $\verif(par,C,m,D) = 1$ and $\verif(par,C,m',D') = 1$.
That is, the adversary wins also when the messages $m$ and $m'$ are equal but the decommitments $D$ and $D'$ are different.

Given a hash function family $H_{par} \colon \str \to \bin^{2 \secpar}$ indexed by some parameter $par$, one can define a simple commitment scheme with
\[
 \commit(par, m; r) \defeq H_{par}(m \Vert r)
\]
where $r \sample \bin^{\lambda}$.
It can be shown to be (computationally) strongly binding assuming the family $(H_{par})$ is collision-resistant.
On the other hand, there is no reason for this scheme to be hiding in general ($H$ could for example reveal the first bit of the message, allowing to distinguish commitments to two messages with distinct first bits).
It is however easily seen to be (computationally) hiding in the random oracle model.

Reciprocally, it is straightforward to derive a collision-resistant hash function family from a strongly binding commitment scheme.

**Proposition.**
*Consider a commitment scheme $\Pi$ with a $\commit$ function taking parameters $par$, a message $m \in \cM$, and explicit random coins $r \in \cR$.
If $\Pi$ is strongly binding, then the function family
\[
 H_{par} \colon (m,r) \mapsto \commit(par,m;r)
\]
is collision-resistant.*

> *Proof.*
Assume that $H$ is not collision-resistant and that there is an adversary which on input $par$ returns $(m,r) \neq (m',r')$ such that $H_{par}(m,r) = H_{par}(m',r')$.
Let $C \defeq H_{par}(m,r) = H_{par}(m',r')$.
Then $\verif(par,C,m,r) = \verif(par,C,m',r') = 1$, hence this adversary can be used to break strong binding of $\Pi$.

Note that the assumption that $\Pi$ is binding is not sufficient: it could be easy to find $(m,r) \neq (m',r')$ such that $\commit(par,m;r) = \commit(par,m';r')$ but with $m=m'$, which would break collision-resistance but not binding.

It is not hard to see that the Pedersen commitment scheme is actually strongly binding, which directly gives an algebraic family of collision-resistant hash functions usually called *Pedersen hashing*.
A specific instance of the family is specified by a tuple of parameters $par = (\GG, p, G_0, \dots, G_{n-1})$ where $n\ge 2$, $\GG$ is a cyclic group of order $p$, and $G_0, \dots, G_{n-1}$ are generators chosen in a way such that nobody knows any discrete logarithm relation between them.
Then $H_{par}$ has domain $(\ZZ_p)^n$, range $\GG$, and is defined by
\[
 H_{par}(m_0,\dots,m_{n-1}) = \sum_{i=0}^{n-1} m_i G_i.
\]
(Note that in the context of hashing, there is no distinction between the "message" and the "randomness" as in the context of commitment schemes.)

This family of hash functions is collision-resistant assuming the discrete logarithm problem is hard.

Variants are possible: for example, if inputs are bit strings of length exactly $L$, one can split the input $m$ into chunks of $w$ consecutive bits with $2^w \le p$ and $L = nw$, convert the $i$-th chunk into an integer $m_i$, and let $H_{par}(m) = \sum_{i=0}^{n-1} m_i G_i$.

## Further Resources

For more background on commitment schemes, see for example [this article](https://homepages.cwi.nl/~schaffne/courses/crypto/2014/papers/ComZK08.pdf) by Damgård and Nielsen and [this lecture](https://piazza.com/class_profile/get_resource/iiauye4zvj2hu/inhhhsyt9x74kl) by Dodis.

----

[^par]: The name can vary; these parameters are sometimes called a *commitment key* or a *public key*.

[^decommit]: Again, the name can vary and it is sometimes called an *opening* or a *hint*.