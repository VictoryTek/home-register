<div align="center">
  <img src="logo_full.png" alt="Home Register Logo" width="400">
  <h1>Home Inventory Management</h1>
</div>

A modern home inventory management system built with **Rust + Actix-Web + PostgreSQL**.

I used and enjoyed Homebox for years, but when the project was archived in 2024 I thought I would try to create a successor.

## ✨ Features

- 📱 Modern web interface
- 🏠 Manage multiple inventories
- 📦 Track items with categories and locations
- 💰 Price and warranty tracking
- 🔍 Search and organize belongings
- 🚀 Fast Rust backend with PostgreSQL

## 🚀 Quick Start

```bash
# Start with Docker (recommended)
docker compose up -d

# Access the application
open http://localhost:8000
```

## � Development

```bash
# Install Rust dependencies
cargo build

# Set database URL
export DATABASE_URL="postgres://homeregister:password@localhost:5432/homeregister"

# Run locally
cargo run
```

## 📝 License

GNU General Public License v3.0 - see [LICENSE](LICENSE) for details.