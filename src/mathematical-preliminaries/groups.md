> **Chapter status:** almost final, some proofs missing
>
> **TODO:**
> - number propositions

# Groups

## Contents

<!-- toc -->

## Basic Definitions

A group is a non-empty set $\GG$ equipped with a binary operation $\star$ satisfying the following properties:

- *associativity*: for every $a,b,c \in \GG$, $(a \star b) \star c = a \star (b \star c)$;
- *identity element*: there exists $e \in \GG$ such that for every $a \in \GG$, $e \star a = a \star e =a$; $e$ is called the *identity element* of $\GG$;
- *inverse element*: for every $a \in \GG$, there exists $b \in \GG$ such that $a \star b = b \star a =e$; $b$ is called the *inverse* of $a$.

**Proposition.**
*Let $\GG$ be a group. Then $\GG$ has a unique identity element and every element $a \in \GG$ has a unique inverse.*

> *Proof.*
> Assume that $\GG$ has two identity elements $e$ and $e$.
> Then $e \star e' = e$ (because $e'$ is an identity element) and $e \star e' = e'$ (because $e$ is an identity element), hence $e=e'$.
> Assume that some group element $a$ has two inverses $b$ and $b'$.
> Then
> \[\begin{aligned}
   b & = b \star e \\
     & = b \star (a \star b') \\
     & = (b \star a) \star b' \\
     & = e \star b' \\
     & = b'.
  \end{aligned}\]

The group consisting of a single element $e$ such that $e \star e = e$ is called the *trivial group*.

If the binary operation is commutative, i.e., for every $a,b \in \GG$, $a \star b = b \star a$, then $\GG$ is said to be *abelian*.

If $\GG$ is finite, the number of elements of $\GG$ is called the *order* of $\GG$ and denoted $\abs{\GG}$.
If $\GG$ is infinite, $\GG$ is said to have infinite order.

## Direct Product

Let $(\GG,\star)$ and $(\HH,\bullet)$ be two groups.
The *direct product* of $\GG$ and $\HH$ is the Cartesian product
\[
 \GG \times \HH \defeq \{(a,b) \mid a \in \GG, b \in \HH \}
\]
equipped with the binary operation $\diamond$ defined component-wise:
\[
 (a,b) \diamond (c,d) = (a \star c, b \bullet d).
\]
It is a group with identity element $(e_{\GG},e_{\HH})$, where $e_{\GG}$ and $e_{\HH}$ are respectively the identity element of $\GG$ and $\HH$.
The inverse of $(a,b)$ is $(\bar{a},\bar{b})$, where $\bar{a}$ and $\bar{b}$ are respectively the inverse of $a$ and $b$.

For abelian groups, the direct product is sometimes called *direct sum* and denoted $\GG \oplus \HH$.[^sum]

## Additive/Multiplicative Notation

There are two standard notation types for the group operation:

- *additive*: the group operation is denoted $+$, the identity element is denoted $0$, and the inverse of $a$ is denoted $-a$; moreover, for $k \in \ZZ$ and $a \in \GG$, one defines
\[
 ka \defeq
 \begin{cases}
  0 & \text{if } k=0 \\
  \underbrace{a + \cdots + a}_{k \text{ times}} & \text{if } k > 0 \\
  (-k)(-a) & \text{if } k < 0;
 \end{cases}
\]
- *multiplicative*: the group operation is denoted $*$ or $\cdot$, the identity element is denoted $1$, and the inverse of $a$ is denote $a^{-1}$; moreover, for $k \in \ZZ$ and $a \in \GG$, one defines
\[
 a^k \defeq
 \begin{cases}
  1 & \text{if } k=0 \\
  \underbrace{a * \cdots * a}_{k \text{ times}} & \text{if } k > 0 \\
  (a^{-1})^{-k} & \text{if } k < 0.
 \end{cases}
\]

In the case of multiplicative notation, the operation symbol might be omitted and the group law simply denoted by juxtaposition.
By convention, for an "abstract" group, additive notation is restricted to abelian groups (meaning multiplicative notation is used either when the group is known to be non-abelian, or when the abelian/non-abelian character of the group is unspecified).

*For the rest of this chapter, unless specified otherwise, the group operation will be denoted multiplicatively but the identity element will be denoted $e$ for clarity.*

## Subgroups

Let $\GG$ be a group and $\HH$ be a non-empty subset of $\GG$.
The subset $\HH$ is a *subgroup* of $\GG$ if $\HH$ equipped with the binary operation of $\GG$ is a group.

**Proposition.**
*Let $\GG$ be a group and $\HH$ be a non-empty subset of $\GG$.
Then $\HH$ is a subgroup of $\GG$ if and only if for every $a,b \in \HH$, $a b^{-1} \in \HH$.*

A subgroup of $\GG$ is said to be *proper* if it is different from $\GG$.
Any non-trivial group has at least one proper subgroup, namely $\{e\}$, called the *trivial subgroup*.

Let $\HH$ be a subgroup of $\GG$ and $g \in \GG$.
The *left coset* , resp. *right coset* of $\HH$ associated with $g$ is the subset of $\GG$ defined as
\[\begin{aligned}
 g \HH & \defeq \{g h \mid h \in \HH\} \\
 \HH g & \defeq \{h g \mid h \in\ \HH\}.
\end{aligned}\]

<a name="th:lagrange"></a>
**Lagrange's Theorem.**
*Let $\GG$ be a finite group.
Then the order of any subgroup $\HH$ of $\GG$ divides the order of $\GG$.*

> *Proof.*
> Consider the relation defined by $a \sim b$ if and only if $ab^{-1} \in \HH$.
> One can easily check that this is an equivalence relation and that the equivalence class of $g \in \GG$ is the right coset of $\HH$ associated with $g$.
> For any $g \in \GG$, the mapping $h \mapsto hg$ is a bijection from $\HH$ to $\HH g$: it is obviously surjective and $hg=h'g \Rightarrow hgg^{-1} = h'gg^{-1} \Rightarrow h=h'$, hence it is injective.
> Hence, all right cosets have $|\HH|$ elements.
> Since right cosets form a partition of $\GG$, we have
> \[ |\GG| = n |\HH|\]
> where $n$ is the number of right cosets.

The number of right cosets of $\HH$ (which is equal to the number of left cosets) is called the *index* of $\HH$ in $\GG$ and denoted $[\GG:\HH]$.
Hence, Lagrange theorem states that
\[
 |\GG| = [\GG:\HH] |\HH|.
\]

## Normal Subgroups and Quotient Groups

Let $\GG$ be a group.
A subgroup $\HH$ of $\GG$ is said to be *normal* if for every $g \in \GG$, $g \HH = \HH g$ (i.e., left and right cosets are equal).

**Proposition.**
*A subgroup $\HH$ of $\GG$ is normal if and only if for every $g \in \GG$, $g \HH g^{-1} \subseteq \HH$.*

**Proposition.**
*Every subgroup of an abelian group is normal.*

Let $\GG$ be a group and let $\sim$ be an equivalence relation on $\GG$.
We say that $\sim$ is *compatible with the group structure of $\GG$* if $a \sim b$ and $c \sim d$ implies $ac \sim bd$.
If $\sim$ is compatible with the group structure, then one can equip the quotient set $\GG/\!\!\sim$ (the set of all equivalence classes) with a binary operation defined as $[a][b] = [ab]$, where $[a]$ denotes the equivalence class of $a \in \GG$.
This is well defined as compatibility of $\sim$ with the group structure ensures that this binary operation does not depend on the specific representatives $a$ and $b$ of each equivalence class.
Moreover, one can easily check that $\GG/\!\!\sim$ equipped with this binary operation is a group with identity element $[e]$ and where $[a]^{-1} = [a^{-1}]$.

**Proposition.**
*Let $\GG$ be a group and let $\sim$ be an equivalence relation on $\GG$.
Then $\sim$ is compatible with the group structure of $\GG$ if and only if there exists a normal subgroup $\HH$ such that $a \sim b \Leftrightarrow ab^{-1} \in \HH$.*

> *Proof.*
> Assume that there exists a normal subgroup $\HH$ such that $a \sim b \Leftrightarrow ab^{-1} \in \HH$.
> Let us show that $\sim$ is compatible with the group structure of $\GG$.
> Let $a,b,c,d \in \GG$ such that $a \sim b$ and $c \sim d$.
> Then $ac(bd)^{-1} = acd^{-1}b^{-1} = acd^{-1}a^{-1}ab^{-1}$.
> We have $cd^{-1} \in \HH$ because $c \sim d$, which implies that $acd^{-1}a^{-1} \in \HH$ because $\HH$ is normal.
> We also have $ab^{-1} \in \HH$ because $a \sim b$, hence $ac(bd)^{-1} = acd^{-1}a^{-1}ab^{-1} \in \HH$ and $ac \sim bd$.
>
> Conversely, assume that $\sim$ is compatible with the group structure of $\GG$.
> Define $\HH$ as $[e]$, the class of the identity element.
> Let us first show that $\HH$ is a normal subgroup.
> Let $a,b \in \HH$, i.e., $a \sim e$ and $b \sim e$.
> Then, by compatibility of $\sim$ with the group structure, we have
> \[\begin{aligned}
   a \sim e & \Rightarrow ab^{-1} \sim eb^{-1} & & ( b^{-1} \sim b^{-1})\\
   & \Rightarrow ab^{-1}e \sim b^{-1}b & & (e \sim b) \\
   & \Rightarrow ab^{-1} \sim e. &
  \end{aligned}\]
> Hence $ab^{-1} \in \HH$ and $\HH$ is a subgroup.
>
> To show that $\HH$ is normal, let us show that for every $g \in \GG$, $g\HH g^{-1} \subseteq \HH$.
> Let $g \in \GG$ and $h \in \HH$. Then
> \[\begin{aligned}
   h \sim e & \Rightarrow gh \sim g & & ( g \sim g)\\
   & \Rightarrow ghg^{-1} \sim gg^{-1} & & (g^{-1} \sim g^{-1}) \\
   & \Rightarrow ghg^{-1} \sim e. &
  \end{aligned}\]
> Hence $ghg^{-1} \in \HH$ and $\HH$ is normal.
>
> It remains to show $a \sim b \Leftrightarrow ab^{-1} \in \HH$.
> By compatibility of $\sim$ with the group structure, we have $a \sim b \Rightarrow ab^{-1} \sim bb^{-1} \Rightarrow ab^{-1} \sim e$ and $ab^{-1} \sim e \Rightarrow ab^{-1}b \sim b \Rightarrow a \sim b$.
> Hence $a \sim b \Leftrightarrow ab^{-1} \sim e \Leftrightarrow ab^{-1} \in \HH$, which concludes the proof.

Hence, normal subgroups completely characterize the equivalence relations $\sim$ for which the quotient set $\GG/\!\!\sim$ can be naturally equipped with a group structure.

Let $\HH$ be a normal subgroup of $\GG$ and let $\sim$ be the equivalence relation defined by $a \sim b \Leftrightarrow ab^{-1} \in \HH$.
Then the quotient set $\GG/\!\!\sim$ equipped with the binary operation defined by $[a][b] = [ab]$ is called the *quotient group* associated with $\HH$ and denoted $\GG/\HH$.
Note that the order of $\GG/\HH$ is $[\GG:\HH]$, the index of $\HH$.

## Homomorphisms and Isomorphisms

Let $\GG$ and $\GG'$ be two groups.
A *group homomorphism* is a function $f$ from $\GG$ to $\GG'$ such that for every $a,b \in \GG$, $f(ab) = f(a) f(b)$.
If $f$ is bijective, then $f$ is called a *group isomorphism* and groups $\GG$ and $\GG'$ are said *isomorphic*, denoted $\GG \cong \GG'$.
If moreover $\GG= \GG'$, $f$ is called a *group automorphism*.

**Proposition.**
*Let $\GG$ and $\GG'$ be two groups and $f \colon \GG \to \GG'$ be a group homomorphism. Then:*
- *$f(e_{\GG}) = e_{\GG'}$,*
- *for every $a \in \GG$, $f(a^{-1}) = f(a)^{-1}$,*
- *for every subgroup $\HH$ of $\GG$, $f(\HH) \defeq \{f(a) \mid a \in \HH\}$ is a subgroup of $\GG'$,*
- *for every subgroup $\HH'$ of $\GG'$, $f^{-1}(\HH') \defeq \{a \in \GG \mid f(a) \in \HH'\}$ is a subgroup of $\GG$.*

Let $f \colon \GG \to \GG'$ be a group homomorphism.
The *kernel* of $f$ is the subset of $\GG$ defined as
\[
 \ker(f) \defeq \{a \in \GG \mid f(a) = e_{\GG'}\}.
\]
The *image* of $f$ is the subset of $\GG'$ defined as
\[
 \im(f) \defeq \{f(a) \mid a \in \GG\}.
\]

By the previous proposition, $\ker(f)$ is a subgroup of $\GG$ since it is equal to $f^{-1}(\{e_{\GG'}\})$ and $\im(f)$ is a subgroup of $\GG'$ since it is equal to $f(\GG)$.

<a name="th:first_isomorphism"></a>
**First Isomorphism Theorem.**
*Let $f \colon \GG \to \GG'$ be a group homomorphism.
Then $\ker(f)$ is a normal subgroup of $\GG$ and $\GG/\!\ker(f) \cong \im(f)$.*

> *Proof.*
> Let us first show that $\ker(f)$ is normal.
> Let $g \in \GG$ and $h \in \ker(f)$.
> Then
> \[
   f(ghg^{-1}) = f(g)f(h)f(g^{-1}) = f(g) e_{\GG'} f(g)^{-1} = e_{\GG'}.
  \]
> Hence $g \ker(f) g^{-1} \subseteq \ker(f)$ and hence $\ker(f)$ is normal.
>
> Consider now the mapping $\bar{f} \colon \GG/\!\ker(f) \to \im(f)$ defined by $\bar{f}([a]) = f(a)$.
> it is well-defined since
> \[
   [a] = [b] \Leftrightarrow ab^{-1} \in \ker(f) \Leftrightarrow f(ab^{-1}) = e_{\GG'} \Leftrightarrow f(a) = f(b).
  \]
> In other words, equivalence classes of $\GG/\!\ker(f)$ are just subsets of elements of $\GG$ with the same image under $f$.
> Consequently, the definition of $\bar{f}$ does not depend on the representative of the equivalence class.
>
> It is a group homomorphism since
> \[
   \bar{f}([a][b]) = \bar{f}([ab]) = f(ab) = f(a)f(b) = \bar{f}([a]) \bar{f}([b]).
  \]
> Moreover, it is injective since $\bar{f}([a]) = \bar{f}([b]) \Leftrightarrow f(a) = f(b) \Leftrightarrow [a] =[b]$.
> it is also surjective since any element in $h \in \im(f)$ is of the form $f(a)$ for some $a \in \GG$ and hence $h = \bar{f}([a])$.
> Hence, $\bar{f}$ is a group isomorphism.

## Group Generation

Let $\GG$ be a group and $A$ be a subset of $\GG$.
The *subgroup generated by $A$*, denoted $\langle A \rangle$, is the smallest subgroup of $\GG$ which contains $A$.
This can be shown to be equivalent to the two following alternative definitions:

- $\langle A \rangle$ is the intersection of all subgroups of $\GG$ containing $A$;
- $\langle A \rangle$ is the subgroup of all elements of $\GG$ that can be expressed as the finite product of elements of $A$ and inverse of elements of $A$.

If $A = \{a_1, \dots, a_k\}$ is finite, the subgroup generated by $A$ is also denoted $\langle a_1, \dots, a_k \rangle$.
When $\GG$ is abelian, then
\[
 \langle a_1, \dots, a_k \rangle = \left\{a_1^{z_1} \cdots a_k^{z_k} \mid z_1,\dots,z_k \in \ZZ \right\}.
\]

A group $\GG$ is said to be *finitely generated* is there exists a finite number of elements $g_1, \dots g_k \in \GG$ such that $\GG = \langle g_1, \dots, g_k \rangle$, in which case $\{g_1,\dots,g_k\}$ is called a *generating set* of $\GG$.

A group $\GG$ is said *cyclic* or *monogenous*[^cyclic] if there exists $g \in \GG$ such that $\GG = \langle g \rangle$, in which case $g$ is called a *generator* of $\GG$.

The *order* of an element $a \in \GG$ is the order of the subgroup $\langle a \rangle$.
If $\GG$ has infinite order, the order of an element $a \in \GG$ can be finite or infinite.

**Proposition.**
*Let $\GG$ be a group and $a \in \GG$ be a group element.
Then $a$ has finite order if and only if there exists $k \in \NN^*$ such that $a^k = e$.
In that case, $a$'s order is the smallest integer $n \ge 1$ such that $a^n = e$ and one has
\[
 \langle a \rangle = \{e, a, a^2, \dots, a^{n-1}\}.
\]*

<a name="prop:el_order_divides_group_order"></a>
**Proposition.**
*If $\GG$ has finite order $n$, then the order of any element $a \in \GG$ divides $n$.
In particular, for any $a \in \GG$, $a^n = e$.*

> *Proof.*
> The first part is a direct consequence of [Lagrange's theorem](#th:lagrange).
> For the second part, let $m$ be the order of $a$ and write $n=dm$.
> Then $a^n = a^{dm} = (a^m)^d = e^d = e$.

<a name="prop:order_divides_killer"></a>
**Proposition.**
*Let $\GG$ be some group and $a \in \GG$ be an element of order $n$.
Then for every $k \in \ZZ$, $a^k = e$ if and only if $n$ divides $k$.*

> *Proof.*
> If $n$ divides $k$ then $k = dn$ for some integer $d$, which implies $a^k = a^{dn} = (a^n)^d = e^d = e$.
> Conversely, assume that $a^k = e$.
> By Euclid's division lemma, there exists $q,r \in \ZZ$ such that $k = qn+r$ and $0 \le r < n$.
> Then $a^k = a^{qn+r} = (a^n)^q a^r = a^r$ and consequently $a^r =e$.
> This implies that $r=0$ as otherwise $a$ would have order $r < n$.
> Hence, $k = qn$ and $n$ divides $k$.

## Properties of Cyclic Groups

**Proposition.**
*Any cyclic group is abelian.*

> *Proof.*
> Let $\GG$ be a cyclic group, let $g$ be a generator of $\GG$, and let $a,b \in \GG$ be two group elements.
> Then there exists $k,\ell \in \ZZ$ such that $a = g^k$ and $b = g^\ell$, which implies that
> \[ ab = g^k g^\ell = g^{k+ \ell} = g^\ell g^k = ba. \]

<a name="prop:subgroup_cyclic"></a>
**Proposition.**
*Let $\GG$ be a cyclic group and $\HH$ be a subgroup of $\GG$.
Then $\HH$ and $\GG/\HH$ are cyclic.*

> *Proof.*
> Let $\GG$ be a cyclic group, $g$ be a generator of $\GG$, and $\HH$ be a subgroup of $\GG$.
>
> Let us first show that $\HH$ is cyclic.
> If $\HH = \{e\}$ then $\HH$ is clearly cyclic.
> Otherwise, let $n \ge 1 $ be the smallest integer such that $g^n \in \HH$ (which necessarily exists since $\HH$ contains at least one element different from $e$ and either this element or its inverse can be written $g^m$ for some $m \ge 1$).
> We will prove that $\HH = \langle g^n \rangle$.
> Clearly, $\langle g^n \rangle \subseteq \HH$ since $\HH$ is a subgroup.
> Conversely, let $a \in \HH$.
> Then $a = g^k$ for some $k \in \ZZ$.
> By Euclid's division lemma, there exists $q,r \in \ZZ$ such that $k = qn+r$ and $0 \le r < n$.
> Then $a = g^k = g^{qn+r} = (g^n)^q g^r$, and consequently $g^r \in \HH$.
> This implies that $r=0$ as otherwise this would contradict the minimality of $n$.
> Hence $a = (g^n)^q \in \langle g^n \rangle$ and $\HH \subseteq \langle g^n \rangle$.
> Thus, $\HH = \langle g^n \rangle$ and hence $\HH$ is cyclic.
>
> Let us now show that $\GG/\HH$ is cyclic.
> More precisely, let us prove that $\GG/\HH = \langle [g] \rangle$.
> Let $[a] \in \GG/\HH$ be an element of the quotient group specified by an arbitrary representative $a \in \GG$.
> Then there exists $k \in \ZZ$ such that $a = g^k$.
> Thus, $[a] = [g^k] = [g]^k$ and hence $[g]$ is a generator of $\GG/\HH$.


**Proposition.**
*Any group with prime order is cyclic and any element different from the identity element is a generator of $\GG$.*

> *Proof.*
> Let $\GG$ be a group of prime order $p$.
> Let $a \in \GG$ be an element different from the identity element and let $n$ be the order of $a$.
> Since the order of an element divides the order of the group by [Proposition](#prop:el_order_divides_group_order), one has either $n=1$ or $n=p$.
> Since $a \neq e$, one cannot have $n=1$, hence $n=p$ and $a$ generates $\GG$.

<a name="prop:order_of_gk"></a>
**Proposition.**
*Let $\GG$ be a cyclic group of order $n$ , $g$ be a generator of $\GG$, and $k \in \ZZ$.
Then the order of $g^k$ is $n/\gcd(n,k)$.*

> *Proof.*
> Let $d = \gcd(k,n)$ and let $\ell$ be the order of $g^k$.
> Then $(g^k)^{\ell} = g^{k\ell} = e$ and hence by [Proposition](#prop:order_divides_killer), $n \divides k\ell$, which implies $(\frac{n}{d}) \divides (\frac{k}{d})\ell$.
> Since $\gcd(\frac{n}{d},\frac{k}{d}) = 1$, this implies $\frac{n}{d} \divides \ell$.
>
> On the other hand, $(g^{k})^{\frac{n}{d}} = (g^n)^{\frac{k}{d}} = e^{\frac{k}{d}} = e$, and hence by [Proposition](#prop:order_divides_killer), $\ell \divides \frac{n}{d}$.
> We conclude that $\ell = n/d$.


<a name="prop:cyclic_generators"></a>
**Proposition.**
*Let $\GG$ be a cyclic group of order $n$ , $g$ be a generator of $\GG$, and $k \in \ZZ$.
 Then $\langle g^k \rangle = \GG$ if and only if $\gcd(n,k)=1$.
In particular, $\GG$ has $\phi(n)$ generators, where $\phi$ is Euler's function.*

> *Proof.*
> We have $\langle g^k \rangle = \GG$ if and only if the order of $g^k$ is $n$, which by [Proposition](#prop:order_of_gk) is equivalent to $\gcd(n,k) = 1$.
>
> For the second part of the proposition, write $\GG = \{e, g, \dots, g^{n-1}\}$.
> Then generators of $\GG$ are exactly elements of the form $g^k$ with $\gcd(n,k) = 1$ and hence there are $\phi(n)$ such elements.

<a name="prop:product_cyclic"></a>
**Proposition.**
*Let $\GG_1$ and $\GG_2$ be two cyclic groups of order $n_1$ and $n_2$ respectively.
Then the direct product $\GG_1 \times \GG_2$ is cyclic if and only if $\gcd(n_1,n_2) = 1$.
Moreover, $(g_1,g_2)$ is a generator of $\GG_1 \times \GG_2$ if and only if $g_1$ is a generator of $\GG_1$ and $g_2$ is a generator of $\GG_2$.*

> *Proof.*
> Let us first show the following lemma:
> Let $a_1 \in \GG_1$ be an element of order $k_1$ and $a_2 \in \GG_2$ be an element of order $k_2$.
> Then $(a_1,a_2)$ has order $\lcm(k_1,k_2)$.
>
> Let $e_1$ be the identity element of $\GG_1$ and $e_2$ be the identity element of $e_2$.
> Then
> \[
   (a_1,a_2)^k = (e_1,e_2) \Leftrightarrow a_1^k = e_1 \wedge a_2^k = e_2 \Leftrightarrow k_1 \divides k \wedge k_2 \divides k.
  \]
> The smallest such positive integer $k$ is by definition $\lcm(k_1,k_2)$, which proves the lemma.
>
> Let us now prove the proposition.
> Assume that $\GG_1 \times \GG_2$ is cyclic and let $(g_1,g_2)$ be a generator of $\GG_1 \times \GG_2$.
> Then clearly $g_1$ must be a generator of $\GG_1$ and $g_2$ a generator of $\GG_2$.
> By the previous lemma, $(g_1,g_2)$ has order $\lcm(n_1,n_2)$.
> On the other hand, $(g_1,g_2)$ has order $\abs{\GG_1 \times \GG_2} = n_1 n_2$ as it generates $\GG_1 \times \GG_2$.
> Hence, $\lcm(n_1,n_2) = n_1 n_2$, which implies $\gcd(n_1,n_2) = 1$.
>
> Conversely, assume that $\gcd(n_1,n_2) = 1$ and let $g_1$ be a generator of $\GG_1$ and $g_2$ be a generator of $\GG_2$.
> By the lemma, $(g_1,g_2)$ has order $\lcm(n_1,n_2) = n_1 n_2 = \abs{\GG_1 \times \GG_2}$ and hence generates $\GG_1 \times \GG_2$.


## Classification of Cyclic Groups

The set of integers $\ZZ = \{\dots, -2, -1 , 0, 1, 2, \dots\}$ equipped with addition is a cyclic group of infinite order with identity element $0$ and generators $1$ and $-1$.
For any integer $n \in \NN$, the set $n\ZZ \defeq \{nz \mid z \in \ZZ\}$ is a subgroup of $\ZZ$ of infinite order.
It is cyclic and generated by $n$ and $-n$.
These are in fact the only subgroups of $\ZZ$.

**Proposition.**
*Let $\GG$ be a subgroup of $\ZZ$.
Then there exists a unique $n \in \NN$ such that $\GG = n\ZZ$.*

> *Proof.*
> Le us show existence first.
> If $\GG = \{0\}$ then $\GG = 0\ZZ$.
> Assume now that $\GG \neq \{0\}$, implying that $\GG$ contains at least one positive integer (it contains at least one non-zero integer $a$ and either $a$ or $-a$ is positive).
> Let $n$ be the smallest positive integer in $\GG$.
> Let us show that $\GG = n\ZZ$.
> Clearly, $n\ZZ \subseteq \GG$ since $n \in \GG$.
> Conversely, let $k \in \GG$.
> By Euclid's division lemma, there exists $q,r \in \ZZ$ such that $k = qn+r$ and $0 \le r < n$.
> Since $n$ and $k$ are in $\GG$, $r = k-qn$ is also in $\GG$, which implies $r=0$ as otherwise this would contradict the minimality of $n$.
> Thus, $k = qn \in n\ZZ$ and $\GG \subseteq n\ZZ$.
> Hence, $\GG = n\ZZ$ which proves existence.
>
> For uniqueness, assume that $\GG = n\ZZ = n'\ZZ$ for $n,n' \in \NN$.
> Then $n \in n'\ZZ$ implies $n' \divides n$ and $n' \in n\ZZ$ implies $n \divides n'$, and consequently $n=n'$ since $n$ and $n'$ are in $\NN$.

For any $n \in \NN$, we can consider the quotient group $\ZZ/n\ZZ$.
The equivalence class of $a \in \ZZ$ is $[a] = \{a+nz \mid z \in \ZZ\}$, also called the *residue class of $a$ modulo $n$*.
There are $n$ distinct classes.
Hence, the index of $n\ZZ$ is $n$ and $\ZZ/n\ZZ$ has order $n$.
It is cyclic with generator $[1]$ (and $[-1]$).
More generally, by [Proposition](#prop:cyclic_generators), the generators of $\ZZ/n\ZZ$ are $[k]$ for $k \in \ZZ$ such that $\gcd(k,n)=1$.

Another way to think of $\ZZ/n\ZZ$ is as the set $\{0, 1 ,\dots, n-1\}$ equipped with "modulo $n$" addition and inverses.
More precisely, let $\ZZ_n$ be the set of symbols $\{\cl{0}, \cl{1}, \dots, \cl{n-1}\}$ equipped with the binary operation
\[
 \cl{a} + \cl{b} = \cl{(a+b) \bmod n}.
\]
This is a group with identity $\cl{0}$ and inverse $-\cl{a} = \cl{(-a) \bmod n}$.
One can easily see that $\ZZ/n\Z$ and $\ZZ_n$ are isomorphic, the isomorphism being $[a] \mapsto \cl{a \bmod n}$.
From now on, we will work with the latter formalism.

For any integer $m \in \ZZ$, $m\ZZ_n \defeq \{m\cl{u} \mid \cl{u} \in \ZZ_n\}$ is a subgroup of $\ZZ_n$.
What does this subgroup look like?
One has
\[\begin{aligned}
  \cl{a} \in m\ZZ_n & \Leftrightarrow \exists \cl{u} \in \ZZ_n, \cl{a} = m\cl{u} \\
  & \Leftrightarrow \exists u \in \{0,1,\dots,n-1\}, a = mu \bmod n \\
  & \Leftrightarrow \exists u \in \ZZ, a = mu \bmod n \\
  & \Leftrightarrow \exists u,v \in \ZZ, a = um+vn \\
  & \Leftrightarrow \gcd(m,n) \divides a & (\text{by Bézout's lemma}).
\end{aligned}\]
Hence, letting $d = \gcd(m,n)$, we see that $m\ZZ_n$ is the subset of $\ZZ_n$ containing multiples of $d$, i.e.,
\[
 m\ZZ_n = \{\cl{kd} \mid k=0,\dots,n/d-1\} = d\ZZ_n.
\]
In particular, $m\ZZ_n$ has order $n/d$ and index $d$.
(As we will see shorty, $m\ZZ_n$ being cyclic, one has $m\ZZ_n \cong \ZZ_{n/d}$.)

Again, we can show that these are in fact the only subgroups of $\ZZ_n$.

**Proposition.**
*Let $n \in \NN$ and $\GG$ be a subgroup of $\ZZ_n$.
Then there is a unique $d \in \NN$ such that $d \divides n$ and $\GG = d\ZZ_n$.*

> *Proof.*
> Let $\GG$ be a subgroup of $\ZZ_n$.
> Let us prove existence first.
> Let
> \[
   \HH \defeq \{a+kn \mid \cl{a} \in \GG, k \in \ZZ\}.
  \]
> Then $\HH$ is a subgroup of $\ZZ$: it is clearly non-empty and for $b,b' \in \HH$, $b = a+kn$ and $b' = a'+k'n$, one has
> \[\begin{aligned}
   b-b' & = (a-a') + (k-k')n \\
   & = (a-a' \bmod n) + k''n & \text{for some $k'' \in \ZZ$},
  \end{aligned}\]
> hence $b-b' \in \HH$.
> Thus, there exists $d \in \NN$ such that $\HH = d\ZZ$.
> Moreover, since $0 \in \GG$, $n \in \HH$ and hence $d \divides n$.
> Since $\GG = \{\cl{a} \mid a \in \HH \cap \{0,\dots,n-1\}\}$, it follows that $\GG = d\ZZ_n$.
>
> Let us now prove uniqueness.
> Assume that $\GG = d\ZZ_n = d'\ZZ_n$ for $d,d' \in \NN$ with $d$ and $d'$ dividing $n$.
> Since $\cl{d} \in d\ZZ_n$, this implies $\cl{d} \in d'\ZZ_n = \{\cl{kd'} \mid k=0,\dots,n/d'-1\}$ and hence $d' \divides d$.
> Conversely, $d \divides d'$ and hence $d=d'$ since $d$ and $d'$ are in $\NN$.

We can now prove the following two "structure theorems" stating that, up to isomorphism, $\ZZ$ and $\ZZ_n$ are the only cyclic groups of infinite, resp. finite order $n$.

<a name="th:structure_cyclic_groups"></a>
**Fundamental Theorem of Cyclic Groups, Infinite Order.**
*Let $\GG$ be a cyclic group of infinite order.
Then $\GG$ is isomorphic to $\ZZ$ and the subgroups of $\GG$ are exactly the subsets $\GG^n \defeq \{a^n \mid a \in \GG\}$ for $n \in \NN$.*

**Fundamental Theorem of Cyclic Groups, Finite Order.**
*Let $\GG$ be a cyclic group of order $n$.
Then $\GG$ is isomorphic to $\ZZ_n$ and the subgroups of $\GG$ are exactly the subsets $\GG^d \defeq \{a^d \mid a \in \GG\}$ for $d \in \NN$ such that $d \divides n$.
In particular, $\GG$ has exactly one subgroup of order $d$ for each divisor $d$ of $n$, namely $\GG^{n/d}$.*

> *Proof.*
> Let $\GG$ be a cyclic group and $g$ be a generator of $\GG$.
> Consider the mapping $f \colon \ZZ \to \GG$ defined by $f(k) = g^k$.
> Then $f$ is a group homomorphism since $f(k + \ell) = g^{k+\ell} = g^k g^{\ell} = f(k) f(\ell)$.
> It is clearly surjective since $g$ is a generator of $\GG$.
>
> Consider first the case where $\GG$ has infinite order.
> Let us show that $f$ is injective.
> Assume that $f(k) = f(\ell)$ for distinct integers $k$ and $\ell$ with $k < \ell$.
> Then $f(\ell-k) = g^{\ell-k} = e$, contradicting the fact that $g$ has infinite order.
> Hence, $f$ is an isomorphism form $\ZZ$ to $\GG$.
> Since the subgroups of $\ZZ$ are exactly $n\ZZ$ for $n \in \NN$, the subgroups of $\GG$ are exactly
> \[
   f(n\ZZ) = \{g^k \mid k \in n\ZZ\} = \{g^{kn} \mid k \in \ZZ\} = \{a^n \mid a \in \GG\},
  \]
> where the last equality follows from $g$ being a generator and hence $\GG = \{g^k \mid k \in \ZZ\}$.
>
> Consider now the case where $\GG$ has finite order $n$.
> Let us show that $\ker(f) = n\ZZ$.
> Since $g$ has order $n$, by [Proposition](#prop:order_divides_killer), for every $k \in \ZZ$, $g^k = e \Leftrightarrow k \divides n$, which exactly means that $k \in \ker(f) \Leftrightarrow k \in n\ZZ$.
> Hence, $\ker(f) = n\ZZ$.
> By the [first isomorphism theorem](#th:first_isomorphism), $\im(f) = \GG \cong \ZZ/n\ZZ \cong \ZZ_n$.
>
> Let $\bar{f} \colon \ZZ_n \to \GG$ be the isomorphism defined by $\bar{f}(\cl{k}) = g^k$.
> Since the subgroups of $\ZZ_n$ are exactly $d\ZZ_n$ for $d \in \NN$ such that $d \divides n$, the subgroups of $\GG$ are exactly
> \[
   \bar{f}(d\ZZ_n) = \{g^k \mid \cl{k} \in d\ZZ_n\} = \{g^{kd} \mid k =0,\dots,n-1\} = \{a^d \mid a \in \GG\}.
  \]

## Cauchy's Theorem for Abelian Groups

Lagrange's theorem states that the order of a subgroup of a finite group divides the order of the group.
Conversely, given a divisor $d$ of the order of the group, does there always exist a subgroup of order $d$?
A group where this property holds is called a *converse Lagrange theorem (CLT)* group.
The answer is [no in general](https://en.wikipedia.org/wiki/Lagrange%27s_theorem_(group_theory)#Existence_of_subgroups_of_given_order).
However, there are specific cases where the existence of a subgroup is guaranteed.
Cauchy's theorem states that for every group $\GG$ (non-necessarily abelian) of finite order and every *prime* divisor $p$ of $\abs{\GG}$, $\GG$ has a subgroup of order $p$.
Cauchy's theorem is a special case of [Sylow's (first) theorem](https://en.wikipedia.org/wiki/Sylow_theorems).

Here, we will only prove it in the easier case where $\GG$ is abelian.
Actually, a more general result (that will follow easily from the fundamental theorem of finite abelian groups) is that any finite abelian group is CLT.
In other words, for a finite abelian group of order $n$, the existence of a subgroup of order $d$ is guaranteed for any divisor $d$ of $n$, not only for prime divisors.

**Cauchy's Theorem (Abelian Case).**
*Let $\GG$ be an abelian group of finite order $n$.
Then, for every prime divisor $p$ of $n$, there exists a subgroup of $\GG$ of order $p$ (or, equivalently, there exists an element of $\GG$ of order $p$).*

> *Proof.*
> The equivalence between the two conclusions follows from the fact that a group of prime order is necessarily cyclic.
>
> Let $\{a_1,\dots,a_r\}$ be a generating set of $\GG$ and for $i \in \{1,\dots,r\}$ let $n_i$ denote the order of $a_i$.
> Consider the mapping $f \colon \langle a_1 \rangle \times \cdots \times \langle a_r \rangle \to \GG$ defined by $f(x_1,\dots,x_r) = x_1 \cdots x_r$.
> Since $\GG$ is abelian, it is a group homomorphism.
> Moreover, since $\{a_1,\dots,a_r\}$ is a generating set, $f$ is surjective (indeed, by definition of a generating set of an abelian group, any element $y \in \GG$ can be written as $y=a_1^{k_1}\cdots a_r^{k_r}$ for some integers $k_1,\dots,k_r \in \ZZ$ and for each $i \in \{1,\dots,r\}$, $a_i^{k_i} \in \langle a_i \rangle$).
> By the [first isomorphism theorem](#th:first_isomorphism), $\GG$ is isomorphic to $(\langle a_1 \rangle \times \cdots \times \langle a_r \rangle)/\ker(f)$, which implies that
> \[
   \abs{\GG} \cdot \abs{\ker(f)} = \abs{\langle a_1 \rangle \times \cdots \times \langle a_r \rangle} = n_1 \cdots n_r,
  \]
> and hence $n = \abs{\GG}$ divides $n_1 \cdots n_r$.
>
> Let $p$ be a prime divisor of $n$.
> Then $p$ divides $n_1 \cdots n_r$ and hence $p$ divides $n_i$ for some $i \in \{1,\dots,r\}$.
> Then we can write $n_i = kp$ and by [Proposition](#prop:order_of_gk), $a_i^k$ has order $n_i/\gcd(n_i,k)=n_i/k=p$, which concludes the proof.

## Exponent of a Group

The *exponent* of a group $\GG$ is defined as the smallest positive integer $m \ge 1$ such that $\forall g \in \GG$, $g^m=e$.
If no such integer exists, $\GG$ is said to have infinite exponent.
A finite group of order $n$ necessarily has finite exponent $m$ satisfying $m \le n$ (since by [Proposition](#prop:el_order_divides_group_order), $a^n = e$ for every $a \in \GG$).
Moreover, the order of any group element divides $m$ by [Proposition](#prop:order_divides_killer).
Conversely, a group with infinite exponent necessarily has infinite order.
However, a group with finite exponent is not necessarily finite.

In the following, we prove that an abelian group or finite exponent $m$ always contains a element of order $m$.
This will be a key lemma for proving the fundamental theorem of finite abelian groups.
Note that none of the three following propositions holds for a non-abelian group.

<a name="prop:order_of_product"></a>
**Proposition.**
*Let $\GG$ be an abelian group and $a_1$ and $a_2$ be two elements of respective order $n_1$ and $n_2$ such that $\gcd(n_1,n_2)=1$.
Then the order of $a_1 a_2$ is $n_1 n_2$.
More generally, if $a_1,\dots,a_r$ are $r$ group elements of respective orders $n_1,\dots,n_r$ such that $\gcd(n_1,\dots,n_r)=1$, then the order of $a_1 \cdots a_r$ is $n_1 \cdots n_r$.*

> *Proof.*
> Let $n$ be the order of $a_1 a_2$.
> Since $(a_1 a_2)^{n_1 n_2} = (a_1^{n_1})^{n_2} (a_2^{n_2})^{n_1} = e$, by [Proposition](#prop:order_divides_killer), we have $n \divides n_1 n_2$.
>
> On the other hand, since $(a_1 a_2)^n = e$, one has $a_1^n = a_2^{-n}$ and hence $a_1^n \in \langle a_2 \rangle$.
> This implies that $(a_1^n)^{n_2} = a_1^{n n_2} = e$, hence $n_1 \divides n n_2$.
> But since $\gcd(n_1,n_2) = 1$, $n_1 \divides n$.
> Symmetrically, one also has $n_2 \divides n$.
> Since $\gcd(n_1,n_2) = 1$, this implies $n_1 n_2 \divides n$ and hence $n = n_1 n_2$.
> The generalization can be proved by induction on $r$.

<a name="prop:exists_element_order_lcm"></a>
**Proposition.**
*Let $\GG$ be an abelian group and $a_1$ and $a_2$ be two elements of respective order $n_1$ and $n_2$.
Then there exists an element of $\GG$ of order $\lcm(n_1,n_2)$.*

> *Proof.*
> Let $p_1^{e_1} \cdots p_r^{e_r}$ be the prime factor decomposition of $\lcm(n_1,n_2)$.
> For each $i \in \{1,\dots,r\}$, $p_i^{r_i}$ divides either $n_1$ or $n_2$.
> Say it divides $n_1$ (the reasoning is similar if it divides $n_2$).
> Then, by [Proposition](#prop:order_of_gk), $a_1^{n_1/p_i^{r_i}}$ has order $n_1/\gcd(n_1,n_1/p_i^{r_i}) = p_i^{r_i}$.
> Hence, for each $i \in \{1,\dots,r\}$, there exists an element $b_i$ of order $p_i^{r_i}$.
> By [Proposition](#prop:order_of_product), $b_1 \cdots b_r$ has order $p_1^{e_1} \cdots p_r^{e_r} = \lcm(n_1,n_2)$.

<a name="prop:exists_element_order_exponent"></a>
**Proposition.**
*Let $\GG$ be an abelian group of finite exponent $m$.
Then there exists an element of $\GG$ of order $m$.*

> *Proof.*
> By [Proposition](#prop:order_divides_killer), the order of any group element divides $m$.
> In particular, all group elements have order at most $m$ and hence there exists a group element of maximal order $m' \le m$.
> Assume towards a contradiction that $m' < m$.
> If the order of every group element divides $m'$, then for every $a \in \GG$, $a^{m'} = e$, contradicting the minimality of $m$.
> Otherwise, assume that there is a group element of order $m''$ which does not divide $m'$.
> Then, by [Proposition](#prop:exists_element_order_lcm), there exists an element of order $\lcm(m',m'') > m'$ contradicting the maximality of $m'$.
> Hence, it must be that $m=m'$, which concludes the proof.

As a direct corollary, we have that a finite abelian group is cyclic if and only if its order is equal to its exponent.

## Structure Theorem for Finite Abelian Groups

This section presents the fundamental theorem of finite abelian groups, sometimes called Kronecker theorem.
As we will see, finite abelian groups can be "decomposed" in two ways.
The equivalence between these two decompositions relies on the following theorem.

**Chinese Remainder Theorem for Groups.**
*Let $n_1$ and $n_2$ be two positive integers.
Then
\[
 \ZZ_{n_1} \times \ZZ_{n_2} \cong \ZZ_{n_1 n_2}
\]
if and only if $\gcd(n_1,n_2) = 1$.*

> *Proof.*
> Assume that $\gcd(n_1,n_2) = 1$.
> Consider the mapping $f \colon \ZZ \to \ZZ_{n_1} \times \ZZ_{n_2}$ defined by $f(z) = (z \bmod n_1, z \bmod n_2)$.
> One can easily check that $f$ is a group homomorphism.
> Moreover, $z \in \ker(f) \Leftrightarrow (n_1 \divides z) \wedge (n_2 \divides z) \Leftrightarrow n_1 n_2 \divides z$, where the last equivalence follows from $\gcd(n_1,n_2) =1$.
> Hence, $\ker(f) = n_1 n_2 \ZZ$.
> By the [first isomorphism theorem](#th:first_isomorphism), $\ZZ_{n_1n_2} \cong \ZZ/n_1 n_2 \ZZ$ is isomorphic to $\im(f)$.
> In particular, $\abs{\im(f)} = \abs{\ZZ_{n_1n_2}} = n_1 n_2 = \abs{\ZZ_{n_1} \times \ZZ_{n_2}}$, hence $\im(f) = \ZZ_{n_1} \times \ZZ_{n_2}$.
> Thus, $\Z_{n_1n_2} \cong \ZZ_{n_1} \times \ZZ_{n_2}$.
>
> Conversely, assume that $\gcd(n_1,n_2) = d > 1$.
> By [Proposition](#prop:product_cyclic), $\ZZ_{n_1} \times \ZZ_{n_2}$ is not cyclic.
> As $\Z_{n_1n_2}$ is cyclic, these two groups cannot be isomorphic.

<a name="th:finite_abelian_groups"></a>
**Fundamental Theorem of Finite Abelian Groups.**
*Let $\GG$ be a non-trivial finite abelian group. Then:*
- ***(primary decomposition):*** *$\GG$ is isomorphic to a direct sum of cyclic groups
\[
 \ZZ_{p_1^{r_1}} \times \cdots \times \ZZ_{p_t^{r_t}},
\]
where the $p_i$'s are (not necessarily distinct) primes and the $r_i$'s are positive integers.
This decomposition is unique up to the order of factors.
The prime powers $p_1^{r_1},\dots,p_t^{r_t}$ are called the elementary divisors of $\GG$.*
- ***(invariant factor decomposition):*** *$\GG$ is isomorphic to a direct sum of cyclic groups
\[
 \ZZ_{n_1} \times \cdots \times \ZZ_{n_\ell},
\]
where the $n_i$'s are positive integers such that for $i=1, \dots, \ell-1$, $n_i \divides n_{i+1}$.
This decomposition is unique and $n_{\ell}$ is the exponent of the group.
The integers $n_1,\dots,n_{\ell}$ are called the invariant factors of $\GG$.*

> *Proof.*
> TODO

Consider for example $\ZZ_6 \times \ZZ_{15}$.
What are the primary and invariant factor decompositions of this group?
By the Chinese remainder theorem, we have
\[\begin{aligned}
 \ZZ_6 \times \ZZ_{15} & = \ZZ_{2 \times 3} \times \ZZ_{3 \times 5} \\
 & \cong \ZZ_2 \times \ZZ_3 \times \ZZ_3 \times \ZZ_5 \\
 & \cong \ZZ_3 \times \ZZ_{30}.
\end{aligned}\]
The penultimate form is the primary decomposition, while the last form is the invariant factor decomposition.

The smallest abelian non-cyclic group is $\ZZ_2 \times \ZZ_2$ of order 4, usually called the *Klein group*.

**Proposition.**
*A finite abelian group is cyclic if and only if $\ell=1$ in its invariant factor decomposition.*

For an integer $n$, one may ask how many different abelian groups of order $n$ there are, up to isomorphism.
Let $\pi$ denote the [partition function](https://en.wikipedia.org/wiki/Partition_function_(number_theory)) defined as follows: for an integer $n$, $\pi(n)$ is the number of distinct ways of writing $n$ as a sum of positive integers, where the order of these integers does not matter.
For example, $\pi(3) = 3$ since $3$ has $3$ partitions: $1+1+1$, $1+2$, and $3$.

**Number of Finite Abelian Groups of Order $n$.**
*Let $n \ge 1$ be an integer and let its decomposition in prime factors be $n =p_1^{r_1} \cdots p_k^{r_k}$.
Then the number of abelian groups of order $n$, up to isomorphism, is \[\pi(r_1) \cdots \pi(r_k),\] where $\pi$ is the partition function.
In particular, there is a unique abelian group of order $n$ up to isomorphism (namely $\ZZ_n$) if and only if $n$ is square-free, i.e., $r_1 = \cdots = r_k =1$.*

For example, there is a unique (up to isomorphism) abelian group of order $15 = 3 \times 5$, namely $\ZZ_3 \times \ZZ_5 \cong \ZZ_{15}$ (primary/invariant factor decomposition).
On the other hand, there are two (up to isomorphism) abelian groups of order $12 = 2^2 \times 3$, namely $\ZZ_4 \times \ZZ_3 \cong \ZZ_{12}$ and $\ZZ_2 \times \ZZ_2 \times \ZZ_3 \cong \ZZ_{2} \times \ZZ_{6}$.

## Structure Theorem for Finitely Generated Abelian Groups

We now consider abelian groups which are finitely generated (but not necessarily of finite order).

Let $\GG$ be an abelian group.
For $n \in \NN^*$, an element $a \in \GG$ is said to be an *$n$-torsion element* if $a^n = e$ (or equivalently, if $a$ has finite order $k$ dividing $n$).
An element $a \in \GG$ is said to be a *torsion element* if it has finite order.

**Proposition.**
*Let $\GG$ be an abelian group.
Then the set of all $n$-torsion elements of $\GG$, denoted $\GG[n]$, is a subgroup called the $n$-torsion subgroup of $\GG$ and the set of all torsion elements of $\GG$, denoted $\GG_T$, is a subgroup called the torsion subgroup of $\GG$.*

> *Proof.*
> Since $e$ is clearly an $n$-torsion element, $\GG[n]$ is non-empty.
> Let $a$ and $b$ be two $n$-torsion elements.
> Then $(ab^{-1})^n = a^n (b^{n})^{-1} = e e^{-1} = e$, hence $ab^{-1}$ is an $n$-torsion element.
> Hence $\GG[n]$ is a subgroup of $\GG$.
>
> Similarly, $e$ is a torsion element, hence $\GG_T$ is non-empty.
> Let $a$ and $b$ be two torsion elements of order respectively $k$ and $\ell$.
> Then $(ab^{-1})^{k\ell} = a^{k \ell} (b^{\ell k})^{-1} = e^\ell(e^k)^{-1} = e$.
> Hence $ab^{-1}$ has finite order, i.e., it is a torsion element.
> Hence $\GG_T$ is a subgroup of $\GG$.

If $\GG = \GG_T$ then $\GG$ is called a *torsion group* (or *periodic group*).
If $\GG = \{0\}$ then $\GG$ is said *torsion-free*.

**Fundamental Theorem of Finitely Generated Abelian Groups.**
*Let $\GG$ be a finitely generated abelian group.
Let $\GG_T$ be the torsion subgroup of $\GG$.
Then $\GG_T$ is finite and abelian and there exists a free abelian subgroup $\FF$ such that $\GG = \FF \times \GG_T$.
In particular, there exists a integer $r \in \NN$ (called the free rank or simply rank of $\GG$) and integers $n_1, \dots, n_{\ell}$ with $n_1 \divides \cdots \divides n_{\ell}$ such that
\[
 \GG \cong \ZZ^r \times \ZZ_{n_1} \times \cdots \times \ZZ_{n_{\ell}}.
\]
Integers $r$ and $n_1,\dots,n_{\ell}$ are unique.*

> *Proof.*
> TODO

----

[^sum]: While the direct sum is the same as the direct product for a finite number of groups, this is not the case for a infinite number of groups. See [this StackExchange question](https://math.stackexchange.com/questions/39895/the-direct-sum-oplus-versus-the-cartesian-product-times).

[^cyclic]: Sometimes *cyclic* is used for groups which are both monogenous and finite, which makes sense since for an infinite monogenous group such as $\ZZ$ one never "cycles back" when computing $g, g^2, \dots$