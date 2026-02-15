# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Multi-architecture Docker images (linux/amd64, linux/arm64)
- Automated GHCR publishing workflow
- Security attestation (SBOM, provenance)
- Production Docker Compose configuration

### Changed
- Split docker-compose.yml into dev and prod variants
- Improved container deployment documentation

### Security
- Added Trivy vulnerability scanning in release pipeline
- Enabled SLSA provenance attestation

## [0.1.0-beta.1] - 2026-02-15

### Added
- **Authentication & User Management**
  - User authentication with JWT tokens
  - Argon2id password hashing
  - Admin and standard user roles
  - Recovery codes for account recovery
  - User settings management

- **Inventory Management**
  - Create, read, update, delete inventories
  - Multi-user support with data isolation
  - Flexible permission system (view/edit/full access)
  - Inventory sharing between users
  - Inventory image uploads

- **Item Management**
  - Full CRUD operations for items
  - Item images and attachments
  - Purchase date and price tracking
  - Warranty information
  - Custom fields support

- **Organization Features**
  - Categories with hierarchical structure
  - Tags for flexible item organization
  - Organizers (locations/containers)
  - Custom fields with type validation

- **Web Interface**
  - Progressive Web App (PWA) support
  - Mobile-responsive design
  - Offline capability
  - Dark mode support
  - Intuitive dashboard

- **API Features**
  - RESTful API with JWT authentication
  - Rate limiting (100 requests/second)
  - Comprehensive error handling
  - Backup/restore endpoints

- **Infrastructure**
  - Docker & Docker Compose deployment
  - PostgreSQL 17 database
  - Automated database migrations
  - Health check endpoints
  - Auto-generated JWT secrets

### Security
- Argon2id password hashing with secure parameters
- JWT token-based authentication
- Rate limiting middleware
- Non-root container user
- Static-linked Alpine-based image
- HTTPS-ready deployment
- No default credentials in production

### Known Issues
- Recovery codes cannot be retrieved after initial setup (save them immediately)
- No email notification system yet
- Limited search filters in current version
- First-time setup requires manual steps

### Technical Details
- **Backend**: Rust 1.88+ with Actix-Web 4.x
- **Frontend**: React 18 + TypeScript + Vite
- **Database**: PostgreSQL 17 with connection pooling
- **Container**: Alpine Linux with multi-stage builds

---

## Release Links

- [0.1.0-beta.1](https://github.com/VictoryTek/home-registry/releases/tag/v0.1.0-beta.1)

[Unreleased]: https://github.com/VictoryTek/home-registry/compare/v0.1.0-beta.1...HEAD
[0.1.0-beta.1]: https://github.com/VictoryTek/home-registry/releases/tag/v0.1.0-beta.1
