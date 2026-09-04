#!/usr/bin/env bash
# Gather everything a LeetCode solution commit needs that can be determined
# mechanically, in a single command.
#
# Usage:
#   ./scripts/classify-solution.sh            # list candidate stems from git status
#   ./scripts/classify-solution.sh <stem>     # full report for one problem
#
# The commit message itself is NOT this script's job: turning a slug into a
# readable blurb ("...-non-zero-xor" -> "non-zero XOR") needs judgement, and
# committing a non-complete solution needs the user's consent. Everything
# feeding those two decisions is mechanical, so it happens here instead of
# costing a tool call each:
#
#   - identity      frontend id, slug, problem link, a raw blurb to polish
#   - paths         which of the problem's files exist and which are dirty,
#                   including stem-matched fixtures and the lib.rs line
#   - class         incomplete | failing | complete, per language and overall
#   - warnings      C++ compiler lints and Rust Clippy (pedantic + nursery)
#                   lints raised by THIS problem's files, named for the flag
#   - flag          the ready-made "(WIP: ...; warn: ...)" parenthetical
#
# Exit status doubles as the classification, so the caller can branch without
# parsing: 0 complete, 1 failing, 2 incomplete, 3 usage or setup error.
#
# Requires: git, jq, cargo, make.
set -uo pipefail

repo_root=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../../.." && pwd)
cd "$repo_root" || exit 3

# Indent every line of a newline-separated list read on stdin. Several report
# sections print such lists, and a loop keeps each item on its own line without
# depending on word-splitting an unquoted expansion.
indent() { # $1 = prefix
  local prefix=$1 line
  while IFS= read -r line; do
    printf '%s%s\n' "$prefix" "$line"
  done
}

