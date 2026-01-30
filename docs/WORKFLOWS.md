# Workflows & Processes

This document describes the development workflows, milestone review process, and release procedures for the Real Book Search project.

---

## Git Workflow

### Branch Strategy

**Main Branch:**
- `main` - Production-ready code
- All features merge here via pull requests
- Protected branch (requires review)

**Development Flow:**
```bash
# Create feature branch
git checkout -b feature/your-feature-name

# Work on feature, commit regularly
git add .
git commit -m "Clear, descriptive commit message"

# Push to remote
git push -u origin feature/your-feature-name

# Create pull request on GitHub
gh pr create --title "Feature: Your Feature" --body "Description..."
```

### Commit Message Style

Follow the existing repository style:
```
Add comprehensive documentation and frontend-backend integration
basic f/e & b/e
Rename project from 'realbook-server-rs' to 'realbook-search'
```

**Pattern:**
- Start with verb (Add, Update, Fix, Remove, Refactor)
- Be concise but descriptive
- No issue numbers in commit messages (use PR descriptions instead)

---

## Phase Completion Process

### Overview

This comprehensive checklist ensures nothing is missed when completing a development phase. Follow all steps in order before pushing to remote.

### Complete Phase Completion Checklist

#### 1. ✅ Functionality Testing

**Goal:** Verify everything works as expected

- [ ] Test all new features work correctly
- [ ] Verify no regressions in existing features
- [ ] Test on target platforms (desktop + mobile)
- [ ] Verify API endpoints respond correctly

**Commands:**
```bash
# API integration tests
curl http://localhost:8080/api/search?query=test
curl http://localhost:8080/api/random

# Manual UI testing
# - Search functionality
# - Random button
# - Mobile layout (resize browser)
# - Keyboard navigation (if applicable)
# - Loading states
```

**Done when:** All features work, no critical bugs

---

#### 2. 🔧 Code Quality

**Goal:** Clean, maintainable code without warnings

- [ ] Run clippy and address important warnings
- [ ] Verify compilation succeeds (no errors)
- [ ] Apply refactoring criteria (see CLAUDE.md)
- [ ] Remove unused code, dead CSS, misleading comments
- [ ] Fix collapsible ifs and other style issues

**Commands:**
```bash
# Backend
cargo clippy --all-targets 2>&1 | grep -E "(warning|error):"
cargo check -p api

# Frontend
cd ui && cargo clippy --target wasm32-unknown-unknown 2>&1 | grep -E "(warning|error):"
cd ui && cargo check --target wasm32-unknown-unknown

# Check what needs fixing
cargo clippy --all-targets
```

**Refactoring Criteria to Check:**
1. No misleading comments
2. No documentation conflicts
3. No unnecessary/dead code
4. Proper abstractions (no duplication)
5. Readable (clear names, minimal nesting)
6. Maintainable (DRY, proper separation)
7. Consistent with project patterns

**Done when:**
- Zero compilation errors
- Important clippy warnings fixed (🟡 priority or higher)
- Code passes refactoring criteria

---

#### 3. 📝 Update Documentation

**Goal:** Keep docs in sync with code reality

- [ ] Update CLAUDE.md current status
- [ ] Update CLAUDE.md component list if changed
- [ ] Update CLAUDE.md phase status
- [ ] Add doc comments to new public APIs
- [ ] Update ARCHITECTURE.md if structure changed
- [ ] Update README.md if user-facing changes

**Files to check:**
```bash
# Common files to update
CLAUDE.md          # Always update: Current Status, Current Phase
README.md          # Update if features changed
docs/ARCHITECTURE.md  # Update if structure changed
```

**Done when:** All documentation reflects current code state

---

#### 4. 🔍 Audit Documentation

**Goal:** Catch documentation misalignments

- [ ] Run audit script
- [ ] Fix any warnings or mismatches
- [ ] Verify all file paths exist
- [ ] Check version numbers match

