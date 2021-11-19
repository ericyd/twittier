#!/bin/sh

# Pre-commit hook
#
# Find files that are staged, and format them.
# Re-stage all previously-staged files, to ensure any changes are included in the commit
#
# To set up (e.g. after cloning the repo):
# echo "#!/bin/sh" > .git/hooks/pre-commit
# echo "./scripts/pre-commit.sh" >> .git/hooks/pre-commit
# chmod +x .git/hooks/pre-commit

# we only want to operate on files we were originally intending to commit - ignore unstaged files
staged=$(git diff --name-only --cached --diff-filter=d)

cargo fmt

echo $staged | xargs git add
