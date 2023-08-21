> **Chapter status:** in progress
>
> **TODO:**

# Polynomials

## Contents

<!-- toc -->

## Generalities

Let $\FF$ be a field (not necessarily finite).
A (univariate) *polynomial* with coefficients in $\FF$ is an infinite sequence $(a_i)_{i \in \NN}$ such that $a_i=0$ for all but a finite number of indices $i$.
Polynomials are traditionally denoted
\[
 p(X) = a_0 + a_1 X + a_2 X^2 + \cdots = \sum_{i=0}^{\infty} a_i X^i
\]
where $X$ is a symbol called *indeterminate*.

The set of all univariate polynomials over $\FF$ is denoted $\FF[X]$:
\[
 \FF[X] \defeq \left\{ \sum_{i=0}^\infty a_i X^i : a_i \in \FF, \exists N \in \NN, \forall i \ge N, a_i = 0 \right\}.
\]

Polynomials can be added:
\[
 \sum_{i=0}^\infty a_i X^i + \sum_{i=0}^\infty b_i X^i = \sum_{i=0}^\infty (a_i+b_i) X^i.
\]

Polynomials can also be multiplied:
\[
 \sum_{i=0}^\infty a_i X^i \cdot \sum_{j=0}^\infty b_j X^j = \sum_{k=0}^\infty c_k X^k \quad \text{with} \quad c_k = \sum_{i+j=k} a_i b_j.
\]

The set $\FF[X]$ equipped with operations $+$ and $\cdot$ is a ring called the *polynomial ring* in $X$ over $\FF$.
Moreover, if we define scalar multiplication of a polynomial $p(X)$ by $a \in \FF$ as multiplication by the constant polynomial $a(X) = a$, then $\FF[X]$ is a vector space over $\FF$.
The set $\{1, X, X^2, \dots\}$ is a basis of this vector space called the *monomial basis*.

The *degree* of a polynomial $p(X)$ is the largest power of $X$ occurring in $p(X)$ with a non-zero coefficient, with the convention that polynomial $0$ has degree $-\infty$.
Hence, $p(X) = \sum_{i=0}^d a_i X^i$ has degree $d$ provided $a_d \neq 0$.
We let $\deg(p)$ denote the degree of a polynomial $p$ and $\PR{d}$ denote the set of polynomials over $\FF$ of degree at most $d$.

**Proposition 1.**
*Let $p(X)$ and $q(X)$ be two polynomials in $\FF[X]$.
Then
\[\begin{aligned}
 & \deg(p+q) \le \max \left\{ \deg(p), \deg(q) \right\} \\
 & \deg(pq) = \deg(p) \deg(q).
\end{aligned}\]*

Given two non-zero polynomial $a(X)$ and $b(X)$, we say that $b(X)$ *divides* $a(X)$ or that $b(X)$ is a *factor* of $a(X)$ if there exists a polynomial $q(X)$ such that $a(X)=q(X)b(X)$.
The set $\FF[X]$ is a Euclidean domain, meaning one can perform euclidean division in $\FF[X]$: given two polynomials $a(X)$ and $b(X)$ with $b(X) \neq 0$, there exists unique polynomials $q(X)$ and $r(X)$ such that $a(X) = q(X) b(X) + r(X)$ and $\deg(r) < \deg(b)$.


## Roots

Given a polynomial $p(X) = \sum_i a_i X^i$ in $\FF[X]$ and an element $u \in \FF$, the *evaluation* of $p(X)$ at $u$, written $p(u)$, is $\sum_i a_i u^i$.
The function from $\FF$ to $\FF$ mapping $u$ to $p(u)$ is called the *polynomial function* associated with $p$.[^func]
We say that $u$ is a *root* of $p(X)$ if $p(u) = 0$.

**Theorem 1.**
*Let $p \in \FF[X]$ be a polynomial and $u \in \FF$ be a field element.
Then $u$ is a root of $p(X)$ if and only if $X-u$ divides $p(X)$.
More generally, $p(u) = v$ if and only if $X-u$ divides $p(X)-v$.[^kzg]*

