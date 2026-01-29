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
