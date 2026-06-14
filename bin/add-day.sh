#!/bin/bash

# Precheck: Ensure all submodules are on a branch
for sm in $(git submodule foreach --quiet 'echo $sm_path'); do
    if ! git -C "$sm" symbolic-ref -q HEAD >/dev/null; then
        # Find branches pointing to the current commit
        branches=$(git -C "$sm" for-each-ref --points-at HEAD --format="%(refname:short)" refs/heads/)
        
        # Count the branches
        if [ -z "$branches" ]; then
            count=0
        else
            count=$(echo "$branches" | wc -l | tr -d ' ')
        fi
        
        if [ "$count" -eq 1 ]; then
            echo "Submodule $sm is not on a branch. Switching to the only branch pointing to this commit: $branches"
            if ! git -C "$sm" switch "$branches"; then
                echo "Error: Failed to switch submodule $sm to branch $branches" >&2
                exit 1
            fi
        elif [ "$count" -gt 1 ]; then
            remote_head=$(git -C "$sm" symbolic-ref -q refs/remotes/origin/HEAD)
            if [ -z "$remote_head" ]; then
                git -C "$sm" remote set-head origin -a >/dev/null 2>&1
                remote_head=$(git -C "$sm" symbolic-ref -q refs/remotes/origin/HEAD)
            fi
            
            if [ -z "$remote_head" ]; then
                echo "Error: Submodule $sm is not on a branch, has multiple branches pointing to this commit, and origin/HEAD could not be determined." >&2
                exit 1
            fi
            
            local_branch_name=${remote_head#refs/remotes/origin/}
            
            if echo "$branches" | grep -Fxq "$local_branch_name"; then
                echo "Submodule $sm is not on a branch. Switching to the origin/HEAD branch: $local_branch_name"
                if ! git -C "$sm" switch "$local_branch_name"; then
                    echo "Error: Failed to switch submodule $sm to branch $local_branch_name" >&2
                    exit 1
                fi
            else
                echo "Error: Submodule $sm is not on a branch, and the origin/HEAD branch ($local_branch_name) does not point to the current commit." >&2
                exit 1
            fi
        else
            echo "Error: Submodule $sm is not on a branch, and no local branches point to the current commit." >&2
            exit 1
        fi
    fi
done

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
