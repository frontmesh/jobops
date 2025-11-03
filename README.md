# Rust Board

🚀 **A modern job board application built with Rust and WebAssembly**

Rust Board is a professional job board platform built using Leptos (Rust web framework) and deployed on Cloudflare Pages. It demonstrates modern web development practices with server-side rendering, responsive design, and edge computing.

[🌐 Live Demo](https://31eb9d73.leptos-test.pages.dev/)

## ✨ Features

- **🎨 Modern UI/UX**: Clean, professional design with Tailwind CSS and Poppins font
- **📱 Mobile Responsive**: Fully responsive with mobile hamburger menu
- **⚡ Fast Performance**: Server-side rendering with client-side hydration
- **🌍 Edge Deployment**: Deployed on Cloudflare Pages for global performance
- **🦀 Rust-Powered**: Built with Leptos for type-safe, fast web applications
- **📝 Blog System**: Integrated blog with markdown support
- **🔍 SEO Friendly**: Server-side rendering for better search engine optimization

## 🛠️ Tech Stack

- **Frontend**: [Leptos](https://leptos.dev/) (Rust web framework)
- **Styling**: [Tailwind CSS](https://tailwindcss.com/) with custom Rust Board theme
- **Deployment**: [Cloudflare Pages](https://pages.cloudflare.com/)
- **Build Tools**: Rust + WASM + wasm-bindgen
- **Development**: Wrangler CLI for local testing


## 🚀 Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) (for Tailwind CSS)
- [wasm-bindgen-cli](https://github.com/rustwasm/wasm-bindgen)
- [Wrangler CLI](https://developers.cloudflare.com/workers/wrangler/) (for deployment)
- [Just](https://github.com/casey/just) (task runner, recommended)
- [entr](http://entrproject.org/) (optional, for file watching)

### Local Development

1. **Clone the repository**
   ```bash
   git clone git@github.com:frontmesh/rustjobs.git
   cd rustjobs   ```

2. **Install dependencies**
   ```bash
   # Install Node.js dependencies for Tailwind CSS
   npm install
   
   # Install wasm-bindgen-cli if you haven't already
   cargo install wasm-bindgen-cli
   ```

3. **Build the application**
   ```bash
   # Build CSS
   npm run build:css
   
   # Build Rust WASM bundles
   ./build.sh
   ```

4. **Run locally**
   ```bash
   wrangler pages dev site --port 8080
   ```

5. **Open in browser**
   Navigate to `http://localhost:8080`

### Using Just (Recommended)

If you have [Just](https://github.com/casey/just) installed, you can use the convenient commands:

```bash
# Start development server (builds and serves)
just dev

# Watch CSS files for changes (in separate terminal)
just watch-css

# Build everything
just build

# Deploy to Cloudflare Pages
just deploy

# See all available commands
just --list
```

## 📁 Project Structure

```
rustjobs/
├── src/
│   ├── app.rs              # Main application component
│   ├── components/         # Reusable UI components
│   │   ├── header.rs       # Mobile-responsive header
│   │   ├── footer.rs       # Site footer
│   │   └── post/           # Blog post components
│   ├── pages/              # Application pages
│   │   ├── home.rs         # Landing page
│   │   ├── blog.rs         # Blog listing
│   │   └── post.rs         # Individual blog posts
│   ├── api/                # Server functions
│   └── models/             # Data models
├── style/                  # Tailwind CSS source
├── site/                   # Built output (auto-generated)
└── wrangler.toml          # Cloudflare deployment config
```

## 🚀 Deployment

### Deploy to Cloudflare Pages

1. **Login to Cloudflare**
   ```bash
   wrangler auth login
   ```

2. **Deploy to Cloudflare Pages**
   ```bash
   wrangler pages deploy site --project-name your-project-name
   ```

The application will be available at your Cloudflare Pages URL.

## 📝 Development Notes

### Architecture

- **Frontend**: Leptos components with server-side rendering
- **Styling**: Tailwind CSS with custom Rust Board design system
- **State Management**: Leptos reactive signals and server functions
- **Routing**: Leptos Router with client-side navigation
- **WASM Compilation**: Rust compiled to WebAssembly for edge performance

### Key Features Implemented

- **Mobile-First Design**: Responsive layout with hamburger menu
- **Server Functions**: WASM-compatible API endpoints
- **Font Loading**: Custom Poppins font integration
- **SEO Optimization**: Server-side rendering with meta tags
- **Performance**: Optimized for Cloudflare's edge network

### Build Process

The build process involves:
1. Compiling Rust to WASM for both client and server
2. Generating JavaScript bindings with `wasm-bindgen`
3. Processing Tailwind CSS
4. Packaging for Cloudflare Pages deployment

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is open source and available under the [MIT License](LICENSE).

---

**Built with ❤️ using Rust 🦀 and deployed on Cloudflare Pages ⚡**
