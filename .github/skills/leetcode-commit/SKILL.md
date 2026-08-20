---
name: leetcode-commit
description: "Commit a LeetCode solution to version control, classifying it as complete, incomplete, or failing first. Use when the user asks to commit a solution/problem, or to version-control training progress. Produces a minimal message whose first line is `<frontend_id> <problem blurb>` and whose body is the problem link. DO NOT use for tooling, docs, skills, or non-solution commits — defer to commit-message-storyteller for those."
---

# LeetCode Commit

Version-control a LeetCode solution in this training repository. This is
practice, not product work: there is no story, ticket, or rationale to tell.
The commit only records *which problem* and *what state its solution is in*, so
messages stay minimal and mechanical.

See AGENTS.md for the file layout (`<function_name>_<frontend_id>` stem, Rust in
`rust/src/`, C++ in `cpp/src/`) and for the note that registering a module in
`rust/src/lib.rs` is routine and must NOT be mentioned in the message.

## Scope and precedence

This skill is deliberately narrow. Use it ONLY for committing the **solution
files** of a LeetCode problem in this repo — `rust/src/<stem>.rs`,
`cpp/src/<stem>.cpp`, the `rust/src/lib.rs` registration line, and stem-matched
fixtures. Within that scope it takes precedence over any general commit-message
skill: solution commits are a minimal training log, not a narrative.

For anything else — tooling, build config, the skills themselves, docs, `AGENTS.md`,
or a mixed change that is not a single problem's solution — do NOT use this
skill. Defer to the general `commit-message-storyteller` skill, whose narrative
Conventional Commits format is the right fit there. If a change spans both a
solution and unrelated files, split it: commit the solution with this skill, and
the rest with the storyteller.

## 1. Identify what is being committed

Determine the solution stem(s) `<function_name>_<frontend_id>` from the user's
request, the active editor file, or `git status`. A single problem may touch the
Rust file, the C++ file, or both; commit the paired files for one problem
together. Do not sweep unrelated files into the same commit — if `git status`
shows changes to other problems or to tooling, stage only this problem's paths
explicitly.

Per problem, the candidate paths are:
- `rust/src/<stem>.rs` and its registration line in `rust/src/lib.rs`
- `cpp/src/<stem>.cpp`
- any adjacent test-data fixtures sharing the stem (e.g. `<stem>_test*.txt`, `<stem>_test*.json`)

Read each changed solution file. Extract from it:
- `<frontend_id>` — the trailing number in the stem.
- `<slug>` — from the top-of-file `// https://leetcode.com/problems/<slug>` header.
- a short **blurb** — a plain-language phrase for the problem, derived from the
  slug (e.g. `longest-subsequence-with-non-zero-xor` → `longest subsequence with
  non-zero XOR`). Keep it to a handful of words; it is a label, not a summary.

## 2. Classify the solution

Every commit must be labelled with the honest state of the solution. Check, for
each language present:

**Incomplete** — the scaffold is unfinished. Signals:
- Rust: a `todo!(` remains in the solution body.
- C++: a `todo(` remains in the solution body.
- an `unused(...)` / `dbg!(...)` argument-suppression call still stands in for
  real work.

**Failing** — implemented but the official tests do not pass. Run only this
problem's tests:
- Rust: `cargo test --manifest-path rust/Cargo.toml -q <stem>::`
- C++: `make -C cpp test T=<stem>`

If a language build/test command needs the project venv or a specific cwd,
activate/enter it in the same command (see environment notes).

**Complete** — no `todo` markers remain and every official test passes.

If both languages are present they may differ (e.g. Rust complete, C++ failing);
classify the commit by the *weakest* state (incomplete < failing < complete).

### Compilation warnings

