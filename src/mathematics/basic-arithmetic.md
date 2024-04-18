> **Chapter status:** &nbsp; 🚧 &nbsp; draft &nbsp; 🚧

# Basic Arithmetic

In this chapter, we cover a number of fundamental notions and results about arithmetic over integers, such as divisibility, greatest common divisors, and unique factorization.
Most of them will be generalized in the chapter about [rings](rings.md).
We will return to the axiomatic definition of integers in a [later chapter](foundations-of-integers.md) (to wit, the set of integers is, up to isomorphism, the unique non-trivial, ordered, unitary commutative ring whose positive elements are well-ordered).

## Contents

<!-- toc -->

## Basic Definitions

In all the following, we adopt the following notation and conventions:

- we let $\NN$ denote the set of non-negative integers (also called natural numbers):
\[
 \NN \defeq \{0, 1, 2 \dots\};
\]
- we let $\NN^*$ denote the set of positive integers:
\[
 \NN^\ast \defeq \NN \setm \{0\} = \{1, 2 \dots\};
\]
- we let $\ZZ$ denote the set of integers:
\[
 \ZZ \defeq \{\dots, -2, -1, 0, 1, 2, \dots\}.
\]

We will not explicitly formulate here all standard properties of addition, multiplication, and of the order relation and defer a more formal treatment to the chapter about [rings](rings.md).

