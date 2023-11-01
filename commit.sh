#!/usr/bin/bash
if [[ $# -ne 2 ]]; then
    echo  "Usage: $0 \"commit message\" branch_name"
    exit 1
fi
git add -A
git commit -m "$1" 
git push origin "$2"
