## Initial Inspection

First, let us try to define more precisely the goal of the puzzle just from the instructions, without looking into the code yet.

### Notation

Given a group $\GG$ of order $q$, we will write vectors of scalars and vectors of group elements in bold font, e.g. $\bfa = (a_0, \dots, a_{n-1}) \in (\ZZ_q)^n$ and $\bfG = (G_0, \dots, G_{n-1}) \in \GG^n$.
For two vectors $\bfa = (a_0, \dots, a_{n-1}) \in (\ZZ_q)^n$ and $\bfb = (b_0, \dots, b_{n-1}) \in (\ZZ_q)^n$, their scalar product is defined as
\[
 \langle \bfa, \bfb \rangle \defeq \sum_{i=0}^{n-1} a_i b_i \bmod q.
\]
Similarly, given $\bfa = (a_0, \dots, a_{n-1}) \in (\ZZ_q)^n$ and $\bfG = (G_0,\dots, G_{n-1}) \in \GG^n$, we let
\[
 \langle \bfa, \bfG \rangle \defeq \sum_{i=0}^{n-1} a_i G_i.
\]

### Definition of the Proof System

Let us first try to specify the language for which the proof system is designed.
We assume that the cyclic group $\GG$ of prime order $q$ and generators $\bfG = (G_0, \dots, G_{n-1})$ and $H$ are agreed upon by the prover and the verifier.
Then, from the puzzle's description (and also from the code, as we will see), the language $\cL$ of interest is defined by the relation
\[
 \cR = \{(x, w) \mid x = (C_a, \bfb) \in \GG \times (\ZZ_q)^n, w = (\bfa, \alpha) \in \GG^n \times \ZZ_q,
 C_a = \langle \bfa, \bfG \rangle + \alpha H\}.
\]

Recall that the language $\cL$ defined by this relation consists of all instances $x$ such that there exists a witness $w$ such that $(x,w) \in \cR$.

The protocol itself goes as follows:

\[
 \def\arraystretch{\myarraystretch}
 \boxed{
 \begin{array}{lcl}
  \text{Prover} & & \text{Verifier} \\
  \hline
  \text{parameters: } (\GG,q,\bfG,H) & & \text{parameters: } (\GG,q,\bfG,H) \\
  \text{instance: } (C_a, \bfb) \in \GG \times (\ZZ_q)^n & & \text{instance: } (C_a, \bfb) \in \GG \times (\ZZ_q)^n \\
  \text{witness: } (\bfa, \alpha) \in \GG^n \times \ZZ_q & & \\
  \text{relation: } C_a = \langle \bfa, \bfG \rangle + \alpha H & & \\
  \hline
  \mathbf{r} \sample (\ZZ_q)^n & & \\
  \rho, \tau, \nu \sample \ZZ_q & & \\
  C_r \defeq \langle \mathbf{r}, \bfG \rangle + \rho H & & \\
  C_1 \defeq \langle \bfa, \bfb \rangle G_1 + \tau H & & \\
  C_2 \defeq \langle \mathbf{r}, \bfb \rangle G_1 + \nu H & & \\
  & \xrightarrow{\displaystyle \ C_r, C_1, C_2 \ } \\
  & & \gamma \sample \ZZ_q \\
  & \xleftarrow{\displaystyle \qquad \gamma \qquad} & \\
  \bfs \defeq \bfa + \gamma \bfr \bmod q & & \\
  u \defeq \alpha + \gamma \rho \bmod q & & \\
  t \defeq \tau + \gamma \nu \bmod q & & \\
  & \xrightarrow{\displaystyle \quad \bfs, u, t \quad} \\
  & & \text{check that} \\
  & & \langle \bfs, \bfG \rangle + u H = C_a + \gamma C_r \\
  & & \langle \bfs, \bfb \rangle G_1 + t H = C_1 + \gamma C_2
 \end{array}
 }
\]

