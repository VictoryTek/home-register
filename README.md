<div align="center">
  <img src="frontend/public/logo_full.png" alt="Home Registry Logo" width="400"/>
</div>

# Home Registry
  <p><em>Universal Home Inventory Management System</em></p>
  <p><strong>Version 0.1.0-beta.2</strong></p>

Inspired by HomeBox project, which I used for years. When it was no longer maintained, I thought I would attempt to build a successor to that project.

A modern, universal, web-based home inventory management system built with **Rust + Actix-Web + PostgreSQL**. Keep track of your belongings with an intuitive interface and a simple to use system. Create an inventory for anything you want to track. Create custom Organizers to expand fields that your inventory can track, such as Serial #, Model #, etc. Then add items to your inventory.

## Features

- 🎨 **Modern Web Interface** - Beautiful responsive design with dark/light theme support
- � **Inventory Management** - Organize items by categories, locations, and custom tags
- 🗄️ **Database-Driven** - PostgreSQL backend with comprehensive data relationships
- 🏷️ **Flexible Organization** - Categories, tags, and custom fields for any item type

## Quick Start with Docker Compose

The easiest way to try Home Registry is with Docker Compose, which sets up both the PostgreSQL database and the application:

### 1. Create Configuration File

**First-time setup:**

```bash
# Copy the example environment file
cp .env.example .env

# Edit .env and set your database password
# The docker-compose.yml has a random default, but you should set your own:
# - Open .env in a text editor
# - Set POSTGRES_PASSWORD to your secure password (16+ characters)
# - Save and close
```

**Recommended password requirements:**
- Minimum 16 characters
- Mix of uppercase, lowercase, numbers, and symbols
- Avoid these characters in passwords: `@` `:` `/` (they conflict with connection strings)
- Use a password manager to generate strong passwords
- Example strong password: `7mK$9pQx2#nLwR5tY8vB3zF`

### 2. Start the Application

```bash
docker compose up -d
```

The application will be available at `http://YOUR_IP_ADDRESS:8210`

**Default Configuration:**
- **Application Port:** `8210` (customizable via `.env`)
- **Database:** `home_inventory`
- **Username:** `postgres`
- **Password:** Set in your `.env` file (defaults to a random password if not configured)
  - **⚠️ Security Note:** Always set your own password in `.env` - never rely on the default

**Data Persistence:**
- Database data is persisted in the `pgdata` Docker volume
- Application data (including JWT secrets) is stored in the `appdata` volume
- Database backups are stored in the `backups` volume
- Your data will survive container restarts and updates

### 3. First-Time Setup

After starting containers:

1. Open your browser to `http://YOUR_IP_ADDRESS:8210`
2. Create your admin account
3. Start adding your inventory items!

**⚠️  Security Checklist for Production:**
- ✅ Set strong `POSTGRES_PASSWORD` in `.env` (16+ characters)
- ✅ Set explicit `JWT_SECRET` in `.env` for token consistency
- ✅ Adjust `RATE_LIMIT_RPS` and `RATE_LIMIT_BURST` based on your traffic
- ✅ Never commit your `.env` file to Git
- ✅ Regularly backup your database (see Backup section)

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
      DATABASE_URL: postgres://postgres:${POSTGRES_PASSWORD:-uK8m3NvQ7wPxRj2Y5tLz}@db:5432/home_inventory
      # ☝️ Database password matches db service above. Override in .env file!
      PORT: ${PORT:-8210}
      RUST_LOG: ${RUST_LOG:-info}
      JWT_SECRET: ${JWT_SECRET}
      JWT_TOKEN_LIFETIME_HOURS: ${JWT_TOKEN_LIFETIME_HOURS:-24}
      RATE_LIMIT_RPS: ${RATE_LIMIT_RPS:-100}
      RATE_LIMIT_BURST: ${RATE_LIMIT_BURST:-200}
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
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD:-uK8m3NvQ7wPxRj2Y5tLz}
      # ☝️ Default above is randomly generated. Override in .env file!
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
  -e JWT_TOKEN_LIFETIME_HOURS=24 \
  -e RATE_LIMIT_RPS=100 \
  -e RATE_LIMIT_BURST=200 \
  -v home-registry-data:/app/data \
  -v home-registry-backups:/app/backups \
  home-registry
```

**Important Notes:**
- Replace `your-db-host` with your PostgreSQL server address
- Ensure your PostgreSQL database is accessible from the container
- The `/app/data` volume persists JWT secrets and other application data
- The `/app/backups` volume stores database backup files
- Rate limiting protects your API from being overwhelmed (adjust RPS/BURST as needed)

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

## Security Best Practices

### Password Configuration

**Why Strong Passwords Matter:**
Your PostgreSQL database contains all your inventory data. A weak password exposes your data to unauthorized access if your server is compromised or exposed to the internet.

**Strong Password Guidelines:**
- ✅ **Length:** Minimum 16 characters (longer is better)
- ✅ **Complexity:** Mix uppercase, lowercase, numbers, special characters
- ✅ **Randomness:** Use password manager generators or passphrases
- ❌ **Avoid:** Dictionary words, personal info, common patterns (e.g., "Password123!")
- ❌ **Special Characters:** Avoid `@`, `:`, `/` in passwords (conflict with connection strings)

**Generating Strong Passwords:**

```bash
# Using OpenSSL (Linux/Mac/Windows with WSL)
openssl rand -base64 24

# Using PowerShell (Windows)
-join ((48..57) + (65..90) + (97..122) | Get-Random -Count 20 | ForEach-Object {[char]$_})

# Using Password Manager
# Recommended: 1Password, Bitwarden, LastPass, KeePass
# Generate 20+ character password with all character types
```

**Example Strong Passwords:**
- Generated: `7mK$9pQx2#nLwR5tY8vB3zF` (23 chars)
- Passphrase: `correct-horse-battery-staple-2026` (37 chars)

