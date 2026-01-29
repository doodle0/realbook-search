#!/bin/bash
# Milestone Review Generator and GitHub Issue Creator
# Usage: ./scripts/milestone-review.sh [milestone-name] [review-file.md]
#
# Example: ./scripts/milestone-review.sh "Phase 1" reviews/phase-1-review.md

set -e

MILESTONE_NAME=$1
REVIEW_FILE=$2

if [ -z "$MILESTONE_NAME" ] || [ -z "$REVIEW_FILE" ]; then
    echo "Usage: $0 <milestone-name> <review-file>"
    echo "Example: $0 'Phase 1' reviews/phase-1-review.md"
    exit 1
fi

if [ ! -f "$REVIEW_FILE" ]; then
    echo "Error: Review file '$REVIEW_FILE' not found"
    exit 1
fi

# Check if gh CLI is available
if ! command -v gh &> /dev/null; then
    echo "Error: GitHub CLI (gh) is not installed"
    echo "Install: https://cli.github.com/"
    exit 1
fi

echo "📝 Creating GitHub issue for milestone: $MILESTONE_NAME"
echo "📄 Using review file: $REVIEW_FILE"
echo ""

# Create the GitHub issue
gh issue create \
  --title "Milestone Review: $MILESTONE_NAME" \
  --body-file "$REVIEW_FILE" \
  --label "milestone,review,pm-review" \
  --assignee "@me"

echo ""
echo "✅ Issue created successfully!"
echo "View all milestone reviews: gh issue list --label milestone"
