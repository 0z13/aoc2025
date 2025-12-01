#!/usr/bin/env bash

cd "$(dirname "$0")" || exit 1


year="$(date +%Y)"
today="$(date +%d)"


. .cookie || exit 1

for day in {1..25}; do
  file_name="$day".in

  if [[ "$today" -lt "$day" ]]; then
    exit 0
  fi

  if [[ -r "$file_name" ]]; then
    continue
  fi

  curl -Ss -o "data/$file_name" -b "$AOC_COOKIE" https://adventofcode.com/"$year"/day/"$day"/input
done