### JWT Secret Configuration

The `JWT_SECRET` is used to sign authentication tokens. If not set, a random secret is auto-generated and persisted to `/app/data/jwt_secret`.

**Production Recommendation:**
Set an explicit JWT secret in your `.env` file to ensure consistency:

```bash
# Generate a secure JWT secret
openssl rand -base64 32

# Add to .env file
JWT_SECRET=your-generated-secret-here
```

**Why set it explicitly?**
- Ensures tokens remain valid if `/app/data` volume is lost
- Allows running multiple app instances with same secret (load balancing)
- Makes disaster recovery easier

### Protecting Your .env File

**✅ Verify .env is in .gitignore:**
```bash
# Should show .env
cat .gitignore | grep "^\.env$"
```

**✅ File Permissions (Linux/Mac):**
```bash
# Restrict .env to owner read/write only
chmod 600 .env
```

**✅ Backup Securely:**
- Store `.env` backup separately from code repository
- Use encrypted storage (password manager, encrypted drive)
- Never email or share via unencrypted channels

### Additional Security Measures

**Firewall Configuration:**
If exposing to internet, use a firewall to restrict access:

```bash
# Example: Allow only specific IP ranges
# (Implementation varies by OS/firewall)
```

**HTTPS/SSL:**
For internet-facing deployments, use a reverse proxy with SSL:
- Nginx with Let's Encrypt certificates
- Traefik with automatic HTTPS
- Caddy (automatic HTTPS)

**Regular Updates:**
```bash
# Pull latest Docker images
docker compose pull

# Recreate containers with new images
docker compose up -d
```

**Database Backups:**
See "Backup and Restore" section for automated backup strategies.

## Troubleshooting

### Database Connection Issues

**Problem:** "FATAL: password authentication failed for user postgres"

**Causes:**
- Password mismatch between `.env` configuration and existing database volume
- Changed password in `.env` after database was already initialized
- Manually changed password without resetting database volume

**Solutions:**

1. **Complete Reset (⚠️  Destroys all data):**
   ```bash
   docker compose down -v
   docker compose up -d
   ```
   This removes all volumes including your database, giving you a fresh start with the current `.env` configuration.

2. **Change Password with Data Preservation:**
   ```bash
   # Backup your data first
   docker compose exec db pg_dump -U postgres home_inventory > backup.sql
   
   # Stop and remove volumes
   docker compose down -v
   
   # Update POSTGRES_PASSWORD in .env file
   # Then start with new configuration
   docker compose up -d
   
   # Restore your data
   cat backup.sql | docker compose exec -T db psql -U postgres home_inventory
   ```

3. **Verify Current Configuration:**
   ```bash
   # Check what password is configured (on Linux/Mac)
   grep POSTGRES_PASSWORD .env
   
   # On Windows PowerShell
   Select-String -Path .env -Pattern "POSTGRES_PASSWORD"
   ```

**Understanding Volume Persistence:**
Docker volumes persist data across container restarts. Once a PostgreSQL database is created with a password, that password is stored in the database itself. Changing the `POSTGRES_PASSWORD` environment variable only affects new database initialization—it won't update an existing database.

**If you don't have a .env file:**
If you didn't create a `.env` file, the system uses a randomly generated default password from docker-compose.yml. However, you should always set your own password in `.env` for security:

```bash
# Create .env file from example
cp .env.example .env

# Edit .env and set POSTGRES_PASSWORD
# Then recreate containers
docker compose down -v  # ⚠️  Destroys existing data
docker compose up -d
```

### JWT Secret and Permission Issues

**Problem:** "Permission denied" errors for `/app/data/jwt_secret` in logs

**Causes:**
- Older Docker image without proper `/app/data` directory permissions
- Manual volume mounts overriding container directory ownership

**Solutions:**

1. **Rebuild Container Image:**
   ```bash
   docker compose down
   docker compose build --no-cache
   docker compose up -d
   ```

2. **Check JWT Secret Persistence:**
   ```bash
   # Verify the secret file exists and has correct permissions
   docker compose exec app ls -la /app/data/
   ```
   You should see `jwt_secret` file owned by `appuser`.

3. **Manual Secret Configuration (Production):**
   If automatic generation fails, you can set a custom JWT secret:
   ```yaml
   environment:
     JWT_SECRET: "your-very-long-secure-random-string-here"
   ```

**Impact of JWT Secret Issues:**
- If the secret can't be persisted, a new one is generated on each restart
- This invalidates all existing user sessions
- Users must log in again after every container restart
- Not critical for operation, but poor user experience

### Container Won't Start

**Check logs:**
```bash
# View application logs
docker compose logs -f app

# View database logs
docker compose logs -f db
```

**Common issues:**
- **Port already in use:** Change port `8210:8210` to `8211:8210` (or any other available port)
- **Database not ready:** Wait 10-15 seconds for PostgreSQL to initialize on first run
- **Missing DATABASE_URL:** Verify environment variables are set correctly

### Starting Fresh

If you want to completely reset and start over:

```bash
# Stop all containers and remove ALL data
docker compose down -v

# Remove any cached images (optional)
docker compose down --rmi all -v

# Pull/build fresh images and start
docker compose build --no-cache
docker compose up -d
```

**⚠️ Warning:** The `-v` flag removes all volumes, including your database, backups, and uploaded images. Make sure you have backups before running this command.

### Getting Help

If you're still having issues:
1. Check the logs: `docker compose logs -f`
2. Verify your configuration matches the documentation
3. Ensure you're using the latest version
4. Check for open issues on GitHub
5. Create a new issue with your logs and configuration (remove sensitive data)