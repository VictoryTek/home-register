<div align="center">
  <img src="frontend/public/logo_full.png" alt="Home Registry Logo" width="400">
  <h1>Home Registry</h1>
  <p><em>Home Inventory Management System</em></p>
  <p><strong>Work in Progress</strong></p>
  
  <p>
    <a href="https://github.com/yourusername/home-registry/releases">
      <img src="https://img.shields.io/github/v/release/yourusername/home-registry?include_prereleases&label=version&color=blue" alt="Version">
    </a>
    <a href="https://github.com/yourusername/home-registry/pkgs/container/home-registry">
      <img src="https://img.shields.io/badge/ghcr.io-home--registry-blue?logo=docker" alt="GHCR">
    </a>
    <a href="https://github.com/VictoryTek/home-registry/blob/main/LICENSE">
      <img src="https://img.shields.io/github/license/VictoryTek/home-registry" alt="License">
    </a>
  </p>
</div>

A modern, web-based home inventory management system built with Rust, PostgreSQL, and Docker. Keep track of your belongings with an intuitive interface featuring flexible organization and full sharing capabilities.

Inspired by the HomeBox project, which was maintained for years. This project aims to be a successor with modern architecture and continued development.

## Features

- **Inventory Management**: Add, edit, and organize your belongings
- **Flexible Organization**: Categories, tags, custom fields, and organizers
- **User Permissions**: Admin and standard user roles with proper access control
- **Inventory Sharing**: Share collections with other users (view/edit/full permissions)
- **Search & Filter**: Find items quickly with powerful filtering options
- **Progressive Web App**: Install on any device and work offline
- **Mobile-Friendly**: Responsive design for phones and tablets
- **Multi-User Support**: Complete data isolation with secure sharing

## Tech Stack

- **Backend**: Rust with Actix-Web framework
- **Database**: PostgreSQL with deadpool-postgres
- **Frontend**: TypeScript, React, Vite
- **Deployment**: Docker & Docker Compose

## Installation

### Option 1: Pre-Built Image from GHCR (Recommended)

The easiest way to get started is using pre-built images from GitHub Container Registry:

```bash
# Download production compose file
curl -sSL https://raw.githubusercontent.com/yourusername/home-registry/main/docker-compose.prod.yml -o docker-compose.yml

# Start services
docker compose up -d

# Access at http://localhost:8210
```

**Multi-Architecture Support:**
- ✅ **linux/amd64** (Intel/AMD x86_64)
- ✅ **linux/arm64** (Raspberry Pi 4/5, Apple Silicon, AWS Graviton)

Docker automatically pulls the correct architecture for your system.

**Available Tags:**

| Tag | Description | Stability |
|-----|-------------|-----------|
| `beta` | Latest beta pre-release | Testing |
| `v0.1.0-beta.1` | Specific beta version | Fixed |
| `latest` | Latest stable release | Production (when available) |
| `v1.0.0` | Specific stable version | Fixed (when available) |

### Option 2: Local Build (Development)

For development or customization:

```bash
# Clone repository
git clone https://github.com/yourusername/home-registry.git
cd home-registry

# Start with local build
docker compose up -d
```

### Verification

Verify image authenticity and inspect SBOM:

```bash
# Check SBOM (Software Bill of Materials)
docker sbom ghcr.io/yourusername/home-registry:v0.1.0-beta.1

# Inspect image metadata
docker inspect ghcr.io/yourusername/home-registry:v0.1.0-beta.1
```

## Quick Start

### Zero-Config Docker Deployment ⚡

**For Production (Pre-built Image):**

```bash
# Download production compose file
curl -sSL https://raw.githubusercontent.com/yourusername/home-registry/main/docker-compose.prod.yml -o docker-compose.yml
docker compose up -d
```

**For Development (Local Build):**

```bash
# Clone repository first
git clone https://github.com/yourusername/home-registry.git
cd home-registry

# Build and run
docker compose up -d
```

**That's it!** The app automatically:
- ✅ Generates and persists JWT secret
- ✅ Sets up database and runs migrations
- ✅ Uses secure defaults
- ✅ Works with localhost or your server's IP

