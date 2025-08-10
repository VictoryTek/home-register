<div align="center">
  <img src="logo_full.png" alt="Home Register Logo" width="400"/>
</div>

# Home Register

A modern, web-based home inventory management system built with **Rust + Actix-Web + PostgreSQL**. Keep track of your belongings with an intuitive interface featuring categories, tags, custom fields, and comprehensive search capabilities.

## Features

- 🎨 **Modern Web Interface** - Beautiful responsive design with dark/light theme support
- � **Inventory Management** - Organize items by categories, locations, and custom tags
- 🗄️ **Database-Driven** - PostgreSQL backend with comprehensive data relationships
- 🏷️ **Flexible Organization** - Categories, tags, and custom fields for any item type
- 🔍 **Advanced Search** - Find items quickly with powerful filtering options
- 📊 **Dashboard Analytics** - Overview of your inventory with statistics and insights

## Quick Start with Docker

To run Home Register on your server, simply use Docker Compose:

```bash
# Clone the repository
git clone https://github.com/VictoryTek/home-register.git
cd home-register

# Start the application
docker-compose up -d

# View application logs
docker-compose logs -f app
```

The application will be available at `http://localhost:8000`

**Default Database Credentials:**
- Database: `home_inventory`
- Username: `postgres`
- Password: `password`

To stop the application:
```bash
docker-compose down
```