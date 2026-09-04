This repository holds solutions in two languages, each in its own folder:

- `rust/src/<name>_<number>.rs` — Cargo workspace, tests and benchmarks.
- `cpp/src/<name>_<number>.cpp` — Makefile build, tests only (no benchmarks yet).

Both use the same `<name>_<number>` stem for the same problem, so the two
implementations sit side by side. `<name>` is the function name from the
language's own LeetCode template, converted to `snake_case`; `<number>` is the
frontend problem ID.

Strange ineffective top-level inputs come from https://leetcode.com/ challenges and can't be changed. Consider them an external unavoidable limitation.
Public functions have an extra indent for easier pasting into https://leetcode.com/ (both languages: the LeetCode template wraps them in `impl Solution` / `class Solution`).

The LeetCode Rust environment ships with `itertools`, so solutions may freely use it. See [What are the environments for the programming languages?](https://support.leetcode.com/hc/en-us/articles/360011833974-What-are-the-environments-for-the-programming-languages#:~:text=Rust).


# Commit messages
Registering the new module in `rust/src/lib.rs` is a routine part of adding
every solution — do not mention it in commit messages. Describe only the
problem-specific work (algorithm, approach, edge cases).

# Tests
When asked to add tests, add them mechanically per specification, ignoring existing implementation and its defects.
Fix implementation only when asked explicitly, let user practice.

# Two-dimensional inputs
Some inputs in examples are given as two-dimensional grids:

```rust
[
    [0, 1, 0, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 0, 1, 0],
]
```
Often those have to be converted to a `Vec<Vec<_>>`.

Use an utility method and `#[rustfmt::skip]` to make the code compact and readable:

```rust
fn to_vector<const N: usize, const M: usize>(input: [[i32; M]; N]) -> Vec<Vec<i32>> { // replace i32 with a type required by a problem
    input.iter().map(|row| row.to_vec()).collect()
}

#[rustfmt::skip]
let grid = [
    [0, 1, 0, 0, 0], // each row on its own line
    [0, 1, 0, 1, 0], // no nested vec![]
    [0, 0, 0, 1, 0], // input textual representation closely follows the example
];
assert!(find_safe_walk(to_vector(grid), 1));
```

In C++ no helper is needed — nested braced initialisers already read like the
example, and `check_eq` prints them back in the same shape:

```cpp
// clang-format off
check_eq(true, findSafeWalk({
    {0, 1, 0, 0, 0}, // each row on its own line
    {0, 1, 0, 1, 0}, // input textual representation closely follows the example
    {0, 0, 0, 1, 0},
}, 1));
// clang-format on
```

# C++
Each `cpp/src/*.cpp` is a standalone translation unit: the solution, its tests,
and — via `#include "test.hpp"` — its own `main`. There is no shared library and
nothing to register, so adding a file is enough to make it build and run.

The harness in `cpp/include/test.hpp` is deliberately tiny:

```cpp
// https://leetcode.com/problems/<slug>

#include "test.hpp"

#include <vector>

    int findGcd(std::vector<int> nums) {
        unused(nums);              // silences unused-parameter warnings
        todo();                    // C++ counterpart of Rust's todo!()
    }

TEST(official1) {
    check_eq(2, findGcd({2, 5, 6, 9, 10}));
}
```

- `TEST(name) { ... }` registers a test; names follow `official1`, `official2`, ...
- `check_eq(expected, actual)` — expected first, matching Rust's `assert_eq!`.
  `expected` is a non-deduced parameter, so a braced initialiser works directly:
  `check_eq({9, 5, 3}, pivotArray({...}, 10))`. Failures print both values;
  vectors, nested vectors, pairs and strings all format themselves.
- `check(condition)` for boolean results, `todo()` for unimplemented scaffolds.
- `unused(a, b)` where a scaffold would otherwise warn about its arguments.

Build and run:

```sh
make -C cpp test              # everything
make -C cpp test T=find_gcd   # substring filter on the file name
./cpp/build/find_gcd_1979 official2   # substring filter on the test name
make -C cpp list
make -C cpp clean
```

Warnings are on (`-Wall -Wextra -Wpedantic -Wshadow -Wconversion`) but not
fatal; the standard is C++23, matching the LeetCode judge.

# Finding the LeetCode problem statement
Each `rust/src/<name>_<number>.rs` and `cpp/src/<name>_<number>.cpp` corresponds
to LeetCode problem `<number>`.

To locate the canonical problem statement (including official examples,
constraints, and the full slug):
1. If the file already has a `// https://leetcode.com/problems/<slug>/` header,
   use that URL.
2. Otherwise resolve the slug with whatever web-search affordance you have
   (a dedicated web-search tool, the editor's search action, or fetching a
   search-engine results page) using the predicate:

       site:leetcode.com/problems/ <number>

   The first hit is the canonical `https://leetcode.com/problems/<slug>/`
   page. If you have no dedicated tool, prefer endpoints that return
   server-rendered HTML so `fetch_webpage`-style tools can read the results
   directly — for example,
   `https://html.duckduckgo.com/html/?q=site%3Aleetcode.com%2Fproblems%2F+<number>`
   is known to work. Fetch the resolved page for the problem text and
   examples, and add the `// https://leetcode.com/problems/<slug>/` URL as a
   top-of-file comment so the next agent doesn't have to look it up again.
3. To fetch the actual descritpion use shell command:
    ````
    ./.github/skills/leetcode-task-scaffold/scripts/fetch-problem.sh <slug>
    ```
4. When writing tests named `official1`, `official2`, ..., copy the inputs
   verbatim from the problem's "Example N" sections; do not synthesize them
   from the function signature. Do not invent test data for `official` tests.
   Tests with other names can be created with synthetic inputs.


# Reviewing a solution
A solution file states its signature but never its constraints, so a reviewer
without the problem statement cannot tell an overflow from a safe subtraction,
an unreachable branch from a missing one, or an `official` test that drifted
from its example. Review subagents have been observed unable to close that gap
themselves — a review of `first_stable_index_3903` reported the fetch blocked
and left the overflow question open, while the same command succeeded from the
agent that spawned it moments later — so do not rely on a subagent fetching for
itself.

The user starts a review — by typing `/code-review` or `/simplify`, or by
asking for one in words. The agent receiving that request is the one that must
fetch, before it spawns any review subagent. When the review covers
`rust/src/<name>_<number>.rs` or `cpp/src/<name>_<number>.cpp`:

1. Take every `// https://leetcode.com/problems/<slug>` URL from the top of the
   file — a file that answers two problems carries two, and both bind the code.
2. Run the fetch for each slug, picking the template language of the file under
   review:

    ```
    ./.github/skills/leetcode-task-scaffold/scripts/fetch-problem.sh <slug> rust
    ```

3. Put the **Constraints** list and the official examples into the prompt of
   every review subagent spawned, next to the problem URL. Pass those, not the
   whole `--- content ---` HTML: the constraints are what a reviewer reasons
   from, and the rest is payload.

# Polishing procedure
Rust only — C++ has no benchmark harness yet, so polishing a C++ solution means
building warning-clean with `make -C cpp test` and nothing more.

Steps:
- Run lint checks (optionally strict Clippy groups).
    To see Clippy warnings statistics, use:
    ```
    cargo clippy --manifest-path rust/Cargo.toml -q --message-format=json -- -W clippy::pedantic -W clippy::nursery | jq -Rr 'fromjson? | select(.reason=="compiler-message") | .message.code.code? // empty' | sed '/^$/d' | sort | uniq -c | sort -nr
    ```
- Save benchmark baseline.
- Fix lint detections.
- Run tests.
- Compare new benchmark to baseline.

# Benchmarking
- Run becnhmark saving baseline:
    ```
    cargo bench --manifest-path rust/bench/Cargo.toml  -- --save-baseline pre_lint_fix
    ```
- Run benchmarks comparing with saved baseline:
    ```
    cargo bench --manifest-path rust/bench/Cargo.toml  -- --baseline pre_lint_fix
    ```


When to use:
- Use this loop when polishing results.
- Use this loop when polish is explicitly requested.
- It is not required for every change.