Access at: `http://localhost:8210` (or your server's IP)

**First Run:**
- Complete the 3-step setup wizard:
  1. Create your admin account (username and full name)
  2. Set a secure password (minimum 8 characters)
  3. Save your recovery codes for account recovery
- After setup, create your first inventory from the main page
- Start adding items to track

### Docker Compose Commands

```bash
# Start services
docker compose up -d

# View logs
docker compose logs -f app

# Stop services
docker compose down

# Stop and remove all data (⚠️ destructive)
docker compose down -v
```

**Using Specific Versions:**

```bash
# Use specific beta version
VERSION=v0.1.0-beta.1 GITHUB_REPOSITORY_OWNER=victorytek docker compose -f docker-compose.prod.yml up -d

# Use latest beta
VERSION=beta GITHUB_REPOSITORY_OWNER=victorytek docker compose -f docker-compose.prod.yml up -d
```

### Docker Run (Manual)

```bash
docker run -d --name home-registry -p 8210:8210 \
  -e DATABASE_URL=postgres://postgres:password@db:5432/home_inventory \
  -e RUST_LOG=info \
  -e PORT=8210 \
  home-registry:latest
```

**Note**: This requires a separate PostgreSQL database. Use Docker Compose for a complete setup.

## Docker Compose Files

Home Registry provides two Docker Compose configurations:

### docker-compose.yml (Development/Local Build)

For development or when you want to build from source:

```yaml
services:
  db:
    image: postgres:17
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: password
      POSTGRES_DB: home_inventory
    volumes:
      - pgdata:/var/lib/postgresql/data
      - ./migrations:/docker-entrypoint-initdb.d
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 5s
      timeout: 5s
      retries: 5

  app:
    build: .  # Builds from local Dockerfile
    depends_on:
      db:
        condition: service_healthy
    environment:
      DATABASE_URL: postgres://postgres:password@db:5432/home_inventory
      PORT: 8210
      RUST_LOG: info
    ports:
      - "8210:8210"
    volumes:
      - appdata:/app/data  # Auto-generated secrets stored here
    restart: on-failure

volumes:
  pgdata:
  appdata:
```

### docker-compose.prod.yml (Production/Pre-built Image)

For production deployments using pre-built images from GHCR:

```yaml
services:
  db:
    image: postgres:17
    # ... same as above

  app:
    image: ghcr.io/${GITHUB_REPOSITORY_OWNER:-yourusername}/home-registry:${VERSION:-beta}
    pull_policy: always  # Always check for updates
    # ... rest of configuration
```

**Key Differences:**
- **Production**: Uses `image:` to pull from GHCR, includes health checks
- **Development**: Uses `build:` to compile from source locally

**Key Differences:**
- **Production**: Uses `image:` to pull from GHCR, includes health checks
- **Development**: Uses `build:` to compile from source locally

## Configuration

### What's Automated

Both configurations provide:
- 🔐 JWT secret generation and persistence
- 🗄️ Database migrations
- ⚙️ Sensible defaults for all settings

### Optional Production Configuration

For production, create a `.env` file or export environment variables:

```env
# Override repository owner (for forks)
GITHUB_REPOSITORY_OWNER=yourusername

# Specify version
VERSION=v0.1.0-beta.1  # or 'beta' for latest beta, 'latest' for stable

# Database credentials (recommended for production)
POSTGRES_PASSWORD=your_secure_password

# Application settings
RUST_LOG=info
JWT_TOKEN_LIFETIME_HOURS=24
```

Then run:
```bash
docker compose -f docker-compose.prod.yml up -d
```

## Environment Variables

The following environment variables can be configured (all have sensible defaults):

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `POSTGRES_DB` | No | `home_inventory` | PostgreSQL database name |
| `POSTGRES_USER` | No | `postgres` | PostgreSQL username |
| `POSTGRES_PASSWORD` | No | `password` | PostgreSQL password |
| `DATABASE_URL` | Auto | Auto-generated | Full PostgreSQL connection string |
| `PORT` | No | `8210` | Port for the web server to listen on |
| `RUST_LOG` | No | `info` | Logging level (`trace`, `debug`, `info`, `warn`, `error`) |
| `JWT_SECRET` | No | Auto-generated | Secret key for JWT token signing |
| `JWT_TOKEN_LIFETIME_HOURS` | No | `24` | JWT token lifetime in hours |

**Note**: For production deployments, always override the default database credentials and JWT secret!

## Development

### Local Development

For active development requiring faster iteration:

**Prerequisites:**
- Rust 1.85+
- Node.js 18+ and npm 9+
- PostgreSQL 16+ (or use Docker)

**Setup:**

```bash
# Start PostgreSQL
docker run -d --name home-registry-db -p 5432:5432 \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=home_inventory \
  postgres:17

# Configure environment (Windows PowerShell)
$env:DATABASE_URL = "postgres://postgres:password@localhost:5432/home_inventory"
$env:RUST_LOG = "info"

# Configure environment (Linux/macOS)
export DATABASE_URL="postgres://postgres:password@localhost:5432/home_inventory"
export RUST_LOG=info

# Build frontend
cd frontend
npm install
npm run build:full
cd ..

# Run backend
cargo run
```

Access at: `http://localhost:8210`

### Development Workflows

**Frontend changes:**
```bash
cd frontend
npm run build:full  # Build and sync to static/
cd ..
cargo run  # Restart backend
```

**Backend changes:**
```bash
cargo run  # Just restart backend
```

## Security Considerations

**Default Credentials:** The default database credentials (`postgres`/`password`) are intended for development only.

**Production Recommendations:**
- Use strong, unique passwords for database access
- Set a secure `JWT_SECRET` (minimum 32 characters)
- Configure HTTPS/TLS with a reverse proxy (nginx, Traefik, Caddy)
- Use Docker Swarm secrets, Kubernetes secrets, or cloud provider secret managers
- Regular security updates and backups

## Credits

Home Registry is inspired by HomeBox. Thanks to the HomeBox team for creating an amazing inventory management app!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.