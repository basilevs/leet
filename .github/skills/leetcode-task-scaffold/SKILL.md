---
name: leetcode-task-scaffold
description: "Prepare training scaffolding for a LeetCode problem, including today's daily challenge. Use when user asks to scaffold a new task/problem file or the daily question."
---

# LeetCode Task Scaffold (Rust and/or C++)

Use this skill to prepare a new training scaffold in this repository.

## Goal

Create one solution file per requested language that:
- Has a canonical LeetCode origin link at the top of the file.
- Has no implementation (training scaffold only).
- Includes populated tests (official examples) that fail until the solution is correctly implemented by the user.
- Is registered where the language's build requires it (Rust: `rust/src/lib.rs`; C++: nothing to register).

See also AGENTS.md for repository-wide conventions (slug resolution, `official<N>`
test naming, copying example inputs verbatim, C++ harness reference).

## Choosing the language

Ask the user which language to scaffold — **Rust**, **C++**, or **both** — unless
they already said so in their request (e.g. "scaffold today's daily in C++",
"scaffold both"). Do not assume Rust by default. When both are requested, produce
both files from a single problem fetch; the tests are the same examples
transcribed into each language.

## Conventions

Shared:
- Both languages use the same file stem `<function_name>_<frontend_id>`, so the
  two implementations of a problem sit side by side. `<function_name>` is the
  function name from the language's own LeetCode template converted to
  `snake_case` (the C++ template's `twoSum` becomes the file stem `two_sum`).
  `<frontend_id>` is the displayed problem number (GraphQL `questionFrontendId`,
  NOT the internal `questionId`).
- Top-of-file origin link format (no trailing slash):
  - `// https://leetcode.com/problems/<slug>`
- Tests are in the same file, named `official1`, `official2`, ...
- Solution functions carry an extra 4-space indent so they paste straight back
  into LeetCode's `impl Solution` / `class Solution` block.

Rust (`rust/src/<function_name>_<frontend_id>.rs`):
- Exposes a top-level public function (or, for design problems, a `struct` with an `impl`).
- Must be registered in `rust/src/lib.rs` as `pub mod <function_name>_<frontend_id>;`.

C++ (`cpp/src/<function_name>_<frontend_id>.cpp`):
- Function keeps the template's camelCase name (`twoSum`), the file stem does not.
- LeetCode's snippet relies on an implicit `using namespace std` and unqualified
  `vector`/`string`. Qualify everything as `std::` in the repository file.
- LeetCode passes containers as non-const references (`vector<int>& nums`). A
  braced initialiser in a test cannot bind to a non-const reference, so change
  such parameters to `const std::vector<int>&` (or by value if the solution
  mutates them). Restore the `&` when pasting back into LeetCode.
- Nothing to register: `make -C cpp` discovers `src/*.cpp` by globbing.

## Templates

Fill in `<slug>`, `<function_name>`, signature and return type from the official
implementation template for that language.

### Rust — `rust/src/<function_name>_<frontend_id>.rs`

```rust
// https://leetcode.com/problems/<slug>

pub fn <function_name>(/* signature */) -> /* return */ {
    dbg!(<arg1>, <arg2>); // suppress unused variable warnings for function arguments
    todo!("training scaffold: implement solution");
}

#[cfg(test)]
mod tests {
    use super::<function_name>;

    #[test]
    fn official1() {
        // Input from exampleTestcases; expected output copied from examples given in content.
        assert_eq!(<expected_1>, <function_name>(<input_1>));
    }

    #[test]
    fn official2() {
        // Input from exampleTestcases; expected output copied from examples given in content.
        assert_eq!(<expected_2>, <function_name>(<input_2>));
    }
}
```

For design problems (template is a `struct` plus `impl Solution` with multiple
methods), keep the type and its `impl`, leave each method body as `todo!()`, and
import with `use super::*;` in the test module.

### C++ — `cpp/src/<function_name>_<frontend_id>.cpp`

```cpp
// https://leetcode.com/problems/<slug>

#include "test.hpp"

#include <vector> // and whatever else the signature needs

    /* return */ <functionName>(/* signature */) {
        unused(<arg1>, <arg2>); // suppress unused-parameter warnings
        todo();                 // counterpart of Rust's todo!()
    }

TEST(official1) {
    // Input from exampleTestcases; expected output copied from examples given in content.
    check_eq(<expected_1>, <functionName>(<input_1>));
}

TEST(official2) {
    // Input from exampleTestcases; expected output copied from examples given in content.
    check_eq(<expected_2>, <functionName>(<input_2>));
}
```

`check_eq` takes expected first, and its expected parameter is non-deduced, so
braced initialisers work directly — `check_eq({0, 1}, twoSum({2, 7, 11, 15}, 9))`
and nested `check_eq({{1, 2}, {3}}, ...)` both compile and print readably on
failure. See `cpp/include/test.hpp` for the full harness.

For design problems keep the `struct`/`class` and its methods, leave each body
calling `todo()`, and drive it from `TEST(...)` blocks.

## Required Workflow

1. Determine the target language(s) per "Choosing the language" above.
2. Determine the canonical problem slug.
  - If known, use it directly.
  - If the user asks to scaffold "today's daily problem", "the daily
    challenge", or similar, without naming a specific problem, resolve the
    slug automatically instead of asking:

    ```sh
    ./.github/skills/leetcode-task-scaffold/scripts/fetch-daily-slug.sh
    ```

    This calls the LeetCode GraphQL `activeDailyCodingChallengeQuestion`
    query and prints just the slug. Use it for the rest of this workflow
    exactly as if the user had supplied it.
  - Otherwise prompt user.
3. Fetch the problem scaffold data with the bundled helper script (the rendered
   problem page is client-side and does NOT contain the code template):

    ```sh
    ./.github/skills/leetcode-task-scaffold/scripts/fetch-problem.sh <slug> [rust|cpp|both]
    ```

   The second argument is optional and defaults to `both`. Run the script using
   a repository-relative or absolute path to pass security audit. It calls the
   LeetCode GraphQL API and prints, in one shot:
   - `frontendId` — the displayed problem number used in filenames.
   - the origin link comment (`// https://leetcode.com/problems/<slug>`).
   - one `--- <lang> template ---` section per requested language.
   - `exampleTestcases` — newline-separated official inputs ONLY (no expected
     outputs).
   - `content` — HTML problem statement and examples (the source of expected
     outputs, from the `Output:` lines).

   See [fetch-problem.sh](./scripts/fetch-problem.sh) for the exact query. It
   requires `curl` and `jq`. Because the whole request/parse happens in a single
   command, the user only sees one terminal safety prompt — so fetch once even
   when scaffolding both languages.

4. Create the scaffold file(s) using the templates above.
  - For algorithm problems the template is wrapped in a `Solution` impl/class
    block. Remove the wrapper and expose the inner function at top level, keeping
    its extra indent. For design problems keep the type and leave method bodies
    unimplemented.
  - Name, signature and return type must match the official implementation
    template for that language (subject to the C++ reference-parameter note
    above).
  - Convert textual examples from the problem statement into test code. Take
    inputs from `exampleTestcases` and expected outputs from `content` (the
    `Output:` lines), copied verbatim.
  - Name tests `official<example_number>`.
  - Keep test inputs as close to example data as possible. In Rust, if the input
    is multidimensional add a helper to `mod tests` that parses the example
    format into the required structures, so the test text matches the example:
    ```rust
    fn to_vector<const N: usize>(input: &[[i32; N]]) -> Vec<Vec<i32>> {
      input.into_iter().map(Vec::from).collect()
    }
    ```
    In C++ no helper is needed — pass nested braced initialisers directly.
  - Format test inputs per AGENTS.md conventions, e.g. one row per line for
    multidimensional arrays.
5. Register the module where required:
  - Rust: append `pub mod <function_name>_<frontend_id>;` to the end of
    `rust/src/lib.rs`. Ignore existing order.
  - C++: nothing to do.
6. Verify the scaffold compiles and is wired up:
  - Rust: `cargo test --manifest-path rust/Cargo.toml --lib <function_name>_<frontend_id> --no-run`
    confirms the module is registered and the populated tests compile.
  - C++: `make -C cpp test T=<function_name>_<frontend_id>` builds and runs it.
  - Running the tests is expected to FAIL: `todo!()` / `todo()` aborts the test,
    which confirms the tests exercise the (not-yet-written) implementation.

## Guardrails

- Do not implement the algorithm in scaffold mode.
- Do not assume a language — ask for Rust, C++, or both when the user has not said.
- Do not skip `rust/src/lib.rs` registration for Rust scaffolds.
- Do not omit the origin link.
- No empty tests: populate every test from an official example so it fails against the unimplemented body until implemented.
- Keep test names aligned with repository style (official1, official2, ...).
- Use `questionFrontendId` (displayed number) for filenames/modules, never the internal `questionId`.