# ---------------------------------------------------------------- discovery --
# No stem given: name the problems with uncommitted changes, so the caller does
# not need a separate `git status` to find them.
if [[ $# -eq 0 ]]; then
  changed=$(git status --porcelain | sed 's/^...//')
  stems=$(printf '%s\n' "$changed" |
    sed -n -e 's|^rust/src/\(.*\)\.rs$|\1|p' -e 's|^cpp/src/\(.*\)\.cpp$|\1|p' \
      -e 's|^rust/src/\(.*_[0-9][0-9]*\)_test.*$|\1|p' |
    grep -E '_[0-9]+$' | sort -u)

  if [[ -n $stems ]]; then
    echo "solution stems with uncommitted changes:"
    indent '  ' <<<"$stems"
  else
    echo "no changed solution files (nothing matching rust/src/*_<id>.rs or cpp/src/*_<id>.cpp)"
  fi

  # Anything else that is dirty is out of scope. Listing it is how the caller
  # recognises what to keep its hands off; it is not a queue of further work.
  other=$(printf '%s\n' "$changed" | grep -vE '^(rust/src/[a-z0-9_]*_[0-9]+(_test[^/]*)?\.(rs|txt|json)|cpp/src/[a-z0-9_]*_[0-9]+\.cpp|rust/src/lib\.rs)$')
  if [[ -n $other ]]; then
    echo
    echo "out of scope — leave untouched, do not stage or commit:"
    indent '  ' <<<"$other"
  fi
  exit 3
fi

if [[ $# -ne 1 || -z ${1:-} ]]; then
  echo "usage: $0 [<function_name>_<frontend_id>]" >&2
  exit 3
fi

stem=$1
if [[ ! $stem =~ _[0-9]+$ ]]; then
  echo "ERROR: '$stem' does not look like <function_name>_<frontend_id>" >&2
  exit 3
fi
frontend_id=${stem##*_}

rust_src="rust/src/$stem.rs"
cpp_src="cpp/src/$stem.cpp"
[[ -f $rust_src ]] || rust_src=""
[[ -f $cpp_src ]] || cpp_src=""

if [[ -z $rust_src && -z $cpp_src ]]; then
  echo "ERROR: neither rust/src/$stem.rs nor cpp/src/$stem.cpp exists" >&2
  exit 3
fi

# ----------------------------------------------------------------- identity --
read_slug() { # $1 = source file
  sed -n '1s|^// https://leetcode\.com/problems/\([a-z0-9-]*\)$|\1|p' "$1"
}

slug=""
slug_note=""
for f in $rust_src $cpp_src; do
  s=$(read_slug "$f")
  if [[ -z $s ]]; then
    slug_note="WARNING: $f has no canonical origin link on line 1"
  elif [[ -z $slug ]]; then
    slug=$s
  elif [[ $s != "$slug" ]]; then
    slug_note="WARNING: rust and cpp origin links disagree ($slug vs $s)"
  fi
done

# ------------------------------------------------------------ test plumbing --
# cargo and cpp/include/test.hpp print the same shapes, so one parser serves
# both: "test <name> ... ok|FAILED" and "test result: ok. N passed; M failed".
tests_passed=0
tests_failed=0
parse_test_result() {
  tests_passed=$(sed -n 's/^test result:.*[^0-9]\([0-9][0-9]*\) passed.*/\1/p' <<<"$1" |
    awk '{s+=$1} END {print s+0}')
  tests_failed=$(sed -n 's/^test result:.*[^0-9]\([0-9][0-9]*\) failed.*/\1/p' <<<"$1" |
    awk '{s+=$1} END {print s+0}')
}

# --------------------------------------------------------------------- rust --
rust_state=""
rust_detail=""
rust_warns=""
if [[ -n $rust_src ]]; then
  if grep -q 'todo!(' "$rust_src"; then
    rust_state=incomplete
    rust_detail="todo!( still in the solution body"
  else
    out=$(cargo test --manifest-path rust/Cargo.toml --lib -- "$stem::" 2>&1)
    parse_test_result "$out"
    if [[ $((tests_passed + tests_failed)) -eq 0 ]]; then
      rust_state=incomplete
      rust_detail="no tests ran — is 'pub mod $stem;' missing from rust/src/lib.rs?"
    elif [[ $tests_failed -gt 0 ]]; then
      rust_state=failing
      rust_detail="$tests_failed of $((tests_passed + tests_failed)) tests failing"
      rust_detail+=$'\n'$(grep -E '^test .* FAILED$' <<<"$out" | sed 's/^/                  /')
    else
      rust_state=complete
      rust_detail="$tests_passed/$tests_passed tests pass"
    fi
  fi

  # Clippy at the strictness AGENTS.md uses for polishing, filtered to this
  # file: the crate carries lint debt elsewhere that is none of this commit's
  # business. Cargo replays cached diagnostics, so this needs no rebuild.
  rust_warns=$(cargo clippy --manifest-path rust/Cargo.toml --lib --tests \
    --message-format=json -- -W clippy::pedantic -W clippy::nursery 2>/dev/null |
    jq -r --arg t "src/$stem.rs" '
      select(.reason == "compiler-message")
      | select([.. | objects | .file_name? // empty] | index($t))
      | .message.code.code? // empty' |
    sed 's/^clippy:://' | sort -u)

  if grep -q 'dbg!(' "$rust_src" && [[ $rust_state == complete ]]; then
    rust_detail+=$'\n''                  NOTE: dbg!( remains — scaffold leftover?'
  fi
fi

# ---------------------------------------------------------------------- cpp --
cpp_state=""
cpp_detail=""
cpp_warns=""
if [[ -n $cpp_src ]]; then
  # `make test T=` filters by substring, so a stem that prefixes another would
  # silently drag it in. Removing just this binary forces a recompile, which is
  # what re-emits the compiler warnings; `make clean` would rebuild everything.
  rm -f "cpp/build/$stem"
  out=$(make -C cpp test T="$stem" 2>&1)
  built=$(grep -cE "^==== build/" <<<"$out")

  if [[ $built -gt 1 ]]; then
    cpp_detail="WARNING: T=$stem matched $built binaries; results below are combined"
  fi

  cpp_warns=$(grep -oE '\[-W[a-z0-9-]+\]' <<<"$out" | tr -d '[]' | sort -u)

  if grep -q 'todo()' "$cpp_src"; then
    cpp_state=incomplete
    cpp_detail="todo() still in the solution body"
  else
    parse_test_result "$out"
    if [[ $((tests_passed + tests_failed)) -eq 0 ]]; then
      cpp_state=incomplete
      cpp_detail="no tests ran (build failure?)"
      cpp_detail+=$'\n'$(grep -E 'error' <<<"$out" | head -5 | sed 's/^/                  /')
    elif [[ $tests_failed -gt 0 ]]; then
      cpp_state=failing
      cpp_detail="$tests_failed of $((tests_passed + tests_failed)) tests failing"
      cpp_detail+=$'\n'$(grep -E '^test .* FAILED$' <<<"$out" | sed 's/^/                  /')
    else
      cpp_state=complete
      cpp_detail="$tests_passed/$tests_passed tests pass"
    fi
  fi
fi

# ------------------------------------------------------------------ overall --
# Weakest state wins: incomplete < failing < complete.
rank() { case $1 in incomplete) echo 0 ;; failing) echo 1 ;; complete) echo 2 ;; *) echo 9 ;; esac; }
overall=complete
for s in $rust_state $cpp_state; do
  [[ $(rank "$s") -lt $(rank "$overall") ]] && overall=$s
done

# -------------------------------------------------------------------- paths --
# An array, not a space-joined string: these become separate argv entries for
# git, and a string would re-split on any path containing a space.
paths=()
for p in $rust_src $cpp_src; do
  paths+=("$p")
done
for f in "rust/src/${stem}"_test*; do
  [[ -e $f ]] && paths+=("$f")
done

lib_note=""
if ! git diff --quiet HEAD -- rust/src/lib.rs 2>/dev/null; then
  lib_diff=$(git diff HEAD -- rust/src/lib.rs | grep -E '^[+-][^+-]')
  if grep -q "pub mod $stem;" <<<"$lib_diff"; then
    paths+=("rust/src/lib.rs")
    if grep -vq "pub mod $stem;" <<<"$lib_diff"; then
      lib_note="WARNING: rust/src/lib.rs also has changes unrelated to $stem — stage that hunk selectively"
    fi
  fi
fi

dirty=$(git status --porcelain --no-renames -- "${paths[@]}" | sed 's/^...//')

# The index may already hold unrelated work staged before this script ran. A
# bare `git commit` would sweep that into the solution commit, so the commit
# line below is pathspec-limited and this names anything at risk.
staged_other=""
staged=$(git diff --cached --name-only --no-renames)
if [[ -n $staged ]]; then
  staged_other=$(grep -vxF "$(printf '%s\n' "${paths[@]}")" <<<"$staged")
fi

# --------------------------------------------------------------------- flag --
all_warns=$(printf '%s\n%s\n' "$rust_warns" "$cpp_warns" | grep -v '^$' | sort -u)
warn_count=$(grep -c . <<<"$all_warns")
[[ -z $all_warns ]] && warn_count=0

state_marker=""
case $overall in
  incomplete)
    reason="todo"
    [[ $rust_state == incomplete && $cpp_state == complete ]] && reason="Rust todo"
    [[ $cpp_state == incomplete && $rust_state == complete ]] && reason="C++ todo"
    state_marker="WIP: $reason"
    ;;
  failing)
    reason="tests failing"
    [[ $rust_state == failing && -z $cpp_state ]] || [[ $rust_state == failing && $cpp_state == complete ]] && reason="Rust tests failing"
    [[ $cpp_state == failing && $rust_state == complete ]] && reason="C++ tests failing"
    state_marker="WIP: $reason"
    ;;
esac

warn_marker=""
if [[ $warn_count -gt 2 ]]; then
  warn_marker="warn: $warn_count"
elif [[ $warn_count -gt 0 ]]; then
  warn_marker="warn: $(paste -sd, - <<<"$all_warns" | sed 's/,/, /g')"
fi

flag=""
if [[ -n $state_marker && -n $warn_marker ]]; then
  flag=" ($state_marker; $warn_marker)"
elif [[ -n $state_marker ]]; then
  flag=" ($state_marker)"
elif [[ -n $warn_marker ]]; then
  flag=" ($warn_marker)"
fi

# ------------------------------------------------------------------- report --
blurb_raw=${slug//-/ }

echo "stem:          $stem"
echo "frontend_id:   $frontend_id"
echo "slug:          ${slug:-<MISSING>}"
echo "link:          https://leetcode.com/problems/$slug"
echo "blurb (raw):   $blurb_raw"
echo "               ^ polish this by hand: fix acronyms/numerals, trim to a label"
[[ -n $slug_note ]] && echo "$slug_note"
echo
echo "class:         $overall"
[[ -n $rust_state ]] && echo "  rust:        $rust_state — $rust_detail"
[[ -n $cpp_state ]] && echo "  cpp:         $cpp_state — $cpp_detail"
echo
if [[ $warn_count -eq 0 ]]; then
  echo "warnings:      none"
else
  echo "warnings:      $warn_count"
  [[ -n $rust_warns ]] && indent '  rust  ' <<<"$rust_warns"
  [[ -n $cpp_warns ]] && indent '  cpp   ' <<<"$cpp_warns"
fi
echo
if [[ -z $dirty ]]; then
  echo "changed files: none — this problem is already committed"
else
  echo "changed files:"
  indent '  ' <<<"$dirty"
fi
[[ -n $lib_note ]] && echo "$lib_note"
echo
if [[ -n $staged_other ]]; then
  echo "ALREADY STAGED, not part of this problem:"
  indent '  ' <<<"$staged_other"
  echo "  ^ the commit line below ends in '-- <paths>', which commits only this"
  echo "    problem's files and leaves the above staged and untouched. Do not"
  echo "    drop that pathspec, and do not unstage their work to tidy up."
  echo
fi
echo "commit (blurb is yours to write):"
echo "  git add ${paths[*]}"
echo "  git commit -m \"$frontend_id <blurb>$flag\" -m \"https://leetcode.com/problems/$slug\" -- ${paths[*]}"
if [[ $overall != complete ]]; then
  echo
  echo "NOT complete — ask the user before committing (skill step 3)."
fi

case $overall in
  complete) exit 0 ;;
  failing) exit 1 ;;
  incomplete) exit 2 ;;
  *) exit 3 ;;
esac
