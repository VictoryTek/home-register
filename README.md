<div align="center">
  <img src="logo_full.png" alt="Home Register Logo" width="400">
  <h1>Inventory Management System</h1>
</div>

A modern, web-based home inventory management system built with **Rust + Actix-Web + PostgreSQL**.

I used Homebox for years, then it was no longer maintained. So I thought I would try to create a successor.

## ✨ Features

### 🎨 **Beautiful Modern Web UI**
- **Fixed HTML Escaping Issues** - No more broken quotes in the interface
- **Responsive Design** - Beautiful gradient UI that works on all devices
- **Interactive Dashboard** - Action cards with hover effects
- **Real-time API Integration** - Add items directly from the web interface

### 🔧 **Robust Backend API**
- **RESTful JSON API** - Standard HTTP endpoints for all operations
- **Proper Error Handling** - Structured error responses
- **Request Logging** - Monitor all incoming requests
- **Health Check Endpoint** - Monitor server status

### 🗃️ **Database Ready**
- **PostgreSQL Integration** - Production-ready database setup
- **Connection Pooling** - Efficient database connections
- **Environment Configuration** - Flexible database URL setup

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+ 
- Docker & Docker Compose
- PostgreSQL (if running locally)

### Run with Docker
```bash
# Start the database and application
docker-compose up -d

# View logs
docker-compose logs -f app
```

### Run Locally
```bash
# Install dependencies
cargo build

# Set environment variables
export DATABASE_URL="postgres://postgres:password@localhost:5432/home_inventory"
export RUST_LOG=info

# Start the server
cargo run
```

The server will start at `http://localhost:8000`

## 🔌 API Endpoints

### Core Endpoints
- **GET `/`** - Modern web dashboard
- **GET `/health`** - Server health check
- **GET `/api/items`** - List all inventory items
- **POST `/api/items`** - Create new inventory item

### Sample API Usage

#### Get All Items
```bash
curl http://localhost:8000/api/items
```

#### Create New Item
```bash
curl -X POST http://localhost:8000/api/items \
  -H "Content-Type: application/json" \
  -d '{
    "name": "MacBook Pro",
    "category": "Electronics", 
    "location": "Home Office",
    "description": "Work laptop",
    "purchase_price": 2499.99
  }'
```

## 🏗️ Technical Architecture

### **Frontend**
- **Modern HTML5/CSS3** - Responsive design with flexbox/grid
- **Vanilla JavaScript** - No framework dependencies
- **Interactive UI** - Dynamic forms and API integration

### **Backend**  
- **Rust + Actix-Web 4.x** - High-performance async web framework
- **Structured Logging** - Request logging with env_logger
- **JSON API** - Proper content-type handling and serialization

### **Database**
- **PostgreSQL 16** - Robust relational database
- **Connection Pooling** - deadpool-postgres for efficient connections
- **Environment Config** - Flexible database URL configuration

## 🛠️ Recent Improvements

### ✅ **Fixed Critical Issues**
1. **HTML Escaping Bug** - Fixed broken quotes in web interface
2. **Database Configuration** - Proper URL parsing instead of hardcoded values  
3. **Missing Dependencies** - Added logging, CORS, and file serving support
4. **API Structure** - Added proper JSON responses with error handling

### ✅ **Enhanced Features**
1. **Modern UI Design** - Beautiful gradient design with cards and animations
2. **Interactive Dashboard** - Clickable action cards with JavaScript functionality
3. **API Documentation** - Built-in API endpoint information 
4. **Proper Logging** - Request logging and structured server logs
5. **Error Handling** - Proper HTTP status codes and error messages

## 📂 Project Structure

```
├── src/
│   ├── main.rs          # Server setup & configuration
│   ├── api/mod.rs       # API routes & web interface
│   ├── db/mod.rs        # Database connection & pooling  
│   └── models/mod.rs    # Data models & structures
├── Cargo.toml           # Dependencies & project config
├── docker-compose.yml   # Docker services setup
├── Dockerfile           # Container build instructions
└── README.md           # This file
```

## 🔄 Development Workflow

```bash
# Check code compilation
cargo check

# Run with logging
RUST_LOG=info cargo run

# Build for production
cargo build --release

# Run tests
cargo test
```

## 🚀 Next Steps

- [ ] **Database Migrations** - Add proper schema management
- [ ] **Authentication** - User login and session management  
- [ ] **Search & Filters** - Advanced item searching capabilities
- [ ] **Image Upload** - Photo support for inventory items
- [ ] **Reports** - Export and analytics features
- [ ] **Mobile App** - React Native or Flutter companion

## 📝 License

GNU General Public License v3.0 - see [LICENSE](LICENSE) for details.

---

**Built with ❤️ using Rust + Actix-Web + PostgreSQL**