Warnings do NOT change the completeness class — this repo builds warning-enabled
but non-fatal (`-Wall -Wextra -Wpedantic -Wshadow -Wconversion` in C++; the Rust
compiler's own lints). A solution with warnings is still "complete" if its tests
pass. But warnings are training signal, so the skill MUST surface them.

Capture warnings from a clean build of each language present and report every
one (file:line, lint name, message):
- C++: `make -C cpp clean >/dev/null 2>&1; make -C cpp test T=<stem> 2>&1 | grep -iE 'warning|error'`
- Rust: the `cargo test` run above already prints `warning:` lines; scan them.
  For lint-level detail optionally run `cargo clippy --manifest-path rust/Cargo.toml -q`.

Emit a **warnings report** before composing the message: list each warning, or
state "no compilation warnings" when the build is clean. Do not silently drop
warnings just because the tests are green.

## 3. Confirm intent for non-complete states

Committing a *complete* solution needs no confirmation — proceed. If it carries
compilation warnings, still commit (warnings are non-fatal), but surface the
warnings report so the user can choose to clean them up first.

For an **incomplete** or **failing** solution, tell the user the exact state you
found (which language, which tests failed, or that a `todo!` remains) and ask
whether to commit it as work-in-progress or to hold off. Never silently commit a
broken solution as if it were finished, and never bypass a failing test by
editing the test or skipping hooks.

## 4. Compose the message

Format is fixed and minimal. **First line:**

```
<frontend_id> <blurb>
```

for a complete solution. For a non-complete solution, mark the state inline so
history stays honest:

```
<frontend_id> <blurb> (WIP: <reason>)
```

where `<reason>` is terse — `todo` for an unfinished scaffold, `tests failing`
for a red suite, optionally naming the language when only one side lags
(`C++ tests failing`).

**Compilation warnings must be flagged on the first line.** Whenever the build
emits any warning, append a `warn` flag naming the lint(s) so the flag is visible
in `git log` without opening the diff:

```
<frontend_id> <blurb> (warn: <lint>)
```

Use the lint name (`warn: -Wsign-conversion`); list a couple comma-separated when
distinct, or a count (`warn: 3`) when there are many. A clean build adds no flag.
When a solution is both non-complete and warns, combine the markers in one
parenthetical, state first: `(WIP: tests failing; warn: -Wsign-conversion)`.

**Body:** a blank line, then only the problem link:

```
https://leetcode.com/problems/<slug>
```

Add nothing else for a complete solution — no algorithm write-up, no "add
solution", no bullet list, no mention of registering the Rust module or of
routine build glue. The diff already shows the work; the message just names the
problem and its link. (This is a training log, so brevity is the point — do not
reach for a Conventional Commits `feat(...)` narrative here.)

If a genuinely non-obvious constraint had to be worked around AND the user asked
for a note, a single extra body line is acceptable — but the default is link-only.

### Examples

Complete:

```
3702 longest subsequence with non-zero XOR

https://leetcode.com/problems/longest-subsequence-with-non-zero-bitwise-xor
```

Complete but with a compiler warning:

```
3069 distribute elements into two arrays I (warn: -Wsign-conversion)

https://leetcode.com/problems/distribute-elements-into-two-arrays-i
```

Incomplete scaffold committed as WIP at the user's request:

```
1510 stone game IX (WIP: todo)

https://leetcode.com/problems/stone-game-ix
```

Implemented but red suite:

```
2213 longest repeating substitution (WIP: tests failing)

https://leetcode.com/problems/longest-repeating-character-replacement-with-substitution
```

## 5. Stage and commit

Stage only this problem's paths (the solution file(s), the `lib.rs` registration
line for a new Rust module, and any stem-matched fixtures), then commit with the
composed message. Prefer a single `-m`/`-m` pair (title, body) so the blank-line
separation is exact.

```sh
git add rust/src/<stem>.rs rust/src/lib.rs cpp/src/<stem>.cpp
git commit -m "<frontend_id> <blurb>" -m "https://leetcode.com/problems/<slug>"
```

Do not `git push` unless the user explicitly asks. After committing, report the
one-line title, the classification you used, and the warnings report (each
warning, or "no compilation warnings").
