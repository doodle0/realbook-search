# realbook-search

A modern Real Book search service built with Rust and WebAssembly. This project provides a fast, efficient way to search through the Real Book jazz fake book collection by title, volume, and page number.

## About the Real Book

The Real Book is a collection of jazz standards that is widely used by jazz musicians. This service makes it easy to quickly find specific songs and their page numbers across different volumes.

## Project Status

**Current Phase:** Early Development (v0.1.0)

This is a complete rewrite of the [original realbook project](https://github.com/doodle0/realbook) ([realbook.kro.kr](https://realbook.kro.kr)) using modern Rust-based technologies for improved performance, maintainability, and scalability.

**What's Implemented:**
- ✅ Basic Rust monorepo structure (Cargo workspace)
- ✅ Rocket-based backend API with static file serving
- ✅ Yew-based WebAssembly frontend with hot-reload development
- ✅ Frontend-backend integration (HTTP client setup)

**What's Coming Next:**
- 🚧 Real Book data integration (JSON data structure)
- 🚧 Search API endpoints (title, volume, page search)
- 🚧 Search UI components
- 🚧 Filtering and sorting capabilities
- 🚧 Responsive design
- 📋 Database integration (optional, for future scalability)
- 📋 Advanced features (random song selection, favorites, etc.)

## Tech Stack

### Backend (api/)
- **Language:** Rust (Edition 2024)
- **Framework:** Rocket 0.5.1
- **Purpose:** RESTful API for search operations and data serving

### Frontend (ui/)
- **Language:** Rust (Edition 2024)
- **Framework:** Yew 0.22.0 (WebAssembly)
- **HTTP Client:** reqwest 0.12.25
- **Build Tool:** Trunk
- **Purpose:** Client-side rendered single-page application

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                   Browser (WASM)                    │
│  ┌───────────────────────────────────────────────┐ │
│  │         Yew Frontend (ui/)                    │ │
│  │  - Search Components                          │ │
│  │  - Result Display                             │ │
│  │  - Client-side State Management               │ │
│  └───────────────────────────────────────────────┘ │
└──────────────────────┬──────────────────────────────┘
                       │ HTTP/REST
                       │
┌──────────────────────▼──────────────────────────────┐
│              Rocket Backend (api/)                  │
│  ┌───────────────────────────────────────────────┐ │
│  │  REST API Endpoints                           │ │
│  │  - /api/search - Full-text search             │ │
│  │  - /api/volumes - List volumes                │ │
│  │  - /api/random - Random selection             │ │
│  └───────────────────────────────────────────────┘ │
│  ┌───────────────────────────────────────────────┐ │
│  │  Data Layer                                   │ │
│  │  - Real Book JSON data                        │ │
│  │  - Search indexing                            │ │
│  └───────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────┘
```

## Development Roadmap

### Phase 1: Core Search Functionality (Current)
- [ ] Design Real Book data structure (JSON schema)
- [ ] Port data from original project or create new dataset
- [ ] Implement backend search API
  - [ ] Search by title (case-insensitive, partial match)
  - [ ] Filter by volume
  - [ ] Filter by page number
- [ ] Build frontend search interface
  - [ ] Search input component
  - [ ] Results list component
  - [ ] Volume/page display

### Phase 2: Enhanced Features
- [ ] Add random song selection feature
- [ ] Implement sorting options (alphabetical, by volume, by page)
- [ ] Add Korean language support (multilingual UI)
- [ ] Responsive mobile design
- [ ] Loading states and error handling

### Phase 3: Performance & Polish
- [ ] Optimize search algorithm (fuzzy matching, relevance scoring)
- [ ] Add search result caching
- [ ] Implement frontend state persistence (localStorage)
- [ ] Add keyboard shortcuts for power users
- [ ] Performance benchmarking vs. original JS version

### Phase 4: Deployment & Scaling (Future)
- [ ] Docker containerization
- [ ] CI/CD pipeline
- [ ] Production deployment setup
- [ ] Database migration (optional, if dataset grows)
- [ ] API rate limiting
- [ ] Analytics integration

## Getting Started

### Prerequisites
- Rust (nightly toolchain, Edition 2024)
- Trunk (for frontend builds): `cargo install trunk`
- wasm-bindgen-cli (for WASM): `cargo install wasm-bindgen-cli`

### Development Setup

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd realbook-search
   ```

2. **Start the backend API**
   ```bash
   cd api
   cargo run
   # Backend runs on http://localhost:8000
   ```

3. **Start the frontend development server** (in a new terminal)
   ```bash
   cd ui
   trunk serve
   # Frontend runs on http://localhost:8080
   ```

4. **Open your browser**
   Navigate to `http://localhost:8080`

### Project Structure

```
realbook-search/
├── api/                    # Backend Rocket API
│   ├── src/
│   │   ├── main.rs        # Application entry point
│   │   └── controller.rs  # Route handlers
│   └── resources/         # Static assets
├── ui/                     # Frontend Yew WebAssembly
│   ├── src/
│   │   └── main.rs        # Frontend components
│   ├── index.html         # HTML template
│   └── Trunk.toml         # Trunk build config
└── Cargo.toml             # Workspace configuration
```

## Design Principles

### Why Rust + WebAssembly?
- **Performance**: Near-native speed for search operations
- **Type Safety**: Catch errors at compile time
- **Modern Tooling**: Excellent development experience
- **Unified Codebase**: Share types between frontend and backend
- **Future-Proof**: Easy to scale and add features

### Comparison with Original Project
| Feature | Original (JS) | This Project (Rust) |
|---------|---------------|---------------------|
| Tech Stack | Vanilla JS | Rust + WebAssembly |
| Deployment | GitHub Pages | Flexible (Docker, VPS, etc.) |
| Search Speed | Client-side JS | Optimized Rust algorithms |
| Scalability | Limited to static data | Database-ready architecture |
| Type Safety | None | Full type safety |
| Bundle Size | Small | Larger initial (WASM overhead) |

## Contributing

This project is in early development. Contributions are welcome once the core functionality is complete.

## License

MIT License - Copyright 2025 doodle0

## Acknowledgments

- Original [realbook](https://github.com/doodle0/realbook) project by doodle0
- Real Book community and jazz musicians worldwide

## Contact

For bug reports or feature requests, please open an issue on GitHub.

---

**Note:** This is an educational project. The Real Book content itself is copyrighted material. This service is intended to help musicians quickly locate songs they already own in physical or digital format.