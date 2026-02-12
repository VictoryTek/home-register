<div align="center">
  <img src="frontend/public/logo_full.png" alt="Home Registry Logo" width="400"/>
</div>

# Home Registry

** WORK IN PROGRESS **

Inspired by HomeBox project, which I used for years. When it was no longer maintained, I thought I would attempt to build a successor to that project.

A modern, web-based home inventory management system built with **Rust + Actix-Web + PostgreSQL**. Keep track of your belongings with a simple and intuitive interface featuring custom organizers and full sharing capabilities.

## Features

- 🎨 **Modern Web Interface** - Beautiful responsive design with dark/light theme support
- � **Inventory Management** - Organize items by categories, locations, and custom tags
- 🗄️ **Database-Driven** - PostgreSQL backend with comprehensive data relationships
- 🏷️ **Flexible Organization** - Categories, tags, and custom fields for any item type
- 🔍 **Advanced Search** - Find items quickly with powerful filtering options
- 📊 **Dashboard Analytics** - Overview of your inventory with statistics and insights

## Development Setup

### Option A: Docker Compose (Recommended)

The easiest way to develop with Home Registry is using Docker Compose, which sets up both the PostgreSQL database and the application:

```bash
# Clone the repository
git clone https://github.com/VictoryTek/home-registry.git
cd home-registry

# Start the application
docker-compose up -d

# View logs
docker-compose logs -f app
```

The application will be available at `http://localhost:8210`

**Pros:** 
- ✅ No local setup needed
- ✅ Service worker works out of the box
- ✅ Matches production environment

**Cons:** 
- ⚠️ Slower rebuild times
- ⚠️ Container overhead

**Useful Commands:**
```bash
# Stop the application
docker-compose down

# Stop and remove all data (⚠️ destructive)
docker-compose down -v

# View database logs
docker-compose logs -f db

# Rebuild after code changes
docker-compose up -d --build
```

### Option B: Local Development (Without Docker)

For active frontend/backend development requiring faster iteration:

**Prerequisites:**
- Rust 1.85+ (`rustc --version`)
- Node.js 18+ and npm 9+ (`node --version && npm --version`)
- PostgreSQL 16+ (running locally or via Docker)

**Setup Steps:**

1. **Start PostgreSQL:**
   ```bash
   # Option 1: Using Docker (just database)
   docker run -d --name home-registry-db -p 5432:5432 \
     -e POSTGRES_USER=postgres \
     -e POSTGRES_PASSWORD=password \
     -e POSTGRES_DB=home_inventory \
     postgres:17
   
   # Option 2: Use your local PostgreSQL installation
   # Make sure database 'home_inventory' exists
   ```

2. **Configure Environment:**
   ```bash
   # Windows PowerShell
   $env:DATABASE_URL = "postgres://postgres:password@localhost:5432/home_inventory"
   $env:RUST_LOG = "info"
   
   # Linux/macOS
   export DATABASE_URL="postgres://postgres:password@localhost:5432/home_inventory"
   export RUST_LOG=info
   ```

3. **Build Frontend and Sync to Static:**
   ```bash
   cd frontend
   
   # Install dependencies (first time only)
   npm install
   
   # Build frontend and sync to ../static/ directory
   # This replicates what Docker does: frontend/dist/ → static/
   npm run build:full
   
   cd ..
   ```

4. **Run Rust Backend:**
   ```bash
   # Backend will serve files from static/ directory
   cargo run
   # Server starts on http://localhost:8210
   ```

5. **Access Application:**
   ```
   http://localhost:8210
   ```

### Development Workflows

#### Frontend Development

When making frontend changes (React, TypeScript, CSS):

```bash
cd frontend

# Make your changes to files in src/...

# Option 1: Full rebuild + sync (recommended)
npm run build:full

# Option 2: Separate build and sync
npm run build
npm run sync-dist

cd ..

# Restart backend to serve new files
cargo run
```

**Available Frontend Scripts:**
- `npm run build` - Build TypeScript and React app to dist/
- `npm run sync-dist` - Copy dist/ contents to ../static/
- `npm run build:full` - Build and sync in one command (recommended)
- `npm run clean` - Remove dist/ and static/ directories
- `npm run dev` - Start Vite dev server (port 3000, for component development only)

#### Backend Development

Backend-only changes (Rust, API routes, database) don't require frontend rebuild:

```bash
# Make your changes to files in src/...

# Just restart the backend
cargo run
```

#### Database Migrations

```bash
# Migrations are in migrations/ directory
# They run automatically on first backend start

# To manually apply migrations:
psql -U postgres -d home_inventory -f migrations/001_create_items_table.sql
```

### Service Worker & PWA Development

