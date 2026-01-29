# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Real Book Search is a full-stack Rust application that helps jazz musicians find songs by title, volume, and page number. It's a complete rewrite of the original JavaScript-based [realbook project](https://github.com/doodle0/realbook) using Rust and WebAssembly.

**Current Status:** Phase 1 Complete (v0.2.0) - Core search functionality working with 1,161 Real Book entries

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
- Main app in `ui/src/main.rs`
- API client in `ui/src/api.rs`
- Models in `ui/src/models.rs`

### Development Workflow
See [WORKFLOWS.md](docs/WORKFLOWS.md) for full process, but key points:
- Run documentation audit before milestone completions: `./scripts/audit-docs.sh`
- Create milestone reviews: `./scripts/milestone-review.sh "Phase N" reviews/file.md`
- Follow existing commit message style (verb-first, concise)

## Current Phase

**Phase 1:** ✅ Complete - Core search functionality
**Phase 2:** 🚧 Next - Mobile responsive design, Pico CSS, loading states

See [Phase 1 Review](reviews/phase-1-completion.md) for detailed assessment and next steps.

---

_For detailed information on any topic, see the documentation index above._