The protocol is complete, meaning that for any instance $(C_a, \bfb) \in \cL$, an honestly generated proof is always accepted by the verifier.
This holds because
\[\begin{aligned}
 \langle \bfs, \bfG \rangle + u H & = \langle \bfa + \gamma \bfr, \bfG \rangle + (\alpha + \gamma \rho) H \\
 & = (\langle \bfa, \bfG \rangle + \alpha H) + \gamma (\langle \bfr, \bfG \rangle + \rho H) \\
 & = C_a + \gamma C_r
\end{aligned}\]
and
\[\begin{aligned}
 \langle \bfs, \bfb \rangle G_1 + t H & = \langle \bfa + \gamma \bfr, \bfb \rangle G_1 + (\tau + \gamma \nu) H \\
 & = (\langle \bfa, \bfb \rangle G_1 + \tau H) + \gamma (\langle \mathbf{r}, \bfb \rangle G_1 + \nu H) \\
 & = C_1 + \gamma C_2.
\end{aligned}\]

### Proof of Knowledge

One can also check that the proof system is extractable, which follows from a property called special soundness: for any instance in language $\cL$, given two accepting transcripts with the same commitments $(C_r, C_1, C_2)$ but different challenges $\gamma^{(1)}$ and $\gamma^{(2)}$, one can compute a witness.
Indeed, let
\[\begin{aligned}
 & ((C_r,C_1,C_2), \gamma^{(1)}, (\bfs^{(1)},u^{(1)},t^{(1)})), \\
 & ((C_r,C_1,C_2), \gamma^{(2)}, (\bfs^{(2)},u^{(2)},t^{(2)}))
\end{aligned}\]
be two accepting transcripts for some instance $(C_a,\bfb)$ such that $\gamma^{(1)} \neq \gamma^{(2)}$.
Let
\[\begin{aligned}
 \bfa \defeq \frac{\gamma^{(1)}\bfs^{(2)} - \gamma^{(2)}\bfs^{(1)}}{\gamma^{(1)}-\gamma^{(2)}}, \\
 \alpha \defeq \frac{\gamma^{(1)}u^{(2)} - \gamma^{(2)}u^{(1)}}{\gamma^{(1)}-\gamma^{(2)}}.
\end{aligned}\]
Since both transcripts are accepting, we have
\[\begin{aligned}
 & \langle \bfs^{(1)}, \bfG \rangle + u^{(1)} H = C_a + \gamma^{(1)} C_r, \\
 & \langle \bfs^{(2)}, \bfG \rangle + u^{(2)} H = C_a + \gamma^{(2)} C_r.
\end{aligned}\]
Then
\[\begin{aligned}
 \langle \bfa, \bfG \rangle + \alpha H & = \frac{1}{\gamma^{(1)}-\gamma^{(2)}} \left(\gamma^{(1)}\langle \bfs^{(2)}, \bfG \rangle - \gamma^{(2)}\langle \bfs^{(1)}, \bfG \rangle + \gamma^{(1)}u^{(2)}H - \gamma^{(2)}u^{(1)}H \right) \\
 & = \frac{1}{\gamma^{(1)}-\gamma^{(2)}} \left(\gamma^{(1)}(C_a + \gamma^{(2)}C_r-u^{(2)}H) - \gamma^{(2)}(C_a + \gamma^{(1)}C_r-u^{(1)}H) + \gamma^{(1)}u^{(2)}H - \gamma^{(2)}u^{(1)}H \right) \\
 & = C_a.
\end{aligned}\]
Hence, the relation defining the language is satisfied and $(\bfa,\alpha)$ is indeed a witness that the instance $(C_a, \bfb)$ is in $\cL$.

### Zero-Knowledge?

The puzzle instructions asks us to find $\bfa$, which is part of the witness, from a few proofs generated by Bob.
If the protocol is zero-knowledge, this shouldn't be possible.
Is it zero-knowledge, though, from a theoretical point of view?

The answer is yes.
Namely, the interactive protocol as defined above can be shown to be honest-verifier zero-knowledge.
The simulator, whose task is to generate a fake transcript distributed as a true transcript between a prover and an honest verifier without knowing the witness $(\bfa, \alpha)$, works as follows:

