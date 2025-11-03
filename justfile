# Rust Board Development Commands
# Run with `just <command>`

# Default recipe - show available commands
default:
    @just --list

# Development workflow commands
# ==========================

# Start development server with hot-reload
dev:
    #!/usr/bin/env bash
    echo "🚀 Starting Rust Board development environment..."
    echo "📦 Building initial assets..."
    just build-css
    just build-wasm
    echo "🌐 Starting development server on http://localhost:8080"
    wrangler pages dev site --port 8080

# Watch and rebuild CSS on changes
watch-css:
    #!/usr/bin/env bash
    echo "👀 Watching CSS files for changes..."
    npm run watch:css

# Watch and rebuild everything (CSS + WASM) - requires multiple terminals
watch:
    #!/usr/bin/env bash
    echo "⚠️  This requires multiple terminal windows:"
    echo "Terminal 1: just watch-css"
    echo "Terminal 2: just watch-wasm" 
    echo "Terminal 3: just serve"
    echo ""
    echo "Or use 'just dev' for a simpler development setup"

# Watch and rebuild WASM on file changes (requires entr)
watch-wasm:
    #!/usr/bin/env bash
    if ! command -v entr &> /dev/null; then
        echo "❌ 'entr' is required for watching WASM files"
        echo "Install with: brew install entr (macOS) or your package manager"
        exit 1
    fi
    echo "👀 Watching Rust files for changes..."
    find src -name "*.rs" | entr -r just build-wasm

# Serve the built application locally
serve PORT="8080":
    echo "🌐 Starting local server on http://localhost:{{PORT}}"
    wrangler pages dev site --port {{PORT}}

# Build commands
# ==============

# Build everything (CSS + WASM)
build: build-css build-wasm
    echo "✅ Build complete!"

# Build CSS with Tailwind
build-css:
    echo "🎨 Building CSS..."
    npm run build:css

# Build WASM bundles
build-wasm:
    echo "🦀 Building WASM bundles..."
    ./build.sh

# Build for production (optimized)
build-prod: clean build-css
    echo "🏭 Building for production..."
    cargo build --release --bin server --no-default-features --target wasm32-unknown-unknown --features ssr
    cargo build --release --bin client --no-default-features --target wasm32-unknown-unknown --features hydrate
    wasm-bindgen target/wasm32-unknown-unknown/release/server.wasm --out-name index --no-typescript --target bundler --out-dir site
    wasm-bindgen target/wasm32-unknown-unknown/release/client.wasm --out-name index --no-typescript --target web --out-dir site/pkg
    echo "✅ Production build complete!"

# Deployment commands
# ===================

# Deploy to Cloudflare Pages
deploy PROJECT="leptos-test":
    echo "🚀 Deploying to Cloudflare Pages..."
    just build-prod
    wrangler pages deploy site --project-name {{PROJECT}}

# Deploy with commit dirty flag (if you have uncommitted changes)
deploy-dirty PROJECT="leptos-test":
    echo "🚀 Deploying to Cloudflare Pages (allowing dirty commits)..."
    just build-prod
    wrangler pages deploy site --project-name {{PROJECT}} --commit-dirty=true

# Preview deployment (creates a preview URL)
preview PROJECT="leptos-test":
    echo "👀 Creating preview deployment..."
    just build-prod
    wrangler pages deploy site --project-name {{PROJECT}} --env preview

# Maintenance commands
# ====================

# Clean build artifacts
clean:
    echo "🧹 Cleaning build artifacts..."
    rm -rf site/index.js site/index_bg.js site/index_bg.wasm site/pkg/index.js site/pkg/index_bg.wasm
    cargo clean

# Install dependencies
install:
    echo "📦 Installing dependencies..."
    npm install
    cargo install wasm-bindgen-cli --force

# Update dependencies  
update:
    echo "⬆️  Updating dependencies..."
    npm update
    cargo update

# Check if all tools are installed
check-tools:
    #!/usr/bin/env bash
    echo "🔍 Checking development tools..."
    
    tools=("cargo" "npm" "wrangler" "wasm-bindgen")
    missing=()
    
    for tool in "${tools[@]}"; do
        if command -v "$tool" &> /dev/null; then
            echo "✅ $tool - $(command -v $tool)"
        else
            echo "❌ $tool - not found"
            missing+=("$tool")
        fi
    done
    
    if [ ${#missing[@]} -gt 0 ]; then
        echo ""
        echo "❌ Missing tools: ${missing[*]}"
        echo "Please install missing tools to continue"
        exit 1
    else
        echo ""
        echo "✅ All required tools are installed!"
    fi

# Development utilities
# =====================

# Format Rust code
fmt:
    echo "🎨 Formatting Rust code..."
    cargo fmt

# Run Rust linter
lint:
    echo "🔍 Running Rust linter..."
    cargo clippy

# Run tests
test:
    echo "🧪 Running tests..."
    cargo test

# Git helpers
# ===========

# Quick commit and push
commit MESSAGE:
    git add .
    git commit -m "{{MESSAGE}}"
    git push

# Create new branch
branch BRANCH_NAME:
    git checkout -b {{BRANCH_NAME}}
    git push -u origin {{BRANCH_NAME}}

# Reset to clean state (careful!)
reset:
    echo "⚠️  This will reset your working directory to HEAD"
    echo "Press Ctrl+C to cancel, or Enter to continue..."
    @read
    git reset --hard HEAD
    just clean

# Project info
# ============

# Show project status
status:
    echo "📊 Rust Board Project Status"
    echo "========================"
    echo "📁 Current directory: $(pwd)"
    echo "🌿 Git branch: $(git branch --show-current)"
    echo "📦 Git status:"
    git status --porcelain || echo "  Clean working tree"
    echo ""
    echo "📏 Project stats:"
    echo "  Rust files: $(find src -name '*.rs' | wc -l | tr -d ' ')"
    echo "  Components: $(find src/components -name '*.rs' 2>/dev/null | wc -l | tr -d ' ')"
    echo "  Pages: $(find src/pages -name '*.rs' 2>/dev/null | wc -l | tr -d ' ')"

# Show project URLs
urls:
    echo "🌐 Rust Board URLs"
    echo "=============="
    echo "📚 Repository: https://github.com/frontmesh/rustjobs"
    echo "🌍 Live Demo: https://31eb9d73.leptos-test.pages.dev/"
    echo "💻 Local Dev: http://localhost:8080 (when running 'just dev')"

# Development tips
tips:
    echo "💡 Rust Board Development Tips"
    echo "=========================="
    echo "🚀 Quick start: just dev"
    echo "👀 Watch mode: just watch-css (in separate terminal)"
    echo "🏗️  Build everything: just build"
    echo "🚀 Deploy: just deploy"
    echo "🔍 Check tools: just check-tools"
    echo "📊 Project status: just status"
    echo ""
    echo "📖 For more commands: just --list"