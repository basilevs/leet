Strange ineffective top-level inputs in Rust code come from https://leetcode.com/ challenges and can't be changed. Consider them an external unavoidable limitation.
Public functions have an extra indent for easier pasting into https://leetcode.com/.


# Tests
When asked to add tests, add them mechanically per specification, ignoring existing implementation and its defects.
Fix implementation only when asked explicitly, let user practice.


# Finding the LeetCode problem statement
Each `src/<name>_<number>.rs` corresponds to LeetCode problem `<number>`.

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
3. When writing tests named `official1`, `official2`, ..., copy the inputs
   verbatim from the problem's "Example N" sections; do not synthesize them
   from the function signature — the input shape is rarely enough to pin down
   which interpretation the problem actually requires.


# Polishing procedure
Steps:
- Run lint checks (optionally strict Clippy groups).
    To see Clippy warnings statistics, use:
    ```
    cargo clippy -q --message-format=json -- -W clippy::pedantic -W clippy::nursery | jq -Rr 'fromjson? | select(.reason=="compiler-message") | .message.code.code? // empty' | sed '/^$/d' | sort | uniq -c | sort -nr
    ```
- Run becnhmark saving baseline:
    ```
    cargo bench --manifest-path bench/Cargo.toml  -- --save-baseline pre_lint_fix
    ```
- Fix lint detections.
- Run tests.
- Run benchmarks comparing with saved baseline:
    ```
    cargo bench --manifest-path bench/Cargo.toml  -- --baseline pre_lint_fix
    ```


When to use:
- Use this loop when polishing results.
- Use this loop when polish is explicitly requested.
- It is not required for every change.