**Commands:**
```bash
./scripts/audit-docs.sh

# If issues found, fix them and re-run
./scripts/audit-docs.sh
```

**Done when:** Audit script passes with no warnings

---

#### 5. 📊 Create Milestone Review

**Goal:** Comprehensive assessment of what was delivered

- [ ] Create review document in `reviews/phase-N-completion.md`
- [ ] Include all required sections (see template below)
- [ ] Be honest about gaps and remaining work
- [ ] Provide clear recommendations for next phase

**Template:**
```markdown
# Product Manager Review - Phase N Completion

**Status:** ✅ Complete / 🚧 Partial / ❌ Blocked
**Timeline:** Completed in X sessions/days
**Quality:** Production-ready / Needs polish / Has issues
**Date:** YYYY-MM-DD

---

## Executive Summary
[2-3 sentences: What was delivered, overall assessment, recommendation]

## What Was Delivered

### ✅ User-Facing Features
1. **Feature Name** 🎯 **CRITICAL** if applicable
   - Bullet points of what it does
   - ⭐ **User Value:** What benefit this provides

### ✅ Technical Improvements
1. **Improvement Name**
   - Technical details
   - Why it matters

## Strengths
### 🎯 Product Strengths
### 💪 Technical Strengths

## Implementation Highlights
[Key innovations, interesting solutions]

## Gaps & Remaining Work
### ⚠️ For Phase N+1
1. **Issue Name (Priority)**
   - Description
   - **Impact:** What this affects

## Refactoring Process
[What refactoring was done, criteria applied]

## Testing Performed
✅ **Tests run:**
- List of tests

## Metrics & Statistics
- Code stats
- Technical debt summary

## Phase N Priorities vs Delivered
[Compare what was planned vs what was delivered]

## Recommendations
### For Phase N+1:
### Process Improvements:

## Conclusion
[Final assessment and approval]
```

**Done when:**
- Review document is comprehensive and honest
- All sections filled out
- Clear approval/rejection status

---

#### 6. 🐙 Create GitHub Issue

**Goal:** Track milestone completion publicly

- [ ] Create GitHub issue from review document
- [ ] Apply correct labels (milestone, pm-review, review)
- [ ] Assign to yourself
- [ ] Verify issue appears in issue list

**Commands:**
```bash
gh issue create \
  --title "Milestone Review: Phase N - [Brief Title]" \
  --label "milestone,pm-review,review" \
  --assignee "@me" \
  --body-file reviews/phase-N-completion.md

# Verify
gh issue list --limit 5 --label milestone
```

**Done when:** Issue created and visible on GitHub

---

#### 7. 💾 Git Commit (Phase Completion)

**Goal:** Commit all phase work with descriptive message

- [ ] Stage all changes
- [ ] Review what's being committed
- [ ] Create descriptive commit message
- [ ] Include Co-Authored-By trailer
- [ ] Verify commit looks correct

**Commands:**
```bash
# Stage and review
git add -A
git status
git diff --staged --stat

# Commit
git commit -m "$(cat <<'EOF'
Complete Phase N: [Brief descriptive title]

- [Major change 1]
- [Major change 2]
- [Major change 3]
- [More changes as needed]
- Update CLAUDE.md with refactoring criteria and milestone process
- Add Phase N completion review

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>
EOF
)"

# Verify
git log --oneline -3
git show HEAD --stat
```

**Commit message format:**
- Title: `Complete Phase N: [Brief title]` (50 chars max)
- Body: Bullet points of major changes
- Always include doc updates
- Always include review mention
- Always end with Co-Authored-By

**Done when:** Commit created with clear, complete message

---

#### 8. 💾 Git Commit (Post-Phase Fixes)

**Goal:** Commit any fixes done after initial phase completion (e.g., clippy warnings)

- [ ] If fixes were made after initial commit, commit them separately
- [ ] Use descriptive message about what was fixed
- [ ] Keep focused (one type of fix per commit if multiple issues)

