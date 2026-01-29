# Development Guide

This document provides instructions for setting up, building, and running the Real Book Search application.

---

## Prerequisites

### Required
- **Rust:** 1.85.0+ (stable toolchain, Edition 2024)
- **Trunk:** WebAssembly bundler for frontend builds
- **wasm-bindgen-cli:** WASM bindings generation

### Installation

```bash
# Update Rust to latest stable
rustup update stable
rustc --version  # Should show 1.85.0 or higher

# Install Trunk for frontend builds
cargo install trunk

# Install wasm-bindgen-cli for WASM
cargo install wasm-bindgen-cli
```

### Important: Use rustup, not system rust
This project requires Rust 1.85.0+ for Edition 2024 support. System package managers typically lag behind. Use rustup for the latest stable Rust:

```bash
# If you have system rust installed, remove it
sudo apt remove rustc cargo rust-*  # Debian/Ubuntu

# Ensure rustup is used
which rustc  # Should show ~/.cargo/bin/rustc
```

---

## Running the Application

The application requires two processes running simultaneously:

### Terminal 1 - Backend API

```bash
cargo run -p api
# Backend runs on http://localhost:8000
```

**What it does:**
- Loads realbook.json (1,161 entries) into memory
- Starts Rocket web server
- Mounts API routes under `/api` prefix
- Serves static files from `api/resources/`

### Terminal 2 - Frontend Dev Server

```bash
cd ui
trunk serve
# Frontend runs on http://localhost:8080
# Visit http://localhost:8080 in your browser
```

**What it does:**
- Compiles Rust to WebAssembly
- Bundles WASM with HTML/CSS
- Starts dev server with hot-reload
- Proxies `/api` requests to backend at port 8000

---

## Building

### Build Entire Workspace

```bash
cargo build
# Builds both api and ui packages
```

### Build Backend Only

```bash
cargo build -p api
# Output: target/debug/api (or target/release/api with --release)
```

### Build Frontend for Production

```bash
cd ui
trunk build --release
# Output: ui/dist/ (ready for static hosting)
```

**Production build includes:**
- Optimized WASM bundle
- Minified JavaScript
- index.html with embedded assets
- Ready for CDN deployment

---

## Checking Code

### Check All Workspace Members

```bash
cargo check
# Fast compilation check without producing binaries
```

### Check Specific Package

```bash
cargo check -p api
cargo check -p ui
```

**Use case:** Quick validation during development without full build.

---

## Testing

Currently, no automated tests are implemented (Phase 1 focused on core functionality).

**Planned (Phase 3):**
```bash
# Backend tests
cargo test -p api

# Frontend tests
cd ui && wasm-pack test --headless --firefox

# Integration tests
cargo test --workspace
```

---

## Common Development Tasks

### Adding a New API Endpoint

1. Add route handler in `api/src/controller.rs`:

```rust
#[get("/your-route")]
pub fn your_handler() -> &'static str {
    "response"
}
```

2. Register route in `api/src/main.rs`:

```rust
rocket::build()
    .manage(realbook_data)
    .mount("/api", routes![
        index, rickroll, search, volumes, random, your_handler  // <-- add here
    ])
```

3. Test the endpoint:

```bash
curl http://localhost:8000/api/your-route
```

### Adding a New Frontend Component

