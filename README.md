# realbook-search

A modern Real Book search service built with Rust and WebAssembly. This project provides a fast, efficient way to search through the Real Book jazz fake book collection by title, volume, and page number.

## About the Real Book

The Real Book is a collection of jazz standards that is widely used by jazz musicians. This service makes it easy to quickly find specific songs and their page numbers across different volumes.

## Project Status

**Current Phase:** Early Development (v0.1.0)

This is a complete rewrite of the [original realbook project](https://github.com/doodle0/realbook) ([realbook.kro.kr](https://realbook.kro.kr)) using modern Rust-based technologies for improved performance, maintainability, and scalability.

**What's Implemented (Phase 1 Complete ✅):**
- ✅ Rust monorepo structure (Cargo workspace with api/ and ui/)
- ✅ Rocket-based backend API
  - Search by title (case-insensitive, partial match)
  - Filter by volume (1, 2, 3) and page number
  - Random song selection endpoint
  - Volume listing endpoint
  - Data: 1,161 Real Book entries across 3 volumes
- ✅ Yew-based WebAssembly frontend
  - Search input with text query and volume filter
  - Results list (clickable song entries)
  - Sheet music image viewer (split-screen layout)
  - Random song button
  - Basic loading states and error handling
- ✅ Frontend-backend integration via Trunk proxy

**What's Coming Next (Phase 2 - UI Refactoring):**
- 🚧 **Component architecture:** Break App into reusable Yew components
- 🚧 **Pico CSS integration:** Replace inline CSS with Pico framework
- 🚧 **Mobile-responsive design:** Stack layout for mobile users
- 📋 Sorting options, i18n (Korean), advanced features
- 📋 Performance optimizations (fuzzy search, caching)
- 📋 Database integration (optional, for future scalability)

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

### Phase 1: Core Search Functionality ✅ COMPLETE
- [x] Design Real Book data structure (JSON schema)
- [x] Port data from original project or create new dataset
- [x] Implement backend search API
  - [x] Search by title (case-insensitive, partial match)
  - [x] Filter by volume
  - [x] Filter by page number
- [x] Build frontend search interface
  - [x] Search input component
  - [x] Results list component
  - [x] Volume/page display with sheet images
- [x] Add random song selection feature
- [x] Basic loading states and error handling

### Phase 2: UI Refactoring & Polish (Current)
- [ ] **Component Architecture:** Break monolithic App into smaller Yew components
  - [ ] SearchInput component (search bar, filters, random button)
  - [ ] ResultsList component (song results display)
  - [ ] SheetViewer component (image viewer)
  - [ ] Header component (app title, navigation)
- [ ] **Styling Migration:** Replace inline CSS with Pico CSS framework
  - [ ] Minimal custom CSS approach
  - [ ] Semantic HTML with Pico's built-in styles
- [ ] **Responsive Design:** Mobile-first approach (most users are mobile)
  - [ ] Mobile: Stack layout (search → results → viewer)
  - [ ] Desktop: Keep current split-screen layout
  - [ ] Tablet: Adaptive intermediate layout

### Phase 3: Enhanced Features
- [ ] Implement sorting options (alphabetical, by volume, by page)
- [ ] Add Korean language support (multilingual UI)
- [ ] Improved loading states with skeletons
- [ ] Better error messages and empty states

### Phase 3: Performance & Polish
- [ ] Optimize search algorithm (fuzzy matching, relevance scoring)
- [ ] Add search result caching
- [ ] Implement frontend state persistence (localStorage)
- [ ] Add keyboard shortcuts for power users
- [ ] Performance benchmarking vs. original JS version

### Phase 4: Advanced Features
- [ ] Fuzzy search with relevance scoring
- [ ] User favorites and bookmarks
- [ ] Recent searches history
- [ ] Keyboard shortcuts for power users
- [ ] Search result caching
- [ ] Frontend state persistence (localStorage)

### Phase 5: Deployment & Scaling (Future)
- [ ] Docker containerization
- [ ] CI/CD pipeline
- [ ] Production deployment setup
- [ ] Database migration (optional, if dataset grows)
- [ ] API rate limiting
- [ ] Analytics integration

## Getting Started

### Prerequisites
- Rust 1.85.0+ (stable toolchain, Edition 2024)
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