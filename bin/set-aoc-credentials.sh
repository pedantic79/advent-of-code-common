#!/usr/bin/env bash
set -euo pipefail

if (( $# != 0 )); then
    printf 'Usage: %s\n' "$0" >&2
    exit 2
fi

if [[ ${HOME:-} != /* ]]; then
    printf 'HOME must be an absolute path.\n' >&2
    exit 1
fi

session=''
if [[ -t 0 ]]; then
    printf 'Advent of Code session cookie: ' >&2
    IFS= read -r -s session || true
    printf '\n' >&2
else
    IFS= read -r session || true
fi

if [[ ! $session =~ ^[[:xdigit:]]+$ ]]; then
    printf 'Expected a nonempty hexadecimal session cookie.\n' >&2
    exit 1
fi

session_file="$HOME/.adventofcode.session"
if [[ -d $session_file || -L $session_file ]]; then
    printf 'Refusing to replace a directory or symbolic link: %s\n' "$session_file" >&2
    exit 1
fi

umask 077
tmp_session=$(mktemp "$session_file.tmp.XXXXXX")
trap 'rm -f "$tmp_session"' EXIT
printf '%s\n' "$session" > "$tmp_session"
unset session

mv -f "$tmp_session" "$session_file"
printf 'Updated aoc-harness session file: %s\n' "$session_file"

if [[ ${ADVENT_OF_CODE_SESSION:-} =~ [^[:space:]] ]]; then
    printf 'ADVENT_OF_CODE_SESSION takes precedence over the aoc-harness session file.\n' >&2
fi