\[\begin{aligned}
 & \bfs \sample (\ZZ_q)^n, u \sample \ZZ_q, t \sample \ZZ_q \\
 & \gamma \sample \ZZ_q \\
 & C_r \defeq \gamma^{-1}(\langle \bfs, \bfG \rangle + u H - C_a) \\
 & C_1 \sample \GG \\
 & C_2 \defeq \gamma^{-1}(\langle \bfs, \bfb \rangle G_1 + t H - C_1) \\
 & \pcreturn ((C_r,C_1,C_2), \gamma, (\bfs,u,t))
\end{aligned}\]

### Something Strange

There is something odd with the way we described the proof system: vector $\bfb$ plays no role in the relation
\[
 \cR = \{(x, w) \mid x = (C_a, \bfb) \in \GG \times (\ZZ_q)^n, w = (\bfa, \alpha) \in \GG^n \times \ZZ_q,
 C_a = \langle \bfa, \bfG \rangle + \alpha H\}.
\]
In particular, note that the proof that the system is extractable did not use the second equation checked by the verifier (meaning extractability still holds even if this check is omitted).

For this reason, it would make more sense to actually include $C_1$, the commitment to the inner product $\langle \bfa, \bfb \rangle$, in the instance and randomness $\tau$ in the witness and to define the relation as
\[
 \begin{split}
  \cR' = \{(\stat, \wit) \mid \stat = (C_a, C_1, \bfb) \in \GG \times \GG \times (\ZZ_q)^n, \wit = (\bfa, \alpha, \tau) \in \GG^n \times \ZZ_q \times \ZZ_q,\\
  C_a = \langle \bfa, \bfG \rangle + \alpha H, C_1 = \langle \bfa, \bfb \rangle G_1 + \tau H\}.
 \end{split}
\]

By doing this, the protocol would now be specified as follows:

\[
 \def\arraystretch{\myarraystretch}
 \boxed{
 \begin{array}{lcl}
  \text{Prover} & & \text{Verifier} \\
  \hline
  \text{parameters: } (\GG,q,\bfG,H) & & \text{parameters: } (\GG,q,\bfG,H) \\
  \text{instance: } & & \text{instance: } \\
  (C_a, C_1, \bfb) \in \GG \times \GG \times (\ZZ_q)^n & & (C_a, C_1, \bfb) \in \GG \times \GG \times (\ZZ_q)^n \\
  \text{witness: } & & \\
  (\bfa, \alpha, \tau) \in \GG^n \times \ZZ_q \times \ZZ_q & & \\
  \text{relation: } & & \\
  C_a = \langle \bfa, \bfG \rangle + \alpha H & & \\
  C_1 = \langle \bfa, \bfb \rangle G_1 + \tau H & & \\
  \hline
  \mathbf{r} \sample (\ZZ_q)^n & & \\
  \rho, \nu \sample \ZZ_q & & \\
  C_r \defeq \langle \mathbf{r}, \bfG \rangle + \rho H & & \\
  C_2 \defeq \langle \mathbf{r}, \bfb \rangle G_1 + \nu H & & \\
  & \xrightarrow{\displaystyle \quad C_r, C_2 \quad } \\
  & & \gamma \sample \ZZ_q \\
  & \xleftarrow{\displaystyle \qquad \gamma \qquad} & \\
  \bfs \defeq \bfa + \gamma \bfr \bmod q & & \\
  u \defeq \alpha + \gamma \rho \bmod q & & \\
  t \defeq \tau + \gamma \nu \bmod q & & \\
  & \xrightarrow{\displaystyle \quad \bfs, u, t \quad} \\
  & & \text{check that} \\
  & & \langle \bfs, \bfG \rangle + u H = C_a + \gamma C_r \\
  & & \langle \bfs, \bfb \rangle G_1 + t H = C_1 + \gamma C_2
 \end{array}
 }
\]

However, this is not important for solving the puzzle and we will stick to the original, albeit unnatural, description.
