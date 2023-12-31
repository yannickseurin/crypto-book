> **Chapter status:** in progress
>
> **TODO:**

# Polynomials

In this chapter, we cover general results about polynomials with coefficients in a ring.

We recall some abbreviations from the [chapter about rings](./rings.md):

- UCR stands for *unitary commutative ring*,
- PID stands for *principal ideal domain*,
- UFD stands for *unique factorization domain*.

## Contents

<!-- toc -->

## Generalities

Let $\RR$ be a ring.
A (univariate) ***polynomial*** with coefficients in $\RR$ is an infinite sequence $(a_i)_{i \in \NN}$ such that $a_i=0$ for all but a finite number of indices $i$.
Polynomials are traditionally denoted
\[
 p(X) = a_0 + a_1 X + a_2 X^2 + \cdots = \sum_{i=0}^{\infty} a_i X^i
\]
where $X$ is a symbol called ***indeterminate***.

The set of all univariate polynomials over $\RR$ is denoted $\RR[X]$:
\[
 \RR[X] \defeq \left\{ \sum_{i=0}^\infty a_i X^i : a_i \in \RR, \exists N \in \NN, \forall i \ge N, a_i = 0 \right\}.
\]

Polynomials can be added:
\[
 \sum_{i=0}^\infty a_i X^i + \sum_{i=0}^\infty b_i X^i = \sum_{i=0}^\infty (a_i+b_i) X^i.
\]

Polynomials can also be multiplied:
\[
 \sum_{i=0}^\infty a_i X^i \cdot \sum_{j=0}^\infty b_j X^j = \sum_{k=0}^\infty c_k X^k \quad \text{with} \quad c_k = \sum_{i+j=k} a_i b_j.
\]

{{prop}}
*Let $\RR$ be a ring.
Then $\RR[X]$ equipped with operations $+$ and $\cdot$ as defined above is a ring.
If $\RR$ is commutative, then so is $\RR[X]$ and if $\RR$ has a unity 1, then the constant polynomial $p(X) = 1$ is the unity of $\RR[X]$.*

The set $\RR[X]$ is called the ***polynomial ring*** in $X$ over $\RR$.
If we embed $\RR$ into $\RR[X]$ by identifying $a \in \RR$ with the constant polynomial $p(X) = a$, then $\RR$ is a subring of $\RR[X]$.

The ***degree*** of a polynomial $p(X)$ is the largest power of $X$ occurring in $p(X)$ with a non-zero coefficient, with the convention that polynomial $0$ has degree $-\infty$.
Hence, $p(X) = \sum_{i=0}^n a_i X^i$ has degree $n$ provided $a_n \neq 0$.
We let $\deg(p)$ denote the degree of a polynomial $p$ and $\RR^{\le n}[X]$ denote the set of polynomials over $\RR$ of degree at most $n$.
The ***leading term*** of $p(X)$ is its highest degree term $a_n X^n$ and the ***leading coefficient*** is $a_n$.
If the leading coefficient is 1 then the polynomial is said to be ***monic***.

We will often drop the indeterminate from the notation, simply writing "polynomial $p$", keeping it mostly when writing the polynomial coefficients explicitly as in $p(X) = \sum_{i=0}^n a_i X^i$.

{{prop}}
*Let $\RR$ be a ring and let $p$ and $q$ be two polynomials in $\RR[X]$.
Then
\[\begin{aligned}
 & \deg(p+q) \le \max \left\{ \deg(p), \deg(q) \right\} \\
 & \deg(pq) \le \deg(p) + \deg(q).
\end{aligned}\]*

> *Proof.*
This follows straightforwardly from the definition of addition and multiplication in $\RR[X]$.

