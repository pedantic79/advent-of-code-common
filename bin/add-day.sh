#!/bin/bash

DAY=$1
if [ -z "$DAY" ]; then
    LAST_DAY=$(ls src/day[0-9][0-9].rs 2>/dev/null | grep -o '[0-9]\+' | sort -n | tail -n 1)
    if [ -z "$LAST_DAY" ]; then
        DAY=1
    else
        DAY=$((10#$LAST_DAY + 1))
    fi
fi

NUM=$(printf "%02d" "$DAY")

if [ -f "src/day$NUM.rs" ]; then
    echo "day$NUM already exists"
    exit 1
fi

cp "src/template.rs" "src/day$NUM.rs"
gsed -i "s/dayN/day$DAY/" "src/day$NUM.rs"
gsed -i "/Insert before/i pub mod day$NUM;" "src/lib.rs"

if [ -e .year ]; then
    year=$(cat .year | tr -d '\n')
else
    year=$(date +%Y | tr -d '\n')
fi

gsed -i "/Insert before/i - \[Day $DAY:\]\(https://adventofcode.com/$year/day/$DAY\)\n  - \[solution\]\(src/day$NUM.rs\)" README.md
cargo fmt