We only state two salient properties of the integers: the cancellation law (a consequence of $\ZZ$ being a so-called [integral domain](rings.md#zero-divisors-and-integral-domains)) and the well-ordering principle (or well-ordering axiom), which is one of the axioms governing integers.

The ***absolute value*** of an integer $a \in \Z$, denoted $|a|$, is defined as
\[
 |a| \defeq
 \begin{cases}
  a & \text{if } a > 0 \\
  0 & \text{if } a = 0 \\
  -a & \text{if } a < 0.
 \end{cases}
\]

{{prop}}{prop:cancellation_z}[cancellation law for integers]
*Let $a,b,c \in \ZZ$ be integers such that $a \neq 0$.
Then $ab = ac$ implies $b=c$.*

{{prop}}{prop:well_ordering}[well-ordering axiom]
*Let $S \subseteq \NN$ be a non-empty subset of $\NN$.
Then $S$ contains a smallest element, i.e., there exists $m \in S$ such that for any $s \in S$, $m \le s$.*

It is immediate that the smallest element is necessarily unique.

{{prop}}
*Let $S \subseteq \NN$ be a non-empty subset of $\NN$.
Then it has a unique smallest element.*

```admonish proof collapsible=true
Assume that $S$ has two smallest elements $m$ and $m'$.
Then $m \le m'$ (since $m$ is a smallest element) and $m' \le m$ (since $m'$ is a smallest element).
Hence, $m = m'$.
```

It is also the case that a *finite* non-empty subset of $\NN$ (or $\ZZ$) has a largest element.

{{prop}}{prop:largest-element}
*Let $S \subseteq \ZZ$ be a finite non-empty subset of $\ZZ$.
Then $S$ contains a unique largest element, i.e., there exists a unique $M \in S$ such that for any $s \in S$, $s \le M$.*

## Divisibility

Let $a,b \in \ZZ$.
We say that $a$ ***divides*** $b$, or that $a$ is a ***divisor*** of $b$, or that $b$ is a ***multiple*** of $a$, denoted $a \divides b$, if there exists $q \in \ZZ$ such that $b = qa$.

Let us list a number of basic properties of divisibility.

{{prop}}
*For every $a,b \in \ZZ$,
\[
a \divides b \iff -a \divides b \iff a \divides -b \iff -a \divides -b.
\]*

```admonish proof collapsible=true
Assume that $a \divides b$.
Then $b = qa$ for some $q \in \ZZ$.
This implies that $b = (-q)(-a)$, hence $-a \divides b$.
All other implications can be proven in a similar way.
```

{{prop}}{prop:divisors_of_one}
*The only integers dividing $1$ are $1$ and $-1$.*

```admonish proof collapsible=true
Let $a \in \NN^*$ be a positive divisor of $1$.
By {{ref: prop:no_integer_between_zero_and_one}}, $a \ge 1$.
On the other hand, since $a \divides 1$, there exists $q$ such that $aq=1$.
Clearly, $q$ must be positive and cannot be $0$.
Hence, by {{ref: prop:no_integer_between_zero_and_one}}, $q \ge 1$.
Multiplying both sides by $a$, we obtain $aq = 1 \ge a$.
Hence, we have both $a \le 1$ and $a \ge 1$, which implies that $a=1$.
```

```admonish jump
In a general ring with a multiplicative identity $1$, divisors of $1$ are called *units*.
The previous proposition states that units of $\ZZ$ are $1$ and $-1$.
See [Rings, Units](rings.md#units).
```

{{prop}}
*For every $a,b,c \in \ZZ$ with $c \neq 0$,
\[
 a \divides b \iff ca \divides cb.
\]*

```admonish proof collapsible=true
If $a \divides b$ then $b = qa$ for some $q \in \ZZ$.
This implies that $cb = q(ca)$ and hence $ca \divides cb$.

Conversely, assume that $ca \divides cb$.
Then $cb = q(ca)$ for some $q \in \ZZ$.
Since $c \neq 0$, by the {{tref: prop:cancellation_z}}, $b = qa$ and hence $a \divides b$.
```

{{prop}}{prop:associated_integers}
*For every $a,b \in \ZZ$,
\[
 a \divides b \text{ and } b \divides a \iff a = \pm b.
\]*

```admonish proof collapsible=true
If $a = \pm b$ then clearly $a \divides b$ and $b \divides a$.

Conversely, assume that $a \divides b$ and $b \divides a$.
Then $b = qa$ and $a = q'b$ for some integers $q,q' \in \ZZ$.
If $a = b = 0$ then $a = \pm b$ and the conclusion holds.
Assume now that either $a$ or $b$ is not zero.
Let us consider the case $a \neq 0$ (the case $b \neq 0$ is similar).
Multiplying $b = qa$ by $q'$, we get $q'b = qq'a$, hence $a = qq'a$.
Since $a \neq 0$, by the {{tref: prop:cancellation_z}}, this implies that $qq' = 1$, i.e., $q'$ divides $1$.
By {{ref: prop:divisors_of_one}}, this implies $q' = \pm 1$, hence $a = \pm b$.
```

```admonish jump
In a general commutative ring, two elements $a$ and $b$ such that $a \divides b$ and $b \divides a$ are called *associates*.
The previous proposition shows that associates in $\ZZ$ are opposites.
See [Rings, Divisibility](rings.md#divisibility).
```

{{prop}}{prop:divisibility_transitive}
*For every $a,b,c \in \ZZ$,
\[
 a \divides b \text { and } b \divides c \implies a \divides c.
\]*

```admonish proof collapsible=true
If $a \divides b$ then $b = qa$ for some $q \in \ZZ$.
Similarly, if $b \divides $c then $c = q'b$ for some $q' \in \ZZ$.
This implies that $c = qq'a$, i.e., $a \divides c$.
```

Divisibility defines an order relation over $\NN^\ast$:

- it is reflexive since $a \divides a$;
- it is anti-symmetric since by {{ref: prop:associated_integers}}, if $a \divides b$ and $b \divides a$, then $a = \pm b$, but for $a,b \in \NN^\ast$ this implies $a = b$;
- it is transitive by {{ref: prop:divisibility_transitive}}.

Unlike $\le$ which is *total*, this order relation is only *partial* since there are integers $a$ and $b$ such that neither $a \divides b$ nor $b \divides a$ hold.
These two order relations are "consistent" if the sense of the following proposition.

{{prop}}
*For every $a,b \in \NN^*$, $a \divides b \implies a \le b$.*

```admonish proof collapsible=true
Let $a,b \in \NN^*$ such that $a \divides b$.
Then $b = qa$ for some $q \in \ZZ$.
We cannot have $q = 0$ as this would imply $a = 0$.
Moreover, $q \ge 0$ as $q < 0$ would imply $b < 0$.
Hence, $q \ge 1$, which implies $b = qa \ge a$.
```


## Euclidean Division

{{prop}}{prop:euclid_division}[Euclid's division lemma]
*Let $a, n \in \ZZ$ be integers with $n > 0$.
Then there exists unique integers $q$ and $r$ such that
\[
 a = qn + r
\]
and $0 \le r < n$.
Moreover, $n \divides a$ if and only if $r=0$.*

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

Finally, let us show that $n \divides a$ if and only if $r = 0$.
If $r = 0$, then $a = qn$ and hence $n \divides a$.
Conversely, if $n \divides a$, then $a = q'n$ for some $q' \in \ZZ$.
But then uniqueness of $q$ and $r$ implies that $q = q'$ and $r=0$.
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

Let $a,b \in \ZZ$ be integers such that $a$ and $b$ are not both zero.
A ***common divisor*** of $a$ and $b$ is an integer $d \in \ZZ$ such that $d$ divides both $a$ and $b$.
A ***greatest common divisor*** (GCD) of $a$ and $b$ is a common divisor $d$ of $a$ and $b$ such that $d \ge 0$ and any other common divisor of $a$ and $b$ divides $d$.

Note that this is different from the definition one usually encounters in high school, which is as follows.
Consider the set $S$ of all common divisors of $a$ and $b$.
This set is non-empty since $1 \in S$.
It is also finite since any common divisor $d$ of $a$ and $b$ satisfies $-a \le d \le a$ if $a \neq 0$ and $-b \le d \le b$ if $b \neq 0$ and $a$ and $b$ are not both zero.
Then the GCD of $a$ and $b$ is defined as the largest element of $S$, which by {{ref: prop:largest-element}} exists and is unique.

The reason we prefer to former definition is that it is closer to the definition [of a GCD in a commutative ring](rings.md#divisibility) (the only difference is that condition $d \ge 0$ is dropped since in general, there might be no order relation over an arbitrary ring).
Working with this definition, it is not immediately clear though that a greatest common divisor always exists and is unique (in fact, there exists commutative rings where two elements might not have a GCD), nor that it is equal to the GCD according to the later definition.
The following proposition proves this, and more.


\[
 \Lambda_{a,b} \defeq \{ua + bv \colon u,v \in \ZZ\}.
\]
This set has the following interesting property: if $c \in \Lambda_{a,b}$, then every multiple $kc$ of $c$ also belongs to $\Lambda_{a,b}$.

Consider for example $a = 6 = 2 \cdot 3$ and $b = 10 = 2 \cdot 5$.
We can quickly establish (using any of the two definitions above) that the greatest common divisor of $6$ and $10$ is $2$.


{{prop}}{prop:bezout}[Bézout's lemma]
*Let $a,b \in \ZZ$ be integers such that $a$ and $b$ are not both zero.
Then there exists a unique greatest common divisor $d$ of $a$ and $b$.
Moreover, $d$ is the smallest non-negative integer which can be written as a linear combination of $a$ and $b$.
In particular, there exists integers $u,v \in \ZZ$ such that
\[
 d = ua + vb.
\]*

```admonish proof collapsible=true
Let us show uniqueness first.
Assume that $a$ and $b$ have two GCDs $d$ and $d'$.
Then $d \divides d'$ and $d' \divides d$ and hence by {{ref: prop:associated_integers}}, one has $d = \pm d'$.
Since a GCD must be positive, it follows that $d=d'$.

Let us show existence by proving that the smallest non-negative integer which can be written as a linear combination of $a$ and $b$ in indeed their GCD.
consider the set of all non-negative linear combinations of $a$ and $b$, namely
\[
 S \defeq \{ua + vb \colon u,v \in \ZZ\} \cap \NN^*.
\]
This set is non-empty.
Indeed, assume that $a \neq 0$ (the case $b \neq 0$ is similar).
Then either $a$ or $-a$ is in $S$.
By the {{tref: prop:well_ordering}}, $S$ has a minimal element $d$.
We claim that $d$ is the GCD of $a$ and $b$.

First, we clearly have $d \ge 0$.
Second, let us show that $d$ is a common divisor of $a$ and $b$.
For this, we will show the following stronger statement: every element in $S$ is divisible by $d$.
Since $a \in S$ and $b \in S$, this will imply in particular that $d$ divides both of them.
Let $c = ua + vb$ be any element in $S$.
Since $d \in S$, let us also write $d = u_0a + v_0b$.
By {{tref: prop:euclid_division}}, there exists $q$ and $r$ with $0 \le r < d$ such that
\[
 c = qd + r.
\]
Then
\[\begin{aligned}
 r & = c - qd \\
 & = ua + bv - q(u_0 a + b_0 v) \\
 & = (u-q u_0)a + (v-q v_0)b.
\end{aligned}\]
Hence, $r \in S$.
But since $0 \le r < d$ and $d$ is the minimal element of $S$, one must have $r=0$.
Hence, $c=qd$, i.e., $d$ divides $c$.

It remains to show that every common divisor $d'$ of $a$ and $b$ divides $d$.
```

The greatest common divisor of $a$ and $b$ is denoted $\gcd(a,b)$ (or sometimes $(a,b)$ but this notation is confusing and will not be used here).
By convention, although the set of common divisors of $a = 0$ and $b= 0$ is $\ZZ$, $\gcd(0,0)$ is defined as $0$ (we will see why this makes sense when relating GCDs and ideals).

Expressing the GCD as a linear combination of $a$ and $b$, i.e., writing
\[
 \gcd(a,b) = ua + vb
\]
with explicit values $u,v \in \ZZ$ is often called a ***Bézout relation***.

```admonish remark
In a general commutative ring, greatest common divisors are defined as above, except condition $d \ge 0$ is dropped (indeed, in a general ring, an order relation might not exist).
Under this more general definition, two integers $a$ and $b$ have *two* greatest common divisors $d$ and $-d$.
The fact that $\ZZ$ is ordered allows to single out the positive one as *the* greatest common divisor of $a$ and $b$.
```

Two integers $a,b \in \ZZ$ are said ***coprime*** or ***relatively prime*** if $\gcd(a,b) = 1$, which is equivalent to $a$ and $b$ having only $1$ and $-1$ as common divisors.

{{prop}}
*Let $a,b \in \ZZ$.
Then $a$ and $b$ are coprime if and only if there exists $u,v \in \ZZ$ such that
\[
 ua + vb = 1.
\]*

```admonish proof collapsible=true
If $a$ and $b$ are coprime, then existence of $u,v \in \ZZ$ such that $ua + bv = 1$ is established by {{ref: prop:bezout}}.
Conversely, assume that there exists $u,v \in \ZZ$ such that $ua + bv = 1$.
```

## Computing GCDs and Bézout Relations

The GCD can be computed using Euclid's algorithm.
It relies on the following lemma.

{{prop}}
*Let $a,b \in \ZZ$ such that $a$ and $b$ are not both zero.
Then, for any $k \in \ZZ$,
\[
 \gcd(a+kb, b) = \gcd(a,b).
\]*

```admonish proof collapsible=true
```

## GCDs and Ideals

In this section, we give another interpretation of GCDs in terms of *ideals* of $\ZZ$.
As pretty much any notion we encounter in this chapter, ideals can be defined [for any ring](rings.md#ideals-and-quotient-rings).

A subset $\II \subseteq \ZZ$ of the integers is called an ***ideal*** of $\ZZ$ if it satisfies the following properties:

{{prop}}
*Let $a,b \in \ZZ$.
Then
\[
 a\ZZ + b\ZZ = \gcd(a,b) \ZZ.
\]*





## Prime Numbers
