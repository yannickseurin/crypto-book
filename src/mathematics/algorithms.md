# Algorithms


## Scalar Multiplication

Let $\GG$ be a group of order $n$ denoted additively for which we assume that we have an implementation of the group operation given as a black box.
Let $G \in \GG$ be a group element and $m \in \NN$ be a positive integer for which we want to compute $mG$.
First, we can assume that $0 \le m \le n-1$, otherwise we simply reduce $m$ modulo $n$ since $nG = 0$.
We let $[m]_2 = b_0 \dots b_k$ denote the binary representation of $m$ such that
\[
 m = \sum_{i=0}^k b_i 2^i.
\]
We can see this expression of $m$ as the evaluation of polynomial $\sum_{i=0}^k b_iX^i$ at point $2$.
Using Horner's rule, we can rewrite this as
\[
 m = b_0 +2 \Big(b_1 + 2 \big(b_2 + \cdots + 2(b_{k-1}+2b_k) \cdots \big)\Big).
\]

\[
 \def\arraystretch{\myarraystretch}
 \boxed{
 \begin{array}{l}
  \underline{\text{double-and-add}(G, m):} \\
  b_0 \dots b_k \defeq [m]_ 2 \\
  D \defeq G \quad \pclinecomment{tracks $2^i G$} \\
  P \defeq 0 \quad \pclinecomment{output} \\
  \pcfor i = 0, \dots, k-1 \pcdo \\
   \t \pcif b_i = 1 \pcthen \\
    \t\t P \defeq P+D \\
   \t D \defeq 2D \\
  \pcif b_k = 1 \pcthen \\
   \t P \defeq P+D \\
  \pcreturn P
 \end{array}
 }
\]