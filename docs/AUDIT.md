# Documentation Audit Process

**Purpose:** Ensure documentation stays aligned with codebase as the project evolves.

**Frequency:** Run audit at each milestone completion or major change.

---

## Audit Checklist

### 1. Package Versions
**What to check:** Dependencies in Cargo.toml match versions documented in README.md and CLAUDE.md

```bash
# Compare actual vs documented versions
grep -h "version\|rocket\|yew\|reqwest" api/Cargo.toml ui/Cargo.toml
grep "Rocket\|Yew\|reqwest" README.md CLAUDE.md | grep -i "version\|0\."
```

**Expected:** All version numbers match

---

### 2. Rust Edition & Version
**What to check:** Edition in Cargo.toml files matches documentation

```bash
# Check actual edition
grep "edition" api/Cargo.toml ui/Cargo.toml Cargo.toml

# Check documented edition
grep -i "edition" README.md CLAUDE.md docs/*.md
```

**Expected:** All files show same edition (currently 2024)

---

### 3. Port Numbers
**What to check:** Port numbers consistent across docs and code

```bash
# Find all port mentions
grep -n "localhost:[0-9]\|port.*[0-9]" README.md CLAUDE.md docs/*.md
```

**Expected:**
- Backend: 8000
- Frontend dev server (Trunk): 8080
- Proxy configured correctly in Trunk.toml

---

### 4. API Endpoints
**What to check:** Documented endpoints match actual route handlers

```bash
# Documented endpoints
grep "GET /api" README.md CLAUDE.md

# Actual routes
grep "#\[get" api/src/controller.rs
```

**Expected:** All routes documented, no undocumented routes

---

### 5. File Structure
**What to check:** Documented directory structure matches reality

```bash
# Check key files exist
ls -la api/src/
ls -la ui/src/
ls -la api/resources/
```

**Expected:** All mentioned files exist, no orphaned references

---

### 6. Data & Statistics
**What to check:** Entry counts and data statistics are accurate

```bash
# Count actual entries
grep -c '"title"' api/resources/realbook.json

# Check documented count
grep -i "entries\|songs" README.md CLAUDE.md
```

**Expected:** Count matches (currently 1,161)

---

### 7. URLs & Links
**What to check:** CDN URLs, GitHub links, external references

```bash
# Check CDN URLs
grep "wypn9z41ir5bzmgjjalyna\|drv.tw" api/src/models.rs ui/src/models.rs
grep "wypn9z41ir5bzmgjjalyna\|drv.tw" CLAUDE.md

# Check GitHub URLs
grep "github.com/doodle0/realbook" README.md CLAUDE.md
```

**Expected:** All URLs functional and consistent

---

### 8. API Configuration
**What to check:** Constants and configuration values

```bash
# Check API_BASE_URL location and value
grep -n "API_BASE_URL" ui/src/api.rs
grep -n "API_BASE_URL" CLAUDE.md
```

**Expected:** Documentation points to correct file and value

---

### 9. Project Status
**What to check:** Status claims match actual progress

```bash
# Check status mentions
grep -i "status\|phase.*complete\|early development" README.md CLAUDE.md
```

**Expected:** Status reflects current state accurately

---

### 10. Build Commands
**What to check:** Command examples are correct and work

```bash
# Test build commands
cargo check --workspace
cargo build -p api
cd ui && trunk build
```

**Expected:** All documented commands succeed

---

## Quick Audit Script

Create `scripts/audit-docs.sh`:

```bash
#!/bin/bash
echo "🔍 Running documentation audit..."
echo ""

# 1. Check package versions
echo "1️⃣ Package Versions:"
grep "rocket\|yew\|reqwest" api/Cargo.toml ui/Cargo.toml | head -5
echo ""

# 2. Check edition
echo "2️⃣ Rust Edition:"
grep "edition" api/Cargo.toml ui/Cargo.toml | uniq
echo ""

# 3. Check port numbers
echo "3️⃣ Port Configuration:"
grep "port.*808" ui/Trunk.toml
echo ""

# 4. Check API endpoints
echo "4️⃣ API Endpoints:"
grep "#\[get" api/src/controller.rs
echo ""

# 5. Check data count
echo "5️⃣ Data Count:"
echo "Actual: $(grep -c '"title"' api/resources/realbook.json) entries"
echo "Documented: $(grep -o '1,[0-9]*' README.md | head -1) entries"
echo ""

# 6. Check project status
echo "6️⃣ Project Status:"
grep -i "phase.*complete\|status:" README.md CLAUDE.md | head -3
echo ""

echo "✅ Audit complete! Review output above for discrepancies."
```

---

## When to Run Audit

### Required:
- ✅ Before each milestone completion
- ✅ After major dependency updates
- ✅ Before creating milestone review issues
- ✅ After significant architecture changes

### Recommended:
- After adding new features
- When onboarding new contributors
- Before major releases
- Monthly for active projects

---

## Common Issues Found

### Issue: Outdated Status Claims
**Example:** Docs say "early development" but Phase 1 is complete
**Fix:** Update status in README.md and CLAUDE.md headers

### Issue: Wrong File Paths
**Example:** Documentation references `ui/src/main.rs` but constant is in `ui/src/api.rs`
**Fix:** Update file path references in docs

### Issue: Version Mismatches
**Example:** Code uses Edition 2024 but docs say 2021
**Fix:** Update all Cargo.toml and documentation to match

### Issue: Missing Features
**Example:** New endpoint `/api/random` exists but not documented
**Fix:** Add to API endpoint list in docs

---

## Automated Audit (Future)

**Goal:** Run audit checks in CI/CD pipeline

**Implementation ideas:**
- GitHub Actions workflow on PR
- Pre-commit hook for doc changes
- Automated issue creation for discrepancies
- Link checker for external URLs

---

## Audit History

| Date | Auditor | Issues Found | Status |
|------|---------|--------------|--------|
| 2026-01-30 | Claude | 2 (API_BASE_URL location, project status) | ✅ Fixed |

---

_Last updated: 2026-01-30_
