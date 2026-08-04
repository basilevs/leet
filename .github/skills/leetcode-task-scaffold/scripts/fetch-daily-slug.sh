#!/usr/bin/env bash
# Resolve the title slug of LeetCode's current daily coding challenge, so a
# scaffold can be created without the user looking up the slug themselves.
#
# Usage:
#   ./scripts/fetch-daily-slug.sh
#
# Prints the slug (e.g. "two-sum") on stdout. Feed it to fetch-problem.sh.
# Requires: curl, jq.
set -euo pipefail

query='query questionOfToday { activeDailyCodingChallengeQuestion { date question { titleSlug } } }'

curl -s 'https://leetcode.com/graphql' \
  -H 'Content-Type: application/json' \
  -H 'Referer: https://leetcode.com' \
  --data "$(jq -n --arg query "$query" \
    '{operationName: "questionOfToday", variables: {}, query: $query}')" \
  | jq -r '
    .data.activeDailyCodingChallengeQuestion as $d
    | if $d == null then
        "ERROR: could not resolve the daily challenge\n" | halt_error(1)
      else
        $d.question.titleSlug
      end'
