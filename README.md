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

## Quick Start with Docker Compose

The easiest way to try Home Registry is with Docker Compose, which sets up both the PostgreSQL database and the application:

The application will be available at `http://YOUR_IP_ADDRESS:8210`

**Default Configuration:**
- **Application Port:** `8210` (can be changed in docker-compose.yml)
- **Database:** `home_inventory`
- **Username:** `postgres`
- **Password:** `changeme` (⚠️ **change this to a strong password!**)

**Data Persistence:**
- Database data is persisted in the `pgdata` Docker volume
- Application data (including JWT secrets) is stored in the `appdata` volume
- Database backups are stored in the `backups` volume
- Your data will survive container restarts and updates

**For Production Use:**
1. Copy the docker-compose.yml configuration above
2. **Change the database password** from `changeme` to your own strong password
3. Update the `DATABASE_URL` to match your new password
4. Adjust `RATE_LIMIT_RPS` and `RATE_LIMIT_BURST` based on your expected traffic

**Useful Commands:**
```bash
# Start the application
docker-compose up -d

# build the application
docker-compose build

# Stop the application
docker-compose down

# Stop and remove all data (⚠️ destructive)
docker-compose down -v

# View database logs
docker-compose logs -f db
```

### Docker Compose Configuration

Here's the complete `docker-compose.yml` file:

```yaml
services:
  app:
    image: ghcr.io/victorytek/home-registry:beta
    container_name: home-registry-app
    depends_on:
      db:
        condition: service_healthy
    environment:
      DATABASE_URL: postgres://postgres:changeme@db:5432/home_inventory
      PORT: 8210
      RUST_LOG: info
      RATE_LIMIT_RPS: 100  # API requests per second limit
      RATE_LIMIT_BURST: 200  # Burst capacity for traffic spikes
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

  db:
    image: postgres:17
    container_name: home-registry-db
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: changeme  # ⚠️ Change this password
      POSTGRES_DB: home_inventory
    ports:
      - "5432:5432"
    volumes:
      - pgdata:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 5s
      timeout: 5s
      retries: 5
    restart: unless-stopped

volumes:
  pgdata:
    name: home-registry-pgdata
  appdata:
    name: home-registry-appdata
  backups:
    name: home-registry-backups
```

## Running with Docker (Standalone)

If you prefer to run just the application container (with an external PostgreSQL database), you can use the standalone Docker command:

```bash
# Build the image
docker build -t home-registry .

# Run the container
docker run -d \
  --name home-registry \
  -p 8210:8210 \
  -e DATABASE_URL="postgres://postgres:password@your-db-host:5432/home_inventory" \
  -e PORT=8210 \
  -e RUST_LOG=info \
  -e JWT_SECRET="your-secure-secret-here" \
  -e JWT_TOKEN_LIFETIME_HOURS=24 \
  -e RATE_LIMIT_RPS=100 \
  -e RATE_LIMIT_BURST=200 \
  -v home-registry-data:/app/data \
  -v home-registry-backups:/app/backups \
  home-registry
```

**Important Notes:**
- Replace `your-db-host` with your PostgreSQL server address
- Change `your-secure-secret-here` to a secure random string (or omit for auto-generation)
- Ensure your PostgreSQL database is accessible from the container
- The `/app/data` volume persists JWT secrets and other application data
- The `/app/backups` volume stores database backup files
- Rate limiting protects your API from being overwhelmed (adjust RPS/BURST as needed)

**Useful Docker Commands:**
```bash
# View logs
docker logs -f home-registry

# Stop the container
docker stop home-registry

# Remove the container
docker rm home-registry

# Run with host network (Linux only)
docker run -d \
  --name home-registry \
  --network host \
  -e DATABASE_URL="postgres://postgres:password@localhost:5432/home_inventory" \
  -e RUST_LOG=info \
  home-registry
```

## Environment Variables

The application supports the following configuration through environment variables:

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `DATABASE_URL` | PostgreSQL connection string | - | ✅ Yes |
| `PORT` | HTTP server port | `8210` | No |
| `RUST_LOG` | Logging level (`error`, `warn`, `info`, `debug`, `trace`) | `info` | No |
| `JWT_SECRET` | Secret key for JWT token signing (auto-generated if not set) | Auto-generated | No* |
| `JWT_TOKEN_LIFETIME_HOURS` | JWT token expiration time in hours | `24` | No |
| `RATE_LIMIT_RPS` | Maximum API requests per second | `50` | No |
| `RATE_LIMIT_BURST` | Burst capacity for temporary traffic spikes | `100` | No |

**\*JWT_SECRET Note:** If not explicitly set, a random secret is generated and persisted to `/app/data/jwt_secret`. This ensures tokens remain valid across container restarts. For production, it's recommended to set this explicitly.

**Rate Limiting Explained:**
- **RATE_LIMIT_RPS**: Controls sustained API request throughput. If set to `100`, the server accepts up to 100 requests per second continuously.
- **RATE_LIMIT_BURST**: Allows temporary spikes above the RPS limit. With `BURST: 200`, the server can handle short bursts of 200 requests before enforcing the RPS limit.
- **Use Case**: Protects your server from being overwhelmed by aggressive API clients, accidental infinite loops, or potential DoS attacks.
- **Production Recommendation**: Start with `RPS: 100` and `BURST: 200`, then adjust based on your usage patterns and server capacity.