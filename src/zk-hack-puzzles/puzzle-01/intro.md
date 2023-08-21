# Puzzle 1: Let's Hash it Out

- [puzzle page](https://zkhack.dev/events/puzzle1.html)
- [GitHub repository](https://github.com/kobigurk/zkhack-bls-pedersen)
- puzzle description:

```text
Alice designed an authentication system in which users gain access by presenting
it a signature on a username, which Alice provided.
One day, Alice discovered 256 of these signatures were leaked publicly, but the
secret key wasn't. Phew.
The next day, she found out someone accessed her system with a username she
doesn't know! This shouldn't be possible due to existential unforgeability, as
she never signed such a message.

Can you find out how it happend and produce a signature on your username?
```

From the puzzle's instructions, it seems that we have to mount a universal forgery attack against the signature scheme used by Alice (universal because one should be able to forge a signature for *any* username).
Let's find out what signature scheme Alice is using exactly by taking a look at the code (the name of the package gives us a good hint already).
