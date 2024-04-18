# Signatures: Generalities

In this chapter, we define the syntax of a signature scheme and explore security definitions, asking ourselves: what makes a signature scheme secure?

## Syntax

A signature scheme consists of four algorithms:

- a $\setup$ algorithm which takes as input the security parameter $\secparam$ and returns public parameters $par$;
- a key generation algorithm $\keygen$ which takes as input parameters $par$ and returns a secret key $sk$ and a public key $pk$;
- a signature algorithm $\sign$ which takes as input parameters $par$, a secret key $sk$, and a message $m$ and returns a signature $\sigma$;
- a verification algorithm $\verif$ which takes as input parameters $par$, a public key $pk$, a message $m$, and a signature $\sigma$ and returns $1$ if the signature is valid for the pair $(pk,m)$ and $0$ otherwise.

The scheme is correct if for every security parameter $\secpar$ and every message $m$, the following game capturing the nominal execution of algorithms returns true with probability one:

\[\boxed{\begin{aligned}
 & par \gets \setup(\secparam) \\
 & (sk,pk) \gets \keygen(par) \\
 & \sigma \gets \sign(par,sk,m) \\
 & b \gets \verif(par,pk,m,\sigma) \\
 & \pcassert (b=1)
\end{aligned}}\]

This syntax considers that the message space is $\bin^*$, meaning that the signing algorithm takes any string as input message.
In practice, this is never the case: quite often, the signing algorithm starts with hashing the input message with a hash function that has a finite message space.
More generally, the message space could depend on the public parameters returned by the setup algorithm (e.g., the public parameters could specify a group and admissible messages could be restricted to group elements.)
This can be accommodated by adapting the syntax as follows: on input $(par, sk, m)$, the $\sign$ algorithms returns either a signature $\sigma$ or a distinguished error symbol $\bot$ indicating the message is invalid (i.e., cannot be signed).
Correctness must be modified to require that $b=1$ only if $\sigma \neq \bot$.
Other variables ($par$, $sk$, $pk$, and $\sigma$) usually have a specific format depending on the security parameter.
Similarly, we assume that $\keygen$, $\sign$, and $\verif$ return an error symbol in case any input does not abide to the expected format.

## EUF-CMA, the Standard Security Notion

The standard security notion for a signature scheme is *existential unforgeability against chosen message attacks* (EUF-CMA): no polynomial-time adversary, being given a target public key $pk^*$ and having access to a signature oracle for the corresponding secret key $sk^*$, should be able to compute a valid signature for a message it has not queried to the signature oracle, except with negligible probability.
This is formally captured by the following security game:

\[
 \def\arraystretch{\myarraystretch}
 \boxed{
 \begin{array}{cc}
  \begin{array}{l}
   \text{\underline{Game EUF-CMA:}} \\
   par \gets \setup(\secparam) \\
   (sk^\ast,pk^\ast) \gets \keygen(par) \\
   \cQ \defeq \emptyset \\
   (m^\ast, \sigma^\ast) \gets \adv^{\orcl{Sign}}(par,pk^\ast) \\
   \pcassert (m^\ast \notin \cQ) \\
   \pcassert (\verif( par,pk^\ast,m^\ast,\sigma^\ast)=1)
  \end{array}
  &
  \begin{array}{l}
   \text{\underline{Oracle $\orcl{Sign}(m)$:}} \\
   \sigma \gets \sign(par,sk^\ast,m) \\
   \cQ \gets \cQ \cup \{m\} \\
   \pcreturn \sigma \\ \\ \\ \\
  \end{array}
 \end{array}
 }
\]

Here, *chosen message attack* means that the adversary can query the signature oracle on messages of its choice, while *existential unforgeability* means that the adversary wins if it returns a forgery on *any* message that has not been queried to the signature oracle (there *exists* some message $m^*$ for which the adversary can forge a signature).

One can weaken this security definition in two directions.
One one hand, one can restrict how the adversary can access the signature oracle.
For example, in a *no message attack*, the adversary does not have access to the signature oracle.
In a *known message attack*, the adversary cannot choose the message when querying the signature oracle (the message is randomly drawn according to some specific probability distribution).
On the other hand, one can modify how the adversary can choose the message for which it forges its message.
For example, *selective unforgeability* requires the adversary to commit to the message for which it will forge at the beginning of the game, before receiving the target public key and making any query to the signature oracle.
For *universal unforgeability*, the message is imposed by the game rather than chosen by the adversary (typically, it is drawn at random by the game according to some specific distribution).

All these weaker notions are mostly of theoretical interest (for example, there exists generic conversion methods to construct signature schemes that are EUF-CMA-secure from schemes that are only meet *selective* unforgeability against chosen message attacks).
In practice, a signature scheme must be EUF-CMA-secure to be deployed (more precisely, conjectured EUF-CMA-secure, preferably supported by a security proof).

## Strong EUF-CMA and Non-malleability

## Binding