Note that when $\RR$ has [zero divisors](./rings.md#zero-divisors-and-integral-domains), then one might have $\deg(pq) < \deg(p) + \deg(q)$ if the leading terms of $p$ and $q$ are $a X^n$ and $b X^m$ with $ab=0$.

When the coefficients are in an integral domain, we have additional properties.

{{prop}}{prop:pol_ring_over_id}
*Let $\DD$ be an integral domain.
Then*
- *$\DD[X]$ is an integral domain;*
- *for any polynomials $p,q \in \DD[X]$, $\deg(pq) = \deg(p) + \deg(q)$;*
- *the units of $\RR[X]$ are the constant polynomials $p(X) = u$ for $u \in \RR^*$.*

> *Proof.*
Let $p$ and $q$ be two non-zero polynomials in $\DD[X]$.
If $p$ has leading term $a X^n$ and $q$ has leading term $b X^m$ with $a \neq 0$ and $b \neq 0$, then $pq$ has leading term $ab X^{n+m}$ with $ab \neq 0$ since $\DD$ is an integral domain.
Hence, $pq$ is not the zero polynomial.
This also shows that $\deg(pq) = \deg(p) + \deg(q)$.
Clearly, for any unit $u \in \RR^*$, the constant polynomial $p(X) = u$ is a unit with inverse the constant polynomial $u^{-1}$.
Conversely, if polynomials $p$ and $q$ are such that $pq = 1$, then (by the second point) necessarily $\deg(p) = \deg(q) = 0$, i.e., $p$ and $q$ are constant polynomials, and by definition these constants are units of $\RR$.

## Divisibility in Polynomial Rings

All definitions regarding divisibility that we gave for general rings apply to polynomial rings.
We restate these definitions here for convenience.

In all the following, we restrict ourselves to the case where coefficients are in an integral domain $\DD$.

Given two polynomials $a$ and $b$ in $\DD[X]$, we say that $b$ ***divides*** $a$, or that $b$ is a ***factor*** of $a$, or that $a$ is a ***multiple*** of $b$, denoted $b \divides a$, if there exists a polynomial $q \in \RR[X]$ such that $a(X) = q(X)b(X)$.

{{prop}}{prop:degree_divisor}
*Let $\DD$ be an integral domain and $a \in \DD[X]$ be a non-zero polynomial.
Then for every $b \in \DD[X]$, $b \divides a \Rightarrow \deg(b) \le \deg(a)$.*

> *Proof.*
Let $b$ be a polynomial dividing $a$.
By definition, there exists $q \in \DD[X]$ such that $a = qb$.
Since $\DD$ is an integral domain, by {{ref: prop:pol_ring_over_id}},
\[
 \deg(a) = \deg(q) + \deg(b).
\]
Note that $q$ cannot be the zero polynomial as this would imply $a = 0$, hence $\deg(q) \ge 0$ and thus $\deg(a) \ge \deg(b)$.

It is easy to see that this proposition does not hold when the coefficient ring has zero divisors: for example, over $\ZZ_4$,
\[
 (2X^2+1)(2X^2+1) = 1
\]
and hence $2X^2+1 \divides 1$.

Two polynomials $a$ and $b$ are said to be ***associates*** if $a \divides b$ and $b \divides a$.
By {{ref: prop:associates}}, over an integral domain, $a$ and $b$ are associates if and only if there exists $u \in \DD^*$ such that $a(X) = u b(X)$.

In general, $\DD[X]$ might not be [Euclidean](./rings.md#euclidean-domains) (as we will see shortly, this holds if and only if $\DD$ is a field).
However, one can perform division with remainder for polynomials as soon as the leading coefficient of the divisor is a unit.

{{prop}}{prop:polynomial_division}
*Let $\DD$ be an integral domain.
Then for every polynomials $a,b \in \DD[X]$ such that the leading coefficient of $b$ is in $\DD^*$, there exists unique polynomials $q$ and $r$ such that $a = bq+r$ and $\deg(r) < \deg(b)$.
If $\DD$ is only a UCR, existence of $q$ and $r$ holds but not necessarily uniqueness.*

> *Proof.*
Consider the set of all polynomials of the form $a-cb$ for $c \in \DD[X]$:
\[
 S \defeq \{a-cb \mid c \in \DD[X]\}.
\]
Let $r$ be a polynomial of minimal degree in $S$ and $q$ be such that $r=a-qb$.
Let us show that $\deg(r) < \deg(b)$.
Indeed, assume that this does not hold.
Let $u X^n$ and $v X^m$ be the leading terms of $b$ and $r$ respectively, with $m \ge n$ and $u \in \DD^*$.
Consider the polynomial $r'$ defined as
\[
r'(X) \defeq r(X) - vu^{-1}X^{m-n} b(X).
\]
Then $r' \in S$.
Since the leading terms of $r(X)$ and $vu^{-1}X^{m-n} b(X)$ are both $v X^m$, they cancel and the leading term of $r'$ has degree at most $m-1$, so that $\deg(r') < \deg(r)$, contradicting the assumption that $r$ has minimal degree in $S$.
Hence, $\deg(r) < \deg(b)$, which proves existence of a suitable pair $(q,r)$.
> Note that we did not use the fact that $\DD$ does not have zero divisors, meaning existence also holds when $\DD$ is only a UCR.
>
> Let us show uniqueness.
Assume that there exists pairs of polynomials $(q,r) \neq (q',r')$ such that $a=qb+r$, $a=q'b+r'$, $\deg(r) < \deg(b)$, and $\deg(r') < \deg(b)$.
Then $(q-q')b = r'-r$.
If $r=r'$ then, since $\DD[X]$ is an integral domain and $b$ is non-zero, $q=q'$, contradicting the assumption that $(q,r) \neq (q',r')$.
Hence, we assume that $r \neq r'$.
Then, since $\DD$ is an integral domain and $b$ divides the non-zero polynomial $r-r'$, by {{ref: prop:degree_divisor}},
\[
 \deg(b) \le \deg(r'-r).
\]
On the other hand,
\[
\deg(r-r') \le \max \{\deg(r),\deg(r')\} < \deg(b).
\]
This is a contradiction and hence we must have $(q,r) = (q',r')$.



| $\RR$ | impl./equ. | $\RR[X]$      | see |
|:-----:|:----------:|:-------------:|:---:|
| ID    | $\iff$     | ID            | {{ref: prop:pol_ring_over_id}} |
| UFD   | $\iff$     | UFD           |     |
| PID/Euclidean | $\implies$ | UFD           |     |
| field | $\iff$     | PID/Euclidean |     |

## Polynomial Evaluation and Roots

Let $\RR$ be a commutative ring.
Given a polynomial $p(X) = \sum_i a_i X^i$ in $\RR[X]$ and an element $u \in \RR$, the ***evaluation*** of $p(X)$ at $u$, written $p(u)$, is $\sum_i a_i u^i$.
The function from $\FF$ to $\FF$ mapping $u$ to $p(u)$ is called the ***polynomial function*** associated with $p$.

In general, there is not a one-to-one correspondence between polynomials and polynomial functions.
For example, over a finite commutative ring $\RR = \{r_1,\dots,r_n\}$, the polynomial
\[
 p(X) = (X-r_1) \cdots (X-r_n)
\]
evaluates to 0 at every element $r_i \in \RR$ but $p$ is clearly different from the constant polynomial 0.
Hence, this gives an example where two different polynomials yield the same polynomial function.

As wee will see below, if $\RR$ is an infinite integral domain, though, there is a one-to-one correspondence between polynomials and polynomial functions.

We say that $u$ is a ***root*** of $p(X)$ if $p(u) = 0$.
The following result gives an important sufficient and necessary condition for a ring element to be a root of a polynomial.

{{thm}}{thm:factor}[Factor Theorem]
*Let $\RR$ be a UCR, $p \in \RR[X]$ be a polynomial and $u \in \RR$ be a ring element.
Then $u$ is a root of $p(X)$ if and only if $X-u$ divides $p(X)$.*

The Factor Theorem is actually a special case of the following result.

{{thm}}{thm:polynomial_remainder}[Polynomial Remainder Theorem]
*Let $\RR$ be a UCR, $p \in \RR[X]$ be a polynomial and $u,v \in \RR$ be ring elements.
Then $p(u) = v$ if and only if $X-u$ divides $p(X)-v$.*

> *Proof.*
The *if* direction is straightforward.
Let us prove the *only if* direction.
Since the leading coefficient of $X-u$ is a unit, by {{ref: prop:polynomial_division}}, there exists polynomials $q$ and $r$ such that $p(X) = (X-u) q(X) + r(X)$ with $\deg(r) < \deg(X-u) =1$.
Hence, the polynomial $r$ must be a constant.
Evaluating the polynomial equality at $u$, we obtain that $r = p(u)$.
Hence, if $p(u) = v$ then $p(X)-v = (X-u)q(X)$, which exactly means that $X-u$ divides $p(X)-v$.

Note that the statement "$X-u$ divides $p(X)-v$" is equivalent to the statement "$v$ is the remainder of the division of $p(X)$ by $X-u$", hence the name of the theorem.

The factor theorem (which holds over any UCR) generalizes to multiple roots naturally, however only over *integral domains*.
(We will see how the Polynomial Remainder Theorem generalizes to multiple evaluations [in a moment](#thm:generalized_polynomial_remainder).)

{{thm}}{thm:generalized_factor}[Generalized Factor Theorem]
*Let $\DD$ be an integral domain, $p \in \DD[X]$ be a polynomial, and $u_1,\dots,u_n \in \DD$ be $n$ distinct ring elements.
Then $u_1,\dots,u_n$ are roots of $p$ if and only if $\prod_{i=1}^n (X-u_i)$ divides $p$.*

> *Proof.*
The *if* direction is straightforward.

This has an important consequence regarding the maximal number of roots of a polynomial.

{{prop}}{prop:number_roots}
*Let $\DD$ be an integral domain and let $p \in \DD[X]$ be a non-zero polynomial of degree $d$.
Then $p$ has at most $d$ distinct roots in $\DD$.*

> *Proof.*
This follows easily from the Generalized Factor Theorem.
Indeed, assume that $p$ has degree $d$ and has $n > d$ roots.
Let $u_i,\dots,u_n$ denote the roots of $p$.
Then $\prod_{i=1}^n (X-u_i)$ divides $p$, a contradiction with {{ref: prop:degree_divisor}} as a polynomial of degree $n$ cannot divide a non-zero polynomial of degree $d < n$.
>
> Let us prove the proposition directly by induction on $d$.
The result clearly holds for $d=0$.
Let $d \ge 1$ and assume that the result holds for degree $0, \dots, d-1$.
Let $p \in \DD[X]$ be a polynomial of degree $d$.
If $p$ has no root then the result holds again.
Otherwise, assume that $p$ has a root $u \in \DD$.
Then, by the {{tref: thm:factor}}, $p(X) = (X-u) q(X)$ for some polynomial $q$, where, by {{ref: prop:pol_ring_over_id}}, $q$ has degree $d-1$.
If $u'$ is a root of $p$ distinct from $u$, then $(u'-u) q(u') = 0$ which implies that $q(u') = 0$ since $u'-u \neq 0$ and $\DD$ has no zero divisors.
Since $q$ has at most $d-1$ distinct roots by the induction hypothesis, $p$ has at most $d$ distinct roots.

{{prop}}
*Let $\DD$ be an integral domain and $p \in \DD[X]$ be a polynomial such that for every $u \in \DD$, $p(u) = 0$.
If $\DD$ is infinite, then $p$ is the zero polynomial.*

> *Proof.*

Hence, over an infinite integral domain, there is actually a one-to-one mapping between polynomials and polynomial functions.
However, not every function from $\DD$ to $\DD$ is a polynomial function: for example, the function $f$ such that $f(0) = 1$ and $f(u) =0$ for $u \neq 0$ is not a polynomial since it has infinitely many roots yet it cannot be the function corresponding to the zero polynomial.

{{prop}}
*Let $\DD$ be an integral domain.
Let $\Phi \colon \DD[X] \to \cF(\DD)$ be the ring homomorphism mapping a polynomial to the corresponding polynomial function.
Then:*
1. *If $\DD$ is finite, then $\Phi$ is surjective but not injective.*
2. *If $\DD$ is infinite, then $\Phi$ is injective but not surjective.*

> {{rem}}
Note that being infinite and being an integral domain are two necessary conditions.
Over infinite rings with zero divisors, a non-zero polynomial may evaluate to zero over the entire ring.
See [here](https://math.stackexchange.com/questions/64035/can-a-nonzero-polynomial-evaluate-to-the-zero-function-in-a-suitable-infinite-ri).

## Lagrange Interpolation

In all the following, a set $\cU = \{u_1, \dots, u_n\}$ of $n$ distinct field elements $u_i \in \FF$ will be called an ***evaluation domain*** (or simply ***domain***) of size $n$.

{{thm}}{thm:lagrange_interpolation}[Lagrange Interpolation Theorem]
*Let $\FF$ be a finite field and let
\[
 \cE = \{(u_1, v_1), \dots, (u_n,v_n)\} \subset \FF^2
\]
be a set of $n$ pairs of field elements such that $u_i \neq u_j$ for $i \neq j$.
Then there is a unique polynomial $\ell(X) \in \PR{}{n-1}$, called the Lagrange interpolation polynomial for $\cE$, such that $\ell(u_i) = v_i$ for every $i \in \{1, \dots, n\}$.*

> *Proof.*
Uniqueness is proved as follows: assume there exists two polynomials $p(X)$ and $q(X)$ interpolating the $n$ points.
Then the polynomial $p(X)-q(X)$ has $n$ roots but has degree at most $n-1$, hence must be the 0 polynomial, which implies that $p(X) = q(X)$.
>
> To establish existence, one introduces the *Lagrange basis* associated with the domain $\cU = \{u_1, \dots, u_n\}$.
This is the tuple of polynomials $(\ell_1(X), \dots, \ell_n(X))$ of degree $n-1$ defined as
\[
 \ell_j(X) \defeq \prod_{\substack{1 \le k \le n \\ k \neq j}} \frac{X-u_k}{u_j-u_k}.
\]
One can easily check that
\[
 \ell_j(u_i) =
 \begin{cases}
  0 & \text{if } i \neq j \\
  1 & \text{if } i=j.
 \end{cases}
\]
Then the Lagrange interpolating polynomial for $\{(u_1, v_1), \dots, (u_n,v_n)\}$ is given by
\[
 \ell(X) \defeq \sum_{j=1}^n v_j \ell_j(X).
\]
This polynomial has degree at most $n-1$ and it is easy to see that $\ell(u_i) = v_i$ for every $i \in \{1,\dots,n\}$.

Note that the Lagrange basis $(\ell_1(X),\dots,\ell_n(X))$ associated with any domain $\cU = \{u_1, \dots, u_n\}$ is indeed a basis for the $\FF$-vector space $\PR{}{n-1}$ in the linear algebra sense.
The coordinates of a polynomial $p \in \PR{}{n-1}$ in this basis are $(p(u_1), \dots, p(u_n))$.

A polynomial specified by a tuple $(a_0,\dots,a_{n-1})$ such that $p(X) = \sum_{i=0}^{n-1} a_i X^i$ is sometimes said to be in *coefficients form*, while when it is specified by the values $(v_1,\dots,v_n)$ it takes over some domain $\cU = \{u_1, \dots, u_n\}$ it is said to be in *evaluation form*.
This is merely a change of basis.

Another way to look at Lagrange interpolation is as follows.
Considering the coefficients $a_0,\dots,a_{n-1}$ of a polynomial $p(X) = \sum_{j=0}^{n-1} a_j X^j$ of degree $n-1$ as unknowns, each evaluation $p(u_i) = v_i$, $i \in \{1,\dots,n\}$ yields a linear equation
\[
 \sum_{j=0}^{n-1} u_i^j a_j = v_i.
\]
In matrix form, this yields
\[
 \begin{pmatrix}
  1 & u_1 & u_1^2 & \cdots & u_1^{n-1} \\
  1 & u_2 & u_2^2 & \cdots & u_2^{n-1} \\
  & & \vdots & & \\
  1 & u_n & u_n^2 & \cdots & u_n^{n-1}
 \end{pmatrix}
 \cdot
 \begin{pmatrix}
  a_0 \\
  a_1 \\
  \vdots \\
  a_{n-1}
 \end{pmatrix}

 =

 \begin{pmatrix}
  v_1 \\
  v_2 \\
  \vdots \\
  v_n
 \end{pmatrix} {{numeq}}{vandermonde}
\]
The matrix on the left-hand side is called a [Vandermonde matrix](https://en.wikipedia.org/wiki/Vandermonde_matrix).
It is invertible if and only if the $u_i$'s are distinct, which gives another way to see that there is a unique polynomial of degree at most $n-1$ such that $p(u_i) = v_i$ for every $i \in \{1,\dots,n\}$.

### Generalized Polynomial Remainder Theorem

Lagrange interpolation allows us to formulate a generalization of the {{tref: thm:polynomial_remainder}}.
Given an evaluation domain $\cU = \{u_1,\dots,u_n\}$, the ***vanishing polynomial*** over $\cU$, denoted $z_{\cU}(X)$, is the polynomial defined by
\[
 z_{\cU}(X) \defeq \prod_{i=1}^n (X-u_i).
\]
It is such that $z_{\cU}(u) = 0$ for every $u \in \cU$, but it is not the Lagrange interpolation polynomial for $\{(u_1,0),\dots,(u_n,0)\}$ since it has degree $n$ (the Lagrange interpolation polynomial for $\{(u_1,0),\dots,(u_n,0)\}$ is actually the zero polynomial).

{{thm}}{thm:generalized_polynomial_remainder}[Generalized Polynomial Remainder Theorem]
*Let $p \in \FF[X]$ be a polynomial, $n \in \{1,\dots,\deg(p)\}$ be an integer, $u_1, \dots, u_n \in \FF$ be $n$ distinct field elements, and $v_1, \dots, v_n \in \FF$ be $n$ field elements (not necessarily distinct).
Let $z(X)$ be the vanishing polynomial for $\{u_1, \dots, u_n\}$ and $\ell(X)$ be the Lagrange interpolation polynomial for $\{(u_1,v_1), \dots, (u_n,v_n)\}$.
Then $p(u_i) = v_i$ for every $i \in \{1,\dots,n\}$ if and only if $z(X)$ divides $p(X) - \ell(X)$, or equivalently if and only if $\ell(X)$ is the remainder of the division of $p(X)$ by $z(X)$.*

> *Proof.*
Assume that $z(X)$ divides $p(X) - \ell(X)$, i.e., there exists $q \in \FF[X]$ such that $p(X) = q(X)z(X) + \ell(X)$.
Evaluating this equality at $u_i$ and using $z(u_i) = 0$ and $\ell(u_i) = v_i$ implies that $p(u_i)= v_i$ for every $i \in \{1,\dots,n\}$.
>
> Conversely, assume that $p(u_i) = v_i$ for every $i \in \{1,\dots,n\}$.
Since $\FF[X]$ is Euclidean, there exists polynomials $q(X)$ and $r(X)$ such that $p(X) = q(X)z(X) + r(X)$ with $\deg(r) < \deg(z) = n$.
Evaluating this equality at $u_i$, $i \in \{1,\dots,n\}$, yields $r(u_i) = p(u_i) = v_i$.
Since $\deg(r) < n$, $r$ is necessarily the Lagrange interpolation polynomial for $\{(u_1,v_1), \dots, (u_n,v_n)\}$.

For $n=1$, one exactly recovers the {{tref: thm:polynomial_remainder}} since for a single point $(u,v)$ the vanishing polynomial is $X-u$ and the Lagrange interpolation polynomial is simply the constant polynomial $\ell(X) = v$.

### Computational Aspects

[Newton's method](https://en.wikipedia.org/wiki/Newton_polynomial) and [Neville's algorithm](https://en.wikipedia.org/wiki/Neville%27s_algorithm) have quadratic complexity, 

#### The Barycentric Formula

Assume we are given a set of $n$ evaluations $\cE = \{(u_1,v_1),\dots(u_n,v_n)\}$ over domain $\cU = \{u_1,\dots,u_n\}$.
One can consider various task related to the Lagrange interpolation polynomial $\ell$ for $\cE$.
One is to compute the coefficients $(a_0,\dots,a_{n-1})$ of this polynomial.
Another is to evaluate $\ell$ on a field element $u \notin \{u_1,\dots,u_n\}$.

For the first task, one may be tempted to solve Eq. {{eqref: vandermonde}}.
However, this requires to invert a square matrix of size $n$, which requires $O(n^3)$ field operations.

A very useful form for Lagrange interpolation is the so-called *barycentric formula* [[BT04](../references.md#BT04)].
Let $z(X)$ be the vanishing polynomial for $\{u_1, \dots, u_n\}$ and for $j \in \{1,\dots,n\}$ let $w_j$ denote the *barycentric weights* defined as
\[
 w_j \defeq \frac{1}{\prod_{\substack{1 \le k \le n \\ k \neq j}} (u_j-u_k)}.
\]
Note that the formal derivative of $z(X)$ is
\[
 z'(X) = \sum_{i=1}^n \prod_{\substack{1 \le k \le n \\ k \neq i}} (X-u_k),
\]
hence one also has
\[
 w_j = \frac{1}{z'(u_j)}.
\]
Then the $j$-th polynomial of the Lagrange basis is
\[\begin{aligned}
 \ell_j(X) & \defeq \prod_{\substack{1 \le k \le n \\ k \neq j}} \frac{X-u_k}{u_j-u_k} \\
 & = w_j \frac{z(X)}{X-u_j}
\end{aligned}\]
from which it follows that the Lagrange interpolation polynomial for $\cE$ can be written as
\[
 \ell(X) = z(X) \sum_{i=1}^n \frac{w_i v_i}{X-u_i}.
\]
This is the barycentric Lagrange interpolation formula.

Based on this, here is how one can compute the coefficients of $\ell$ and evaluate $\ell$ on a point outside $\cU$ in quasilinear time:
- compute the coefficients of $z(X)$ using a divide-and-conquer approach (see [here](https://cs.stackexchange.com/questions/116643/what-is-the-most-efficient-algorithm-to-compute-polynomial-coefficients-from-its))
- compute the coefficients of $z'(X)$ from the ones of $z(X)$ (this requires $O(n)$ multiplications)
- compute $w_j = z'(u_j)$, $j \in \{1,\dots,n\}$, using [multipoint evaluation](https://en.wikipedia.org/wiki/Polynomial_evaluation#Multipoint_evaluation)

See also [this](https://www.csa.iisc.ac.in/~chandan/courses/CNT/notes/lec6.pdf).

Once this is done, computing $\ell(u)$ for $u \notin \cU$ takes linear time using
\[
 \ell(u) = z(u) \sum_{i=1}^n \frac{w_i v_i}{u-u_i}.
\]

All in all, this yields an algorithm with complexity $O(n \log^2 n)$ field operations.

#### The Special Case of Roots of Unity

The most favorable case from a computational point of view is when $\cU$ is the subgroup of $\FF^*$ consisting of $n$-th roots of unity.
Then the vanishing polynomial takes a very simple form, namely
\[
 z(X) = X^n-1.
\]
Its formal derivative is simply
\[
 z'(X) = n X^{n-1}.
\]

### Applications

An important application of Lagrange interpolation is [Shamir's secret sharing](https://en.wikipedia.org/wiki/Shamir%27s_secret_sharing).


