> **Chapter status:** &nbsp; 🚧 &nbsp; draft &nbsp; 🚧

# Integers

In this chapter, we cover a number of fundamental results about integers.
Most of these results will be generalized in the chapter about [rings](rings.md), where we will return to the axiomatic definition of integers (to wit, the set of integers is, up to isomorphism, the unique non-trivial, ordered, unitary commutative ring whose positive elements are well-ordered).
A lot of results in the chapter about [groups](groups.md) rely on the results covered here, notably {{tref: prop:euclid_division}}.

## Contents

<!-- toc -->

## Basic Definitions

In all the following, we let
\[
 \NN \defeq \{0, 1, 2 \dots\}
\]
denote the set of natural numbers and
\[
 \ZZ \defeq \{\dots, -2, -1, 0, 1, 2, \dots\}
\]
denote the set of integers.

We will not explicitly formulate here all standard properties of addition, multiplication, and of the order relation and defer a more formal treatment to the chapter about [rings](rings.md).

We only state here two salient properties of the integers: the cancellation law (a consequence of $\ZZ$ being a so-called [integral domain](rings.md#zero-divisors-and-integral-domains)) and the well-ordering principle (or well-ordering axiom), which is one of the axioms governing integers.

{{prop}}{prop:cancellation_z}[cancellation law for integers]
*Let $a,b,c \in \ZZ$ be integers such that $a \neq 0$.
Then $ab = ac$ implies $b=c$.*

{{prop}}{prop:well_ordering}[well-ordering axiom]
*Let $S \subseteq \NN$ be a non-empty subset of $\NN$.
Then $S$ contains a smallest element, i.e., there exists $m \in S$ such that for any $s \in S$, $m \le s$.*

It is immediate that the smallest element is necessarily unique:

{{prop}}
*Let $S \subseteq \NN$ be a non-empty subset of $\NN$.
Then it has a unique smallest element.*

```admonish proof collapsible=true
Assume that $S$ has two smallest elements $m$ and $m'$.
Then $m \le m'$ (since $m$ is a smallest element) and $m' \le m$ (since $m'$ is a smallest element).
Hence, $m = m'$.
```

## Divisibility

Let $a,b \in \ZZ$.
We say that $b$ ***divides*** $a$, or that $b$ is a ***divisor*** of $a$, or that $a$ is a ***multiple*** of $b$, denoted $b \divides a$, if there exists $q \in \ZZ$ such that $a=qb$.

{{prop}}{prop:associated_integers}
*Let $a,b \in \ZZ$.
Then $a \divides b$ and $b \divides a$ if and only if $a = \pm b$.*

```admonish proof
```

```admonish remark
In a general commutative ring, two elements $a$ and $b$ such that $a \divides b$ and $b \divides a$ are called associates.
The previous proposition shows that associates in $\ZZ$ are opposites.
See the [corresponding section](rings.md#divisibility) of the chapter about rings.
```


## Euclidean Division

{{prop}}{prop:euclid_division}[Euclid's division lemma]
*Let $a, n \in \ZZ$ be integers with $n > 0$.
Then there exists unique integers $q$ and $r$ such that
\[
 a = qn + r
\]
and $0 \le r < n$.*


```admonish proof collapsible=true
Let us show existence first.
Consider the set $S$ defined as
\[
 S = \{a-kn \colon k \in \ZZ\} \cap \NN.
\]
This set is non-empty: if $a \ge 0$ then $a = a - 0n \in S$ whereas if $a < 0$ then $a-an = a(1-n) \in S$.
By the {{tref: prop:well_ordering}}, $S$ has a minimal element $r$.
Since $r \in S$, there exists $q \in \ZZ$ such that $r = a-qn$.
Moreover, $r \ge 0$ by definition of $S$.
Assume that $r \ge n$.
Then $r-n \in S$, contradicting the minimality of $r$.
Hence, $a = qn+r$ with $0 \le r < n$ as claimed.

Let us now show uniqueness.
Assume that $a = qn+r = q'n+r$ with $0 \le r < n$ and $0 \le r' < n$.
Then
\[
r-r' = n(q'-q)
\]
meaning $r-r'$ is a multiple of $n$.
But note that
\[
-n < r-r' < n.
\]
This implies that $r=r'$ since the only multiple of $n$ in $\{-(n-1),\dots,0,\dots, n-1\}$ is $0$.
Hence, $n(q'-q) = 0$, which by the {{tref: prop:cancellation_z}} implies that $q=q'$.
```

The four integers $a$, $n$, $q$, and $r$ each have a name:

- $a$ is called the ***dividend***,
- $n$ is called the ***divisor***,
- $q$ is called the ***quotient***,
- $r$ is called the ***remainder***.

```admonish example
- Taking $a = 143$ and $n = 36$, we have $q = 3$ and $r = 35$:
\[
 143 = 3*36 + 35.
\]
- This also works with a negative dividend: taking $a = -101$ and $n = 36$, we have $q = -3$ and $r = 7$:
\[
 -101 = -3*36 +7.
\]
```

## Greatest Common Divisor

Let $a,b \in \ZZ$.
A ***common divisor*** of $a$ and $b$ is an integer $d \in \ZZ$ such that $d$ divides both $a$ and $b$.
A ***greatest common divisor*** (GCD) of $a$ and $b$ is a common divisor $d$ of $a$ and $b$ such that $d \ge 0$ and any other common divisor of $a$ and $b$ divides $d$.

It is not immediately clear that a greatest common divisor always exists and is unique (and in general commutative rings this might not be the case).
This does hold in $\ZZ$, though.

{{prop}}
*Let $a,b \in \ZZ$ be integers such that $a$ and $b$ are not both zero.
Then $a$ and $b$ have a unique greatest common divisor.*

```admonish proof collapsible=true
Let us show uniqueness first.
Assume that $a$ and $b$ have two GCDs $d$ and $d'$.
Then, by {{ref: prop:associated_integers}}, one has $d = \pm d'$.
Since a GCD must be positive, it follows that $d=d'$.
```

The greatest common divisor of $a$ and $b$ is denoted $\gcd(a,b)$ (or sometimes $(a,b)$ but this notation is confusing and will not be used here).
By convention, the greatest common divisor of $a=b=0$ is defined as $0$.

{{prop}}{prop:bezout}[Bézout's lemma]

```admonish proof
```

```admonish remark
In a general commutative ring, greatest common divisors are defined as above, except condition $d \ge 0$ is dropped (indeed, in a general ring, an order relation might not exist).
Under this more general definition, two integers $a$ and $b$ have *two* greatest common divisors $d$ and $-d$.
The fact that $\ZZ$ is ordered allows to single out the positive one as *the* greatest common divisor of $a$ and $b$.
```

{{prop}}
*Let $a,b \in \ZZ$.
Then
\[
 a\ZZ + b\ZZ = \gcd(a,b) \ZZ.
\]*

Two integers $a,b \in \ZZ$ are said ***coprime*** or ***relatively prime*** if $\gcd(a,b) = 1$.




## Prime Numbers

## Modular Arithmetic