See [PATTERNS.md](PATTERNS.md#yew-components) for detailed component patterns.

Quick example:

```rust
#[component]
fn MyComponent() -> Html {
    html! {
        <div>{ "Hello World" }</div>
    }
}
```

### Updating Dependencies

```bash
# Check for outdated dependencies
cargo outdated

# Update specific dependency
cargo update -p rocket

# Update all dependencies
cargo update
```

---

## Troubleshooting

### Issue: "rustc 1.75.0 is not supported"

**Problem:** Using old system rust instead of rustup.

**Solution:**
```bash
# Remove system rust
sudo apt remove rustc cargo

# Ensure rustup is in PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Verify version
rustc --version  # Should show 1.85.0+
```

### Issue: "linker `cc` not found"

**Problem:** C compiler not in PATH.

**Solution:**
```bash
# Ensure /usr/bin is in PATH for linker
export PATH="/usr/bin:$PATH"

# Or install build-essential (if missing)
sudo apt install build-essential
```

### Issue: Backend can't find realbook.json

**Problem:** Running from wrong directory.

**Solution:**
```bash
# Always run from workspace root
cd /path/to/realbook-search
cargo run -p api  # Not: cd api && cargo run
```

### Issue: Frontend shows CORS errors

**Problem:** Trunk proxy not configured or backend not running.

**Solution:**
1. Check `ui/Trunk.toml` has proxy configuration:
   ```toml
   [[proxy]]
   backend = "http://localhost:8000/api"
   ```
2. Ensure backend is running on port 8000
3. Restart trunk serve

### Issue: Port already in use

**Problem:** Previous process still running.

**Solution:**
```bash
# Find process using port
lsof -i :8000  # or :8080

# Kill process
kill -9 <PID>
```

---

## Development Workflow

### Standard Workflow

1. **Start backend:**
   ```bash
   cargo run -p api
   ```

2. **Start frontend** (in new terminal):
   ```bash
   cd ui && trunk serve
   ```

3. **Make changes** to code

4. **Hot reload** happens automatically for frontend

5. **Restart backend** manually if changed (Ctrl+C, then `cargo run -p api`)

### Quick Iteration Cycle

For rapid development:

```bash
# Check code without building
cargo check -p api

# Run frontend tests (when implemented)
cd ui && trunk test

# Lint with clippy
cargo clippy --workspace
```

---

## Project Structure

```
realbook-search/
├── api/                    # Backend Rocket API
│   ├── src/
│   │   ├── main.rs        # Application entry point
│   │   ├── controller.rs  # Route handlers
│   │   └── models.rs      # Data models
│   ├── resources/         # Static assets
│   │   ├── realbook.json  # Song data (1,161 entries)
│   │   └── rickroll.gif   # Easter egg
│   └── Cargo.toml
│
├── ui/                     # Frontend Yew WebAssembly
│   ├── src/
│   │   ├── main.rs        # App component
│   │   ├── api.rs         # HTTP client
│   │   └── models.rs      # Frontend models
│   ├── index.html         # HTML template
│   ├── Trunk.toml         # Trunk build config
│   └── Cargo.toml
│
├── docs/                   # Documentation
│   ├── ARCHITECTURE.md    # System design
│   ├── DEVELOPMENT.md     # This file
│   ├── PATTERNS.md        # Code patterns
│   ├── WORKFLOWS.md       # Git & release workflows
│   ├── AUDIT.md           # Documentation audit
│   └── AWS_MIGRATION_PLAN.md
│
├── scripts/                # Automation scripts
│   ├── milestone-review.sh
│   └── audit-docs.sh
│
├── reviews/                # Milestone reviews
│   └── phase-1-completion.md
│
├── Cargo.toml             # Workspace configuration
├── CLAUDE.md              # Claude Code guidance (navigation hub)
└── README.md              # Project overview
```

---

## IDE Setup

### VS Code

**Recommended Extensions:**
- rust-analyzer (official Rust language server)
- CodeLLDB (debugging)
- Even Better TOML
- crates (dependency management)

**Settings:**
rust-analyzer should automatically detect the workspace and rustup toolchain. If issues occur, check that rustup's cargo/rustc are found first in PATH.

### Custom Snippets

See `.vscode/snippets.code-snippets` for:
- `yewfc` - Generate Yew function component
- `yewsc` - Generate Yew struct component

---

## Performance Tips

### Faster Builds

```bash
# Use cargo-watch for auto-rebuild
cargo install cargo-watch
cargo watch -x 'check -p api'

# Use sccache for caching
cargo install sccache
export RUSTC_WRAPPER=sccache

# Parallel compilation (in Cargo.toml or config.toml)
[build]
jobs = 8  # Adjust to your CPU cores
```

### Faster Frontend Iteration

```bash
# Trunk watch mode (automatic)
trunk serve  # Already has hot-reload

# Check without building WASM
cargo check -p ui
```

---

## Environment Variables

### Optional Configuration

```bash
# Custom backend port (default: 8000)
ROCKET_PORT=9000 cargo run -p api

# Custom frontend port (default: 8080)
# Edit ui/Trunk.toml: port = 8081

# Rust compilation options
RUSTFLAGS="-C target-cpu=native"  # Optimize for your CPU
```

---

## Next Steps

- See [PATTERNS.md](PATTERNS.md) for code conventions
- See [WORKFLOWS.md](WORKFLOWS.md) for git and milestone workflows
- See [ARCHITECTURE.md](ARCHITECTURE.md) for system design details

---

_Last updated: 2026-01-30_
