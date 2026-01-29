# Architecture

This document describes the technical architecture of the Real Book Search application.

---

## Overview

Real Book Search is a full-stack Rust application that provides search functionality for jazz musicians looking up songs in the Real Book collection. The application uses a client-server architecture with a Rocket backend API and a Yew WebAssembly frontend.

---

## System Architecture

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

---

## Key Architectural Decisions

### 1. Unified Language
Both frontend and backend use Rust, enabling type sharing and consistent tooling across the stack.

### 2. WebAssembly Frontend
Yew compiles to WASM for near-native performance in the browser. This provides better performance than JavaScript alternatives while maintaining type safety.

### 3. Separate Processes
Backend and frontend run as independent services during development:
- Backend (Rocket): `http://localhost:8000`
- Frontend (Trunk): `http://localhost:8080`

### 4. Trunk Proxy
The Trunk dev server at port 8080 proxies `/api` requests to the Rocket backend at port 8000. This avoids CORS issues during development.

### 5. External Image CDN
Sheet music images are hosted externally at `https://wypn9z41ir5bzmgjjalyna.on.drv.tw/realbook/rendered/`. This avoids storage costs and leverages existing infrastructure from the original JavaScript project.

**URL Pattern:** `{volume * 1000 + page}.jpeg`
- Example: Volume 1, Page 5 → `1005.jpeg`

### 6. In-Memory Data Store
Real Book entries (1,161 songs) are loaded once at startup and stored in Rocket's managed state as `Arc<Vec<RealBookEntry>>`. This provides fast search without database overhead.

---

## Data Flow

```
User Browser (Yew/WASM)
    │
    │ HTTP Request: http://localhost:8080/api/search?query=autumn
    ▼
Trunk Dev Server (Port 8080)
    │
    │ Proxy to backend
    ▼
Rocket Backend (Port 8000)
    │
    │ Route: /api/search
    ▼
Controller (api/src/controller.rs)
    │
    │ Access managed state
    ▼
In-Memory Data (Arc<Vec<RealBookEntry>>)
    │
    │ Filter & search
    ▼
JSON Response → Trunk → Browser
```

---

## Component Structure

### Backend Components

```
api/
├── src/
│   ├── main.rs           # Application entry, state management
│   ├── controller.rs     # Route handlers, API endpoints
│   └── models.rs         # Data models (RealBookEntry, etc.)
└── resources/
    ├── realbook.json     # Song data (1,161 entries)
    └── rickroll.gif      # Easter egg static file
```

### Frontend Components

```
ui/
├── src/
│   ├── main.rs           # App component, UI logic
│   ├── api.rs            # API client, HTTP requests
│   └── models.rs         # Frontend data models
├── index.html            # HTML template with embedded CSS
└── Trunk.toml            # Build config, proxy setup
```

---

## API Endpoints

All endpoints are prefixed with `/api`:

| Endpoint | Method | Parameters | Description |
|----------|--------|------------|-------------|
| `/api/` | GET | - | Health check / API root |
| `/api/search` | GET | `query`, `volume`, `page` | Search songs with optional filters |
| `/api/volumes` | GET | - | List all volumes with entry counts |
| `/api/random` | GET | - | Get random Real Book entry |
| `/api/rickroll` | GET | - | Easter egg (returns GIF) |

### Example Requests

```bash
# Search by title
GET /api/search?query=autumn

# Filter by volume
GET /api/search?query=blue&volume=1

# Filter by page
GET /api/search?page=100&volume=2

# List volumes
GET /api/volumes

# Random song
GET /api/random
```

---

## State Management

### Backend State (Rocket)
- **Data:** Loaded once at startup from `api/resources/realbook.json`
- **Storage:** `Arc<Vec<RealBookEntry>>` in Rocket managed state
- **Thread Safety:** Arc provides shared ownership across request handlers
- **Lifecycle:** Lives for entire application runtime

### Frontend State (Yew)
- **Component State:** `use_state` hooks for local UI state
- **Search Results:** Stored in App component state
- **Selected Entry:** Current song being viewed
- **Loading State:** Boolean flag for async operations
- **Error State:** Optional error messages

---

## Tech Stack Summary

| Layer | Technology | Version | Purpose |
|-------|-----------|---------|---------|
| Backend Language | Rust | 1.85.0+ (Edition 2024) | Type safety, performance |
| Backend Framework | Rocket | 0.5.1 | Web server, routing |
| Frontend Language | Rust | 1.85.0+ (Edition 2024) | Type safety, WASM |
| Frontend Framework | Yew | 0.22.0 | UI components, reactivity |
| Frontend Build | Trunk | Latest | WASM bundler, dev server |
| HTTP Client | reqwest | 0.12.25 | API calls from frontend |
| Serialization | serde | 1.0 | JSON handling |
| Data Format | JSON | - | Song metadata storage |

---

## Performance Considerations

### Backend
- **In-memory search:** O(n) linear search through 1,161 entries (fast enough for this dataset)
- **No database overhead:** Simple data structure, no query parsing
- **Thread-safe:** Arc allows concurrent access without locks

### Frontend
- **WASM compilation:** Slightly larger initial bundle (~2MB) but fast runtime
- **Client-side routing:** No page reloads, instant navigation
- **Lazy image loading:** Images loaded on-demand when user selects song

### Future Optimizations
- Fuzzy search with pre-built index
- Image optimization (WebP format, responsive sizes)
- Frontend state persistence (localStorage)
- Bundle size reduction (code splitting)

---

## Security Considerations

### Current State
- **No authentication:** Public read-only API
- **No rate limiting:** Open access to all endpoints
- **External CDN:** Relies on third-party image hosting
- **No input validation:** Search queries passed directly (safe for read-only operations)

### Future Improvements
- Rate limiting for API abuse prevention
- Input sanitization (if write operations added)
- CDN migration to owned infrastructure (see docs/AWS_MIGRATION_PLAN.md)

---

## Deployment Architecture (Planned)

### Development
- Backend: Local Rocket server (port 8000)
- Frontend: Trunk dev server with HMR (port 8080)
- Proxy: Trunk forwards `/api` requests to backend

### Production (Future)
- Backend: Containerized Rocket server behind nginx
- Frontend: Static WASM files served via CDN
- Images: Migrated to AWS S3 + CloudFront
- Domain: Custom domain with SSL

---

_See also:_
- [DEVELOPMENT.md](DEVELOPMENT.md) - Setup and build instructions
- [PATTERNS.md](PATTERNS.md) - Code patterns and conventions
- [AWS_MIGRATION_PLAN.md](AWS_MIGRATION_PLAN.md) - Image CDN migration plan
