Strange ineffective top-level inputs in Rust code come from https://leetcode.com/ challenges and can't be changed. Consider them an external unavoidable limitation.
Run tests after reformat and lint fixes.


# Polishing procedure
Steps:
- Run lint checks (optionally strict Clippy groups).
    To see Clippy warnings statistics, use:
    ```
    cargo clippy -q --message-format=json -- -W clippy::pedantic -W clippy::nursery | jq -Rr 'fromjson? | select(.reason=="compiler-message") | .message.code.code? // empty' | sed '/^$/d' | sort | uniq -c | sort -nr
    ```
- Run becnhmark saving baseline:
    ```
    cargo bench -- --save-baseline pre_lint_fix
    ```
- Fix lint detections.
- Run tests.
- Run benchmarks comparing with saved baseline:
    ```
    cargo bench -- --baseline pre_lint_fix
    ```


When to use:
- Use this loop when polishing results.
- Use this loop when polish is explicitly requested.
- It is not required for every change.
