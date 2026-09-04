#!/usr/bin/env bash
# Fetch a LeetCode problem's scaffold data (frontend id, origin link, code
# templates, official example testcases, and problem content) in one shot.
#
# Usage:
#   ./scripts/fetch-problem.sh <title-slug> [rust|cpp|both]
#
# The <title-slug> is the last path segment of the problem URL, e.g. for
# https://leetcode.com/problems/two-sum/ the slug is "two-sum".
#
# The second argument selects which language templates to print; it defaults to
# "both" so a single call can scaffold Rust, C++, or both without refetching.
#
# Requires: curl, jq.
set -euo pipefail

if [[ $# -lt 1 || $# -gt 2 || -z "${1:-}" ]]; then
  echo "usage: $0 <title-slug> [rust|cpp|both]" >&2
  exit 2
fi

slug=$1
lang=${2:-both}

case "$lang" in
  rust) langs='["rust"]' ;;
  cpp | c++) langs='["cpp"]' ;;
  both) langs='["rust","cpp"]' ;;
  *)
    echo "ERROR: unknown language '$lang' (expected rust, cpp, or both)" >&2
    exit 2
    ;;
esac

# $titleSlug here is a GraphQL variable, not a shell one — the server binds it
# from the JSON payload below. Single quotes are required; expanding it in the
# shell would send an empty name and the query would fail.
# shellcheck disable=SC2016
query='query questionData($titleSlug: String!) { question(titleSlug: $titleSlug) { questionFrontendId title titleSlug content exampleTestcases codeSnippets { lang langSlug code } } }'

curl -s 'https://leetcode.com/graphql' \
  -H 'Content-Type: application/json' \
  -H 'Referer: https://leetcode.com' \
  --data "$(jq -n --arg slug "$slug" --arg query "$query" \
    '{operationName: "questionData", variables: {titleSlug: $slug}, query: $query}')" \
  | jq -r --argjson langs "$langs" '
    .data.question as $q
    | if $q == null then
        "ERROR: no problem found for the given slug\n" | halt_error(1)
      else
        "frontendId: \($q.questionFrontendId)",
        "// https://leetcode.com/problems/\($q.titleSlug)\n",
        ( $langs[]
          | . as $l
          | "--- \($l) template ---",
            ( ($q.codeSnippets[] | select(.langSlug == $l) | .code)
              // "ERROR: no \($l) template offered for this problem" ),
            ""
        ),
        "--- exampleTestcases ---",
        $q.exampleTestcases,
        "\n--- content ---",
        $q.content
      end'
