# Puzzle 12: Gamma Ray

- [puzzle page](https://zkhack.dev/zkhackIV/puzzleF1.html)
- [GitHub repository](https://github.com/ZK-Hack/puzzle-gamma-ray)
- puzzle description:

```text
Bob was deeply inspired by the Zcash design [1] for private transactions
and had some pretty cool ideas on how to adapt it for his requirements.
He was also inspired by the Mina design for the lightest blockchain and
wanted to combine the two. In order to achieve that, Bob used the MNT6753
cycle of curves to enable efficient infinite recursion, and used elliptic
curve public keys to authorize spends. He released a first version of the
system to the world and Alice soon announced she was able to double spend
by creating two different nullifiers for the same key...
```

ZCash, Mina, MNT curves, nullifiers...
Let's jump into the code to see what this is about.