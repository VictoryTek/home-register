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

## Quick Start

```bash
docker network create home-registry-net 2>/dev/null; docker run -d --name home-registry-db --network home-registry-net -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=homeregistry2026 -e POSTGRES_DB=home_inventory -v home-registry-pgdata:/var/lib/postgresql/data postgres:17 && sleep 5 && docker run -d --name home-registry-app --network home-registry-net -p 8210:8210 -e DATABASE_URL=postgres://postgres:homeregistry2026@home-registry-db:5432/home_inventory -e PORT=8210 -e RUST_LOG=info -v home-registry-appdata:/app/data -v home-registry-backups:/app/backups ghcr.io/victorytek/home-registry:latest
```

Access at **http://localhost:8210**

---

## Docker Compose

Copy this configuration to use with Portainer, Dockge, or standalone:

```yaml
services:
  db:
    image: postgres:17
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: homeregistry2026
      POSTGRES_DB: home_inventory
    volumes:
      - pgdata:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 5s
      timeout: 5s
      retries: 5
    restart: unless-stopped

  app:
    image: ghcr.io/victorytek/home-registry:latest
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
  appdata:
  backups:
```

---

**License:** MIT | **Documentation:** [Release Notes](release_notes/) | **Issues:** [GitHub Issues](https://github.com/VictoryTek/home-registry/issues)
