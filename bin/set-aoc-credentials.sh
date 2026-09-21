#!/usr/bin/env bash
set -euo pipefail

if (( $# != 0 )); then
    printf 'Usage: %s\n' "$0" >&2
    exit 2
fi

if [[ -z ${HOME:-} ]]; then
    printf 'HOME must be set.\n' >&2
    exit 1
fi

# cargo-aoc 0.3.8 stores its session in this platform-specific config directory.
case "$(uname -s)" in
    Darwin)
        cargo_aoc_dir="$HOME/Library/Application Support/com.github.gobanos.cargo-aoc"
        ;;
    Linux)
        if [[ -n ${XDG_CONFIG_HOME:-} ]]; then
            if [[ $XDG_CONFIG_HOME != /* ]]; then
                printf 'XDG_CONFIG_HOME must be an absolute path.\n' >&2
                exit 1
            fi
            cargo_aoc_dir="$XDG_CONFIG_HOME/cargo-aoc"
        else
            cargo_aoc_dir="$HOME/.config/cargo-aoc"
        fi
        ;;
    *)
        printf 'Unsupported operating system.\n' >&2
        exit 1
        ;;
esac

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

aoc_file="$HOME/.adventofcode.session"
cargo_aoc_file="$cargo_aoc_dir/credentials.toml"

for target in "$aoc_file" "$cargo_aoc_file"; do
    if [[ -d $target || -L $target ]]; then
        printf 'Refusing to replace a directory or symbolic link: %s\n' "$target" >&2
        exit 1
    fi
done

umask 077
mkdir -p "$cargo_aoc_dir"

tmp_aoc=''
tmp_cargo_aoc=''
cleanup() {
    [[ -z $tmp_aoc ]] || rm -f "$tmp_aoc"
    [[ -z $tmp_cargo_aoc ]] || rm -f "$tmp_cargo_aoc"
}
trap cleanup EXIT

tmp_aoc="$(mktemp "$aoc_file.tmp.XXXXXX")"
tmp_cargo_aoc="$(mktemp "$cargo_aoc_file.tmp.XXXXXX")"
printf '%s\n' "$session" > "$tmp_aoc"
printf 'session = "%s"\n' "$session" > "$tmp_cargo_aoc"
chmod 600 "$tmp_aoc" "$tmp_cargo_aoc"
unset session

mv -f "$tmp_aoc" "$aoc_file"
mv -f "$tmp_cargo_aoc" "$cargo_aoc_file"
printf 'Updated credentials for aoc and cargo-aoc.\n'

if [[ -n ${ADVENT_OF_CODE_SESSION:-} ]]; then
    printf 'ADVENT_OF_CODE_SESSION is set and takes precedence over the aoc session file.\n' >&2
fi
