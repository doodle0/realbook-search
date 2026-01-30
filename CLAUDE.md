# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Real Book Search is a full-stack Rust application that helps jazz musicians find songs by title, volume, and page number. It's a complete rewrite of the original JavaScript-based [realbook project](https://github.com/doodle0/realbook) using Rust and WebAssembly.

**Current Status:** Phase 2 Complete - Mobile responsive, Pico CSS 2.0, per-image loading indicators

## Quick Start

```bash
# Terminal 1 - Backend
cargo run -p api

# Terminal 2 - Frontend
cd ui && trunk serve
# Visit http://localhost:8080
```

## Documentation Index

### For Implementation Work
- **[ARCHITECTURE.md](docs/ARCHITECTURE.md)** - System design, data flow, component structure, tech stack
- **[PATTERNS.md](docs/PATTERNS.md)** - Code conventions for backend routes, frontend components, API calls
- **[DEVELOPMENT.md](docs/DEVELOPMENT.md)** - Setup, building, running, troubleshooting, project structure

### For Process & Planning
- **[WORKFLOWS.md](docs/WORKFLOWS.md)** - Git workflow, milestone reviews, release process, CI/CD
- **[AUDIT.md](docs/AUDIT.md)** - Documentation audit checklist and process
- **[AWS_MIGRATION_PLAN.md](docs/AWS_MIGRATION_PLAN.md)** - Image hosting migration from Google Drive to AWS

## Tech Stack

- **Backend:** Rocket 0.5.1 (Rust web framework)
- **Frontend:** Yew 0.22.0 (WebAssembly framework)
- **Build Tool:** Trunk (WebAssembly bundler)
- **Data:** 1,161 Real Book entries in `api/resources/realbook.json`
- **Images:** External CDN (Google Drive, migration to AWS planned)
- **Rust Edition:** 2024 (requires Rust 1.85.0+)

## Project Structure

```
realbook-search/
├── api/           # Backend Rocket API (port 8000)
├── ui/            # Frontend Yew WebAssembly (port 8080)
├── docs/          # Documentation (see index above)
├── reviews/       # Milestone reviews
├── scripts/       # Automation scripts
├── Cargo.toml     # Workspace configuration
└── CLAUDE.md      # This file (navigation hub)
```

## Key Implementation Notes

### Backend API Endpoints
- `GET /api/search?query=<text>&volume=<num>&page=<num>` - Search with filters
- `GET /api/volumes` - List all volumes
- `GET /api/random` - Get random entry

### Frontend Components
- **Main app**: `ui/src/main.rs` - Root component with state management
- **Components**: `ui/src/components/` - Header, SearchInput, ResultsList, SheetViewer, SheetImage
- **API client**: `ui/src/api.rs` - HTTP requests with reqwest
- **Models**: `ui/src/models.rs` - Data structures
- **Utils**: `ui/src/utils.rs` - Navigation helpers

### Development Workflow & Best Practices

**Refactoring Criteria (do regularly after changes!):**

**User-Defined Criteria:**
1. **Misleading comments** - Comments that don't match implementation
2. **Documentation conflicts** - Code that contradicts docs
3. **Unnecessary code** - Unused functions, dead CSS, redundant HTML
4. **Abstraction opportunities** - Repeated code that should be extracted

**Code Quality Criteria:**
5. **Readability** - Can someone understand this in 30 seconds?
   - Nested ifs → combined patterns
   - Magic numbers → named constants
   - Long functions → smaller functions
   - Unclear names → descriptive names

6. **Maintainability** - Can someone change this safely?
   - DRY violations → extract common code
   - God objects → split responsibilities
   - Tight coupling → dependency injection
   - Missing tests → add tests

7. **Consistency** - Does this match project patterns?
   - Naming conventions (snake_case, camelCase)
   - Error handling patterns
   - Component structure
   - Code organization

8. **Performance** - Are there obvious inefficiencies?
   - Unnecessary clones
   - N+1 queries
   - Blocking operations
   - Memory leaks

9. **Documentation** - Is the code self-documenting?
   - Public APIs have doc comments
   - Complex logic has explanation comments
   - READMEs are current
   - Examples work

**When to Refactor:**
- ✅ After implementing new features
- ✅ Before committing changes
- ✅ During code review
- ✅ When touching old code
- ✅ At milestone completion
- ❌ NOT when under time pressure
- ❌ NOT without tests passing

**Tools:**
```bash
cargo clippy --target wasm32-unknown-unknown  # Style + safety
cargo fmt --check  # Formatting
cargo test  # Ensure nothing breaks
```

**Testing Before Commits:**
```bash
# Backend
cargo test -p api

# Frontend (check compilation)
cd ui && cargo check --target wasm32-unknown-unknown

# API integration test
curl http://localhost:8080/api/search?query=test
curl http://localhost:8080/api/random
```

**Priority Framework for Issues:**
- 🔴 **Critical** (fix immediately): Safety, security, crashes, data loss
- 🟡 **Important** (fix before merge): Readability, maintainability, obvious bugs
- 🟢 **Nice-to-have** (fix when convenient): Style preferences, micro-optimizations

**Milestone Completion Checklist:**
1. ✅ **Test** - Verify all functionality works (API calls, UI interactions)
2. ✅ **Refactor** - Apply refactoring criteria, fix important issues
3. 📝 **Update docs** - Update CLAUDE.md, ARCHITECTURE.md, component docs
4. 🔍 **Audit docs** - Run `./scripts/audit-docs.sh`
5. 📊 **Create review** - Run `./scripts/milestone-review.sh "Phase N" reviews/phase-N.md`
6. 💾 **Commit** - Use conventional commit style (verb-first, concise)
7. 🏷️ **Tag** - Create version tag if releasing

## Current Phase

**Phase 1:** ✅ Complete - Core search functionality (1,161 entries, RESTful API, basic UI)
**Phase 2:** ✅ Complete - Mobile responsive, Pico CSS 2.0, per-image loading, keyboard nav
**Phase 3:** 🎯 Next - Error handling, empty states, offline support

See milestone reviews in [reviews/](reviews/) for detailed assessments.

---

_For detailed information on any topic, see the documentation index above._
