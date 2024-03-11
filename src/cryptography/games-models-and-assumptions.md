> **Chapter status:** in progress
>
> **TODO:**
> - modify advantage notation to display the scheme

# Games, Models, and Assumptions

This section presents how security assumptions and security properties of cryptographic schemes are formalized.
It also gives a list of cryptographic assumptions relevant for this book (see also the [ECRYPT II MAYA report](https://www.ecrypt.eu.org/ecrypt2/documents/D.MAYA.6.pdf)).

## Contents

<!-- toc -->


## Big-O Notation and Negligible Functions

In all the following, we consider functions from $\NN$ to $\RR^+ = \{x \in \RR \mid x \ge 0\}$ and we let $\secpar$ denote the variable.

A function $f$ is said to be ***negligible*** if $f \in \secpar^{-\omega(1)}$ or equivalently if
\[
 \forall k \in \NN, \exists n \in \NN, \forall \secpar \ge n, f(\secpar) \le \secpar^{-c}.
\]

In words, $f$ is negligible if it approaches zero faster than the inverse of any polynomial.

The set of all negligible functions is denoted $\negl$.
We often write $f(\secpar) = \negl(\secpar)$ in place of $f \in \negl$.

## Cryptographic Games

### Game: Tentative Definition

A *game* consists of a main algorithm $\game$ and a (potentially empty) finite tuple of *oracle* algorithms $\orcl{O}_1, \dots, \orcl{O}_n$.
Then main algorithm (to which we simply refer as the game from here) takes as input $\secparam$ where $\secpar \in \NN$ is an integer called *security parameter* and runs in three phases:
- *initialization*: the game initializes variables and generates some input $inp$;
- *attack*: the game invokes an algorithm $\adv$ called *adversary* on input $inp$; the adversary has oracle access to $\orcl{O}_1, \dots, \orcl{O}_n$;
- *finalization*: when the adversary halts and returns some output $out$, the game evaluates a predicate of $out$ and all variables and returns the truth value of this predicate.

To make this definition rigorous, the programming language used to write the game should be completely specified.
Here, we will content ourselves with specifying games in pseudocode.


A very simple game called ADD drawing two random $\secpar$-bit integers and returning $\pctrue$ if the adversary successfully adds them would look like this:

\[
 \def\arraystretch{\myarraystretch}
 \boxed{
 \begin{array}{l}
  \text{\underline{Game ADD$(\secparam)$:}} \\
  a, b \sample \{2^{\secpar-1},\dots,2^{\secpar}-1\} \\
  c \gets \adv(a,b) \\
  \pcreturn (c = a+b)
 \end{array}
 }
\]

### Notation and Conventions for Writing Games

- Given a predicate $A$, we use $\pcassert A$ as a shorthand for $\pcif \neg A \pcthen\ \pcreturn \pcfalse$
- Games return $\pctrue$ by default, meaning if the end of the game code is reached and the game has not returned yet, then it returns $\pctrue$. Finalization often consists in returning the truth vale of $A \wedge B \wedge C \wedge \cdots$. Under this convention, the following finalization code
\[
 \pcreturn A \wedge B \wedge C
\]
is equivalent to
\[\begin{aligned}
 & \pcassert A \\
 & \pcassert B \\
 & \pcassert C \\
\end{aligned}\]

### Advantage

For each security parameter $\secpar$, a game together with an adversary define a finite probability space specified by all random choices made by the game and the random coins of the adversary.
This allows to define the probability of various events related to the execution of the game with a specific adversary.

In particular, given a game $\game$ and an adversary $\adv$, we write $\game^{\adv}(\secparam) \Rightarrow b$, or simply $\game \Rightarrow b$ when the context is clear, for the event that an execution of $\game$ with adversary $\adv$ for security parameter $\secpar$ returns $b$.
When the game returns $\pctrue$ we also say that the adversary "wins".


A game can be *computational* or *decisional* depending on how a quantity called *advantage*, measuring how well an adversary performs at winning the game, is defined.

The advantage of an adversary $\adv$ against a game $\game$ is a function of the security parameter $\secpar \in \NN^*$ into $[0,1]$ defined as
\[
 \advantage{\adv}{game}(\secpar) \defeq \pr{\game \Rightarrow \pctrue}
\]
if the game is computational and
\[
 \advantage{\adv}{game}(\secpar) \defeq \abs{2\pr{\game \Rightarrow \pctrue}-1}
\]
if the game is decisional.
(We write the name of the game in small caps in the advantage superscript to lighten notation.)

We say that a game $\game$ is *computationally hard* if for every probabilistic polynomial-time (ppt) algorithm $\adv$,
\[
 \advantage{\adv}{game}(\secpar) = \negl(\secpar).
\]
We say that a game $\game$ is *statistically hard* (or *information-theoretically hard* or *unconditionally hard*) if for every algorithm $\adv$ (not necessarily polynomial-time),
\[
 \advantage{\adv}{game}(\secpar) = \negl(\secpar).
\]
In the special case where the advantage of any algorithm is zero, one says that the game is *perfectly hard* (for example, a [commitment scheme](./commitment-schemes.md) that can be perfectly hiding or perfectly binding).

When we simply say that a game is hard, it usually means computationally hard (but this should always be clear from the context).

Hence, for a computationally, resp. statistically hard game, every ppt, resp. unbounded adversary "wins" with probability negligible close to 0 for a computational game and with probability negligibly close to $1/2$ for a decisional game.

### Reductions

Given two games $X$ and $Y$, we say that $X$ *reduces* to $Y$, denoted $X \leqq Y$, if there exists a probabilistic polynomial-time algorithm $\bdv$ (called *reduction*) with access to an oracle such that for every algorithm $\adv$ solving $Y$ with non-negligible advantage, $\bdv^{\adv}$ solves $X$ with non-negligible advantage.

If $X$ reduces to $Y$ and $Y$ reduces to $X$, we say that $X$ and $Y$ are *equivalent*, denoted $X \equiv Y$.

{{prop}}{prop:reduction}
*Assume that $X$ reduces to $Y$.
Then $X$ being hard implies $Y$ being hard.*

``` admonish proof collapsible=true
Contraposing, assume that $Y$ is not hard, which by definition means that there exists a ppt algorithm $\adv$ such that
\[
 \advantage{\adv}{Y}(\secpar) \neq \negl(\secpar).
\]
Consider the reduction $\bdv$ from $X$ to $Y$.
Since $\adv$ and $\bdv$ both run in polynomial time, $\bdv^\adv$ runs in polynomial time as well.
Moreover, by definition of a reduction,
\[
 \advantage{\bdv^{\adv}}{X}(\secpar) \neq \negl(\secpar).
\]
Hence, $X$ is not hard.
```

Thus, $X \leqq Y$ can be read as "$X$ is not harder than $Y$" or $Y$ is at least as hard as $X$".

In cryptography, we are constantly making assumptions of the form "X is hard".
{{ref: prop:reduction}} can be used to compare the *strength* of various assumptions.
Indeed, assuming we proved that $X \leqq Y$, then the assumption that $X$ is hard implies that $Y$ is hard too.
If, in addition, there are some indications that $Y \leqq X$ does not hold, then the assumption that $X$ is hard is *stronger* than the assumption that $Y$ is hard.
Indeed, if $X \leqq Y$ but $Y \leqq X$ is not known to hold, then it might be that $Y$ is hard yet $X$ is easy.

For example, consider the discrete logarithm (DL) problem on one hand (given a group $\GG$ and group elements $G$ and $X=xG$, compute $x$) and the Computational Diffie-Hellman (CDH) problem on the other hand (given a group $\GG$ and group elements $G$, $X=xG$, and $Y$, compute $xY$).
One can easily prove that CDH $\leqq$ DL (CDH reduces to DL) by constructing a reduction from CDH to DL: given an algorithm solving DL, one can solve CDH by first computing the discrete logarithm $x$ of $X$ and then computing $xY$.
However, there is no proof that DL $\leqq$ CDH (except in very specific situations).
Hence, the assumption that CDH is hard is (for most groups) stronger than the assumption that DL is hard.

Another way {{ref: prop:reduction}} is often used in cryptography is for security proofs.
Here, $X$ is some hardness assumption such as DL and $Y$ is a security game, say unforgeability (in the precise sense of EUF-CMA security in the random oracle model) of Schnorr signatures.
Then, a security proof for Schnorr signatures consists in proving that $X$ reduces to $Y$, i.e., the DL problem reduces to the EUF-CMA security of Schnorr signatures in the ROM.
By {{ref: prop:reduction}}, if DL is hard, then Schnorr signatures are EUF-CMA secure in the ROM.

## Idealized Models

### The Random Oracle Model

### The Generic Group Model

### The Algebraic Group Model

## Assumptions

### Group Setup Algorithms

A *standard group setup algorithm* is an algorithm $\groupsetup$ which on input the security parameter $\secparam$ returns a pair $(\GG,p)$ where $p$ is a $2\secpar$-bit prime and $\GG$ is a cyclic group of order $p$.

A *pairing group setup algorithm* is an algorithm $\pairingsetup$ which on input the security parameter $\secparam$ returns a tuple $(\GG_1, \GG_2, \GG_t, r, e)$ where $r$ is a $2\secpar$-bit prime, $\GG_1$, $\GG_2$, and $\GG_t$ are cyclic groups of order $r$, and $e \colon \GG_1 \times \GG_2 \to \GG_t$ is an efficiently computable pairing.

We adopt the convention that group/pairing setup algorithms do not return generators of the groups.
They will be explicitly sampled in the games.

One usually distinguishes three types of pairing group setup algorithms [[GPS08](../references.md#GPS08)]:

- a type-1 pairing group setup algorithm (also called *symmetric* pairing setup algorithm) is such that $\GG_1 = \GG_2$;
- a type-2 pairing group setup algorithm is such that $\GG_1 \neq \GG_2$ and there exists an efficiently computable isomorphism $\psi \colon \GG_2 \to \GG_1$;
- a type-3 pairing group setup algorithm is such that $\GG_1 \neq \GG_2$ an no efficiently computable isomorphism $\psi \colon \GG_2 \to \GG_1$ is known.

Type-2 and type-3 pairing group setup algorithms are called *asymmetric*.

In all the following, we simply talk about "type-1/2/3 pairings" rather than "type-1/2/3 pairing group setup algorithms".

### Assumptions in Standard Groups

#### Discrete Logarithm (DL)

- type: computational
- interactive: no
- falsifiable: yes
- references:
- notes:

\[
 \def\arraystretch{\myarraystretch}
 \boxed{
 \begin{array}{l}
  \text{\underline{Game DL:}} \\
  par \gets \groupsetup(\secparam) \\
  (\GG, p) \parse par \\
  G \sample \GG \setm \{0\} \\
  X \sample \GG \\
  x \gets \adv(par, G, X) \\
  \pcassert (X = xG)
 \end{array}
 }
\]

#### Computational Diffie-Hellman (CDH)

- type: computational
- interactive: no
- falsifiable: yes
- references:
- notes: CDH $\leqq$ DL

\[
 \def\arraystretch{\myarraystretch}
 \boxed{
 \begin{array}{l}
  \text{\underline{Game CDH:}} \\
  par \gets \groupsetup(\secparam) \\
  (\GG, p) \parse par \\
  G \sample \GG \setm \{0\} \\
  x \sample \ZZ_p; X \defeq xG \\
  Y \sample \GG \\
  Z \gets \adv(par, G, X, Y) \\
  \pcassert (Z = xY)
 \end{array}
 }
\]

#### Decisional Diffie-Hellman (DDH)

- type: decisional
- interactive: no
- falsifiable: yes
- references:
- notes: DDH $\leqq$ CDH

\[
 \def\arraystretch{\myarraystretch}
 \boxed{
 \begin{array}{l}
  \text{\underline{Game DDH:}} \\
  par \gets \groupsetup(\secparam) \\
  (\GG, p) \parse par \\
  b \sample \bin \\
  G \sample \GG \setm \{0\} \\
  x \sample \ZZ_p; X \defeq xG \\
  Y \sample \GG \\
  Z_0 \defeq xY; Z_1 \sample \GG \\
  b' \gets \adv(par, G, X, Y, Z_b) \\
  \pcassert (b = b')
 \end{array}
 }
\]

#### $q$-Discrete Logarithm ($q$-DL)

- type: computational
- interactive: no
- falsifiable: yes
- references: [[Che10](../references.md#Che10), [Lip10](../references.md#Lip10), [FKL18](../references.md#FKL18), [Rot22](../references.md#Rot22)]
- notes:
    - sometimes called *$q$-strong DL* or *DL with $q$ auxiliary inputs*
    - 1-Dl = DL
    - $(q+1)$-DL $\leqq$ $q$-DL

\[
 \def\arraystretch{\myarraystretch}
 \boxed{
 \begin{array}{l}
  \text{\underline{Game $q$-DL:}} \\
  par \gets \groupsetup(\secparam) \\
  (\GG, p) \parse par \\
  G \sample \GG \setm \{0\} \\
  x \sample \ZZ_p \\
  x' \gets \adv(par, G, x G, x^2 G, \dots, x^q G) \\
  \pcassert (x=x')
 \end{array}
 }
\]

### Assumptions in Product Groups

The assumptions listed in this section are defined for a pair of groups $(\GG_1,\GG_2)$ of equal order $r$.
They are usually applied to groups returned by a pairing group setup algorithm $\pairingsetup$ but don't make use of the group $\GG_t$ nor the pairing $e$.
For this reason, we simply write $(\GG_1,\GG_2,r) \parse par$ when parsing the parameters $par$ returned by $\pairingsetup$.

#### $(q_1,q_2)$-co-Discrete Logarithm ($(q_1,q_2)$-co-DL)

- type: computational
- interactive: no
- falsifiable: yes
- references: [[Che10](../references.md#Che10), [Lip10](../references.md#Lip10), [FKL18](../references.md#FKL18), [Rot22](../references.md#Rot22)]
- notes:

\[
 \def\arraystretch{\myarraystretch}
 \boxed{
 \begin{array}{l}
  \text{\underline{Game $(q_1,q_2)$-co-DL:}} \\
  par \gets \pairingsetup(\secparam) \\
  (\GG_1, \GG_2, r) \parse par \\
  G_1 \sample \GG_1 \setm \{0\} \\
  G_2 \sample \GG_2 \setm \{0\} \\
  x \sample \ZZ_r \\
  x' \gets \adv(par, G_1, x G_1, \dots, x^{q_1} G_1, G_2, x G_2, \dots, x^{q_2} G_2) \\
  \pcassert (x = x')
 \end{array}
 }
\]

#### Computational co-Diffie-Hellman (co-CDH)

- type: computational
- interactive: no
- falsifiable: yes
- references: [[BLS04](../references.md#BLS04)]
- notes:
    - co-CDH $\leqq$ DL in $\GG_2$
    - co-CDH $\equiv$ CDH in $\GG_1 = \GG_2$ for type-1 pairings
    - co-CDH $\leqq$ CDH in $\GG_1$ for type-2 pairings (see {{ref: prop:cocdh_implies_cdh_in_g1}})

\[
 \def\arraystretch{\myarraystretch}
 \boxed{
 \begin{array}{l}
  \text{\underline{Game co-CDH:}} \\
  par \gets \pairingsetup(\secparam) \\
  (\GG_1, \GG_2, r) \parse par \\
  G_2 \sample \GG_2 \setm \{0\} \\
  x \sample \ZZ_r; X \defeq xG_2 \\
  Y \sample \GG_1 \\
  Z \gets \adv(par, G_2, X, Y) \\
  \pcassert (Z = xY)
 \end{array}
 }
\]

#### Computational co-Diffie-Hellman\* (co-CDH\*)

- type: computational
- interactive: no
- falsifiable: yes
- references: [[CHKM10](../references.md#CHKM10)]
- notes:
    - sometimes simply called (confusingly) co-CDH (e.g. [[BDN18](../references.md#BDN18)])
    - co-CDH$^*$ $\leqq$ CDH in $\GG_1$
    - co-CDH$^*$ $\leqq$ DL in $\GG_2$
    - co-CDH$^*$ $\leqq$ co-CDH
    - co-CDH$^*$ $\equiv$ CDH in $\GG_1 = \GG_2$ for type-1 pairings (see {{ref: prop:cocdhstar_equiv_cdh}})
    - co-CDH$^*$ $\equiv$ co-CDH for type-2 pairings (see {{ref: prop:cocdhstar_equiv_cocdh}})

\[
 \def\arraystretch{\myarraystretch}
 \boxed{
 \begin{array}{l}
  \text{\underline{Game co-CDH$^*$:}} \\
  par \gets \pairingsetup(\secparam) \\
  (\GG_1, \GG_2, r) \parse par \\
  G_1 \sample \GG_1 \setm \{0\} \\
  G_2 \sample \GG_2 \setm \{0\} \\
  x \sample \ZZ_r; X_1 \defeq xG_1; X_2 \defeq xG_2 \\
  Y \sample \GG_1 \\
  Z \gets \adv(par, G_1, X_1, G_2, X_2, Y) \\
  \pcassert (Z = xY)
 \end{array}
 }
\]

#### Computational $\psi$-co-Diffie-Hellman ($\psi$-co-CDH)

- type: computational
- interactive: yes
- falsifiable: no (for type-3 pairings)
- references: [[SV07](../references.md#SV07), [BDN18](../references.md#BDN18)]
- notes:
    - $\psi$-co-CDH $\leqq$ CDH in $\GG_1$
    - $\psi$-co-CDH $\leqq$ DL in $\GG_2$
    - $\psi$-co-CDH $\leqq$ co-CDH$^*$ $\leqq$ co-CDH
    - $\psi$-co-CDH $\equiv$ co-CDH for type-2 pairings (the isomorphism $\psi$ enables to compute $\orcl{SolveCoCDH}(U)$ efficiently as $\psi(U)$, assuming $\psi(G_2) = G_1$)

\[
 \def\arraystretch{\myarraystretch}
 \boxed{
 \begin{array}{ll}
  \text{\underline{Game $\psi$-co-CDH:}} & \qquad \text{\underline{Oracle $\orcl{SolveCoCDH}(U)$:}} \\
  par \gets \pairingsetup(\secparam) & \qquad \pclinecomment{solves co-CDH for $(G_2,U,G_1)$} \\
  (\GG_1, \GG_2, r) \parse par & \qquad \pcassert U \in \GG_2 \\
  G_1 \sample \GG_1 \setm \{0\} & \qquad \pcfor u \in \ZZ_r \pcdo \\
  G_2 \sample \GG_2 \setm \{0\} & \qquad \t \pcif uG_2 = U \pcthen \\
  x \sample \ZZ_r; X_1 \defeq xG_1; X_2 \defeq xG_2 & \qquad \t\t \pcreturn uG_1 \\
  Y \sample \GG_1 & \\
  Z \gets \adv^{\orcl{SolveCoCDH}}(par, G_1, X_1, G_2, X_2, Y) & \\
  \pcassert (Z = xY) &
 \end{array}
 }
\]

#### $(q_1,q_2)$-Strong Diffie-Hellman ($(q_1,q_2)$-SDH)

- type: computational
- interactive: no
- falsifiable: yes
- references: [[BB08](../references.md#BB08)]
- notes:
    - $(q,1)$-SDH usually called $q$-SDH
    - not to be confused with another assumption named SDH introduced in [[ABR01](../references.md#ABR01)]
    - $(q_1+1,q_2)$-SDH $\leqq$ $(q_1,q_2)$-SDH
    - $(q_1,q_2+1)$-SDH $\leqq$ $(q_1,q_2)$-SDH

\[
 \def\arraystretch{\myarraystretch}
 \boxed{
 \begin{array}{l}
  \text{\underline{Game $(q_1,q_2)$-SDH:}} \\
  par \gets \pairingsetup(\secparam) \\
  (\GG_1, \GG_2, r) \parse par \\
  G_1 \sample \GG_1 \setm \{0\} \\
  G_2 \sample \GG_2 \setm \{0\} \\
  x \sample \ZZ_r \\
  (a,Y) \gets \adv(par, G_1, x G_1, \dots, x^{q_1} G_1, G_2, x G_2, \dots, x^{q_2} G_2) \\
  \pcassert (Y = \frac{1}{x+a} G_1)
 \end{array}
 }
\]

### Assumptions in Pairing Groups

#### $(q_1,q_2)$-Bilinear Diffie-Hellman Inversion ($(q_1,q_2)$-BDHI)

- type: computational
- interactive: no
- falsifiable: yes
- references: [[BB04](../references.md#BB04)]
- notes:
    - $(q_1+1,q_2)$-BDHI $\leqq$ $(q_1,q_2)$-BDHI
    - $(q_1,q_2+1)$-BDHI $\leqq$ $(q_1,q_2)$-BDHI

\[
 \def\arraystretch{\myarraystretch}
 \boxed{
 \begin{array}{l}
  \text{\underline{Game $(q_1,q_2)$-BSDH:}} \\
  par \gets \pairingsetup(\secparam) \\
  (\GG_1, \GG_2, \GG_t, r, e) \parse par \\
  G_1 \sample \GG_1 \setm \{0\} \\
  G_2 \sample \GG_2 \setm \{0\} \\
  x \sample \ZZ_r \\
  Y \gets \adv(par, G_1, x G_1, \dots, x^{q_1} G_1, G_2, x G_2, \dots, x^{q_2} G_2) \\
  \pcassert (Y = e(G_1,G_2)^\frac{1}{x})
 \end{array}
 }
\]

#### $(q_1,q_2)$-Bilinear Strong Diffie-Hellman ($(q_1,q_2)$-BSDH)

- type: computational
- interactive: no
- falsifiable: yes
- references: [[KZG10a](../references.md#KZG10a)]
- notes:
    - $(q,1)$-BSDH is usually simply called $q$-BSDH
    - $(q_1+1,q_2)$-BSDH $\leqq$ $(q_1,q_2)$-BSDH
    - $(q_1,q_2+1)$-BSDH $\leqq$ $(q_1,q_2)$-BSDH
    - $(q_1,q_2)$-BSDH $\leqq$ $(q_1,q_2)$-SDH
    - $(q_1,q_2)$-BSDH $\leqq$ $(q_1,q_2)$-BDHI

\[
 \def\arraystretch{\myarraystretch}
 \boxed{
 \begin{array}{l}
  \text{\underline{Game $(q_1,q_2)$-BSDH:}} \\
  par \gets \pairingsetup(\secparam) \\
  (\GG_1, \GG_2, \GG_t, r, e) \parse par \\
  G_1 \sample \GG_1 \setm \{0\} \\
  G_2 \sample \GG_2 \setm \{0\} \\
  x \sample \ZZ_r \\
  (a,Y) \gets \adv(par, G_1, x G_1, \dots, x^{q_1} G_1, G_2, x G_2, \dots, x^{q_2} G_2) \\
  \pcassert (Y = e(G_1,G_2)^\frac{1}{x+a})
 \end{array}
 }
\]