> *Proof.*
> The *if* direction is straightforward.
> For the *only if* direction, note that since $\FF[X]$ is Euclidean, there exists polynomials $q$ and $r$ such that $p(X) = (X-u) q(X) + r(X)$ with $\deg(r) < \deg(X-u) =1$.
> Hence, the polynomial $r$ must be a constant.
> Evaluating the polynomial equality at $u$, we obtain that $r = p(u)$.
> Hence, if $p(u) = v$ then $p(X)-v = (X-u)q(X)$, which exactly means that $X-u$ divides $p(X)-v$.

**Theorem 2.**
*Let $p \in \FF[X]$ be a non-zero polynomial of degree $d$ over $\FF$.
Then $p$ has at most $d$ distinct roots in $\FF$.*

> *Proof.*
> By induction on $d$.
> The result clearly holds for $d=0$.
> Let $d \ge 1$ and assume that the result holds for degree $0, \dots, d-1$.
> Let $p \in \FF[x]$ be a polynomial of degree $d$.
> If $p$ has no root then the result holds again.
> Otherwise, assume that $p$ has a root $u \in \FF$.
> Then, by the previous theorem, $p(X) = (X-u) q(X)$ for some polynomial $q$, where, by Proposition 1, $q$ has degree $d-1$.
> If $u'$ is a root of $p$ distinct from $u$, then $(u'-u) q(u') = 0$ which implies that $q(u') = 0$ since $u'-u \neq 0$ and $\FF$ is a field.
> Since $q$ has at most $d-1$ distinct roots by the induction hypothesis, $p$ has at most $d$ distinct roots.

## Lagrange Interpolation

In all the following, a set $\cD = \{x_0, \dots, x_d\}$ of distinct field elements $x_i \in \FF$ will be called an *evaluation domain* (or simply *domain*) of size $d+1$.

**Lagrange Interpolation Theorem.**
*Let $\FF$ be a finite field and let $(x_i, y_i)_{0 \le i \le d} \in \FF^2$ be $d+1$ pairs of field elements such that $x_i \neq x_j$ for $i \neq j$.
Then there is a unique polynomial $p(X) \in \PR{d}$, called the Lagrange interpolating polynomial, such that $p(x_i) = y_i$ for every $i \in \{0, \dots, d\}$.*

> *Proof.*
> Uniqueness is proved as follows: assume there exists two polynomials $p(X)$ and $q(X)$ interpolating the $d+1$ points.
> Then the polynomial $p(X)-q(X)$ has $d+1$ roots but has degree at most $d$, hence must be the 0 polynomial, which implies that $p(X) = q(X)$.
>
> To establish existence, one introduces the *Lagrange basis* associated with the domain $\cD = \{x_0, \dots, x_d\}$.
> This is the tuple of degree-$d$ polynomials $(\ell_0(X), \dots, \ell_d(X))$ defined as
> \[
   \ell_j(X) \defeq \prod_{\substack{0 \le k \le d \\ k \neq j}} \frac{X-x_k}{x_j-x_k}.
  \]
>
> One can easily check that
> \[
   \ell_j(x_i) =
   \begin{cases}
    0 & \text{if } j \neq i \\
    1 & \text{if } j=i.
   \end{cases}
  \]
>
> Then the Lagrange interpolating polynomial for $(x_i, y_i)_{0 \le i \le d}$ is given by
> \[
   \ell(X) \defeq \sum_{j=0}^d y_j \ell_j(X).
  \]
> This polynomial has degree at most $d$ and it is easy to see that $\ell(x_i) = y_i$ for every $i \in \{0,\dots,d\}$.

Note that the Lagrange basis is indeed a basis for the $\FF$-vector space $\PR{d}$ in the linear algebra sense.
The coordinates of a polynomial $p \in \PR{d}$ in this basis are $(p(x_0), \dots, p(x_d))$.

An important application of Lagrange interpolation is [Shamir's secret sharing](https://en.wikipedia.org/wiki/Shamir%27s_secret_sharing).

----

[^func]: Over infinite fields, two polynomials are different if and only if the two corresponding polynomial functions are different.
This is not the case over finite fields: for example, over $\FF_2$, the polynomial function $x \mapsto x^2-x$ maps $0$ and $1$ to $0$ yet polynomials $X^2-X$ and $0$ are different.

[^kzg]: This property will play an important role for the KZG polynomial commitment scheme.
