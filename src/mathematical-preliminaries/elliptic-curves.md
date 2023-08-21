> **Chapter status:** in progress
>
> **TODO:**

# Elliptic Curves

## Contents

<!-- toc -->

## Generalities

Let $\FF$ be a field of characteristic $p>3$.[^char]
Let $a,b \in \FF$ be two field elements such that
\[
 4a^3 + 27b^2 \neq 0.
\]
An elliptic curve in *short Weierstrass form* over $\FF$, denoted $E(\FF)$, is the set of all pairs $(x,y) \in \FF^2$ that satisfy the equation
\[
 y^2 = x^3 + ax + b
\]
together with a distinguished element denoted $\cO$ called the point at infinity:
\[
 E(\FF) = \{(x,y) \in \FF^2 \mid y^2 = x^3 + ax + b\} \cup \{\cO\}.
\]
This is the so-called *affine representation* of points of the elliptic curve.
There is another family of representations called *projective* (more attractive from an algorithmic point of view) that we will discuss [shortly](#affine-versus-projective-coordinates).

[Hasse's bound](https://en.wikipedia.org/wiki/Hasse%27s_theorem_on_elliptic_curves) establishes that the number of points of $E(\FF)$, called the *order* of $E(\FF)$, is
\[
 |E(\FF)| = |\FF|+1-t
\]
where $t$ is an integer (called the *Frobenius trace* or simply *trace* of the curve) satisfying
\[
 |t| \le 2 \sqrt{|\FF|}.
\]

It is possible to equip $E(\FF)$ with a commutative group law for which the point at infinity $\cO$ is the identity element with the so-called *chord-and-tangent rule*.
This group operation is denoted additively and called *addition law*.
The inverse of a point $(x,y) \in E(\FF) \setm \{\cO\}$ is the point $(x,-y)$.

From the addition law we can define *scalar multiplication*: for $m \in \ZZ$ and $P \in E(\FF)$, the scalar multiplication of $P$ by $m$, denoted $[m]P$ or $mP$ if there is no ambiguity, is given by
\[
 [m] P \defeq
 \begin{cases}
  \cO & \text{if } m=0 \\
  \underbrace{P + \cdots + P}_{m \text{ times}} & \text{if } m > 0 \\
  [-m](-P) & \text{if } m < 0.
 \end{cases}
\]

The group structure of $E(\FF)$ is either cyclic or "almost" cyclic.
Namely, a general theorem establishes that $E(\FF)$ has at most two [invariant factors](./mathematical-preliminaries/groups.md#th:finite_abelian_groups).
In other words, $E(\FF)$ is isomorphic to either the cyclic group $\ZZ_n$ or a direct product of cyclic groups $\ZZ_{n_1} \times \ZZ_{n_2}$ with $n_1 \divides n_2$.

## Affine versus Projective Coordinates

Earlier we saw how to defined an elliptic curve using affine coordinates.
This is not how elliptic curves are usually defined by mathematicians, who usually prefer to use projective geometry.

### Projective Plan

Let $\FF$ be a finite field of order $q$ and let $\FF^* \defeq \FF \setm \{0\}$.
The *projective plan* over $\FF$, denoted $P^2(\FF)$, is the set of equivalence classes of $\FF^3 \setm \{(0,0,0)\}$ where two tuples $(x,y,z)$ and $(x',y',z')$ are equivalent, denoted $(x,y,z) \sim (x',y',z')$, if there is a scalar $k \in \FF^*$ such that
\[
 (x',y',z') = (kx,ky,kz).
\]
Such an equivalence class is called a *projective point*.
A projective point contains $q-1$ tuples $(x,y,z) \in \FF^3$.
The convention is to denote projective points with capital letters and colon separators, i.e., $(X \colon Y \colon Z)$ (or sometimes $[X \colon Y \colon Z]$) will denote the equivalence class
\[
 \{ (kX, kY, kZ) \mid k \in \FF^* \}.
\]

Equivalently, projective points can be seen as 1-dimensional subspaces ("lines") of the 3-dimensional vector space $V = \FF^3$ over $\FF$.
How many projective points are there?
There are $q^3-1$ non-zero vectors in $\FF^3$, but each of the $q-1$ non-zero vectors in a subspace generates this subspace, hence the total number of 1-dimensional subspaces is
\[
 \frac{q^3-1}{q-1} = q^2 + q +1.
\]

Another way to count the number of projective points is as follows:

- there are $q^2$ projective points of the form $(X \colon Y \colon 1)$, $X, Y \in \FF$;
- there are $q$ projective points of the form $(1 \colon Y \colon 0)$, $Y \in \FF$;
- there is one projective point of the form $(0 \colon 1 \colon 0)$.

It is customary to identify the ordinary "affine" plane $\FF^2 = \{(x,y) \mid x,y \in \FF\}$ with the first type of projective points, meaning there is an injective map from $\FF^2$ to $P^2(\FF)$ given by $(x,y) \mapsto (x \colon y \colon 1)$.
The inverse of this map is $(X \colon Y \colon Z) \mapsto (X/Z, Y/Z)$.

The $q+1$ points of the second and third types (sharing the property that $Z=0$) are called "points at infinity" and form the so-called "line at infinity".

> It is possible to define *projective lines* in a similar way to projective points: projective lines are 2-dimensional subspaces of $\FF^3$ (with vector $(0,0,0)$ removed).
There are also $q^2 + q + 1$ projective lines in $P^2(\FF)$, which is quite natural since any 1-dimensional subspace of $\FF^3$ defines a unique 2-dimensional subspace via its orthogonal complement.
A projective point "lies on" a projective line if it is included (in the set-theoretical sense) in the projective line.
From this definition, it follows that (i) given any two projective points, there is exactly one projective line containing both of them, and (ii) given any two projective lines, there is exactly one projective point lying on both of them (meaning there are no parallel lines). Properties (i) and (ii) are in fact the axiomatic definition of a projective plan.
Each projective line contains $q^2-1$ vectors $(x,y,z) \in \FF^3 \setm \{(0,0,0)\}$, and is the disjoint union of $q+1$ projective points.
In particular, the $q+1$ projective points of the second and third type are indeed on the same projective line corresponding to the 2-dimensional subspace orthogonal to vector $(0,0,1)$.

### Elliptic Curves in Projective Coordinates

To obtain the equation defining an elliptic curve in projective coordinates, we substitute $X/Z$ to $x$ and $Y/Z$ to $y$ in the affine short Weierstrass equation
\[
 y^2 = x^3 + ax + b \label{1} \tag{1}
\]
and multiply by $Z^3$ to clear the denominators.
This way, we obtain the *projective short Weierstrass equation*:
\[
 Y^2 Z = X^3 + a X Z^2 + b Z^3. \label{2} \tag{2}
\]

It is easy to see that a projective point $(X \colon Y \colon Z)$ with $Z \neq 0$ satisfies $\eqref{2}$ if and only if the corresponding affine point $(x,y)$ with $x=X/Z$ and $y=Y/Z$ satisfies $\eqref{1}$.
Moreover, a projective point $(X \colon Y \colon Z)$ on the line at infinity ($Z=0$) satisfies $\eqref{2}$ if and only $X=0$, meaning the only of the $q+1$ projective points at infinity satisfying $\eqref{2}$ is $(0 \colon 1 \colon 0)$.
This is the "curve point at infinity", the identity element of the group law, that we denoted $\cO$ when we defined the elliptic curve in affine coordinates.

Hence, one of the main advantages of projective coordinates over affine ones is that it unifies ordinary points and the point at infinity $\cO$, which now has a projective representation as any other point, namely $(0 \colon 1 \colon 0)$.

Another advantage is that computing the group law is more efficient because it does not require to perform modular division (which is only required to perform projective-to-affine conversion).
A ballpark estimation is that a modular inversion is 20 to 100 times more costly than a modular multiplication depending on the platform and the implementation.

The projective coordinates obtained with the substitution $x \leftarrow X/Z$, $y \leftarrow Y/Z$ is just one possibility among others, called *homogeneous* projective coordinates because the resulting projective equation $\eqref{2}$ for the curve is homogeneous, meaning all terms have the same total degree, 3 here.
An very common alternative are *Jacobian coordinates* defined by the substitution
\[
 x \leftarrow X/Z^2, \quad y \leftarrow Y/Z^3.
\]
The resulting projective equation is
\[
 Y^2 = X^3 + a X Z^4 + b Z^6.
\]
Projective points in Jacobian coordinates are defined by the equivalence relation
\[
 (x,y,z) \sim (x',y,',z') \quad \text{if} \quad \exists k \in \FF^* , (x',y',z') = (k^2 x, k^3 y, k z).
\]
The point at infinity ($Z=0$) is the equivalence class $(1 \colon 1 \colon 0) = \{(k^2, k^3, 0) : k \in \FF^*\}$.

See <http://www.hyperelliptic.org/EFD/> for a list of various other possible coordinates systems.

----

[^char]: It is possible to define elliptic curves over fields of characteristic 2 or 3 but equations are more complicated.