**Commands:**
```bash
git add -A
git commit -m "$(cat <<'EOF'
Fix [what was fixed]

[Brief description of changes]
- [Specific fix 1]
- [Specific fix 2]

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>
EOF
)"
```

**Done when:** All phase-related commits are clean and descriptive

---

#### 9. 🏷️ Version Tag (Optional)

**Goal:** Mark release points for version tracking

- [ ] Create annotated tag (if releasing)
- [ ] Use semantic versioning (v0.N.0)
- [ ] Include phase description in tag message

**Commands:**
```bash
# Create tag
git tag -a v0.N.0 -m "Phase N: [Brief title]"

# Verify
git tag -l
git show v0.N.0

# Push tag (do this with step 10)
git push origin v0.N.0
```

**When to tag:**
- Major milestones (Phase 1, 2, 3 complete)
- Before production deployments
- When creating GitHub releases

**Done when:** Tag created (don't push yet)

---

#### 10. 🚀 Push to Remote

**Goal:** Share completed phase with team/repository

- [ ] Final verification (all commits ready?)
- [ ] Push commits to origin
- [ ] Push tags if created
- [ ] Verify on GitHub web interface

**Commands:**
```bash
# Final check
git log --oneline -5
git status

# Push
git push origin main

# Push tags if created
git push origin --tags

# Or push specific tag
git push origin v0.N.0

# Verify
gh repo view --web
gh issue list --limit 5
```

**Done when:**
- Commits visible on GitHub
- Issue visible on GitHub
- Tags pushed (if applicable)

---

### Phase Completion Verification Script

Use this script for quick status checking:

```bash
#!/bin/bash
# Quick diagnostic for phase completion status

echo "=== Phase Completion Diagnostic ==="
echo ""

echo "1. Code Quality Check"
echo "   Backend:"
cargo check -p api 2>&1 | tail -1
echo "   Frontend:"
cd ui && cargo check --target wasm32-unknown-unknown 2>&1 | tail -1 && cd ..
echo ""

echo "2. Clippy Warnings"
cargo clippy --all-targets 2>&1 | grep -c "warning"
echo ""

echo "3. Documentation Audit"
./scripts/audit-docs.sh 2>&1 | grep "✅\|⚠️"
echo ""

echo "4. Git Status"
git status --short
echo ""

echo "5. Recent Commits"
git log --oneline -3
echo ""

echo "6. GitHub Issues (Milestones)"
gh issue list --limit 3 --label milestone
echo ""

echo "=== End Diagnostic ==="
```

**Save to:** `scripts/phase-check.sh`

---

### Common Pitfalls

**🚫 Don't:**
- Skip testing before committing
- Forget to update CLAUDE.md phase status
- Commit without running clippy
- Push without creating GitHub issue
- Tag before thorough testing
- Batch multiple unrelated changes in one commit

**✅ Do:**
- Test thoroughly at step 1
- Refactor regularly, not just at phase end
- Update docs as you code, not after
- Create detailed milestone reviews
- Commit with descriptive messages
- Verify everything before pushing

---

### Time Estimates

A complete phase completion typically takes:
- Simple phase (few changes): 30-45 minutes
- Medium phase (moderate changes): 1-2 hours
- Complex phase (major changes): 2-3 hours

**Time breakdown:**
1. Testing: 10-15 minutes
2. Code quality: 15-30 minutes
3. Documentation: 15-30 minutes
4. Audit: 5 minutes
5. Review creation: 30-60 minutes
6. GitHub issue: 5 minutes
7. Git commits: 10-15 minutes
8. Verification: 5-10 minutes

---

## Milestone Review Process

### Overview

At the completion of each milestone/phase, a comprehensive review is conducted and automatically uploaded as a GitHub issue.

### Automated Review Script

**Location:** `scripts/milestone-review.sh`

**Usage:**
```bash
./scripts/milestone-review.sh "Phase N - Description" reviews/phase-n-completion.md
```

**What it does:**
1. Validates milestone name and review file exist
2. Extracts summary from the review markdown
3. Creates a GitHub issue with:
   - Title: "Milestone Review: [Phase Name]"
   - Body: Full review content
   - Labels: `milestone-review`, `phase-N`

**Prerequisites:**
```bash
# GitHub CLI must be authenticated
gh auth login
gh auth setup-git
```

### Review Template

Create reviews in `reviews/` directory following this structure:

```markdown
# Product Manager Review - Phase N Completion

**Status:** ✅ Complete / 🚧 In Progress / ❌ Blocked
**Timeline:** [Description]
**Quality:** [Assessment]
**Date:** YYYY-MM-DD

---

## Executive Summary
[High-level overview]

## What Was Delivered
[Feature list with user value]

## Strengths
[What went well]

## Gaps & Concerns
[Issues and blockers]

## Risk Assessment
[Technical and product risks]

## Recommendation
[Next steps]
```

**Example:** See `reviews/phase-1-completion.md`

### When to Run Reviews

- ✅ After completing a major phase (Phase 1, Phase 2, etc.)
- ✅ Before starting significant refactoring
- ✅ When seeking stakeholder approval
- ✅ At project milestones (Beta, RC, GA)

### Documenting Reviews

1. Write review in `reviews/phase-n-completion.md`
2. Run milestone review script to create GitHub issue
3. Update `CLAUDE.md` and `README.md` with current status
4. Commit all changes:
   ```bash
   git add reviews/ README.md CLAUDE.md
   git commit -m "Add Phase N milestone review"
   git push
   ```

---

## Documentation Audit Process

### Overview

Regular audits ensure documentation stays aligned with code reality.

### Automated Audit Script

**Location:** `scripts/audit-docs.sh`

**Usage:**
```bash
./scripts/audit-docs.sh
```

**What it checks:**
1. Package versions (Rocket, Yew, reqwest)
2. Rust Edition (2024)
3. Port configuration (8000, 8080)
4. API endpoints (matches controller.rs)
5. Data statistics (entry count in realbook.json)
6. Project status references
7. Common misalignments (file path references, edition mismatches)

**Output:**
```
🔍 Running documentation audit...
================================

1️⃣  Package Versions:
   Actual:
      rocket = "0.5.1"
      ...

[continues with checks]

================================
✅ Audit complete!
```

### When to Run Audits

- ✅ Before milestone completions
- ✅ After dependency updates
- ✅ After architectural changes
- ✅ Before creating releases
- ✅ When documentation feels out of sync

### Audit Checklist

See `docs/AUDIT.md` for the full 10-point manual checklist.

### Fixing Audit Issues

1. Run audit script: `./scripts/audit-docs.sh`
2. Note any ⚠️ warnings or mismatches
3. Fix issues in documentation files
4. Re-run audit to verify
5. Commit fixes:
   ```bash
   git add docs/ README.md CLAUDE.md
   git commit -m "Fix documentation misalignments"
   ```

---

## Release Process

### Version Numbering

Follow Semantic Versioning (SemVer):
- `0.1.0` - Initial development
- `0.2.0` - Phase 1 complete (current)
- `0.3.0` - Phase 2 complete
- `1.0.0` - Production-ready release

**Format:** `MAJOR.MINOR.PATCH`
- MAJOR: Breaking changes
- MINOR: New features (backwards compatible)
- PATCH: Bug fixes

### Release Checklist

**Pre-Release:**
1. ✅ Run documentation audit: `./scripts/audit-docs.sh`
2. ✅ Run milestone review (if applicable)
3. ✅ Update version in `Cargo.toml` files
4. ✅ Update `README.md` with new version and features
5. ✅ Test both backend and frontend
6. ✅ Build production artifacts:
   ```bash
   cargo build --release -p api
   cd ui && trunk build --release
   ```

**Creating Release:**
1. Tag the release:
   ```bash
   git tag -a v0.2.0 -m "Phase 1 Complete - Core Search Functionality"
   git push origin v0.2.0
   ```

2. Create GitHub release:
   ```bash
   gh release create v0.2.0 \
     --title "v0.2.0 - Phase 1 Complete" \
     --notes "See reviews/phase-1-completion.md for details"
   ```

**Post-Release:**
1. Update `CLAUDE.md` current status
2. Archive milestone review if not already done
3. Plan next phase priorities

---

## Development Phases

### Phase 1: Core Search Functionality ✅
**Status:** Complete (v0.2.0)
- Backend API with search, volumes, random endpoints
- Frontend with split-screen layout
- 1,161 Real Book entries loaded
- Basic styling (inline CSS)

### Phase 2: UI Refinement & Polish 🚧
**Status:** Planned
**Priorities:**
1. Mobile responsive design (CRITICAL)
2. Pico CSS integration
3. Loading states and error handling
4. Component refactoring

### Phase 3: Advanced Features 📋
**Status:** Backlog
- Fuzzy search
- Favorites/bookmarks
- Keyboard shortcuts
- Analytics integration

---

## CI/CD Pipeline

### Current Status
**Not Yet Implemented**

### Planned (Future)

**GitHub Actions Workflow:**
```yaml
# .github/workflows/ci.yml
name: CI
on: [push, pull_request]
jobs:
  test:
    - cargo check --workspace
    - cargo clippy --workspace
    - cargo test --workspace
  build-frontend:
    - cd ui && trunk build --release
```

**Deployment:**
- Backend: AWS EC2 or ECS
- Frontend: S3 + CloudFront (static hosting)
- See `docs/AWS_MIGRATION_PLAN.md` for details

---

## Troubleshooting Workflows

### Documentation Out of Sync

**Symptoms:**
- Version numbers don't match
- API endpoints missing from docs
- File path references wrong

**Solution:**
1. Run audit: `./scripts/audit-docs.sh`
2. Fix issues flagged
3. Re-run to verify

### GitHub CLI Issues

**Symptom:** `gh auth login` fails or pushes require username

**Solution:**
```bash
gh auth login
gh auth setup-git
git config --global credential.helper ""
```

### Milestone Script Fails

**Symptom:** `./scripts/milestone-review.sh` errors

**Solution:**
```bash
# Make executable
chmod +x scripts/*.sh

# Check GitHub CLI is authenticated
gh auth status

# Verify review file exists
ls -la reviews/
```

---

## Best Practices

### Documentation Maintenance

- ✅ Update docs immediately when code changes
- ✅ Run audit before milestone completions
- ✅ Keep CLAUDE.md in sync with project status
- ✅ Archive old review files (don't delete)

### Git Hygiene

- ✅ Commit frequently with clear messages
- ✅ Don't commit build artifacts (`ui/dist/`, `target/`)
- ✅ Keep commits focused (one logical change per commit)
- ✅ Push regularly to avoid conflicts

### Code Review

- ✅ Self-review before creating PR
- ✅ Run `cargo check` and `cargo clippy` first
- ✅ Test both backend and frontend
- ✅ Update docs in the same PR as code changes

---

## Useful Commands Reference

```bash
# Development
cargo run -p api                    # Start backend
cd ui && trunk serve                # Start frontend
cargo check --workspace             # Fast validation

# Git
git status                          # Check current state
git log --oneline -10              # Recent commits
gh pr list                          # View open PRs

# Auditing
./scripts/audit-docs.sh            # Documentation audit
./scripts/milestone-review.sh ...  # Create milestone issue

# Building
cargo build --release -p api       # Production backend
cd ui && trunk build --release     # Production frontend

# Cleanup
cargo clean                         # Remove build artifacts
rm -rf ui/dist/                    # Remove frontend dist
```

---

_Last updated: 2026-01-30_
