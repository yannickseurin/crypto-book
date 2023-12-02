## Small Subgroup Attacks

Actually, the puzzle webpage was giving us a hint by advising us to read a paper by Cremers and Jackson [[CJ19](../../references.md#CJ19)] about so-called *small subgroup attacks*.
Small subgroup attacks (more specifically, small subgroup *key recovery* attacks, as there are other flavors such as small subgroup *confinement* attacks that we won't touch here) have been proposed by Lim and Lee in 1997 [[LL97](../../references.md#LL97)] and vulnerable implementations have been found in the wild [[VAS+17](../../references.md#VAS+17)].

In a nutshell, small subgroup attacks occur when some party, holding some secret scalar $x$, is tricked into computing $x P$ thinking $P$ generates some subgroup $\GG$ (of some larger group $\mathbb{H}$) where the discrete logarithm problem is hard whereas in fact $P$ generates another subgroup $\GG'$ of $\mathbb{H}$ where the discrete logarithm problem is much easier.
This can happen for example in Diffie-Hellman key exchange or in some blind signing protocols where the party computing the scalar multiplication $x P$ gets the point $P$ from another (potentially malicious) party rather than from trusted parameters.
Before going into details, we need to recall some basic facts about groups and their subgroups.

### Computing Discrete Logarithms in Groups of Composite Order

Let $\GG$ be a finite group of order $n$.
Then, by {{tref: thm:lagrange}}, the order of any subgroup of $\GG$ divides $n$.
Moreover, if $\GG$ is cyclic, then every subgroup of $\GG$ is also cyclic ({{ref: prop:subgroup_cyclic}}) and by the {{tref: thm:structure_cyclic_groups}}, for each divisor $d$ of $n$ there exists a unique subgroup of order $d$.

Small subgroup attacks derive from a simple observation about the discrete logarithm problem in groups of *composite* order.
Let $\GG$ by a cyclic group of composite order $n = n_1 n_2$ where $n_1$ and $n_2$ are coprime and let $G$ be a generator of $\GG$.
Say we want to solve the discrete logarithm problem for a group element $X \in \GG$ in base $G$, i.e., we want to find the unique integer $x \in \{0, \dots ,n-1\}$ such that $X = x G$.
Then, we can take advantage of the structure of group $\GG$ as follows:

- First, we compute $G'_1 = n_2 G$ and $X'_1 = n_2 X$.
Then $G'_1$ has order $n_1$ (why?) and
\[
 X'_1 = n_2 (x G) = x (n_2 G) = x G'_1.
\]
If we let $x_1 \in \{0, \dots ,n_1-1\}$ denote the discrete logarithm of $X'_1$ in base $G'_1$, which can be computed in $O(\sqrt{n_1})$ group operations with generic algorithms, then $X'_1 = x G'_1$ implies that $x = x_1 \bmod n_1$.
- Similarly, one can compute $G'_2 = n_1 G$ and $X'_2 = n_1 X$.
Then the discrete logarithm $x_2$ of $X'_2$ in base $G'_2$ can be computed in $O(\sqrt{n_2})$ group operations and satisfies $x = x_2 \bmod n_2$.
- Finally, one can combine the two equations $x = x_ 1\bmod n_1$ and $x = x_2 \bmod n_2$ using the [Chinese remainder theorem](https://en.wikipedia.org/wiki/Chinese_remainder_theorem) to obtain $x$ (modulo $n_1 n_2 = n$).

All in all, $x$ has been computed in $O(\sqrt{n_1}+\sqrt{n_2})$ rather than $O(\sqrt{n_1 n_2})$ group operations.

The procedure above is the basis of the [Pohlig-Hellman algorithm](https://en.wikipedia.org/wiki/Pohlig%E2%80%93Hellman_algorithm) which computes discrete logarithms in a group of order $n = \prod p_i^{e_i}$ in $O(\sum e_i \sqrt{p_i})$ group operations, assuming the factorization of $n$ is known.
Hence, if one wants 128-bit security for the discrete logarithm problem, a *necessary* condition is that the order of the group has a prime factor of size at least 256 bits.

This explains why cryptographers are so obsessed with groups of prime order: by Lagrange's theorem, groups of prime order don't have any subgroups other than the trivial ones (itself and the subgroup of order 1 consisting of the identity element) and hence the discrete logarithm problem cannot be "broken" into smaller pieces as above (which, of course, does not imply that the DL problem cannot by broken by other, non-generic means).

### Small Subgroup Attacks

Groups used in cryptography often have composite order.
For example, the multiplicative group $\ZZ_p^*$ of integers modulo some prime number $p$ has order $p-1$, which is always even.
For elliptic curves, although it is possible to construct secure curves with a prime number of points such as [secp256k1](https://en.bitcoin.it/wiki/Secp256k1), many curves that are attractive for efficiency reasons (such as [twisted Edwards curves](https://en.wikipedia.org/wiki/Twisted_Edwards_curve)) have order $h*p$, where $p$ is prime and $h$ is small (usually 4 or 8).

When using such groups of composite order $n$, a "base" point $G$ of prime order $r$ is usually specified.
As long as group elements used in a protocol are computed as multiples of $G$, one never "gets out" of the prime-order subgroup $\langle G \rangle$.
The [index](../../mathematical-preliminaries/groups.md#subgroups) of subgroup $\langle G \rangle$, i.e., the ratio $h \defeq n/r$, is often called the *cofactor* of $G$ in a cryptographic context.

However, what happens if Alice, holding some secret value $x$, is tricked by an attacker into computing $Q \defeq x P$ where $P$ is not in subgroup $\langle G \rangle$?
If $Q$ is made available to the attacker, then it can use the Pohlig-Hellman algorithm to compute $x \bmod k$ where $k$ is the "smooth" part of $P$'s order (meaning, informally, the product of all "small" prime factors of $P$'s order), which might be somewhere between a few bits (if $h$ is $4$ or $8$) to enough to retrieve $x$ entirely.
Note that $P$ does not have have to actually *be* in a small subgroup for the attack to work, the only condition is that some multiples of $P$ be in small subgroups (equivalently, that $\langle P \rangle$ has small subgroups).
If, for example, $P$ generates the entire ambient group of order $n = h * r$, then one can "project" the discrete logarithm on the subgroup of order $h$ by computing $P' = r P$ and $Q' = r Q$ and work with the pair $(P',Q')$ instead.

Observe that the maximal amount of information that can leak about secret $x$ in a small subgroup attack is $\log_2 h$ bits, hence having a "small" cofactor as 4 or 8 might seem benign.
However, there are actually plenty of other ways a small cofactor can mess with your protocol, an interesting example being [Monero's multiple-spend bug](https://www.getmonero.org/2017/05/17/disclosure-of-a-major-bug-in-cryptonote-based-currencies.html).

What about pairing-based cryptography?
While there exists families of pairing-friendly curve pairs where the first "small" curve has prime order, such as Baretto-Naehrig (BN) curves [[BN05](../../references.md#BN05)], the second "large" curve always has composite order with a very large cofactor.
For members of the BLS family, even the first small curve has composite order.
Hence, small subgroup attacks are especially relevant when using pairing-based cryptographic primitives.
For more information, see [[BCM+15](../../references.md#BCM+15)].

### Subgroup Membership Tests

How can we prevent small subgroup attacks?
By performing a *subgroup membership test*.
Given a group $\GG$ of composite order $n$ and a prime factor $r$ of $n$, an element $P$ is in the subgroup of order $r$ if and only if $rP = 0_{\GG}$.
This test is simple yet rather costly since $r$ is large.
However, there are a number of tricks to make subgroup membership testing more efficient [[HGP22](../../references.md#HGP22)].
For curves with small cofactors (4 or 8), some techniques such as Decaf [[Ham15](../../references.md#Ham15)] or [Ristretto](https://ristretto.group/) allow to "eliminate the cofactor" and construct a prime-order group.

### Invalid Curve Attacks

Finally, assuming we work with a prime-order curve such as secp256k1, is it safe to use any point $P$ received from an untrusted source without verification? If the curve has prime order $r$, any point other than 0 has order $r$, right? Well, not exactly: if $P$ was receive in so-called "uncompressed" form (meaning both coordinates $x$ and $y$ were explicitly given), $P$ might not be on the curve at all!
It might be on another curve with a different equation but where the same addition formulas apply.
If this "ghost" curve has a smooth order, computing $sP$ might end up leaking information about the secret scalar $s$ exactly as described above.
This is called an *invalid curve attack* and has affected for example some implementations of TLS [[JSS15](../../references.md#JSS15)] and the Bluetooth protocol [[BN19](../../references.md#BN19)].

Let's now see how to apply all this to the puzzle.
