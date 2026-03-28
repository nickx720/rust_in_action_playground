# FNV-1 and FNV-1a for This Bloom Filter

This note explains what `FNV-1` and `FNV-1a` are, why they are a better fit than "sum the bytes", and how to think about them when building a Bloom filter for a spell-checker.

Primary references:

- Wikipedia overview: <https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function>
- RFC 9923, The FNV Non-Cryptographic Hash Algorithm: <https://datatracker.ietf.org/doc/rfc9923/>

## 1. What problem are we solving?

A Bloom filter does not store full words. It stores a pattern of bits.

When you insert a word:

1. Hash the word several times.
2. Map each hash to a bit position.
3. Set those bits to `1`.

When you query a word:

1. Hash the word in the same way.
2. Map those hashes to the same bit positions.
3. If any checked bit is `0`, the word is definitely absent.
4. If all checked bits are `1`, the word is maybe present.

The entire structure depends on one thing going well:

`different words should usually touch different-looking sets of bits`

If the hash is weak, many unrelated words collide too early, the same regions of the bit array fill up, and the filter starts saying `MaybePresent` too often.

## 2. Why "sum the bytes" is too weak

Suppose your hash is:

```text
hash("cat") = 'c' + 'a' + 't'
```

Then:

```text
hash("cat") == hash("tac")
```

because both have the same letters, only in a different order.

That means the hash is not sensitive to byte order.

ASCII picture:

```text
"cat" -> c + a + t -> same total
"tac" -> t + a + c -> same total
"act" -> a + c + t -> same total
```

A good string hash should react to both:

- which bytes appear
- where they appear

This is exactly where FNV helps.

## 3. The big idea behind FNV

FNV is a simple non-cryptographic hash family designed to be fast and to spread ordinary inputs reasonably well. RFC 9923 describes FNV as a fast non-cryptographic hash with good dispersion for non-adversarial use cases.

At a high level, FNV processes the input one byte at a time and keeps updating a running hash value.

Each new byte affects the current state, and that state has already been influenced by all prior bytes.

That means order matters.

ASCII intuition:

```text
start with offset_basis

read 'c' -> update hash
read 'a' -> update again using the new state
read 't' -> update again using the newer state
```

So:

```text
"cat"
```

and

```text
"tac"
```

do not usually evolve through the same intermediate states.

## 4. FNV-1 vs FNV-1a

The difference is small but important: the order of `multiply` and `XOR` is reversed.

### FNV-1

For each byte:

```text
hash = hash * FNV_prime
hash = hash XOR byte
```

### FNV-1a

For each byte:

```text
hash = hash XOR byte
hash = hash * FNV_prime
```

Wikipedia and RFC 9923 both describe this structure, and Wikipedia notes that `FNV-1a` tends to have slightly better avalanche behavior than `FNV-1`.

### Why does the order matter?

Because the order changes when each input byte gets mixed into the running state.

Think of it like stirring dye into water:

```text
FNV-1:   stir first, then add dye
FNV-1a:  add dye first, then stir
```

Both mix, but the second variant tends to spread the effect of each byte a bit better in practice for common string data.

For an undergraduate-level mental model, that is enough:

- `XOR` injects the next byte into the current state.
- multiplication by a carefully chosen prime spreads that change through the word-sized state.
- repeating this for every byte makes the final value depend on the entire ordered sequence.

## 5. The constants: offset basis and prime

FNV is not "pick any random constant". The constants are part of the algorithm.

For the common 64-bit version:

```text
offset_basis = 14695981039346656037
             = 0xcbf29ce484222325

FNV_prime    = 1099511628211
             = 0x100000001b3
```

RFC 9923 gives these standard constants.

Why start from a non-zero `offset_basis`?

Because a hash function needs an initial state. Starting from a carefully chosen non-zero value avoids the degenerate "everything starts from zero" behavior of the deprecated `FNV-0`.

Why multiply by a prime?

You can think of the prime as a mixing step. The RFC explains that the chosen FNV primes were selected for useful dispersion properties. You do not need to re-derive that math for this project; you mainly need to use the standard constants correctly and consistently.

## 6. Why FNV is a better fit for your Bloom filter

Your Bloom filter needs two things from its base hashes:

- order sensitivity
- broad spread across the hash space

FNV is much better than byte summation on both points.

### Byte summation

```text
"cat" -> same total as "tac"
"ab"  -> same total as "ba"
```

So many words collapse together immediately.

### FNV-style rolling update

```text
state0
  -> mix 'c' -> state1
  -> mix 'a' -> state2
  -> mix 't' -> state3
```

Reversing the letters changes the sequence of intermediate states:

```text
state0
  -> mix 't' -> different state1
  -> mix 'a' -> different state2
  -> mix 'c' -> different state3
```

This usually produces different final hashes, which means different Bloom filter bit positions.

That is the main educational takeaway:

`a large bit array is wasted if the hash function does not explore it well`

## 7. Are FNV-1 and FNV-1a perfect?

No.

RFC 9923 explicitly describes FNV as non-cryptographic. That means:

- it is fine for ordinary data-structure tasks
- it is not appropriate when an active adversary is trying to cause collisions

For this spell-check Bloom filter, that is acceptable. You are not defending against maliciously crafted inputs; you are trying to distribute ordinary words better.

So the practical engineering answer is:

- for this project, `FNV-1` and `FNV-1a` are reasonable
- for security-sensitive hashing, they are not sufficient

## 8. How to use them in this project

You do not need ten separate hash functions.

A common Bloom filter approach is:

```text
h1 = first base hash of word
h2 = second base hash of word

index_i = (h1 + i * h2) mod bit_count
```

This is often called double hashing.

So one practical next step is:

- implement one function as `FNV-1`
- implement the other as `FNV-1a`
- keep your existing "derive k indexes from h1 and h2" structure

That gives you two stronger base hashes without changing the rest of the Bloom filter design.

## 9. A small worked picture

This is not a numeric FNV computation. It is a conceptual trace.

### Weak byte-sum style hash

```text
"cat" -> total bytes = 312
"tac" -> total bytes = 312
```

Then both words generate the same probe pattern:

```text
index0 = f(312)
index1 = g(312)
index2 = ...
```

So the Bloom filter cannot distinguish them.

### FNV-style rolling hash

```text
"cat":
  state0 -> mix 'c' -> mix 'a' -> mix 't' -> H_cat

"tac":
  state0 -> mix 't' -> mix 'a' -> mix 'c' -> H_tac
```

Usually:

```text
H_cat != H_tac
```

Then their probe patterns also differ:

```text
cat -> bits 41, 912, 12003, ...
tac -> bits 77, 904, 11811, ...
```

Not guaranteed collision-free, but much healthier.

## 10. What to watch when you implement it

Common implementation points:

- Pick either 32-bit or 64-bit FNV and stay consistent.
- Use the standard constants for that size.
- In Rust, use wrapping arithmetic for the multiply step.
- Rebuild `words.bf` after changing the hash functions, because old Bloom files are tied to the old hashing scheme.
- Keep in mind that better hashes improve the filter, but they do not fix CLI interpretation mistakes.

## 11. Final takeaway

If you were hearing this in office hours, I would summarize it this way:

`A Bloom filter is only as good as the way it turns words into bit positions.`

Your current hash throws away too much information about the word.

FNV-1 and FNV-1a keep the implementation simple while preserving much more of the word's structure, especially byte order. That makes them a strong educational next step for this project.
