<div align="center">
  <img src="frontend/public/logo_full.png" alt="Home Registry Logo" width="400">
  <h1>Home Registry</h1>
  <p><em>Home Inventory Management System</em></p>
  
  <p>
    <a href="https://github.com/VictoryTek/home-registry/releases">
      <img src="https://img.shields.io/github/v/release/VictoryTek/home-registry?include_prereleases&label=version&color=blue" alt="Version">
    </a>
    <a href="https://github.com/VictoryTek/home-registry/pkgs/container/home-registry">
      <img src="https://img.shields.io/badge/ghcr.io-home--registry-blue?logo=docker" alt="GHCR">
    </a>
    <a href="https://github.com/VictoryTek/home-registry/blob/main/LICENSE">
      <img src="https://img.shields.io/github/license/VictoryTek/home-registry" alt="License">
    </a>
  </p>
</div>

---

## About

A modern, self-hosted home inventory management system built with Rust and React. Track your belongings with flexible organization through categories, tags, custom fields, and multi-user support with secure sharing capabilities.

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

---

## Deployment

### 🎉 Zero-File Deployment

**No repository cloning required!** Home Registry uses embedded migrations bundled directly into the Docker image. Deploy with just the pre-built image from GitHub Container Registry:

✅ **No local files needed** - Migrations are compiled into the binary  
✅ **Automatic schema setup** - Migrations run on every startup (idempotent)  
✅ **Version-matched** - Migrations always match your app version  
✅ **Portainer/Dockge friendly** - Just paste the compose config and go  

---

### Option 1: Docker Compose (Recommended)

**Prerequisites:**
- Docker and Docker Compose installed
- PostgreSQL database (included in docker-compose.yml)

**✨ New:** No repository cloning required! Migrations are bundled in the image and run automatically on startup.

**Tested and working configuration** - Compatible with Portainer, Dockge, or standalone:

```yaml
services:
  db:
    image: postgres:17
    container_name: home-registry-db
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: homeregistry2026
      POSTGRES_DB: home_inventory
    ports:
      - "5432:5432"
    volumes:
      - pgdata:/var/lib/postgresql/data
      # Migrations run automatically from app container - no volume mount needed!
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 5s
      timeout: 5s
      retries: 5
    restart: unless-stopped

  app:
    image: ghcr.io/victorytek/home-registry:beta
    container_name: home-registry-app
    depends_on:
      db:
        condition: service_healthy
    environment:
      DATABASE_URL: postgres://postgres:homeregistry2026@db:5432/home_inventory
      PORT: 8210
      RUST_LOG: info
      RATE_LIMIT_RPS: 100
      RATE_LIMIT_BURST: 200
    ports:
      - "8210:8210"
    volumes:
      - appdata:/app/data
      - backups:/app/backups
    restart: unless-stopped
    healthcheck:
      test: ["CMD-SHELL", "curl -f http://localhost:8210/health || exit 1"]
      interval: 30s
      timeout: 10s
      start_period: 10s
      retries: 3

volumes:
  pgdata:
    name: home-registry-pgdata
  appdata:
    name: home-registry-appdata
  backups:
    name: home-registry-backups
```

**To deploy:**
```bash
# Save the above as docker-compose.yml, then:
docker compose up -d
```

**Access the app at:** **http://localhost:8210**

> **Note:** First deployment may take 30-60 seconds while the database initializes and migrations run.

---

### Option 2: Docker Run Commands

**Now simplified** - No migration volumes required:

```bash
# Create network
docker network create home-registry-net

# Start database (no migration volumes needed!)
docker run -d \
  --name home-registry-db \
  --network home-registry-net \
  -p 5432:5432 \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=homeregistry2026 \
  -e POSTGRES_DB=home_inventory \
  -v home-registry-pgdata:/var/lib/postgresql/data \
  --restart unless-stopped \
  postgres:17

# Wait for database to be ready
sleep 10

# Start application (migrations run automatically!)
docker run -d \
  --name home-registry-app \
  --network home-registry-net \
  -p 8210:8210 \
  -e DATABASE_URL=postgres://postgres:homeregistry2026@home-registry-db:5432/home_inventory \
  -e PORT=8210 \
  -e RUST_LOG=info \
  -e RATE_LIMIT_RPS=100 \
  -e RATE_LIMIT_BURST=200 \
  -v home-registry-appdata:/app/data \
  -v home-registry-backups:/app/backups \
  --restart unless-stopped \
  ghcr.io/victorytek/home-registry:beta
```

**That's it!** The app will automatically run migrations on startup.

**Access the app at:** **http://localhost:8210**

---

## Troubleshooting

### Migration Troubleshooting

**Error:** "Database migrations failed"

**Cause:** Application cannot connect to database or migration SQL has errors.

**Solution:**
1. Check database is running: `docker compose ps`
2. Verify DATABASE_URL is correct
3. Check application logs: `docker compose logs app`
4. Ensure database user has CREATE TABLE permissions

**Note:** Migrations run automatically on every startup. The app will not start if migrations fail, ensuring schema consistency.

### Upgrading from Previous Versions

**If you previously volume-mounted migrations:**

1. Remove the volume mount from your `docker-compose.yml`:
   ```yaml
   # REMOVE this line:
   - ./migrations:/docker-entrypoint-initdb.d
   ```

2. Restart the application:
   ```bash
   docker compose down
   docker compose pull  # Get latest image with refinery
   docker compose up -d
   ```

3. Check logs to confirm migrations ran:
   ```bash
   docker compose logs app | grep -i migration
   ```

**Expected output:**
```
home-registry-app | Running database migrations...
home-registry-app | Database schema is up to date. No new migrations to apply
```

Your existing data is preserved - refinery only applies NEW migrations.

---

**License:** MIT | **Documentation:** [Release Notes](release_notes/) | **Issues:** [GitHub Issues](https://github.com/VictoryTek/home-registry/issues)
