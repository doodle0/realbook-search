# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Real Book search service that helps jazz musicians find songs by title, volume, and page number. It's a complete rewrite of the original JavaScript-based [realbook project](https://github.com/doodle0/realbook) using Rust and WebAssembly for improved performance and type safety.

**Current Status:** Early development (v0.1.0) - basic infrastructure is in place but core search functionality is not yet implemented.

## Architecture

This is a Cargo workspace monorepo with two separate applications:

### Backend (api/)
- **Framework:** Rocket 0.5.1 web framework
- **Entry Point:** `api/src/main.rs` - mounts all routes under `/api` prefix
- **Routes:** Defined in `api/src/controller.rs` using Rocket's attribute macros (`#[get("/")]`, etc.)
- **Static Assets:** Served from `api/resources/` directory
- **Default Port:** 8000 (Rocket default)

### Frontend (ui/)
- **Framework:** Yew 0.22.0 (React-like framework for Rust/WebAssembly)
- **Rendering:** Client-side rendering (CSR) only
- **HTTP Client:** reqwest 0.12.25 for API communication
- **Build Tool:** Trunk (WebAssembly bundler with hot-reload)
- **API Integration:** Configured to connect to backend at `http://localhost:8080/api` (via Trunk proxy)
- **Default Port:** 8080 (Trunk dev server)

### Key Architectural Decisions

1. **Unified Language:** Both frontend and backend use Rust, enabling type sharing and consistent tooling
2. **WebAssembly Frontend:** Yew compiles to WASM for near-native performance in the browser
3. **Separate Processes:** Backend and frontend run as independent services during development
4. **Trunk Proxy:** The Trunk dev server at port 8080 proxies `/api` requests to the Rocket backend at port 8000

## Development Commands

### Prerequisites
```bash
# Install Trunk for WebAssembly builds
cargo install trunk

# Install wasm-bindgen-cli
cargo install wasm-bindgen-cli
```

### Running the Application

**Start both services in separate terminals:**

Terminal 1 - Backend:
```bash
cd api
cargo run
# Runs on http://localhost:8000
```

Terminal 2 - Frontend:
```bash
cd ui
trunk serve
# Runs on http://localhost:8080
# Visit this URL in your browser
```

### Building

```bash
# Build entire workspace
cargo build

# Build backend only
cargo build -p api

# Build frontend only (for production)
cd ui
trunk build --release
# Output: ui/dist/
```

### Checking Code

```bash
# Check all workspace members
cargo check

# Check specific package
cargo check -p api
cargo check -p ui
```

### Rust Edition

This project uses **Rust Edition 2024**, which requires Rust 1.85.0 or later (released February 2025):
```bash
rustup update stable
```

## Code Patterns

### Adding Backend Routes

1. Add route handler function in `api/src/controller.rs`:
```rust
#[get("/your-route")]
pub fn your_handler() -> &'static str {
    "response"
}
```

2. Register route in `api/src/main.rs`:
```rust
rocket::build().mount("/api", routes![index, rickroll, your_handler])
```

### Creating Yew Components

**Function Components (Recommended):**

Use the `yewfc` VSCode snippet or manually:
```rust
#[derive(PartialEq, Properties)]
pub struct MyComponentProps {
    // props here
}

#[component]
pub fn MyComponent(props: &MyComponentProps) -> Html {
    html! {
        <div>{"content"}</div>
    }
}
```

**Struct Components (For Complex State):**

Use the `yewsc` VSCode snippet or manually:
```rust
pub struct MyComponent;

pub enum MyComponentMsg {
    // messages here
}

impl Component for MyComponent {
    type Message = MyComponentMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! { /* ... */ }
    }
}
```

### Frontend State Management

- Use `use_state` hook for simple component state
- Use `use_reducer` for complex state logic
- API calls should use `reqwest` via `use_effect` or event handlers

### API Communication

Frontend makes requests to `API_BASE_URL` constant defined in `ui/src/main.rs`. During development, this points to `http://localhost:8080/api` which Trunk proxies to the backend.

## Important Implementation Notes

### Static File Paths
When serving static files from the backend, paths must be relative to workspace root:
```rust
// Correct - relative to workspace root
NamedFile::open(Path::new("api/resources/file.gif")).await
```

### Frontend Build Artifacts
The `ui/dist/` directory contains compiled WASM and is gitignored. Never commit these files.

