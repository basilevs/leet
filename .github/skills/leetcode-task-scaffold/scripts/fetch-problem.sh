#!/usr/bin/env bash
# Fetch a LeetCode problem's scaffold data (frontend id, origin link, Rust
# template, official example testcases, and problem content) in one shot.
#
# Usage:
#   ./scripts/fetch-problem.sh <title-slug>
#
# The <title-slug> is the last path segment of the problem URL, e.g. for
# https://leetcode.com/problems/two-sum/ the slug is "two-sum".
#
# Requires: curl, jq.
set -euo pipefail

if [[ $# -ne 1 || -z "${1:-}" ]]; then
  echo "usage: $0 <title-slug>" >&2
  exit 2
fi

slug=$1

query='query questionData($titleSlug: String!) { question(titleSlug: $titleSlug) { questionFrontendId title titleSlug content exampleTestcases codeSnippets { lang langSlug code } } }'

curl -s 'https://leetcode.com/graphql' \
  -H 'Content-Type: application/json' \
  -H 'Referer: https://leetcode.com' \
  --data "$(jq -n --arg slug "$slug" --arg query "$query" \
    '{operationName: "questionData", variables: {titleSlug: $slug}, query: $query}')" \
  | jq -r '
    .data.question as $q
    | if $q == null then
        "ERROR: no problem found for the given slug\n" | halt_error(1)
      else
        "frontendId: \($q.questionFrontendId)",
        "// https://leetcode.com/problems/\($q.titleSlug)\n",
        "--- rust template ---",
        ($q.codeSnippets[] | select(.langSlug == "rust") | .code),
        "\n--- exampleTestcases ---",
        $q.exampleTestcases,
        "\n--- content ---",
        $q.content
      end'
