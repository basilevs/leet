---
name: leetcode-commit
description: "Commit a LeetCode solution to version control, classifying it as complete, incomplete, or failing first. Use when the user asks to commit a solution/problem, or to version-control training progress. Produces a minimal message whose first line is `<frontend_id> <problem blurb>` and whose body is the problem link. Commits only a problem's solution files and leaves every unrelated change untouched. DO NOT use for tooling, docs, skills, or non-solution commits."
---

# LeetCode Commit

Version-control a LeetCode solution in this training repository. This is
practice, not product work: there is no story, ticket, or rationale to tell.
The commit only records *which problem* and *what state its solution is in*, so
messages stay minimal and mechanical.

See AGENTS.md for the file layout (`<function_name>_<frontend_id>` stem, Rust in
`rust/src/`, C++ in `cpp/src/`) and for the note that registering a module in
`rust/src/lib.rs` is routine and must NOT be mentioned in the message.

## Scope

This skill is deliberately narrow. It covers exactly one thing: committing the
**solution files** of a single LeetCode problem — `rust/src/<stem>.rs`,
`cpp/src/<stem>.cpp`, the `rust/src/lib.rs` registration line, and stem-matched
fixtures. Within that scope its minimal training-log format is the right one;
a solution commit records which problem and what state, not a narrative.

Everything else in the working tree — tooling, build config, the skills
themselves, docs, `AGENTS.md`, other problems — is **out of scope. Leave it
untouched.** Do not stage it, do not commit it, and do not decide what should
happen to it; that is the user's call to make in a separate request.

So when a working tree holds both a solution and unrelated changes, this skill's
job is finished once the solution is committed. Name the untouched files in your
closing report so the user knows what is still pending, and stop there. Do not
carry on to commit them under some other format, and do not ask whether you
should — a request to commit a solution is not a request to clear the tree.

## 1. Gather the facts — one command

Everything mechanical about a solution commit comes from one script. Run it
before anything else, and do not reach for `git status`, `cargo test`,
`make -C cpp test`, `cargo clippy`, or `grep` for `todo` — it has already done
all of them and reconciled the results:

```sh
./.github/skills/leetcode-commit/scripts/classify-solution.sh <stem>
```

Call it with no argument first if you do not yet know the stem; it lists the
problems with uncommitted changes, and separately lists any dirty files that
are *not* part of a solution. That second list exists so you can recognise
those files and leave them alone — it is not a to-do list.

The report gives you the frontend id, the slug and problem link, which files
exist and which are dirty, the exact `git add` path list (including the
`lib.rs` registration line and stem-matched fixtures), the classification per
language and overall, every warning with its lint name, and the ready-made
`(WIP: ...; warn: ...)` parenthetical. Its exit status is the classification —
`0` complete, `1` failing, `2` incomplete, `3` usage or setup error — so you
can branch on it without re-reading the output.

Two things it deliberately leaves to you, because they are not mechanical:

- **The blurb.** It prints the slug with hyphens replaced by spaces as a
  starting point. Turn that into a label a human would write: fix acronyms and
  numerals (`non-zero-xor` → `non-zero XOR`, `two-arrays-i` → `two arrays I`),
  and trim to a handful of words. It is a label, not a summary.
- **Consent for a non-complete state** — see step 3.

Read the solution file yourself only if you need to (e.g. the report flags a
missing origin link, or you want context for the blurb).

## 2. Read the classification

The script has already classified the solution; this section is what its
verdicts mean, not a second set of commands to run.

**Incomplete** — the scaffold is unfinished: a `todo!(` (Rust) or `todo()`
(C++) remains, or no tests ran at all (usually a missing `pub mod <stem>;`).
The report also notes a surviving `dbg!(` on an otherwise-complete solution —
that one is a judgement call, not an automatic downgrade: it is normally a
scaffold leftover worth removing before committing.

**Failing** — implemented, but official tests are red. The report names the
failing tests.

**Complete** — no `todo` markers and every official test passes.

When both languages are present they may differ (e.g. Rust complete, C++
failing); the overall class is the *weakest* of the two (incomplete < failing <
complete), and the script has already applied that rule.

### Compilation warnings

Warnings do NOT change the completeness class — this repo builds
warning-enabled but non-fatal (`-Wall -Wextra -Wpedantic -Wshadow
-Wconversion` in C++; Clippy at `pedantic`/`nursery` strictness in Rust). A
solution with warnings is still "complete" if its tests pass. But warnings are
training signal, so always relay them.

The report lists each lint by name, scoped to this problem's files — the crate
carries lint debt elsewhere that is none of this commit's business. Relay that
list to the user, or say "no compilation warnings" when it is empty. Do not
silently drop warnings just because the tests are green.

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

The script composes this whole parenthetical for you and prints it inside a
ready-to-run `git commit` line — take it verbatim and fill in only the blurb.
The rules above are here so you can recognise a wrong-looking flag, not so you
can rebuild one by hand.

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

Run the two commands the report printed, with the blurb filled in. The `git add`
line already lists exactly this problem's paths — the solution file(s), the
`lib.rs` registration line when the diff adds one, and any stem-matched
fixtures — so nothing unrelated is swept in. The `-m`/`-m` pair keeps the
blank-line separation exact.

```sh
git add rust/src/<stem>.rs rust/src/lib.rs cpp/src/<stem>.cpp
git commit -m "<frontend_id> <blurb>" -m "https://leetcode.com/problems/<slug>"
```

One case needs care: if the report warns that `rust/src/lib.rs` also has changes
unrelated to this stem, `git add rust/src/lib.rs` would stage those too. Stage
that hunk selectively instead.

Do not `git push` unless the user explicitly asks. After committing, report the
one-line title, the classification you used, and the warnings report (each
warning, or "no compilation warnings").

If the tree still holds changes that were out of scope, close by naming them as
left untouched — a plain statement of fact, so the user can see what remains and
decide for themselves. Then stop. The task was to commit this solution, and it
is done.
