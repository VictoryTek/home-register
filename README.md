<div align="center">
  <img src="frontend/public/logo_full.png" alt="Home Registry Logo" width="400"/>
</div>

# Home Registry

** WORK IN PROGRESS **

Inspired by HomeBox project, which I used for years. When it was no longer maintained, I thought I would attempt to build a successor to that project.

A modern, universal, web-based home inventory management system built with **Rust + Actix-Web + PostgreSQL**. Keep track of your belongings with an intuitive interface and a simple to use system. Create an inventory for anything you want to track. Create custom Organizers to expand fields that your inventory can track, such as Serial #, Model #, etc. Then add items to your inventory.

## Features

- 🎨 **Modern Web Interface** - Beautiful responsive design with dark/light theme support
- � **Inventory Management** - Organize items by categories, locations, and custom tags
- 🗄️ **Database-Driven** - PostgreSQL backend with comprehensive data relationships
- 🏷️ **Flexible Organization** - Categories, tags, and custom fields for any item type
- 🔍 **Advanced Search** - Find items quickly with powerful filtering options
- 📊 **Dashboard Analytics** - Overview of your inventory with statistics and insights

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