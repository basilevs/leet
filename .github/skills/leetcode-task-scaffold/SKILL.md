---
name: leetcode-task-scaffold
description: "Prepare training scaffolding for a LeetCode problem. Use when user asks to scaffold a new task/problem file."
---

# LeetCode Task Scaffold (Rust)

Use this skill to prepare a new training scaffold in this repository.

## Goal

Create a new solution module that:
- Has a canonical LeetCode origin link at the top of the file.
- Has no implementation (training scaffold only).
- Includes populated tests (official examples) that fail until solution is correctly implemented by user.
- Is registered in src/lib.rs.

See also AGENTS.md for repository-wide conventions (slug resolution, `official<N>`
test naming, copying example inputs verbatim).

## Conventions

- File naming pattern: src/<function_name>_<frontend_id>.rs. <function_name> matches function name from LeetCode implementation template. <frontend_id> is the displayed problem number (GraphQL `questionFrontendId`, NOT the internal `questionId`).
- Solution file exposes a top-level public function (or, for design problems, a `struct` with an `impl`).
- Top-of-file origin link format (no trailing slash):
  - // https://leetcode.com/problems/<slug>
- Tests are in the same file under `#[cfg(test)]` and usually named official1, official2, ...
- New module must be added in src/lib.rs as:
  - pub mod <function_name>_<frontend_id>;

## Template
This is the template for the scaffold file. Fill in <slug>, <function_name>, signature and return type according to the official implementation template. If possible, save the file as src/<function_name>_<frontend_id>.rs

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


## Required Workflow

1. Determine the canonical problem slug.
  - If known, use it directly.
  - Otherwise prompt user.
2. Fetch the problem scaffold data with the bundled helper script (the rendered
   problem page is client-side and does NOT contain the code template):

    ```sh
    ./.github/skills/leetcode-task-scaffold/scripts/fetch-problem.sh <slug>
    ```

   Run it using repository-relative or absolute path to pass security audit.
   It calls the LeetCode GraphQL API and prints, in one shot:
   - `frontendId` — the displayed problem number used in filenames.
   - the origin link comment (`// https://leetcode.com/problems/<slug>`).
   - the Rust template (`impl Solution { pub fn ... }` form).
   - `exampleTestcases` — newline-separated official inputs ONLY (no expected
     outputs).
   - `content` — HTML problem statement and examples (the source of expected
     outputs, from the `Output:` lines).

   See [fetch-problem.sh](./scripts/fetch-problem.sh) for the exact query. It
   requires `curl` and `jq`. Because the whole request/parse happens in a single
   command, the user only sees one terminal safety prompt.

3. Create src/<function_name>_<frontend_id>.rs using the template above.
  - For algorithm problems the template is wrapped in a `Solution` impl block. Remove it and expose the inner function as a top-level function. For design problems keep the `struct`/`impl` and leave method bodies as `todo!()`.
  - name, signature and return type of the implementation must match the official implementation template.
  - Convert textual examples from the problem statement into Rust test code. Take inputs from `exampleTestcases` and expected outputs from `content` (the `Output:` lines), copied verbatim.
  - Name tests `fn official<example_number>() {...}`
  - Keep test inputs as close to example data as possible. If problem input is multidimensional add a helper function to `mod tests`to parse the input from the example testcases format into require Rust data structures. Use it in the tests. This way, multidimensional array would textually match Rust test code. Example:
    ```rust
    fn to_vector<const N: usize>(input: &[[i32; N]]) -> Vec<Vec<i32>> {
      input.into_iter().map(Vec::from).collect()
    }
    ```
4. Add module registration to src/lib.rs:
  - pub mod <function_name>_<frontend_id>;
5. Verify the scaffold compiles and is wired up:
  - `cargo test --lib <function_name>_<frontend_id> --no-run` confirms the
    module is registered and the populated tests compile.
  - Running the tests is expected to FAIL: the `todo!()` body panics, which
    confirms the tests exercise the (not-yet-written) implementation.

## Guardrails

- Do not implement the algorithm in scaffold mode.
- Do not skip src/lib.rs registration.
- Do not omit the origin link.
- No empty tests: populate every test from an official example so it fails against the `todo!()` body until implemented.
- Keep test names aligned with repository style (official1, official2, ...).
- Use `questionFrontendId` (displayed number) for filenames/modules, never the internal `questionId`.
