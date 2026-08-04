# Find Missing Elements (LeetCode 3731) — Benchmark Report

Problem: [find-missing-elements](https://leetcode.com/problems/find-missing-elements).
`nums` holds the present values of a contiguous integer range (its min and max are
always present); return the sorted list of missing integers.

**Constraints:** `2 <= nums.length <= 100`, `1 <= nums[i] <= 100`.

Benchmarks use synthetic data over a range `[1, span]` with the endpoints always
present and each interior value kept with ~50% probability (so roughly half the
range is missing). Fresh data is generated per timed call via SplitMix64 to defeat
branch-predictor memorization. Harness: Criterion, `iter_batched(LargeInput)`.

Source: [`src/find_missing_elements_3731.rs`](../../src/find_missing_elements_3731.rs).
Benchmark: [`find_missing_elements_3731.rs`](find_missing_elements_3731.rs).

## Implementations

### 1. `find_missing_elements` — sort + `tuple_windows` (baseline / submitted)

Sort, then emit the open interval between each adjacent pair.

```rust
pub fn find_missing_elements(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable();
    nums.into_iter()
        .tuple_windows()
        .flat_map(|(a, b)| (a + 1)..b)
        .collect()
}
```

### 1b. `find_missing_elements_loop` — sort + imperative nested loop

Same algorithm, hand-written indexing and `push` instead of iterator adaptors.

```rust
pub fn find_missing_elements_loop(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable();
    let mut ans = Vec::new();
    for i in 0..nums.len() - 1 {
        for x in nums[i] + 1..nums[i + 1] {
            ans.push(x);
        }
    }
    ans
}
```

### 2. `find_missing_elements_bitset` — `Vec<u64>` bitset + full range scan

Mark presence in a heap-allocated bitset, then scan `[min, max]` for unset bits.

```rust
pub fn find_missing_elements_bitset(nums: Vec<i32>) -> Vec<i32> {
    let (&min, &max) = (nums.iter().min().unwrap(), nums.iter().max().unwrap());
    let span = (max - min + 1) as usize;
    let mut bits = vec![0u64; span.div_ceil(64)];
    for &v in &nums {
        let i = (v - min) as usize;
        bits[i / 64] |= 1u64 << (i % 64);
    }
    (min..=max)
        .filter(|&v| {
            let i = (v - min) as usize;
            bits[i / 64] & (1u64 << (i % 64)) == 0
        })
        .collect()
}
```

### 3. `find_missing_elements_bool` — `Vec<bool>` + full range scan

Same shape as the bitset, but one byte per position.

```rust
pub fn find_missing_elements_bool(nums: Vec<i32>) -> Vec<i32> {
    let (&min, &max) = (nums.iter().min().unwrap(), nums.iter().max().unwrap());
    let span = (max - min + 1) as usize;
    let mut present = vec![false; span];
    for &v in &nums {
        present[(v - min) as usize] = true;
    }
    (min..=max)
        .filter(|&v| !present[(v - min) as usize])
        .collect()
}
```

### 4. `find_missing_elements_u128` — single `u128` bitmask

Pack presence into one 128-bit word (no heap allocation). Valid only when all
values fit in `0..128` — true for this problem (`1 <= v <= 100`), so it is
benchmarked at `span = 100` only.

```rust
pub fn find_missing_elements_u128(nums: Vec<i32>) -> Vec<i32> {
    let b = nums.into_iter().fold(0u128, |f, n| f | 1 << n);

    std::iter::successors(Some(b), |n| Some(n >> 1))
        .skip(b.trailing_zeros() as _)
        .zip(b.trailing_zeros() as i32..127 - b.leading_zeros() as i32)
        .filter_map(|(b, m)| (b & 1 == 0).then_some(m))
        .collect()
}
```

### 5. `find_missing_elements_hashset` — `HashSet` + full range scan

Collect into a `HashSet`, then scan the open range `(min, max)` for absent values.

```rust
pub fn find_missing_elements_hashset(nums: Vec<i32>) -> Vec<i32> {
    let st: HashSet<i32> = nums.iter().copied().collect();
    let &mn = nums.iter().min().unwrap();
    let &mx = nums.iter().max().unwrap();

    let mut ans = Vec::new();
    for x in mn + 1..mx {
        if !st.contains(&x) {
            ans.push(x);
        }
    }
    ans
}
```

## Results

Criterion median times (lower is better). **Bold** marks the fastest per row.

| span      | sort/iter (1) | sort/loop (1b) | u128 (4) | bool (3) | Vec&lt;u64&gt; (2) | HashSet (5) |
| --------- | ------------- | -------------- | -------- | -------- | ------------------ | ----------- |
| 100       | 448 ns        | **432 ns**     | 455 ns   | 590 ns   | 653 ns             | 3.24 µs     |
| 1 000     | 1.67 µs       | **1.54 µs**    | —        | 2.66 µs  | 3.40 µs            | 22.2 µs     |
| 10 000    | 12.3 µs       | **12.1 µs**    | —        | 39.9 µs  | 37.3 µs            | 273 µs      |
| 100 000   | 321 µs        | **312 µs**     | —        | 521 µs   | 627 µs             | 3.52 ms     |
| 1 000 000 | 3.42 ms       | **3.35 ms**    | —        | 5.25 ms  | 6.36 ms            | 36.3 ms     |

## Takeaways

- **Sort-based wins at every size.** It touches only the present values and writes
  just the gaps; the imperative nested loop (1b) edges out the `tuple_windows`
  iterator (1) by ~2–8% thanks to tighter codegen.
- **Full-range-scan variants pay `O(span)`** to allocate and walk the entire range.
  The `bool` array beats the `Vec<u64>` bitset at large spans (simpler per-element
  access, no bit arithmetic).
- **Single `u128`** is competitive with the baseline at `span = 100` (no heap
  allocation, mask lives in registers) but cannot scale past value 127.
- **`HashSet` is by far the slowest** (~7× at span 100, ~11× at span 1e6): a hash on
  every insert, a hash lookup per scanned position, and cache-unfriendly probing.