The application uses [VitePWA](https://vite-pwa-org.netlify.app/) with [Workbox](https://developer.chrome.com/docs/workbox/) for Progressive Web App support and offline functionality.

**Key Files:**
- `frontend/vite.config.ts` - VitePWA configuration
- `static/sw.js` - Generated service worker (**DO NOT EDIT**)
- `static/workbox-{hash}.js` - Workbox runtime (**DO NOT EDIT**)
- `static/manifest.webmanifest` - PWA manifest

**Important:** Service worker is **enabled in development** (`devOptions.enabled: true` in vite.config.ts). This means the SW is active even during local development.

**To disable service worker in development:**
1. Edit `frontend/vite.config.ts`
2. Change `devOptions.enabled: false`
3. Run `npm run build:full`
4. Restart backend with `cargo run`

**To unregister service worker in browser:**
1. Open DevTools (F12)
2. Go to **Application** tab → **Service Workers**
3. Click **Unregister** next to `sw.js`
4. Hard refresh page (Ctrl+Shift+R or Cmd+Shift+R)

### Troubleshooting

#### Service Worker 404 Errors

**Problem:** Browser console shows `GET http://localhost:8210/sw.js 404` or `GET http://localhost:8210/workbox-*.js 404`

**Solution:**
```bash
cd frontend
npm run build:full  # Rebuilds frontend and syncs to static/
cd ..
cargo run  # Restart backend
```

**Explanation:** The Rust backend serves files from `static/` directory, but Vite builds to `frontend/dist/`. The sync script copies dist/ → static/, just like Docker does.

#### Stale Content After Rebuild

**Problem:** Changes don't appear after rebuilding frontend

**Solutions:**
1. **Unregister service worker** (see instructions above)
2. Use **Incognito/Private browsing mode** (no cache)
3. Enable **"Update on reload"** in DevTools → Application → Service Workers
4. **Hard refresh**: Ctrl+Shift+R (Windows/Linux) or Cmd+Shift+R (macOS)

#### Database Connection Errors

**Problem:** `ERROR: connection refused` or `ERROR: role "postgres" does not exist`

**Solutions:**
1. Verify PostgreSQL is running: `docker ps` or `pg_isready`
2. Check DATABASE_URL environment variable is set correctly
3. Ensure database `home_inventory` exists: `psql -l`
4. Verify credentials match your PostgreSQL setup

#### Build Fails with TypeScript Errors

**Problem:** `npm run build` fails with type errors

**Solution:**
```bash
cd frontend
npm run typecheck  # Check for type errors
npm run lint:fix   # Auto-fix linting issues
npm run build      # Try building again
```

#### "static/ directory does not exist" Error

**Problem:** Backend fails to serve files

**Solution:**
```bash
# The static/ directory is generated, not committed to Git
cd frontend
npm run build:full  # Creates static/ with all files
```

## Quick Start with Docker Compose

The easiest way to try Home Registry is with Docker Compose, which sets up both the PostgreSQL database and the application:

```bash
# Clone the repository
git clone https://github.com/VictoryTek/home-registry.git
cd home-registry

# Start the application
docker-compose up -d

# View application logs
docker-compose logs -f app
```

The application will be available at `http://localhost:8210`

**Default Configuration:**
- **Application Port:** `8210` (can be changed in docker-compose.yml)
- **Database:** `home_inventory`
- **Username:** `postgres`
- **Password:** `password` (⚠️ change for production!)

**Data Persistence:**
- Database data is persisted in the `pgdata` Docker volume
- Application data (including JWT secrets) is stored in the `appdata` volume
- Your data will survive container restarts and updates

**For Production Use:**
1. Edit `docker-compose.yml` and uncomment the JWT_SECRET line
2. Set a secure random string for JWT_SECRET
3. Change the default database password
4. Optionally adjust JWT_TOKEN_LIFETIME_HOURS (default: 24 hours)

### Understanding the Build Process

**How Docker Build Works:**
1. Stage 1: Builds React frontend → `frontend/dist/`
2. Stage 2: Builds Rust backend → binary
3. Stage 3: Copies `frontend/dist/ → static/` in final image
4. Backend serves files from `static/` directory

**How Local Development Works:**
1. Build React frontend → `frontend/dist/`
2. Sync script copies `frontend/dist/ → static/`
3. Backend serves files from `static/` directory

The sync script (`frontend/sync-dist.js`) replicates what the Docker multi-stage build does automatically, enabling local development without Docker.

### Docker Compose Configuration

Here's the complete `docker-compose.yml` file:

```yaml
services:
  db:
    image: postgres:17
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: password
      POSTGRES_DB: home_inventory
    ports:
      - "5432:5432"
    volumes:
      - pgdata:/var/lib/postgresql/data
      - ./migrations:/docker-entrypoint-initdb.d
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 5s
      timeout: 5s
      retries: 5
  app:
    build: .
    depends_on:
      db:
        condition: service_healthy
    environment:
      DATABASE_URL: postgres://postgres:password@db:5432/home_inventory
      PORT: 8210
      RUST_LOG: info
      # JWT_SECRET: "your-secure-secret-here"  # Uncomment and set for production
      # JWT_TOKEN_LIFETIME_HOURS: 24  # Token lifetime in hours (default: 24)
    ports:
      - "8210:8210"
    volumes:
      - appdata:/app/data  # Persist JWT secret and other app data
    command: ["./home-registry"]
    restart: on-failure
volumes:
  pgdata:
  appdata:
```