Strange ineffective top-level inputs in Rust code come from https://leetcode.com/ challenges and can't be changed. Consider them an external unavoidable limitation.
Run tests after reformat and lint fixes.

To see Clippy warnings statistics, use:
```
cargo clippy -q --message-format=json -- -W clippy::pedantic -W clippy::nursery | jq -Rr 'fromjson? | select(.reason=="compiler-message") | .message.code.code? // empty' | sed '/^$/d' | sort | uniq -c | sort -nr
```
