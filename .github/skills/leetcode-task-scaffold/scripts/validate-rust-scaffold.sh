#!/usr/bin/env bash
# Validate ONE scaffolded Rust module against everything this repository can
# check deterministically, in a single command.
#
# Usage:
#   ./scripts/validate-rust-scaffold.sh <function_name>_<frontend_id>
#
# Checks, in order:
#   1. rust/src/<stem>.rs exists.
#   2. Its first line is the canonical origin link, no trailing slash.
#   3. It is registered in rust/src/lib.rs.
#   4. The solution body is still unimplemented (`todo!(`) — scaffold mode.
#   5. At least one test named official1, and every test named official<N>
#      forms a gap-free run starting at 1.
#   6. It is Clippy-clean, tests included.
#   7. Every test in the module fails by reaching `todo!()` — proof the tests
#      actually drive the unwritten solution instead of asserting nothing.
#
# Checks 1-5 are textual and are reported together; 6 and 7 need a build, so
# they run only once the cheap checks pass.
#
# Checks 6 and 7 are genuinely two builds: `cargo clippy` is check-only (no
# codegen), so it cannot produce the test binaries check 7 has to run. They
# cannot be merged into one build here, because the obvious trick —
# RUSTC_WORKSPACE_WRAPPER=clippy-driver on `cargo test` — makes the crate's
# many pre-existing Clippy findings hard errors under `warnings = "deny"`,
# and the build then dies before emitting any test binary at all. The two
# artifact sets do cache side by side, so only the first run pays full price
# (~19s); an edit-and-rerun cycle is ~3s.
#
# Requires: cargo, jq.
set -uo pipefail

if [[ $# -ne 1 || -z "${1:-}" ]]; then
  echo "usage: $0 <function_name>_<frontend_id>" >&2
  exit 2
fi

stem=$1
repo_root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../../.." && pwd)
manifest="$repo_root/rust/Cargo.toml"
source_file="$repo_root/rust/src/$stem.rs"
target="src/$stem.rs" # how rustc spells it in diagnostics

fail() {
  echo "FAIL: $*" >&2
  failed=$((failed + 1))
}

failed=0

# --- 1. file exists -----------------------------------------------------------
if [[ ! -f "$source_file" ]]; then
  echo "FAIL: $source_file does not exist" >&2
  exit 2
fi

# --- 2. origin link -----------------------------------------------------------
first_line=$(head -n 1 "$source_file")
if ! [[ $first_line =~ ^//\ https://leetcode\.com/problems/[a-z0-9-]+$ ]]; then
  fail "first line is not a canonical origin link (no trailing slash):
        want: // https://leetcode.com/problems/<slug>
        got:  $first_line"
fi

# --- 3. module registration ---------------------------------------------------
if ! grep -qx "pub mod $stem;" "$repo_root/rust/src/lib.rs"; then
  fail "rust/src/lib.rs is missing 'pub mod $stem;'"
fi

# --- 4. still a scaffold ------------------------------------------------------
if ! grep -q 'todo!(' "$source_file"; then
  fail "no 'todo!(' in $target — the scaffold must not ship an implementation"
fi

# --- 5. official test naming --------------------------------------------------
# Newline-separated rather than an array: macOS ships bash 3.2, which has no
# `mapfile` and trips over `${#arr[@]}` on an empty array under `set -u`.
officials=$(grep -oE '\bfn +official[0-9]+\b' "$source_file" |
  grep -oE '[0-9]+$' | sort -n | uniq)
if [[ -z $officials ]]; then
  fail "no '#[test] fn official<N>' in $target — populate tests from the official examples"
else
  expected=1
  for n in $officials; do
    if [[ $n -ne $expected ]]; then
      fail "official test numbers must run 1,2,3,... without gaps; found: ${officials//$'\n'/ }"
      break
    fi
    expected=$((expected + 1))
  done
fi

if [[ $failed -ne 0 ]]; then
  echo "--- $target: $failed check(s) failed; not building until they pass ---" >&2
  exit 1
fi

# --- 6. compile + Clippy, filtered to this file -------------------------------
# The crate holds many older solutions that still trip Clippy, and
# `[lints.rust] warnings = "deny"` turns those into errors, so a bare
# `cargo clippy` buries the new file's problems under dozens of unrelated ones.
# --lib --tests is required: without --tests the #[cfg(test)] block is never
# analysed and lints like clippy::bool_assert_comparison stay invisible.
diagnostics=$(cargo clippy --manifest-path "$manifest" --lib --tests \
  --message-format=json 2>/dev/null |
  jq -r --arg target "$target" '
    select(.reason == "compiler-message")
    | select([.. | objects | .file_name? // empty] | index($target))
    | .message.rendered')

if [[ -n $diagnostics ]]; then
  printf '%s\n' "$diagnostics"
  echo "FAIL: compiler/Clippy diagnostics in $target (see above)" >&2
  echo "      Fix the scaffold. Never add #[allow(...)], never edit another module." >&2
  exit 1
fi

# --- 7. tests run and fail by reaching todo!() --------------------------------
test_output=$(cargo test --manifest-path "$manifest" --lib -- "$stem::" 2>&1)
results=$(grep -E "^test $stem::" <<<"$test_output")
result_count=$(grep -c . <<<"$results")

if [[ -z $results ]]; then
  printf '%s\n' "$test_output"
  echo "FAIL: cargo test ran no tests matching '$stem::'" >&2
  exit 1
fi

passing=$(grep -c '\.\.\. ok$' <<<"$results")
if [[ $passing -ne 0 ]]; then
  printf '%s\n' "$results"
  echo "FAIL: $passing test(s) passed against an unimplemented solution." >&2
  echo "      A test that passes here is not exercising $stem — it asserts nothing," >&2
  echo "      or it never calls the solution. Every test must reach todo!()." >&2
  exit 1
fi

if ! grep -q 'not yet implemented' <<<"$test_output"; then
  printf '%s\n' "$test_output"
  echo "FAIL: tests failed for some reason other than reaching todo!()." >&2
  echo "      Expected every test to panic with 'not yet implemented'." >&2
  exit 1
fi

echo "OK: $target — registered, Clippy-clean, $result_count test(s) all failing at todo!() as intended"