### Workspace Structure
When adding dependencies, be mindful of whether they belong in:
- `api/Cargo.toml` - Backend-only dependencies
- `ui/Cargo.toml` - Frontend-only dependencies
- Root `Cargo.toml` - Shared workspace configuration (rare)

## Current Implementation Status

### ✅ Completed (Backend - Phase 1)

**Data Layer:**
- Downloaded and integrated `realbook.json` from original project (1,161 entries)
- Created `RealBookEntry` model with fields: title, volume, page_s, page_e
- Implemented custom deserializer for title field (handles both strings and numbers)
- Data stored in `api/resources/realbook.json`

**API Endpoints (all in `api/src/controller.rs`):**
- `GET /api/search?query=<text>&volume=<num>&page=<num>` - Search with optional filters
- `GET /api/volumes` - List all volumes with entry counts
- `GET /api/random` - Get random Real Book entry
- All endpoints return JSON using Rocket's `Json<T>` wrapper

**Architecture Notes:**
- Data loaded once at startup in `load_realbook_data()` function
- Stored in Rocket state as `Arc<Vec<RealBookEntry>>` for thread-safe sharing
- Search uses simple case-insensitive string matching (`.to_lowercase().contains()`)
- Image URLs follow pattern: `https://wypn9z41ir5bzmgjjalyna.on.drv.tw/realbook/rendered/{volume*1000+page}.jpeg`

**Build Environment:**
- Rust 1.81+ required (Edition 2021, not 2024 due to older cargo versions)
- Must set RUSTC and PATH environment variables when running cargo:
  ```bash
  RUSTC="/root/.cargo/bin/rustc" PATH="/usr/bin:/bin:/usr/local/bin:/root/.cargo/bin" cargo run
  ```
- Linker requires `/usr/bin` in PATH to find `cc`

### ✅ Completed (Frontend - Phase 1)

**UI Components (all in `ui/src/main.rs`):**
- Search input with text query and volume filter
- Results list displaying song titles, volume, and page numbers
- Image viewer showing Real Book sheet images from external CDN
- Random song button for discovery
- Loading states and error handling

**API Integration (`ui/src/api.rs`):**
- `search(query, volume, page)` - Full-text search with filters
- `get_random()` - Random song selection
- `get_volumes()` - Volume listing (implemented but not yet used in UI)
- Error handling with custom `ApiError` type

**Current Layout:**
- Split-screen design: search results on left, sheet viewer on right
- Works well on large screens
- Basic inline CSS styling (will be replaced with Pico CSS)

**Build & Run:**
```bash
# Terminal 1 - Backend
cargo run -p api
# Runs on http://localhost:8000

# Terminal 2 - Frontend
cd ui && trunk serve
# Runs on http://localhost:8080 (proxies /api to backend)
```

### 🚧 Next Steps (Frontend - Phase 2: Polish & Refactoring)

**Architecture Improvements:**
1. **Component Refactoring:** Break monolithic `App` component into smaller, reusable components
   - `SearchInput` component (search bar, volume selector, random button)
   - `ResultsList` component (search results display)
   - `SheetViewer` component (image display)
   - `Header` component (app title, navigation)
   - Proper props and callbacks between components

2. **Styling Migration:** Replace inline CSS with Pico CSS framework
   - Minimal custom CSS, leverage Pico's semantic HTML styling
   - Maintain current split-screen layout for desktop
   - Add responsive mobile-first design (most users are mobile)

3. **Responsive Design:**
   - Mobile: Stack layout (search → results → viewer vertically)
   - Tablet/Desktop: Keep current side-by-side layout
   - Use CSS Grid or Flexbox with media queries

**Frontend Implementation Note:**
- Yew confidence level: 7/10 (solid fundamentals, may need iteration on version-specific APIs)
- Expect ~80% correctness on first pass, with compiler-guided refinements
- Strong on: component structure, state management, html! macro, event handling
- May require iteration on: Yew 0.22-specific APIs, complex async patterns, WASM-specific quirks
- Approach: Start simple, build incrementally, let compiler guide corrections

## VSCode Snippets

Two custom snippets are available in `.vscode/snippets.code-snippets`:
- `yewfc` - Generate a Yew function component
- `yewsc` - Generate a Yew struct component with message enum