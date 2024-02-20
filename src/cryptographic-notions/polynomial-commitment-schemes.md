> **Chapter status:** ✅ in good shape ✅
>
> **Related puzzles:** [Puzzle 2](../zk-hack-puzzles/puzzle-02/intro.md) and [Puzzle 4]()
>
> **TODO:**
> - cover [Kohrita-Towa variant](https://eprint.iacr.org/2023/917.pdf)
> - batch proofs for multiple polynomials [[GWC19](../references.md#GWC19)] [[BDFG20](../references.md#BDFG20)]
> - multivariate polynomials [[PST13](../references.md#PST13)]
> - degree-checks
>

# Polynomial Commitment Schemes

Polynomial commitment schemes play a central role in the design of zk-SNARKs.
In this section, we introduce them and present the two KZG schemes, DL-KZG and Ped-KZG.

Make sure to read the chapters about [polynomials](../mathematical-preliminaries/polynomials.md) and [standard commitments](./commitment-schemes.md) first.

We recall that given a field $\FF$, we let $\PR{}{d}$ denote the set of all univariate polynomials over $\FF$ of degree at most $d$.

## Contents

<!-- toc -->

## Generalities

As standard commitment schemes, a *polynomial commitment scheme* (PC scheme for short) involves two parties, a committer (or prover) and a verifier.
It allows the committer to send to the verifier a commitment $C$ to some secret polynomial $p \in \PR{}{d}$ and later on prove that $p$ evaluates to some specific value $v \in \FF$ at input $u \in \FF$ (usually of the verifier's choosing), potentially for multiple inputs.

Recall that a polynomial of degree at most $d$ is specified by the tuple $\bfa = (a_0, \dots, a_d) \in \FF^{d+1}$ of coefficients such that $p(X) = \sum_{i=0}^d a_i X^i$.
From a broader perspective, a PC scheme can be seen as the combination of a standard commitment scheme over $\FF^{d+1}$ together with various proof systems for proving assertions about the committed vector $\bfa$.
In particular, proving that $p(u) = v$ is equivalent to proving that
\[
 \sum_{i=0}^d a_i u^i =v
\]
i.e., proving that the inner product of $\bfa$ with the vector $(1,u,u^2, \dots,u^d)$ is equal to $v$.
Other assertions one may want to prove when designing SNARKs are, for example, that $p$ has degree at most $d'<d$, which is equivalent to $a_{d'+1} = \cdots = a_d =0$, or more complex relations about evaluations of $p$ such as $\sum_{u \in \cU} p(u) = 0$ for some subset $\cU \subset \FF$.

### Syntax

More formally, a PC scheme is parameterized by a maximal degree $d \in \NN$ (one can think of $d$ as being given as input to all algorithms) and consists of the five following algorithms (the exact syntax can vary slightly in the literature, here we adhere to the syntax of standard commitment schemes):

- a setup algorithm $\setup$ which on input the security parameter $\secparam$ returns public parameters{{footnote: As for standard commitment schemes, the name can vary and this is sometimes called a *common reference string (crs)* or *structured reference string (srs)* when it does not consist of random bits and has a specific "shape", as it is the case for KZG.}} $par$; these parameters implicitly specify some finite field $\FF$;

- a commitment algorithm $\commit$ which on input parameters $par$ and a polynomial $p \in \PR{}{d}$ returns a commitment $C$ and a decommitment $D$;

- a "polynomial" verification algorithm $\polyverif$ which on input parameters $par$, a commitment $C$, a polynomial $p \in \PR{}{d}$, and a decommitment $D$, returns 1 if $D$ is a valid decommitment for $(par,C,p)$ and 0 otherwise;

- a proving algorithm $\evalprove$ which on input parameters $par$, a polynomial $p \in \PR{}{d}$, a decommitment $D$, and a value $u \in \FF$ returns an evaluation $v \in \FF$ and a proof $\Pi$;

- an "evaluation" verification algorithm $\evalverif$ which on input parameters $par$, a commitment $C$, a pair $(u,v) \in \FF^2$, and a proof $\Pi$, returns 1 if $\Pi$ is a valid proof that the polynomial committed to by $C$ evaluates to $v$ at input $u$, and 0 otherwise.

In some cases (in particular for KZG), it might be possible to split the public parameters $par$ into a *commitment key* $ck$ and a *verification key* $vk$, where typically only $ck$ is needed for algorithms $\commit$, $\polyverif$, and $\evalprove$ and only $vk$ is needed for $\evalverif$.

As already hinted, the three algorithms $\setup$, $\commit$, and $\polyverif$ can be regarded together as a standard commitment scheme with message space $\PR{}{d} \cong \FF^{d+1}$ (with $p$ specified by the tuple $(a_0, \dots, a_d) \in \FF^{d+1}$ of its coefficients) while $\evalprove$ and $\evalverif$ together form a proof system for statements of the form $p(u)=v$.

As for standard commitment schemes, what we just defined here is the syntax for a *non-interactive* PC scheme, where the $\setup$ algorithm is run once and for all and then committing and proving an evaluation of the committed polynomial is non-interactive.
More generally, committing and evaluation proving could be interactive.

As always, the scheme must be correct, meaning two things: first, $(\setup, \commit, \polyverif)$ must be correct [as defined for a standard commitment scheme](./commitment-schemes.md#syntax) with message space $\FF^{d+1}$; second, for every security parameter $\secpar$, every $d \in \NN$, every $p \in \PR{}{d}$, and every $u \in \FF$, the following game capturing the nominal execution of algorithms for evaluation proving must return true with probability 1:

\[\boxed{\begin{aligned}
 & par \gets \setup(\secparam) \\
 & (C,D) \gets \commit(par, p) \\
 & (v, \Pi) \gets \evalprove(par, p ,D, u) \\
 & b \gets \evalverif(par, C, (u,v), \Pi) \\
 & \pcassert (b=1)
\end{aligned}}\]

### Security

Defining security properties for PC schemes is rather subtle.
Almost every paper about PC schemes define slightly different sets of security properties depending on the specific application being targeted.
Here, we focus on the security properties proposed in the seminal paper about PC schemes [[KZG10a](../references.md#KZG10a)], which are also the simplest ones.

First, a PC scheme should be hiding and binding [in the standard sense when seen as a commitment](./commitment-schemes.md#security) to the tuple of coefficients $(a_0, \dots, a_d) \in \FF^{d+1}$ defining the polynomial $p$.
Let us the recall the corresponding games, that we call POLY-HIDING and POLY-BINDING for clarity:

\[
 \def\arraystretch{\myarraystretch}
 \boxed{
 \begin{array}{ll}
  \text{\underline{Game POLY-HIDING:}} & \qquad \text{\underline{Oracle $\orcl{Commit}(p_0, p_1)$:}} \\
  b \sample \bin & \qquad \pcassert (\deg(p_0) \le d) \wedge (\deg(p_1) \le d) \\
  par \gets \setup(\secparam) & \qquad (C,D) \gets \commit(par,p_b) \\
  b' \gets \adv^{\text{Commit}}(par) & \qquad \pcreturn C \\
  \pcassert (b=b') &
 \end{array}
 }
\]

\[
 \def\arraystretch{\myarraystretch}
 \boxed{
 \begin{array}{l}
  \text{\underline{Game POLY-BINDING:}} \\
  par \gets \setup(\secparam) \\
  (C,p,D,p',D') \gets \adv(par) \\
  \pcassert (\deg(p) \le d) \wedge (\deg(p') \le d) \\
  b \gets \polyverif(par,C,p,D) \\
  b' \gets \polyverif(par,C,p',D') \\
  \pcassert (p \neq p') \\
  \pcassert (b = 1) \\
  \pcassert (b' = 1)
 \end{array}
 }
\]

It turns out that some PC schemes (such as the DL-KZG scheme) do not satisfy the poly-hiding notion (in general, when used to construct SNARKs, poly-hiding matters only if one cares about the SNARK being zero-knowledge).
However, they satisfy what we call here *evaluation hiding*,{{footnote: In the seminal paper introducing polynomial commitment schemes [[KZG10a](../references.md#KZG10a)], evaluation hiding is simply called hiding.}} which informally means that for a random polynomial $p$ of degree at most $d$, given a commitment to $p$ and at most $d$ evaluations of $p$ together with the corresponding proofs, no adversary should be able to guess the value of $p(u)$ for a new input $u$.
This is formalized by the following game:

\[
 \def\arraystretch{\myarraystretch}
 \boxed{
 \begin{array}{ll}
  \text{\underline{Game EVAL-HIDING:}} & \qquad \text{\underline{Oracle $\orcl{Prove}(u)$:}} \\
  p(X) \sample \PR{}{d} & \qquad (v,\Pi) \gets \evalprove(par,p,D,u) \\
  ctr \gets 0 & \qquad ctr \gets ctr + 1 \\
  \cQ \gets \emptyset & \qquad \cQ \gets \cQ \cup \{u\} \\
  par \gets \setup(\secparam) & \qquad \pcreturn (v,\Pi) \\
  (C,D) \gets \commit(par,p) & \\
  (u,v) \gets \adv^{\text{Prove}}(par,C) & \\
  \pcassert (ctr \le d) & \\
  \pcassert (u \notin \cQ) & \\
  \pcassert (p(u) = v) &
 \end{array}
 }
\]

To be completely explicit, the line
\[
 p(X) \sample \PR{}{d}
\] means
\[\begin{aligned}
 & a_0,\dots,a_d \sample \FF \\
 & p(X) \defeq \sum_{i=0}^d a_i X^i.
\end{aligned}\]

The condition that the adversary makes at most $d$ queries to the $\orcl{Prove}$ oracle is of course necessary: once the commitment has been opened at $d+1$ distinct points $u_0$, $\dots$, $u_d$, the committed polynomial has been completely revealed by virtue of [Lagrange interpolation](../mathematical-preliminaries/polynomials.md#lagrange-interpolation).

Regarding the binding property of evaluation proving, a PC scheme should be *evaluation binding*, meaning no efficient adversary can produce a commitment and two valid proofs that the committed polynomial evaluates to two different values $v \neq v'$ at the same input $u$.
More formally, this is captured by the following game:

\[
 \def\arraystretch{\myarraystretch}
 \boxed{
 \begin{array}{l}
  \text{\underline{Game EVAL-BINDING:}} \\
  par \gets \setup(\secparam) \\
  (C,u,(v,\Pi),(v',\Pi')) \gets \adv(par) \\
  b \gets \evalverif(par,C,(u,v),\Pi) \\
  b' \gets \evalverif(par,C,(u,v'),\Pi') \\
  \pcassert (v \neq v') \\
  \pcassert (b = 1) \\
  \pcassert (b' = 1)
 \end{array}
 }
\]

As for standard commitments, all these properties can hold statistically or computationally, but poly-hiding and poly-binding cannot hold both statistically for a PC scheme.

## Informal Description of the KZG Schemes

Two closely related and very efficient PC schemes based on pairings were proposed by Kate, Zaverucha, and Goldberg in 2010 [[KZG10a](../references.md#KZG10a)] (see also [[KZG10b](../references.md#KZG10b)] for the full paper with security proofs).
We will call them (for reasons that will become clear soon) DL-KZG and Ped-KZG.
What is usually simply called KZG corresponds to the DL-KZG scheme.

Let us start with a high-level view of DL-KZG.
For a maximal degree $d$, the commitment and opening part is very similar to the (non-hiding version of the) [generalized Pedersen commitment scheme](./commitment-schemes#variants).
The public parameters consist of $d+1$ generators $(W_0, \dots, W_d)$ of some group $\GG_1$ of prime order $r$.
A polynomial $p(X) = \sum_{i=0}^d a_i X^i$ is seen as a vector $(a_0, \dots, a_d) \in (\FF_r)^{d+1}$ and the corresponding commitment is
\[
C \defeq \sum_{i=0}^d a_i W_i.
\]
There is a big difference though with generalized Pedersen commitments: the generators $W_0, \dots, W_d$ are not independent.
They are computed from a single generator $G_1 \in \GG_1$ and a secret random scalar $\tau \in \FF_r$ as
\[
(W_0, W_1, \dots, W_d) = (G_1, \tau G_1, \dots, \tau^d G_1).
\]
These parameters have a precise structure.
For this reason, they are also called a *structured reference string (SRS)*.
This also implies that they cannot be sampled obliviously of $\tau$ and require a trusted setup (more on this [later](#trusted-setup)).

As a result, the commitment $C$ is actually $p(\tau)$ in disguise:
\[
 C = \sum_{i=0}^d a_i (\tau^i G_1) = \left(\sum_{i=0}^d a_i \tau^i\right) G_1 = p(\tau) G_1.
\]

Evaluation proving relies on the {{tref: thm:polynomial_remainder}}: a polynomial $p \in \FF[X]$ satisfies $p(u) = v$ if and only if $p(X)-v$ is divisible by $X-u$, i.e., there exists a polynomial $q$ such that
\[
 p(X)-v = (X-u)q(X).
\]
The proving algorithm therefore consists in computing the polynomial $q(X) = \sum_{i=0}^d b_i X^i$ explicitly and the proof $\Pi$, which consists in $q(\tau)$ in disguise, as
\[
 \Pi \defeq \sum_{i=0}^d b_i (\tau^i G_1) = q(\tau) G_1.
\]

Evaluating this polynomial equality at $\tau$, we see that
\[
p(\tau)-v = (\tau-u)q(\tau).
\]
The verification algorithm consists in checking this equality "in the exponent" (or rather "in the scalar multiplication" here as we use additive notation).
This is where pairings comes in: $\GG_1$ is actually a pairing-friendly group coming with related groups $\GG_2$ and $\GG_t$ and a pairing $e \colon \GG_1 \times \GG_2 \to \GG_t$.
The public parameters include a generator $G_2$ of $\GG_2$ and the group element $H_2 = \tau G_2$.
The verifier can compute $(p(\tau)-v)G_1 = C-vG_1$ and $(\tau-u)G_2 = H_2 - uG_2$ and also knows $\Pi = q(\tau)G_1$.
Then $p(\tau)-v = (\tau-u)q(\tau)$ holds *iff* the following pairing equality does:
\[
 e(C-vG_1, G_2) = e(\Pi, H_2 -uG_2).
\]

The DL-KZG commitment scheme is obviously not hiding because the commitment algorithm is deterministic.
The Ped-KZG scheme remedies this problem by adding a commitment to a random polynomial $\hat p = \sum_{i=0}^d \hat a_i X^i$ with respect to another tuple of points $(H_1, \tau H_1, \dots, \tau^d H_1)$:
\[
 C = \sum_{i=0}^d a_i (\tau^i G_1) + \sum_{i=0}^d \hat a_i (\tau^i H_1) = p(\tau) G_1 + \hat p(\tau) H_1.
\]
Evaluation proving is adapted accordingly.
The form of the commitment $C$ is reminiscent of the (hiding version) of Pedersen commitments, explaining the naming convention.

We now give a detailed description and analysis of the properties of the DL-KZG and Ped-KZG schemes.

## The DL-KZG Scheme

### Description

Let $\pairingsetup$ be an asymmetric [pairing group setup algorithm](./games-models-and-assumptions.md#group-setup-algorithms).{{footnote: KZG polynomial commitments are often described with a symmetric pairing (i.e., $\GG_1 = \GG_2$), but we define them for an asymmetric pairing as this is the preferred option in practice.}}
The DL-KZG scheme for a maximal degree $d$ is defined as follows:

- The $\setup$ algorithm, on input the security parameter $\secparam$, runs
\[
 (\GG_1,\GG_2,\GG_t,r,e) \gets \pairingsetup(\secparam),
\]
draws random generators $G_1$ and $G_2$ of respectively $\GG_1$ and $\GG_2$, draws $\tau \sample \FF_r$, and returns public parameters
\[
 par \defeq (G_1, \tau G_1, \dots, \tau^d G_1, G_2, \tau G_2) \in \GG_1^{d+1} \times \GG_2^2.
\]
Here we assume that pairing parameters $(\GG_1,\GG_2,\GG_t,r,e)$ are implicitly specified in $par$.{{footnote: Quite often, generators $G_1$ and $G_2$ are standard and specified in public parameters alongside $\GG_1$ and $\GG_2$.}}
The field over which polynomials are defined is $\FF_r$.
The public parameters can be split into a commitment key
\[
 ck \defeq (G_1, \tau G_1, \dots, \tau^d G_1) \in \GG_1^{d+1}
\]
and a verification key
\[
 vk \defeq (G_1, G_2, \tau G_2) \in \GG_1 \times \GG_2^2.
\]

- The $\commit$ algorithm, on input a commitment key $ck = (G_1, \tau G_1, \dots, \tau^d G_1)$ and a polynomial $p \in \PR{r}{d}$ where $p(X) = \sum_{i=0}^d a_i X^i$, returns the commitment
\[
 C \defeq \sum_{i=0}^d a_i (\tau^i G_1) = p(\tau) G_1
\]
and an empty decommitment $D = \bot$.

- The $\polyverif$ algorithm, on input a commitment key $ck = (G_1, \tau G_1, \dots, \tau^d G_1)$, a commitment $C \in \GG_1$, and a polynomial $p \in \PR{r}{d}$ where $p(X) = \sum_{i=0}^d a_i X^i$, returns 1 if $C = \sum_{i=0}^d a_i (\tau^i G_1)$ and 0 otherwise.

- The $\evalprove$ algorithm, on input a commitment key $ck= (G_1, \tau G_1, \dots, \tau^d G_1)$, a polynomial $p \in \PR{r}{d}$, and $u \in \FF_r$, computes the polynomial{{footnote: The polynomial $q(X)$ is well-defined by the {{tref: thm:polynomial_remainder}}.}}
\[
 q(X) \defeq \frac{p(X)-p(u)}{X-u} = \sum_{i=0}^d b_i X^i,
\]
the group element
\[
 \Pi \defeq \sum_{i=0}^d b_i (\tau^i G_1) = q(\tau) G_1,
\]
and returns $p(u)$ and the proof $\Pi$.

- The $\evalverif$ algorithm, on input a verification key $vk = (G_1, G_2, \tau G_2)$, a commitment $C$, a pair $(u,v) \in \FF_r^2$, and a proof $\Pi$, returns 1 if
\[
 e(C -v G_1, G_2) = e(\Pi, \tau G_2 - u G_2) {{numeq}}{dlkzg_verif}
\]
and 0 otherwise.

It is straightforward to verify that $(\setup,\commit,\polyverif)$ is correct as a standard commitment scheme.
Let's check that the scheme is correct with respect to evaluation proving, i.e., if the commitment $C$ and the proof $\Pi$ have been honestly computed, then the verification passes.
If $C = p(\tau) G_1$ and $\Pi = q(\tau) G_1$ where $p(X)$ and $q(X)$ are such that
\[
p(X) - v = (X-u) q(X),
\]
then
\[\begin{aligned}
 e(C - v G_1, G_2) & = e((p(\tau)-v)G_1, G_2) \\
 & = e((\tau-u) q(\tau) G_1, G_2) \\
 & = e(q(\tau) G_1, (\tau-u) G_2) \\
 & = e(\Pi, \tau G_2 - u G_2),
\end{aligned}\]
hence Eq. {{eqref: dlkzg_verif}} is satisfied and $\evalverif$ returns 1.

### Hiding Security

The DL-KZG scheme as described above is *not poly-hiding*: the commitment algorithm $\commit$ is deterministic, hence, given two polynomials $p_0$ and $p_1$ and a commitment $C$, it is trivial to distinguish whether $C$ commits to $p_0$ and $p_1$ by computing the corresponding commitments and comparing with $C$.

Regarding the eval-hiding property, note that DL-KZG cannot be statistically eval-hiding.
Indeed, an unbounded adversary can compute $\tau$ from the parameters and $p(\tau)$ from the commitment and return $(\tau,p(\tau))$ to win the EVAL-HIDING game without making any query to the $\orcl{Prove}$ oracle.
However, it is eval-hiding under the discrete logarithm assumption (in group $\GG_1$).

Let us give the intuition before the full-fledged proof.
As the committed polynomial is uniformly random in $\PR{r}{d}$, Lagrange interpolation ensures that given at most $d$ evaluations of $p$ at $u_1,\dots,u_d$, the value of $p$ on any other point $u \notin \{u_1,\dots,u_d\}$ is uniformly random so that even an unbounded adversary can guess it with probability at most $1/\abs{\FF_r}$.
Hence, the only way an adversary can guess $p(u)$ with non-negligible probability is to compute $p(\tau)$ from $C = p(\tau) G_1$.
Together with $d$ queries to the $\orcl{Prove}$ oracle, this yields $d+1$ evaluations of $p$, allowing to compute $p$ with Lagrange interpolation.
But computing $p(\tau)$ requires to solve the discrete logarithm problem for challenge $C$.

{{thm}}{thm:dlkzg_eval_hiding}
*Assume that the DL problem is hard in $\GG_1$ for $\pairingsetup$.
Then the DL-KZG scheme is (computationally) eval-hiding.
More precisely, for every adversary $\adv$ against the EVAL-HIDING game, there exists an adversary $\bdv$ for the DL problem running in time $t+O(\secpar d^2)$, where $t$ is the running time of $\adv$, and such that*
\[
 \advantage{\adv}{eval-hiding}(\secpar) \le \advantage{\bdv}{dl}(\secpar) + \frac{1}{2^{2\secpar-1}}.
\]

> *Proof.*
Let $\adv$ be an adversary against the eval-hiding property of DL-KZG.
Without loss of generality, we assume that $\adv$ makes exactly $d$ queries to the $\orcl{Prove}$ oracle.
We simply denote EH the EVAL-HIDING game.
Let also $E$ denote the event that $\adv$ queries the $\orcl{Prove}$ oracle on $\tau$.
By definition of the advantage, we have
\[\begin{aligned}
 \advantage{\adv}{eval-hiding}(\secpar) & = \pr{\text{EH} \rightarrow \pctrue} \\
 & = \pr{\text{EH} \rightarrow \pctrue \mid E} \pr{E} + \pr{\text{EH} \rightarrow \pctrue \mid \neg E} \pr{\neg E} \\
 & \le \pr{\text{EH} \rightarrow \pctrue \mid E} + \pr{\text{EH} \rightarrow \pctrue \mid \neg E} \pr{\neg E}.
\end{aligned}\]
Let us first show that the first term is negligible.
Just before $\adv$ returns its answer, $p$ has been evaluated on at most $d+1$ points: $\tau$ (when computing the commitment) and the $d$ queries $\{u_1,\dots,u_d\}$ of $\adv$ to the $\orcl{Prove}$ oracle.
Conditioned on $\adv$ querying $\orcl{Prove}$ on $\tau$ (i.e., $\tau \in \{u_1,\dots,u_d\}$), $p$ has been in fact evaluated on $d$ points before $\adv$ returns its output $(u,v)$.
Since $p$ is a random polynomial of degree $d$, the value of $p(u)$ conditioned on these at most $d$ evaluations (for any $u \neq u_1,\dots,u_d$) is uniformly random.
Hence, even a computationally unbounded adversary can guess $p(u)$ with probability at most $1/\abs{\FF_r}$, i.e.,
\[
 \pr{\text{EH} \rightarrow \pctrue \mid E} = \frac{1}{\abs{\FF_r}} \le \frac{1}{2^{2\secpar-1}}.
\]
Let us now upper bound the second term with a reduction.
We construct an adversary $\bdv$ that solves the DL problem by simulating game EH to $\adv$ as follows.
Let $C= c G_1 \in \GG_1$ be the DL instance that $\bdv$ must solve.
$\bdv$ draws $\tau \sample \FF_r$, computes
\[
 par \defeq (G_1, \tau G_1, \dots, \tau^d G_1, G_2, \tau G_2),
\]
and runs $\adv$ on input $(par,C)$.
Note that $C$ implicitly commits to a polynomial $p$ such that $p(\tau) = c$.
$\bdv$ simulates the $\orcl{Prove}$ oracle as follows: when $\adv$ queries the oracle on some field element $u \in \FF_r \setm \{\tau\}$, $\bdv$ draws $v \sample \FF_r$, computes a proof
\[
 \Pi \defeq \frac{1}{\tau-u}(C-vG_1)
\]
and returns $(v,\Pi)$.
The proof is valid because
\[\begin{aligned}
 e(\Pi, \tau G_2 - u G_2) & = e((\tau-u)^{-1}(C-vG_1), (\tau-u)G_2) \\
 & = e(C - vG_1, G_2)
\end{aligned}\]
and hence the verification equation {{eqref: dlkzg_verif}} is satisfied.
Note that $\bdv$ cannot answer this way if $\adv$ queries $\tau$ to $\orcl{Prove}$ since $p(\tau)$ is exactly the solution to its DL challenge.
In such a case (i.e., when $E$ happens), $\bdv$ simply aborts.
Conditioned on $E$ not happening, $p$ is sampled through the commitment evaluation $p(\tau) = c$ and the $d$ evaluations $\{(u_1,v_1),\dots,(u_d,v_d)\}$ corresponding to $\orcl{Prove}$ queries made by $\adv$, with $c$ and $v_1,\dots,v_d$ uniformly random and independent.
By Lagrange interpolation, this is equivalent to drawing the $d+1$ coefficients of $p$ uniformly at random and hence the EVAL-HIDING game is perfectly simulated.
If $\adv$ successfully returns a pair $(u,v)$ such that $p(u) = v$, then $\bdv$ can interpolate the $d$ evaluations corresponding to $\orcl{Prove}$ queries together with $(u,v)$ to recover polynomial $p$ and compute $p(\tau) = c$, which yields the solution to the DL challenge.
>
> Let DL be the discrete logarithm game played with $\bdv$.
Then
\[\begin{aligned}
 \advantage{\bdv}{dl}(\secpar) & = \pr{\text{DL} \rightarrow \pctrue} \\
 & = \pr{\text{DL} \rightarrow \pctrue \mid E} \pr{E} + \pr{\text{DL} \rightarrow \pctrue \mid \neg E} \pr{\neg E} \\
 & = \pr{\text{DL} \rightarrow \pctrue \mid \neg E} \pr{\neg E} \\
 & = \pr{\text{EH} \rightarrow \pctrue \mid \neg E} \pr{\neg E}
\end{aligned}\]
where for the last equality we used that conditioned on $\neg E$, games DL$^{\bdv}(\secpar)$ and EH$^{\adv}(\secpar)$ are identical.
Hence,
\[
 \advantage{\adv}{eval-hiding}(\secpar) \le \advantage{\bdv}{dl}(\secpar) + \frac{1}{2^{2\secpar-1}}
\]
$\bdv$ runs in time $t$ (where $t$ is the running time of $\adv$) plus the time to interpolate $p$, which requires at most $O(d^2\log_2(r)) = O(\secpar d^2)$ operations.

### Binding Security

The only thing that a commitment $C$ commits to, information-theoretically speaking, is the value $p(\tau)$.
Hence, the DL-KZG scheme is certainly not statistically poly-binding: an adversary able to compute $\tau$ from the public parameters can very easily decommit any commitment $C = cG_1$ to any polynomial $p$ such that $p(\tau)=c$.
However, for an adversary unable to compute $\tau$ from the public parameters, which is an instance of what we call the [$(d,1)$-co-DL problem](./games-models-and-assumptions.md#q1q2-co-discrete-logarithm-q1q2-co-dl), there is only a negligible chance that it can find two polynomials $p$ and $q$ such that $p(\tau) G_1 = q(\tau) G_1 = C$.
More formally, we have the following result.

{{thm}}
*Assume that the $(d,1)$-co-DL problem is hard for $\pairingsetup$.
Then the DL-KZG scheme for maximal degree $d$ is poly-binding.
More precisely, for any adversary $\adv$ against the poly-binding security of DL-KZG for maximal degree $d$, there exists an adversary $\bdv$ for the $(d,1)$-co-DL problem running in time $t+O(\secpar d^3)$, where $t$ is the running time of $\adv$, and such that*
\[
 \advantage{\adv}{poly-binding}(\secpar) = \advantage{\bdv}{(*d*,1)-co-dl}(\secpar).
\]

> *Proof.*
Let $\adv$ be an adversary against the poly-binding security of DL-KZG for maximal degree $d$.
We construct an algorithm $\bdv$ for the $(d,1)$-co-DL problem as follows.
$\bdv$ gets pairing group parameters $(\GG_1,\GG_2,\GG_t,r,e)$ and an instance $(G_1, \tau G_1, \dots, \tau^d G_1, G_2, \tau G_2)$ of the $(d,1)$-co-DL problem.
The goal of $\bdv$ is to compute $\tau$.
It runs $\adv$ on public parameters
\[
 par = (G_1, \tau G_1, \dots, \tau^d G_1, G_2, \tau G_2).
\]
Assume that $\adv$ is successful and returns a commitment $C$ and two distinct polynomials $p$ and $p'$ of degree at most $d$ such that
\[
 \polyverif(par,C,p) = \polyverif(par,C,p') = 1.
\]
This implies that $C = p(\tau) G_1 = p'(\tau) G_1$, hence $p(\tau) = p'(\tau)$ and $\tau$ is a root of the non-zero polynomial $(p-p')(X) \in \PR{r}{d}$.
This polynomial can be factored in time $O(d^3 \log(r))$ with the [Cantor–Zassenhaus algorithm](https://en.wikipedia.org/wiki/Cantor%E2%80%93Zassenhaus_algorithm), which allows $\bdv$ to compute all its roots and find $\tau$.
The success probability of $\bdv$ is the same as the one of $\adv$ and the running time of $\bdv$ is $t + O(\secpar d^3)$.

Eval-binding security relies on a stronger assumption, namely that the so-called [$(q_1,q_2)$-strong Diffie-Hellman](./games-models-and-assumptions.md#q1q2-strong-diffie-hellman-q1q2-sdh) ($(q_1,q_2)$-SDH) problem is hard.
This problem is as follows: given
\[
(G_1,x G_1, \dots x^{q_1} G_1,G_2, x G_2, \dots, x^{q_2} G_2) \in \GG_1^{q_1+1} \times \GG_2^{q_2+1},
\]
compute a pair $(a,Y) \in \FF_r \times \GG_1$ such that $Y = \frac{1}{x+a} G_1$.
The $(q,1)$-SDH problem is usually simply called the $q$-SDH problem.

{{thm}}{thm:dlkzg_eval_binding}
*Assume that the $d$-SDH problem is hard for $\pairingsetup$.
Then the DL-KZG scheme for maximal degree $d$ is eval-binding.
More precisely, for any adversary $\adv$ against the eval-binding security of DL-KZG for maximal degree $d$, there exists an adversary $\bdv$ for the $d$-SDH problem running in time similar to the time of $\adv$ and such that*
\[
 \advantage{\adv}{eval-binding}(\secpar) = \advantage{\bdv}{*d*-sdh}(\secpar).
\]

> *Proof.*
Let $\adv$ be an adversary against the eval-binding security of the DL-KZG scheme for maximal degree $d$.
We construct an adversary $\bdv$ for the $d$-SDH problem.
$\bdv$ gets pairing group parameters $(\GG_1,\GG_2,\GG_t,r,e)$ and an instance $(G_1, \tau G_1, \dots, \tau^d G_1, G_2, \tau G_2)$ of the $d$-SDH problem.
The goal of $\bdv$ is to return a pair $(a,Y)$ such that $Y = \frac{1}{\tau+a} G_1$.
It runs $\adv$ on public parameters
\[
 par = (G_1, \tau G_1, \dots, \tau^d G_1, G_2, \tau G_2).
\]
Assume that $\adv$ is successful and returns a commitment $C$, a field element $u \in \FF_r$, and two valid value/proof pairs $(v,\Pi)$ and $(v',\Pi')$ with $v \neq v'$.
Then $\bdv$ proceeds as follows.
First, it checks whether $u = \tau$ (e.g., by checking whether $u G_1$ is equal to the second group element of the parameters $par$).
If this is the case, then $\bdv$ simply picks an arbitrary element $a \in \FF_r \setm \{-\tau\}$ and returns $(a,\frac{1}{\tau+a}G_1)$.
From now on, we assume $u \neq \tau$.
The validity of the two proofs imply that
\[\begin{aligned}
 e(C-vG_1, G_2) & = e(\Pi, \tau G_2 - u G_2) \\
 \text{and } e(C-v'G_1, G_2) & = e(\Pi', \tau G_2 - u G_2).
\end{aligned}\]
Taking the inverse of the second equation and multiplying with the first equation, we get successively
\[\begin{aligned}
 e(C-vG_1, G_2) e(C-v'G_1, G_2)^{-1} & = e(\Pi, (\tau-u)G_2) e(\Pi', (\tau-u)G_2)^{-1} \\
 e(C-vG_1, G_2) e(-C+v'G_1, G_2) & = e(\Pi, (\tau-u)G_2) e(-\Pi', (\tau-u)G_2) \\
 e(C-vG_1-C+v'G_1, G_2) & = e(\Pi-\Pi', (\tau-u)G_2) \\
 e((v'-v)G_1, G_2) & = e(\Pi-\Pi', (\tau-u)G_2) \\
 e\left(\frac{1}{\tau-u}G_1, G_2\right) & = e\left(\frac{1}{v'-v}(\Pi-\Pi'), G_2\right),
\end{aligned}\]
where for the last implication we used that $v'-v \neq 0$ and $\tau-u \neq 0$, which allows us to multiply by $(v'-v)^{-1} (\tau-u)^{-1} \bmod r$.
The last equation implies that
\[
 Y \defeq \frac{1}{v'-v}(\Pi-\Pi') = \frac{1}{\tau-u}G_1.
\]
Hence, $\bdv$ returns $(-u,Y)$ which is a valid solution of the SDH instance.
The success probability of $\bdv$ is the same as the one of $\adv$ and the running time of $\bdv$ is close to the one of $\adv$.

## The Ped-KZG Scheme

### Description

As discussed in the previous section, the DL-KZG scheme is not poly-hiding because the $\commit$ algorithm is deterministic.
It is possible the make the scheme poly-hiding by randomizing the $\commit$ algorithm.
Below we present the Ped-KZG scheme.
The idea is to add to the basic DL-KZG commitment $C = p(\tau) G_1$ a commitment to another random and independent polynomial $\hat p(X)$ with respect to another generator $H_1$ of $\GG_1$.
The commitment becomes $p(\tau) G_1 + \hat p(\tau) H_1$, which is very similar to a [Pedersen commitment](./commitment-schemes.md#pedersen-commitments), hence the name.
This requires to extend the size of the public parameters and the evaluation proofs.
The formal description follows.

- The $\setup$ algorithm, on input the security parameter $\secparam$, runs
\[
 (\GG_1,\GG_2,\GG_t,r,e) \gets \pairingsetup(\secparam),
\]
draws random generators $G_1$ and $H_1$ of $\GG_1$ and $G_2$ of $\GG_2$, draws $\tau \sample \FF_r$, and returns public parameters
\[
 par \defeq (G_1, \tau G_1, \dots, \tau^d G_1, H_1, \tau H_1, \dots, \tau^d H_1, G_2, \tau G_2) \in \GG_1^{2d+2} \times \GG_2^2.
\]
Here we assume that pairing parameters $(\GG_1,\GG_2,\GG_t,r,e)$ are implicitly specified in $par$.
The field over which polynomials are defined is $\FF_r$.
The public parameters can be split into a commitment key
\[
 ck \defeq (G_1, \tau G_1, \dots, \tau^d G_1, H_1, \tau H_1, \dots, \tau^d H_1) \in \GG_1^{2d+2}
\]
and a verification key
\[
 vk \defeq (G_1, H_1, G_2, \tau G_2) \in \GG_1^2 \times \GG_2^2.
\]

- The $\commit$ algorithm, on input a commitment key $ck$ and a polynomial $p \in \PR{r}{d}$ where $p(X) = \sum_{i=0}^d a_i X^i$, draws a random polynomial $\hat p(X) = \sum_{i=0}^d \hat a_i X^i$ with $\hat a_0,\dots,\hat a_d \sample \FF_r$ and returns the commitment
\[
 C \defeq \sum_{i=0}^d a_i (\tau^i G_1) + \sum_{i=0}^d \hat a_i (\tau^i H_1) = p(\tau) G_1 + \hat p(\tau) H_1
\]
and the decommitment $D = \hat p(X)$.

- The $\polyverif$ algorithm, on input a commitment key $ck$, a commitment $C \in \GG_1$, a polynomial $p \in \PR{r}{d}$ where $p(X) = \sum_{i=0}^d a_i X^i$, and a decommitment $\hat p \in \PR{r}{d}$ where $\hat p(X) = \sum_{i=0}^d \hat a_i X^i$, returns 1 if $C = \sum_{i=0}^d a_i (\tau^i G_1) + \sum_{i=0}^d \hat a_i (\tau^i H_1)$ and 0 otherwise.

- The $\evalprove$ algorithm, on input a commitment key $ck$, a polynomial $p \in \PR{r}{d}$, a decommitment $\hat p \in \PR{r}{d}$, and $u \in \FF_r$, computes the polynomials
\[\begin{aligned}
 q(X) & \defeq \frac{p(X)-p(u)}{X-u} = \sum_{i=0}^d b_i X^i, \\
 \hat q(X) & \defeq \frac{\hat p(X)- \hat p(u)}{X-u} = \sum_{i=0}^d \hat b_i X^i,
\end{aligned}\]
the group element
\[
 \Pi \defeq \sum_{i=0}^d b_i (\tau^i G_1) + \sum_{i=0}^d \hat b_i (\tau^i H_1) = q(\tau) G_1 + \hat q(\tau) H_1,
\]
and returns $p(u)$ and the proof $(\hat p(u),\Pi)$.

- The $\evalverif$ algorithm, on input a verification key $vk = (G_1, H_1, G_2, \tau G_2)$, a commitment $C$, a pair $(u,v) \in \FF_r^2$, and a proof $(\hat v,\Pi)$, returns 1 if
\[
 e(C - v G_1 - \hat v H_1, G_2) = e(\Pi, \tau G_2 - u G_2) {{numeq}}{pedkzg_verif}
\]
and 0 otherwise.

Correctness can be verified in a similar way to DL-KZG.

### Hiding Security

Thanks to the commitment randomization, poly-hiding and eval-hiding security both hold statistically for Ped-KZG.

{{thm}}
*The Ped-KZG scheme is perfectly poly-hiding.*

> *Proof.*
For any $\tau \in \FF_r$ and any polynomial $p \in \PR{r}{d}$, the commitment $C$ returned by the $\commit$ algorithm is uniformly random in $\GG_1$ due to the addition of the term $\hat p(\tau)$.
Hence, the $\orcl{Commit}$ oracle in the POLY-HIDING game does not reveal any information about the hidden bit $b$.

{{thm}}
*The Ped-KZG scheme is statistically eval-hiding.
More precisely, for any adversary $\adv$, one has*
\[
 \advantage{\adv}{eval-hiding} \le \frac{1}{2^{2\secpar-1}}.
\]

> *Proof.*
Let $\adv$ by a (computationally unbounded) adversary against the eval-hiding property of Ped-KZG.
We can assume without loss of generality that $\adv$ is given $\tau$, the discrete logarithm $h$ of $H_1$ in base $G_1$, and the discrete logarithm $c = p(\tau) + h\hat p(\tau)$ of the challenge commitment $C$.
Let $u_i_$, $i \in \{1,\dots,d\}$ be the queries of $\adv$ to oracle $\orcl{Prove}$ and $(v_i, (\hat v_i, \Pi_i))$ be the corresponding answers.
Note that $\Pi_i$ does not bring any additional information to $\adv$ as it can be computed from the other quantities, namely
\[
 \Pi_i = \frac{1}{\tau-u_i}(C-v_i G_1-\hat v_i H_1).
\]
Hence, all in all the adversary is given $d$ evaluations of $p$ and $\hat p$ at the same points together with the value $c = p(\tau) + h\hat p(\tau)$.
Note that $h \neq 0$ since $H_1$ is a generator of $\GG_1$.
Hence, conditioned on $(u_i,v_i,\hat v_i)$ for $i \in \{1,\dots,d\}$ and $c$, the value of $p(\tau)$ is uniformly random and $\adv$ only has $d$ evaluations of $p$.
This implies that the probability that $\adv$ guesses $p(u)$ correctly for $u \notin \{u_1,\dots,u_d\}$ is $1/\abs{\FF_r} \le 1/2^{2\secpar-1}$.

### Binding Security

The poly-binding and eval-binding security properties hold under the same assumptions as for DL-KZG.
The proofs are slightly more complex and must account for the possibility that the adversary solves the discrete logarithm problem for $H_1$ in base $G_1$.

{{thm}}
*Assume that the $(d,1)$-co-DL problem is hard for $\pairingsetup$.
Then the Ped-KZG scheme for maximal degree $d$ is poly-binding.
More precisely, for any adversary $\adv$ against the poly-binding security of Ped-KZG for maximal degree $d$, there exists an adversary $\bdv$ for the $(d,1)$-co-DL problem running in time $t+O(\secpar d^3)$, where $t$ is the running time of $\adv$, and such that*
\[
 \advantage{\adv}{poly-binding}(\secpar) = 2 \cdot \advantage{\bdv}{(*d*,1)-co-dl}(\secpar).
\]

> *Proof.*
Let $\adv$ be an adversary against the poly-binding security of the Ped-KZG scheme for maximal degree $d$.
We construct an algorithm $\bdv$ for the $(d,1)$-co-DL problem as follows.
$\bdv$ gets pairing group parameters $(\GG_1,\GG_2,\GG_t,r,e)$ and an instance $(G_1, x G_1, \dots, x^d G_1, G_2, x G_2)$ of the $(d,1)$-co-DL problem.
The goal of $\bdv$ is to compute $x$.
>
> Adversary $\bdv$ randomly chooses between two indistinguishable ways to embed its instance into the parameters.
Namely, it draws $b \sample \bin$ and proceeds as follows depending on $b$:
> - If $b=0$, then $\bdv$ draws $h \sample \FF_r \setm \{0\}$ and runs $\adv$ on public parameters
\[
 par = (G_1, x G_1, \dots, x^d G_1, \alpha G_1, h (x G_1), \dots, h (x^d G_1), G_2, x G_2).
\]
This implicitly sets $\tau = x$ and $H_1 = h G_1$.
> - If $b=1$, then $\bdv$ draws $\tau \sample \FF_r$ and runs $\adv$ on public parameters
\[
 par = (G_1, \tau G_1, \dots, \tau^d G_1, x G_1, \tau (x G_1), \dots, \tau^d (x G_1), G_2, \tau G_2).
\]
This implicitly sets $H_1 = xG_1$.
>
> Assume that $\adv$ is successful and returns a commitment $C$ and two distinct polynomials $p$ and $p'$ of degree at most $d$ together with corresponding decommitments $\hat p$ and $\hat p'$ such that
\[
 \polyverif(par,C,p, \hat p) = \polyverif(par,C,p', \hat p') = 1.
\]
This implies that
\[
 C = p(\tau) G_1 + \hat p(\tau) H_1 = p'(\tau) G_1 + \hat p'(\tau) H_1
\]
and hence
\[
 (p(\tau)-p'(\tau)) G_1 + (\hat p(\tau) - \hat p'(\tau)) H_1 = 0.
\]
> We can distinguish two cases:
> - case $p(\tau) - p'(\tau) = 0$:
If $b=1$ then $\bdv$ aborts.
Otherwise, since $b=0$, we have $x=\tau$.
Hence, $x$ is a root of the non-zero polynomial $(p-p')(X) \in \PR{r}{d}$.
This polynomial can be factored in time $O(d^3 \log(r))$ with the [Cantor–Zassenhaus algorithm](https://en.wikipedia.org/wiki/Cantor%E2%80%93Zassenhaus_algorithm), which allows $\bdv$ to compute all its roots and find $x$.
> - case $p(\tau) - p'(\tau) \neq 0$:
If $b=0$ then $\bdv$ aborts.
Otherwise, since $b=1$ then $H_1 = x G_1$.
This implies that
\[
 p(\tau)-p'(\tau) + x(\hat p(\tau) - \hat p'(\tau)) = 0.
\]
Then necessarily $\hat p(\tau) - \hat p'(\tau) \neq 0$ as otherwise this would contradict $p(\tau)-p'(\tau) \neq 0$.
Hence, $\bdv$ can compute
\[
 x = \frac{p(\tau)-p'(\tau)}{\hat p'(\tau) - \hat p(\tau)}.
\]
>
> The view of $\adv$ is independent from $b$ and hence $\bdv$ aborts with probability $1/2$, so that
\[
 \advantage{\bdv}{(*d*,1)-co-dl}(\secpar) = \frac{1}{2} \advantage{\adv}{poly-binding}(\secpar).
\]
The running time of $\bdv$ is at most $t + O(\secpar d^3)$, which concludes the proof.

{{thm}}
*Assume that the $d$-SDH problem is hard for $\pairingsetup$.
Then the Ped-KZG scheme for maximal degree $d$ is eval-binding.
More precisely, for any adversary $\adv$ against the eval-binding security of Ped-KZG for maximal degree $d$, there exists an adversary $\bdv$ for the $d$-SDH problem running in time similar to the time of $\adv$ and such that*
\[
 \advantage{\adv}{eval-binding}(\secpar) = 2 \cdot \advantage{\bdv}{*d*-sdh}(\secpar).
\]

> *Proof.*
Let $\adv$ be an adversary against the eval-binding security of the Ped-KZG scheme for maximal degree $d$.
We construct an adversary $\bdv$ for the $d$-SDH problem.
$\bdv$ gets pairing group parameters $(\GG_1,\GG_2,\GG_t,r,e)$ and an instance $(G_1, x G_1, \dots, x^d G_1, G_2, x G_2)$ of the $d$-SDH problem.
The goal of $\bdv$ is to return a pair $(a,Y)$ such that $Y = \frac{1}{x+a} G_1$.
>
> Adversary $\bdv$ randomly chooses between two indistinguishable ways to embed its instance into the parameters.
Namely, it draws $b \sample \bin$ and proceeds as follows depending on $b$:
> - If $b=0$, then $\bdv$ draws $h \sample \FF_r \setm \{0\}$ and runs $\adv$ on public parameters
\[
 par = (G_1, x G_1, \dots, x^d G_1, \alpha G_1, h (x G_1), \dots, h (x^d G_1), G_2, x G_2).
\]
This implicitly sets $\tau = x$ and $H_1 = h G_1$.
> - If $b=1$, then $\bdv$ draws $\tau \sample \FF_r$ and runs $\adv$ on public parameters
\[
 par = (G_1, \tau G_1, \dots, \tau^d G_1, x G_1, \tau (x G_1), \dots, \tau^d (x G_1), G_2, \tau G_2).
\]
This implicitly sets $H_1 = xG_1$.
>
> Assume that $\adv$ is successful and returns a commitment $C$, a field element $u \in \FF_r$, and two valid value/proof pairs $(v,(w,\Pi))$ and $(v',(w',\Pi'))$ with $v \neq v'$.
Then $\bdv$ proceeds as follows.
First, it checks whether $u = \tau$ (e.g., by checking whether $u G_1$ is equal to the second group element of the parameters $par$).
If this is the case, then $\bdv$ simply picks an arbitrary element $a \in \FF_r \setm \{-\tau\}$ and returns $(a,\frac{1}{\tau+a}G_1)$.
From now on, we assume $u \neq \tau$.
The validity of the two proofs imply that
\[\begin{aligned}
 e(C-v G_1-w H_1, G_2) & = e(\Pi, \tau G_2 - u G_2) \\
 \text{and } e(C-v' G_1-w'H_1, G_2) & = e(\Pi', \tau G_2 - u G_2).
\end{aligned}\]
Combining these two equations, we get
\[\begin{aligned}
 e((v'-v)G_1+(w'-w)H_1, G_2) & = e(\Pi-\Pi', (\tau-u) G_2) \\
 e\left(\frac{v'-v}{\tau-u} G_1 + \frac{w'-w}{\tau-u}H_1, G_2\right) & = e(\Pi-\Pi',G_2),
\end{aligned}\]
where we used that $u \neq \tau$.
>
> We can now distinguish two cases:
> - case $\Pi \neq \Pi'$: If $b=1$ then $\bdv$ aborts.
Otherwise, since $b=0$, we have $\tau = x$ and $\bdv$ knows the value $h$ such that $H_1 = hG_1$.
The equation above yields
\[
 e\left(\frac{v'-v+h(w'-w)}{x-u} G_1, G_2\right) = e(\Pi-\Pi',G_2),
\]
This implies in particular that $v'-v+h(w'-w) \neq 0$ as otherwise this would imply $\Pi = \Pi'$.
Hence,
\[
 e\left(\frac{1}{x-u} G_1, G_2\right) = e\left(\frac{1}{v'-v+h(w'-w)}(\Pi-\Pi'),G_2\right)
\]
which implies that
\[
 Y \defeq \frac{1}{v'-v+h(w'-w)}(\Pi-\Pi') = \frac{1}{(x-u)}G_1
\]
Thus, $\bdv$ can return $(-u,Y)$ as solution to the $d$-SDH instance.
> - case $\Pi = \Pi'$: If $b=0$ then $\bdv$ aborts.
Otherwise, since $b=1$; we have $H_1 = x G_1$ and the equation above yields
\[
 e\left(\frac{v'-v + x(w'-w)}{\tau-u} G_1, G_2\right) = e(0,G_2),
\]
which implies $(v'-v) + x(w'-w) = 0$.
We cannot have $w=w'$ as this would imply $v = v'$ whereas $v \neq v'$ when $\adv$ is successful.
Hence, $\bdv$ can compute $x = (v'-v)(w'-w)^{-1}$, choose an arbitrary $a \in \FF_r \setm \{-x\}$, and return $(a, \frac{1}{x+a}G_1)$ as solution to the SDH instance.
>
> The view of $\adv$ is independent from $b$ and hence $\bdv$ aborts with probability $1/2$, so that
\[
 \advantage{\bdv}{*d*-sdh}(\secpar) = \frac{1}{2} \advantage{\adv}{eval-binding}(\secpar).
\]
The running time of $\bdv$ is similar to the running time of $\adv$, which concludes the proof.

## Discussion

### Efficiency

DL-KZG commitments are extremely succinct and rather cheap to verify: a commitment and a proof take one elliptic curve point each (e.g., 48 bytes when using BLS12-381) and verifying an opening essentially takes two pairings.
In case one has to verify many openings for the same commitment, the verification equation {{eqref: dlkzg_verif}} can be equivalently written
\[
 e(C, G_2) = e(G_1, G_2)^v e(\Pi, \tau G_2 - u G_2),
\]
where $e(C, G_2)$ and $e(G_1, G_2)$ can be computed once and stored for verifying multiple openings, allowing to trade one pairing for one exponentiation in $\GG_t$.
On the other hand, the size of the commitment key and the complexity of algorithms $\commit$ and $\evalprove$ are linear in $d$, the maximal degree of committed polynomials (which when building SNARKs can be quite large).

### Trusted Setup

The secret value $\tau$ drawn by the $\setup$ algorithm must be securely deleted once the commitment key has been set up as it allows to break the evaluation binding property of the scheme.
Indeed, knowing $\tau$, given an arbitrary commitment $C \in \GG_1$, one can open this commitment at any point $u \neq \tau$ to any value $v$ by computing the proof as $\Pi = (\tau-u)^{-1}(C-vG_1)$.
Then the verification equation {{eqref: dlkzg_verif}} is satisfied as
\[\begin{aligned}
 e(\Pi, \tau G_2 - u G_2) & = e((\tau-u)^{-1}(C-vG_1), (\tau-u)G_2) \\
 & = e(C - vG_1, G_2).
\end{aligned}\]

This is quite different from the $\setup$ procedure of the [Pedersen commitment scheme](./commitment-schemes.md#pedersen-commitments), for which it is possible to proceed without ever generating any trapdoor explicitly.
There is no (efficient) way known to implement the $\setup$ procedure for KZG without explicitly sampling $\tau$.
To the best of my knowledge, there is also no proof that this is impossible.
The *assumption* that this is impossible looks quite similar to many "knowledge of exponent" assumptions, hence the claim that running the KZG setup obliviously of $\tau$ is impossible is presumably true but not provable with known techniques.
It is, however, possible to run the setup in a decentralized fashion, ensuring that the process is secure as long as a single party behaves honestly (see for example [[NRBB22](../references.md#NRBB22)]).

Note that it is possible to check that the trusted setup yielded public parameters having the correct form, namely that there indeed exists $\tau \in \FF_r$ such that $par = (G_1, \tau G_1, \dots, \tau^d G_1, G_2, \tau G_2)$.
Say we are given $par = (P_0, P_1, \dots, P_d, G_2, H_2)$.
Let $\tau$ be defined as the discrete log of $H_2$ in base $G_2$, i.e., $H_2 = \tau G_2$, and set $G_1 = P_0$.
Then $ck$ has the correct form if and only if for every $i \in \{0,\dots,d-1\}$, $e(P_i,H_2) = e(P_{i+1},G_2)$.
Indeed, one has the following equivalences:
\[\begin{aligned}
 e(P_i,H_2) = e(P_{i+1},G_2) & \iff e(P_i,\tau G_2) = e(P_{i+1},G_2) \\
 & \iff e(\tau P_i,G_2) = e(P_{i+1},G_2) \\
 & \iff P_{i+1} = \tau P_i.
\end{aligned}\]

### Summary of KZG Properties

|              | DL-KZG | Ped-KZG |
|--------------|:-----------:|:---------:|
| param. size  | $(d+1)$ $\abs{\GG_1}$ + $2$ $\abs{\GG_2}$ | $(2d+2)$ $\abs{\GG_1}$ + $2$ $\abs{\GG_2}$ |
| comt. size   | $1$ $\abs{\GG_1}$ | $1$ $\abs{\GG_1}$ |
| proof size   | $1$ $\abs{\GG_1}$ | $1$ $\abs{\GG_1}$ + $1$ $\abs{\FF_r}$ |
| poly-hiding  | --- | perfect |
| eval-hiding  | DL in $\GG_1$ | perfect |
| poly-binding | $(d,1)$-co-DL | $(d,1)$-co-DL |
| eval-binding | $d$-SDH | $d$-SDH |


## Multi-evaluation Proofs

We will see that the DL-KZG scheme can be generalized to allow proving multiple evaluations with one single proof consisting of a single $\GG_1$ element.
This technique can also be applied to Ped-KZG but it is much less interesting since the size of the proof for $n$ evaluations is one $\GG_1$ element plus $n$ field elements, hence it grows linearly with $n$.

Recall that for a polynomial $p \in \PR{}{d}$, $p(u) = v$ is equivalent to $p(X)-v$ being divisible by $X-u$.
How does this generalize to multiple evaluations?

First, let us recall some vocabulary from the section about [Lagrange interpolation](../mathematical-preliminaries/polynomials.md#lagrange-interpolation).
An *evaluation domain* (or simply *domain*) of size $n$ is a subset $\cU \subset \FF$ of size $n$.
The *vanishing polynomial* over $\cU$ is the polynomial $z(X)$ defined as
\[
 z_{\cU}(X) \defeq \prod_{i=1}^n (X-u_i).
\]
A *multi-evaluation* of size $n$ is a subset $\cE = \{(u_1,v_1),\dots,(u_n,v_n)\} \subset \FF^2$ such that $u_i \neq u_j$ for $i \neq j$.
The evaluation domain associated with $\cE =\{(u_1,v_1),\dots,(u_n,v_n)\}$ is $\cU \defeq \{u_1,\dots,u_n\}$.
We say that a polynomial $p \in \FF[X]$ *satisfies* a multi-evaluation $\cE =\{(u_1,v_1),\dots,(u_n,v_n)\}$ if $p(u_i) = v_i$ for every $i \in \{1,\dots,n\}$.

The idea of multi-evaluation proofs relies on the {{tref: thm:generalized_polynomial_remainder}} that we restate here.
Let $p\in \FF[X]$, $\cE = \{(u_1,v_1),\dots,(u_n,v_n)\}$ be a multi-evaluation of size $n \le \deg(p)$, and $\cU \defeq \{u_1,\dots,u_n\}$.
Let $z(X)$ be the vanishing polynomial for domain $\cU$ and $\ell(X)$ be the Lagrange interpolation polynomial for $\cE$, i.e., the unique polynomial of degree at most $n-1$ such that $\ell(u_i) = v_i$ for every $i \in \{1,\dots,n\}$.
Then $p$ satisfies $\cE$ if and only if $z(X)$ divides $p(X) - \ell(X)$.

For $n=1$, one recovers the standard polynomial remainder theorem since for a single point $(u,v)$ the vanishing polynomial is $X-u$ and the Lagrange interpolation polynomial is the constant polynomial $\ell(X) = v$, hence $p$ satisfies $p(u) = v$ if and only if $X-u$ divides $p(X)-v$.

### Syntax and Security Definition

Let us now see how to adapt the syntax of a PC scheme to accommodate multi-evaluation proofs.
Concretely, a *PC scheme with multi-evaluation proofs* consists of five algorithms: $\setup$, $\commit$, and $\polyverif$ have the same syntax as for a standard PC scheme, while $\evalprove$ and $\evalverif$ are replaced respectively by the following two algorithms:

- a $\multiprove$ algorithm which on input parameters $par$, a polynomial $p \in \PR{}{d}$, a decommitment $D$, and a tuple $(u_1,\dots,u_n) \in \FF^n$ of $n$ distinct field elements, $n \le d$, returns a tuple $(v_1,\dots,v_n) \in \FF^n$ and a proof $\Pi$;

- a $\multiverif$ algorithm which on input parameters $par$, a commitment $C$, a multi-evaluation $\cE$, and a proof $\Pi$, returns 1 if $\Pi$ is a valid proof that the polynomial committed to by $C$ satisfies $\cE$ and 0 otherwise.

The correctness property can be straightforwardly adapted: for every security parameter $\secpar$, every $d \in \NN$, every $p \in \PR{}{d}$, every $n \in \{1,\dots,d\}$, and every subset $\{u_1,\dots,u_n\} \subset \FF$, the following game capturing the nominal execution of algorithms for multi-evaluation proving must return true with probability 1:

\[\boxed{\begin{aligned}
 & par \gets \setup(\secparam) \\
 & (C,D) \gets \commit(par, p) \\
 & ((v_1,\dots,v_n), \Pi) \gets \multiprove(par, p ,D, (u_1,\dots,u_n)) \\
 & \cE \defeq \{(u_1,v_1),\dots,(u_n,v_n)\} \\
 & b \gets \multiverif(par, C, \cE, \Pi) \\
 & \pcassert (b=1)
\end{aligned}}\]

We must modify the security definitions accordingly.
The poly-hiding and poly-binding notion are identical to the ones defined for a standard PC scheme.
The eval-hiding notion is very similar: one only needs to adapt the $\orcl{Prove}$ oracle so that it may be queried on domains of size larger than 1.

The eval-binding notion requires more care: we still want that no adversary can prove that a polynomial evaluates to two different values $v$ and $v'$ at the same input point $u$; however, now the adversary has the freedom to prove this for two different multi-evaluations $\cE$ and $\cE'$ with the constraint that $(u,v) \in \cE$ and $(u,v') \in \cE'$.
To emphasize the difference, we call this adapted security notion multi-binding.
It is defined via the following game.

\[
 \def\arraystretch{\myarraystretch}
 \boxed{
 \begin{array}{l}
  \text{\underline{Game MULTI-BINDING:}} \\
  par \gets \setup(\secparam) \\
  (C,(u,v,v'),(\cE,\Pi),(\cE',\Pi')) \gets \adv(par) \\
  b \gets \evalverif(par,C,\cE,\Pi) \\
  b' \gets \evalverif(par,C,\cE',\Pi') \\
  \pcassert (u,v) \in \cE \\
  \pcassert (u,v') \in \cE' \\
  \pcassert (v \neq v') \\
  \pcassert (b = 1) \\
  \pcassert (b' = 1)
 \end{array}
 }
\]

### KZG with Multi-evaluation Proofs: Description

The DL-KZG multi-evaluation PC scheme works as follows:

- The $\setup$ algorithm, on input the security parameter $\secparam$, runs
\[
 (\GG_1,\GG_2,\GG_t,r,e) \gets \pairingsetup(\secparam),
\]
draws random generators $G_1$ and $G_2$ of respectively $\GG_1$ and $\GG_2$, draws $\tau \sample \FF_r$, and returns public parameters
\[
 par \defeq (G_1, \tau G_1, \dots, \tau^d G_1, G_2, \tau G_2, \dots, \tau^d G_2) \in \GG_1^{d+1} \times \GG_2^{d+1}.
\]

- The $\commit$ and $\polyverif$ algorithms are defined exactly [as for DL-KZG](#description).

- The $\multiprove$ algorithm, on input a commitment key $ck = (G_1, \tau G_1, \dots, \tau^d G_1)$, a polynomial $p \in \PR{r}{d}$, and a subset $\cU = \{u_1,\dots,u_n\} \subset \FF_r$ of size $n \in \{1,\dots,d\}$, computes the polynomials
\[\begin{aligned}
 z(X) & \defeq \prod_{i=1}^n (X-u_i), \\
 \ell(X) & \defeq \sum_{i=1}^n p(u_i) \prod_{\substack{1 \le j \le n \\ j \neq i}} \frac{X-u_j}{u_i-u_j}, \\
 q(X) & \defeq \frac{p(X) - \ell(X)}{z(X)} = \sum_{i=0}^d b_i X^i,
\end{aligned}\]
and the group element $\Pi \defeq \sum_{i=0}^d b_i (\tau^i G_1) = q(\tau) G_1$ and returns $(p(u_1),\dots,p(u_n))$ and the proof $\Pi$.

- The $\multiverif$ algorithm, on input a verification key $vk = (G_2, \tau G_2, \dots, \tau^d G_2)$, a commitment $C$, a multi-evaluation $\cE = \{(u_1,v_1),\dots,(u_n,v_n)\}$ of size $n \in \{1,\dots,d\}$, and a proof $\Pi$, computes the polynomials
\[\begin{aligned}
 z(X) & \defeq \prod_{i=1}^n (X-u_i), \\
 \ell(X) & \defeq \sum_{i=1}^n v_i \prod_{\substack{1 \le j \le n \\ j \neq i}} \frac{X-u_j}{u_i-u_j},
\end{aligned}\]
and returns 1 if
\[
 e(C-\ell(\tau)G_1, G_2) = e(\Pi, z(\tau)G_2)
\]
and 0 otherwise.

Observe that the $\multiverif$ algorithm must compute $\ell(\tau)G_1$ and $z(\tau) G_2$.
For a multi-evaluation of size $n$, $\ell$ has degree at most $n-1$ and $z$ has degree $n$.
Hence, if proofs for multi-evaluations of size at most $N$ are to be supported, one can restrict the public parameters to
\[
 par = (G_1, \tau G_1, \dots, \tau^d G_1, G_2, \tau G_2, \dots, \tau^N G_2) \in \GG_1^{d+1} \times \GG_2^{N+1}
\]
and derive a commitment key
\[
 ck = (G_1, \tau G_1, \dots, \tau^d G_1) \in \GG_1^{d+1}
\]
and a verification key
\[
 vk = (G_1, \tau G_1, \dots, \tau^{N-1} G_1, G_2, \tau G_2, \dots, \tau^N G_2) \in \GG_1^N \times \GG_2^{N+1}.
\]

### Security Proof

The proof of {{ref: thm:dlkzg_eval_binding}} can be adapted to show that DL-KZG is multi-binding under a slightly different assumption called [$(q_1,q_2)$-bilinear strong Diffie-Hellman](./games-models-and-assumptions.md#q1q2-bilinear-strong-diffie-hellman-q1q2-bsdh) ($(q_1,q_2)$-BSDH).
This problem is as follows: given
\[
(G_1,x G_1, \dots x^{q_1} G_1,G_2, x G_2, \dots, x^{q_2} G_2) \in \GG_1^{q_1+1} \times \GG_2^{q_2+1},
\]
compute a pair $(a,Y) \in \FF_r \times \GG_t$ such that $Y = e(G_1,G_2)^{\frac{1}{x+a}}$.
Note that
\[
 (q_1,q_2)\text{-BSDH} \leqq (q_1,q_2)\text{-SDH}.
\]
Indeed, given a solution $(a,\frac{1}{x+a}G_1)$ for some SDH instance, one can compute a solution $(a,e(\frac{1}{x+a}G_1,G_2))$ for the corresponding BSDH instance.
The converse, though, is not known to hold, so that BSDH is presumably a stronger assumption than SDH.

{{thm}}
*Assume that the $(d,N)$-BSDH problem is hard for $\pairingsetup$.
Then the DL-KZG multi-evaluation scheme for maximal degree $d$ and multi-evaluations of size at most $N$ is multi-binding.
More precisely, for any adversary $\adv$ against the multi-binding security of DL-KZG, there exists an adversary $\bdv$ for the $(d,N)$-BSDH problem running in time similar to the time of $\adv$ and such that*
\[
 \advantage{\adv}{multi-binding}(\secpar) = \advantage{\bdv}{*(d,N)*-bsdh}(\secpar).
\]

> *Proof.*
Let $\adv$ be an adversary against the multi-binding security of the DL-KZG scheme for maximal degree $d$.
We construct an adversary $\bdv$ for the $(d,N)$-BSDH problem.
$\bdv$ gets pairing group parameters $(\GG_1,\GG_2,\GG_t,r,e)$ and an instance
\[
 (G_1, \tau G_1, \dots, \tau^d G_1, G_2, \tau G_2, \dots, \tau^N G_2)
\]
of the $(d,N)$-BSDH problem.
The goal of $\bdv$ is to return a pair $(a,Y)$ such that $Y = e(G_1,G_2)^{\frac{1}{\tau+a}}$.
Adversary $\bdv$ runs $\adv$ on input parameters
\[
 par = (G_1, \tau G_1, \dots, \tau^d G_1, G_2, \tau G_2, \dots, \tau^N G_2).
\]
Assume that $\adv$ returns a commitment $C$, a tuple $(u,v,v') \in \FF_r^3$, and two valid multi-evaluations/proof pairs $(\cE,\Pi)$ and $(\cE',\Pi')$ such that $v \neq v'$, $(u,v) \in \cE$, and $(u,v') \in \cE'$.
If $u = \tau$ (which $\bdv$ can verify by checking whether $u G_1$ is equal to the second group element of the parameters $par$), then $\bdv$ simply picks an arbitrary element $a \in \FF_r \setm \{-\tau\}$ and returns $(a,e(G_1,G_2)^{\frac{1}{\tau+a}})$ as solution to the $d$-BSDH instance.
From now on, we assume that $u \neq \tau$.
>
> Let $\cU$, resp. $\cU'$ be the evaluation domain corresponding to $\cE$, resp. $\cE'$.
Let also $z$, resp. $z'$ be the vanishing polynomial for $\cU$, resp. $\cU'$ and $\ell$, resp. $\ell'$ be the Lagrange interpolation polynomial for $\cE$, resp. $\cE'$.
Validity of the two proofs imply that
\[\begin{aligned}
 e(C-\ell(\tau)G_1, G_2) & = e(\Pi, z(\tau)G_2) \\
 \text{and } e(C-\ell'(\tau)G_1, G_2) & = e(\Pi', z'(\tau)G_2).
\end{aligned}\]
Combining these two equations, we obtain
\[\begin{aligned}
 e((\ell'-\ell)(\tau)G_1, G_2) = e(\Pi, z(\tau)G_2)e(\Pi', z'(\tau)G_2)^{-1}.
\end{aligned}\]
We know that $u$ is a root of both $z(X)$ and $z'(X)$.
Hence, there are polynomials $q$ and $q'$ such that $z(X) = (X-u)q(X)$ and $z'(X) = (X-u)q'(X)$.
We also know that polynomial $(\ell'-\ell)(X)$ evaluates to $v'-v$ at $u$.
Hence, by the {{tref: thm:polynomial_remainder}}, there is a polynomial $q''$ such that
\[
 (\ell'-\ell)(X) = v'-v + (X-u) q''(X).
\]
Note that $\bdv$ can explicitly compute $q$, $q'$, and $q''$.
Injecting this in the previous equation, we get
\[\begin{aligned}
 & e((v'-v)G_1,G_2)e((\tau-u)q''(\tau)G_1,G_2) = e(\Pi, (\tau-u)q(\tau)G_2)e(\Pi', (\tau-u)q'(\tau)G_2)^{-1} \\
 & e((v'-v)G_1,G_2) = e(\Pi, (\tau-u)q(\tau)G_2)e(\Pi', (\tau-u)q'(\tau)G_2)^{-1}e((\tau-u)q''(\tau)G_1,G_2)^{-1} \\
 & e(G_1,G_2)^{1/(\tau-u)} = \big(e(\Pi, q(\tau)G_2)e(-\Pi', q'(\tau)G_2)e(-q''(\tau)G_1,G_2)\big)^{1/(v'-v)},
\end{aligned}\]
where for the last equality we used that $\tau-u \neq 0$ and $v'-v \neq 0$.
Hence, $\bdv$ can return $(-u,Y)$, where $Y$ is the right-hand side of the last equation, as solution to the $(d,N)$-BSDH instance.

As a sanity check, observe that for a single evaluation ($n=1$), one has $q(X) = q'(X) = 1$ and $q''(X) = 0$, in which case the last equation simplifies to
\[
 e\left(\frac{1}{\tau-u}G_1, G_2\right) = e\left(\frac{1}{v'-v}(\Pi-\Pi'), G_2\right)
\]
which allows to solve the $d$-SDH problem and recover {{ref: thm:dlkzg_eval_binding}}.

## A Practical Use Case

Ethereum is planning to use the KZG polynomial commitment scheme for [proto-danksharding](https://notes.ethereum.org/@vbuterin/proto_danksharding_faq).
Its properties make it a convenient solution to the [data](https://hackmd.io/@vbuterin/sharding_proposal) [availability](https://notes.ethereum.org/@vbuterin/r1v8VCULP) [problem](https://www.paradigm.xyz/2022/08/das).
A distributed trusted setup [is being run at the time of writing](https://ceremony.ethereum.org/).

## Additional Resources

There are many resources explaining KZG out there, here are a few:

- [Section 15.2 of PAZK](https://people.cs.georgetown.edu/jthaler/ProofsArgsAndZK.pdf#section.15.2)
- [this post](https://scroll.io/blog/kzg) by Andy Arditi
- [this other one](https://dankradfeist.de/ethereum/2020/06/16/kate-polynomial-commitments.html) by Dankrad Feist
- [yet another one](https://alinush.github.io/2020/05/06/kzg-polynomial-commitments.html) by Alin Tomescu
- or [this video](https://www.youtube.com/watch?v=J4pVTamUBvU) by Dan Boneh.
