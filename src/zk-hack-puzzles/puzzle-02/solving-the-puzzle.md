## Solving the Puzzle

### Subgroup Membership Checks

Now that we know about small subgroup attacks, a natural idea is to check whether $G_1$ and $G_2$ are in the "correct" subgroups, i.e., the subgroups of large prime order specified by the BLS12-381 parameters.
Fortunately, the arkworks library has methods for that:

```rust
{{#rustdoc_include ../../../puzzles/zkhack-trusted-setup/src/bin/verify-trusted-setup.rs:check}}
```

which yields

```text
G1 is on the curve.
G1 is not in the correct subgroup.
G2 is on the curve.
G2 is not in the correct subgroup.
```

Nice! So we know that we must mount a small subgroup attack to retrieve $s$ and solve the puzzle.

You can have a look at the code of methods `is_on_curve` and `is_in_correct_subgroup_assuming_on_curve` [here](https://docs.rs/ark-ec/0.3.0/src/ark_ec/models/short_weierstrass_jacobian.rs.html#127-149).
Note that if the point that is being tested is not for certain on the curve, one should call *both* methods: indeed, `is_in_correct_subgroup_assuming_on_curve` could return `true` when applied to a point which is not on the curve (see [invalid curve attacks](./small-subgroup-attacks.md#invalid-curve-attacks)).

### Solving the Puzzle with Sage

From now on we will be using Sage as it is quite convenient to do all kinds of computations on elliptic curves, as we'll see.

Recall that BLS12-381 actually consists of two curves in short Weierstrass form:

- $E_1(\mathbb{F}_p): y^2 = x^3 + a_1 x + b_1$, defined over the prime field $\mathbb{F}_p$ where $p$ is a 381-bit prime,
- $E_2(\mathbb{F}_{p^2}): y^2 = x^3 + a_2 x + b_2$, defined over the quadratic extension of $\mathbb{F}_p$ obtained from the irreducible polynomial $u^2 + 1$.

The BLS12-381 parameters also specify two points $P_1$ and $P_2$ (more usually denoted $G_1$ and $G_2$ but this conflicts with the puzzle notation), both of prime order $r$, but we won't need them here.

Although I'm not aware of any normative reference for BLS12-381, all the parameters can be found in [the IETF Internet-Draft for pairing-friendly curves](https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-pairing-friendly-curves-11#name-bls-curves-for-the-128-bit-).
With that, we can construct the two curves in Sage (see [here](https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_finite_field.html) for the Sage documentation about elliptic curves over finite fields):

```python
p = 0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab
F1 = GF(p)
a1 = 0
b1 = 4
E1 = EllipticCurve(F1, [a1, b1])

R.<x> = PolynomialRing(F1)
F2.<u> = F1.extension(x^2+1)
a2 = 0
b2 = 4*(1+u)
E2 = EllipticCurve(F2, [a2, b2])
```

Although we know that $G_1$ is not in the subgroup of large prime order $r$, we don't know $G_1$'s order yet.
We will compute it with Sage.
For this, we display this point by using `println!("G1 = {}", _ts1[0]);` in the `main` function of the puzzle and we get:

```text
G1 = GroupAffine(x=Fp384 "(0F99F411A5F6C484EC5CAD7B9F9C0F01A3D2BB73759BB95567F1FE4910331D32B95ED87E36681230273C9A6677BE3A69)",
 y=Fp384 "(12978C5E13A226B039CE22A0F4961D329747F0B78350988DAB4C1263455C826418A667CA97AC55576228FC7AA77D33E5)")
```

We can now copy-paste these values into our Sage file to define $G_1$ there and ask Sage to compute $G_1$'s order $n_1$ and even factor it:

```python
G1 = E1(0x0F99F411A5F6C484EC5CAD7B9F9C0F01A3D2BB73759BB95567F1FE4910331D32B95ED87E36681230273C9A6677BE3A69, \
        0x12978C5E13A226B039CE22A0F4961D329747F0B78350988DAB4C1263455C826418A667CA97AC55576228FC7AA77D33E5)
n1 = G1.order()
L1 = list(n1.factor())
print("The factorization of G1's order n1 is:")
for l in L1:
    print(l)
```

which prints the list of prime factors together with their addicity:

```text
The factorization of G1's order n1 is:
(3, 1)
(11, 1)
(10177, 1)
(859267, 1)
(52437899, 1)
(52435875175126190479447740508185965837690552500527637822603658699938581184513, 1)
```

The largest prime factor is actually $r$, the order of subgroups $\GG_1 = \langle P_1 \rangle$ and $\GG_2 = \langle P_2 \rangle$ one is supposed to work with when using BLS12-381.
It's 255-bit, no way we're gonna be able to compute $s \bmod r$ of course.
On the other hand, the other factors seem small enough so that we can apply the small subgroup attack to compute $s \bmod k_1$ where $k_1 = n_1/r$.
As $k_1$ is a 64-bit number (check with `print(numerical_approx(log(n1/r, 2)))`), we'll be able to get roughly 64 bits of information about $s$, a good start.


Recall the overall principle of the attack as described [earlier](./small-subgroup-attacks.md#computing-discrete-logarithms-in-groups-of-composite-order).
Let $H_1 = s G_1$ (this is `_ts1[1]` from the puzzle data).
We compute $G'_1 = r G_1$ (whose order is $k_1 = n_1/r$) and $H'_1 = r H_1$.
Then
\[
 H'_1 = r (s G_1) = s (r G_1) = s G'_1 = (s \bmod k_1) G'_{1},
\]
where the last equality follows from the fact that $G'_1$ has order $k_1$.
We can now compute the discrete logarithm $s_1$ of $H'_1$ in base $G'_1$, which will give us our first equation $s \bmod k_1 =s_1$.

Sage has a generic [`discrete_log` function](https://doc.sagemath.org/html/en/reference/groups/sage/groups/generic.html#sage.groups.generic.discrete_log) (where generic means it works over any group) which uses various algorithms (Pohlig-Hellman, Pollard-rho, ...) and that we can call directly here:

```python
H1 = E1(0x16C2385B2093CC3EDBC0F2257E8F23E98E775F8F6628767E5F4FC0E495285B95B1505F487102FE083E65DC8E9E3A9181, \
         0x0F4B73F63C6FD1F924EAE2982426FC94FBD03FCEE12D9FB01BAF52BE1246A14C53C152D64ED312494A2BC32C4A3E7F9A)
s1 = discrete_log(r*H1, r*G1, operation='+')
print("s mod k1 =")
print(s1)
```

We get:

```text
s mod k1 =
2335387132884273659
```

How do we continue from here? Could we use information contained in points $(s^2\ G_1, \dots, s^{62} G_1)$?
Repeating all the above with $s^2 G_1$ for example would allow us to compute $s^2 \bmod k_1$, but this is simply $s_1^2 \bmod k_1$ which we can compute from $s_1$ so this does not give us new information.

Maybe we could use points $(\alpha G_1, \alpha s G_1, \dots, \alpha s^{31} G_1, \beta G_1, \beta s G_1, \dots, \beta s^{31} G_1)$ then?
This seems implausible though: as $\alpha$ and $\beta$ are just random scalars independent from $s$, we could just have generated points with the same distribution by ourselves from $G_1, sG_1, \dots, s^{31} G_1$ by drawing random values for $\alpha$ and $\beta$ and computing the corresponding scalar multiplications.
These extra points rather look like some red herring.

Hence, we now consider the set of points on the second curve $E_2(\mathbb{F}_{p^2})$.
Again, we define point $G_2$ in Sage and try to compute its order:

```python
G2 = E2(0x1173F10AD9F2DBEE8B6C0BB2624B05D72EEC87925F5C3633E2C000E699A580B842D3F35AF1BE77517C86AEBCA1130AE4 \
      + 0x0434043A97DA28EF7100AE559167FC613F057B85451476ABABB27CFF0238A32831A0B4D14BA83C4F97247C8AC339841F * u, \
        0x0BEBEC70446CB91BB3D4DC5C8412915E99D612D8807C950AB06BC41583F528FDA9F42EC0FE7CD2991638187EF44258D3 \
      + 0x19528E3B5C90C73A7092BB9AFDC73F86C838F551CCD9DBBA5CC6244CF76AB3372193DBE5B62383FAAE728728D4C1E649 * u)
n2 = G2.order()
```

Unlike for $G_1$ where Sage returned the answer pretty quickly, this seems to take quite some time (I killed the process after a few minutes).
To understand why, one has to keep two things in mind:

- First, the second curve $E_2(\mathbb{F}_{p^2})$ is much bigger than the first one.
By [Hasse's bound](https://en.wikipedia.org/wiki/Hasse%27s_theorem_on_elliptic_curves), its order is close to $p^2$ (hence roughly 762-bit long).
Sage can compute it very quickly with `E2.cardinality()`, however if we try to factor it, this seems quite long again, presumably because it has at least two large prime factors.
- Second, computing the order of $G_2$ efficiently actually requires the factorization of the order of the ambient group, i.e., $|E_2|$ (see Algorithm 4.79 of [Chapter 4 of the Handbook of Applied Cryptography](https://cacr.uwaterloo.ca/hac/about/chap4.pdf)).

Hence, the reason why `G2.order()` takes so much time is presumably that it tries to factor $|E_2|$ first.
Could we help Sage here? We know at least one large prime factor of $|E_2|$, namely $r$, the prime order of subgroup $\GG_2 = \langle P_2 \rangle$ defined by BLS12-381 parameters.
The number $c_2 = |E_2|/r$ is called the *cofactor* of subgroup $\GG_2$ in $E_2$.
We can compute it in Sage and try to factor it:

```python
E2_order = E2.cardinality()
c2 = E2_order/r
L2 = list(c2.factor())
print("The factorization of cofactor c2 = |E2|/r is:")
for l in L2:
    print(l)
```

This gives:

```text
The factorization of cofactor c2 = |E2|/r is:
(13, 2)
(23, 2)
(2713, 1)
(11953, 1)
(262069, 1)
(402096035359507321594726366720466575392706800671181159425656785868777272553337714697862511267018014931937703598282857976535744623203249, 1)
```

We can see that the cofactor has a very large (448-bit long) prime factor that will be denoted $r'$ in the following.
What prevented Sage from factoring $|E_2|$ efficiently was the $r r'$ part, but Sage can factor $|E_2|/r$ very quickly because it has only one large prime factor.

Now that we know the factorization of $|E_2|$, can we hint it to Sage so that it can compute $G_2$'s order efficiently?
It turns out that there is another function called [`order_from_multiple`](https://doc.sagemath.org/html/en/reference/groups/sage/groups/generic.html#sage.groups.generic.order_from_multiple) to which we can pass a multiple $m$ of $G_2$'s order (we will use $m=|E_2|$ as $G_2$'s order necessarily divides $|E_2|$) and the factorization of $m$.
So we give it a try (after inserting the missing factor $r$ in the list to obtain the factorization of $|E_2|$):

```python
L2.insert(5, (r,1))
n2 = order_from_multiple(G2, E2_order, factorization=L2, operation='+')
```

This time, Sage returns instantly.
Trying to factor the result again takes time, meaning $G_2$'s order $n_2$ is a multiple of $rr'$, but as before we can factor $n_2/r$ and insert $r$ afterwards to get the result quickly:

```
L3 = list((n2/r).factor())
print("The factorization of n2/r is:")
for l in L3:
    print(l)
L3.insert(5, (r,1))
print("The factorization of G2's order n2 is:")
for l in L3:
    print(l)
```

We obtain:
```text
The factorization of G2's order n2 is:
(13, 1)
(23, 1)
(2713, 1)
(11953, 1)
(262069, 1)
(52435875175126190479447740508185965837690552500527637822603658699938581184513, 1)
(402096035359507321594726366720466575392706800671181159425656785868777272553337714697862511267018014931937703598282857976535744623203249, 1)
```

Now we have all the information we need and can perform a second small subgroup attack by clearing the large factor $r r'$ from $G_2$'s order, which will give us $s \bmod k_2$ where $k_2 = n_2 /(r r')$, which is a 52-bit integer.
So we compute $G'_2 = (r r') G_2$ and $H'_2 = (r r') H_2$ and ask sage to compute the discrete logarithm of $H'_2$ in base $G'_2$.
There is a catch though: as we're working in $E_2$, the order of which Sage cannot factor quickly, we need to pass Sage the order of $G'_2$, namely $k_2$:

```python
s2 = discrete_log(r * rp * H2, r * rp * G2, ord=k2, operation='+')
print("s mod k2 =")
print(s2)
```

We get:

```text
s mod k2 =
712318409117070
```

As $k_1$ and $k_2$ are coprime, we can now combine the two equations using the Chinese remainder theorem to compute $s_{1,2} = s \bmod (k_1 k_2)$:

```python
s12 = crt([s1, s2], [k1, k2])
print("s mod k1 * k2 =")
print(s12)
```

We get

```text
s mod k1 * k2 =
5592216610550884993006174526481245
```

As $k = k_1 k_2$ is a 115-bit integer and $s$ is 128-bit long, we miss roughly 13 bit of information, which is sufficiently small to allow exhaustive search: writing $s = i k + s_{1,2}$, we can loop through all integers $i = 0, \dots, 2^{13}$ and check whether $(ik + s_{1,2})G_1 = H_1$:

```python
for i in range(2^13):
	s = i*k + s12
	if s * G1 == H1:
		print("discrete log found:")
		print(s)
		break
```

We're finally done:

```text
discrete log found:
114939083266787167213538091034071020048
```

### Conclusion

Keep in mind that it is possible to create instances of the `GroupAffine` type which are not in the correct subgroup of the related curve through the `deserialize_unchecked` function.
Hence, subgroup membership tests should always be performed before creating an instance whose coordinates come from an untrusted source.
If you feel this is a step aside from the type safety philosophy of Rust, it is possible to define an enum such as

```rust
enum ECPoint<P> {
    Checked(GroupAffine<P>),
    Unchecked(GroupAffine<P>),
}
```

and to implement a public constructor for it that will never return an `Unchecked` variant if there is a possibility that the corresponding point is not in the correct subgroup.
