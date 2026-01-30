#!/bin/bash
# Phase Completion Diagnostic Script
# Checks status of all phase completion requirements

set -e

echo "╔════════════════════════════════════════╗"
echo "║   Phase Completion Diagnostic Tool    ║"
echo "╚════════════════════════════════════════╝"
echo ""

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 1. Code Quality Check
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "1️⃣  CODE QUALITY"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

echo "Backend compilation:"
if cargo check -p api 2>&1 | grep -q "Finished"; then
    echo -e "   ${GREEN}✅ Backend compiles${NC}"
else
    echo -e "   ${RED}❌ Backend has errors${NC}"
fi

echo "Frontend compilation:"
cd ui
if cargo check --target wasm32-unknown-unknown 2>&1 | grep -q "Finished"; then
    echo -e "   ${GREEN}✅ Frontend compiles${NC}"
else
    echo -e "   ${RED}❌ Frontend has errors${NC}"
fi
cd ..
echo ""

# 2. Clippy Warnings
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "2️⃣  CLIPPY WARNINGS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

WARNING_COUNT=$(cargo clippy --all-targets 2>&1 | grep -c "warning:" || true)
if [ "$WARNING_COUNT" -eq 0 ]; then
    echo -e "   ${GREEN}✅ No clippy warnings${NC}"
elif [ "$WARNING_COUNT" -lt 5 ]; then
    echo -e "   ${YELLOW}⚠️  $WARNING_COUNT minor warnings${NC}"
else
    echo -e "   ${RED}❌ $WARNING_COUNT warnings (review needed)${NC}"
fi
echo ""

# 3. Documentation Audit
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "3️⃣  DOCUMENTATION AUDIT"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ -f "./scripts/audit-docs.sh" ]; then
    if ./scripts/audit-docs.sh 2>&1 | grep -q "✅ Audit complete"; then
        echo -e "   ${GREEN}✅ Documentation audit passed${NC}"
    else
        echo -e "   ${YELLOW}⚠️  Documentation has issues${NC}"
    fi
else
    echo -e "   ${YELLOW}⚠️  Audit script not found${NC}"
fi
echo ""

# 4. Git Status
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "4️⃣  GIT STATUS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

STAGED=$(git diff --cached --name-only | wc -l)
UNSTAGED=$(git diff --name-only | wc -l)
UNTRACKED=$(git ls-files --others --exclude-standard | wc -l)

if [ "$STAGED" -gt 0 ]; then
    echo -e "   ${YELLOW}📝 $STAGED files staged${NC}"
fi
if [ "$UNSTAGED" -gt 0 ]; then
    echo -e "   ${YELLOW}📝 $UNSTAGED files unstaged${NC}"
fi
if [ "$UNTRACKED" -gt 0 ]; then
    echo -e "   ${YELLOW}📝 $UNTRACKED untracked files${NC}"
fi
if [ "$STAGED" -eq 0 ] && [ "$UNSTAGED" -eq 0 ] && [ "$UNTRACKED" -eq 0 ]; then
    echo -e "   ${GREEN}✅ Working directory clean${NC}"
fi
echo ""

# 5. Recent Commits
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "5️⃣  RECENT COMMITS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
git log --oneline -3
echo ""

# 6. GitHub Issues (Milestones)
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "6️⃣  MILESTONE ISSUES"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if command -v gh &> /dev/null; then
    ISSUE_COUNT=$(gh issue list --label milestone --json number | grep -c "number" || true)
    if [ "$ISSUE_COUNT" -gt 0 ]; then
        echo -e "   ${GREEN}✅ $ISSUE_COUNT milestone issue(s) found${NC}"
        gh issue list --limit 3 --label milestone
    else
        echo -e "   ${RED}❌ No milestone issues found${NC}"
    fi
else
    echo -e "   ${YELLOW}⚠️  GitHub CLI not installed${NC}"
fi
echo ""

# 7. Branch Status
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "7️⃣  BRANCH STATUS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

BRANCH=$(git branch --show-current)
AHEAD=$(git rev-list --count origin/main..HEAD 2>/dev/null || echo "0")
BEHIND=$(git rev-list --count HEAD..origin/main 2>/dev/null || echo "0")

echo "   Current branch: $BRANCH"
if [ "$AHEAD" -gt 0 ]; then
    echo -e "   ${YELLOW}↑ $AHEAD commit(s) ahead of origin/main${NC}"
fi
if [ "$BEHIND" -gt 0 ]; then
    echo -e "   ${YELLOW}↓ $BEHIND commit(s) behind origin/main${NC}"
fi
if [ "$AHEAD" -eq 0 ] && [ "$BEHIND" -eq 0 ]; then
    echo -e "   ${GREEN}✅ In sync with remote${NC}"
fi
echo ""

# Summary
echo "╔════════════════════════════════════════╗"
echo "║              SUMMARY                   ║"
echo "╚════════════════════════════════════════╝"
echo ""
echo "Next steps:"
echo "  • If any ❌ items, fix them before pushing"
echo "  • Review WORKFLOWS.md for detailed checklist"
echo "  • Run: ./scripts/phase-check.sh to re-verify"
echo ""
