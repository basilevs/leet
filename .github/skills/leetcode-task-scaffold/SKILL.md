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
- Includes tests (official examples first).
- Is registered in src/lib.rs.

## Conventions

- File naming pattern: src/<function_name>_<questionId>.rs. <function_name> matches function name from LeetCode implementation template.
- Solution file exposes a top-level public function or struct.
- Top-of-file origin link format:
  - // https://leetcode.com/problems/<slug>/
- Tests are in the same file under #[cfg(test)] and usually named official1, official2, ...
- New module must be added in src/lib.rs as:
  - pub mod <function_name>_<questionId>;

## Template
This is the template for the scaffold file. Fill in <slug>, <function_name>, signature and return type according to the official implementation template. If possible, the file should saved as src/<function_name>_<questionId>.rs

```rust
// https://leetcode.com/problems/<slug>/

pub fn <function_name>(/* signature */) -> /* return */ {
    todo!("training scaffold: implement solution");
}

#[cfg(test)]
mod tests {
    use super::<function_name>;

    #[test]
    fn official1() {
        // Copy Example 1 input verbatim from the problem statement.
        // assert_eq!(expected, <function_name>(...));
    }

    #[test]
    fn official2() {
        // Copy Example 2 input verbatim from the problem statement.
        // assert_eq!(expected, <function_name>(...));
    }
}
```


## Required Workflow

1. Determine the canonical problem slug.
  - If known, use it directly.
  - Otherwise prompt user.
2. Fetch the problem description, implementation template, and example
   testcases from the LeetCode GraphQL API (the rendered problem page is
   client-side and does NOT contain the code template).
  - Endpoint: POST https://leetcode.com/graphql
  - Headers: Content-Type: application/json, Referer: https://leetcode.com
  - Body (substitute <slug>):

    ```json
    {
      "operationName": "questionData",
      "variables": { "titleSlug": "<slug>" },
      "query": "query questionData($titleSlug: String!) { question(titleSlug: $titleSlug) { questionId title titleSlug content exampleTestcases codeSnippets { lang langSlug code } } }"
    }
    ```

  - Example invocation (concrete slug `two-sum`):

    ```sh
    curl -s 'https://leetcode.com/graphql' \
      -H 'Content-Type: application/json' \
      -H 'Referer: https://leetcode.com' \
      --data '{"operationName":"questionData","variables":{"titleSlug":"two-sum"},"query":"query questionData($titleSlug: String!) { question(titleSlug: $titleSlug) { questionId title titleSlug content exampleTestcases codeSnippets { lang langSlug code } } }"}'
    ```

  - Read the response fields: `question.content` (HTML problem statement and
    examples), `question.exampleTestcases` (newline-separated official inputs),
    and the `codeSnippets` entry with `langSlug == "rust"` (the official Rust
    template, in `impl Solution { pub fn ... }` form). Avoid Python, prefer `jq`.

    Example of `jq` extraction (prints the origin link, the Rust template, and the
    example testcases):

    ```sh
    jq -r '.data.question
          | "questionId: \(.questionId)",
            "// https://leetcode.com/problems/\(.titleSlug)/\n",
            (.codeSnippets[] | select(.langSlug=="rust") | .code),
            "\n--- exampleTestcases ---",
            .exampleTestcases,
            "\n--- content ---",
            .content'
    ```

    Output (content truncated):

    ```text
    questionId: 1
    // https://leetcode.com/problems/two-sum/

    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        }
    }

    --- exampleTestcases ---
    [2,7,11,15]
    9
    [3,2,4]
    6
    [3,3]
    6

    --- content ---
    <p>Given an array of integers <code>nums</code>&nbsp;and an integer <code>target</code>, return <em>indices of the two numbers such that they add up to <code>target</code></em>.</p>
    ...
    <pre>
    <strong>Input:</strong> nums = [2,7,11,15], target = 9
    <strong>Output:</strong> [0,1]
    </pre>
    ```

3. Create src/<function_name>_<questionId>.rs using the template above.
  - Most problem implementation templates are wrapped in `Solution` impl block. Remove it and expose the inner function as a top-level function.
  - name, signature and return type of the implemntation must match the official implementation template.
  - Convert textual examples from the problem statement into Rust test code.
  - Name tests `fn official<example_number>() {...}`
4. Add module registration to src/lib.rs:
  - pub mod <function_name>_<questionId>;

## Guardrails

- Do not implement the algorithm in scaffold mode.
- Do not skip src/lib.rs registration.
- Do not omit the origin link.
- Keep test names aligned with repository style (official1, official2, ...).
