use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result, Scope};
use crate::db::DatabaseService;
use crate::models::{ApiResponse, CreateItemRequest, ErrorResponse, UpdateItemRequest, CreateInventoryRequest};
use deadpool_postgres::Pool;
use log::{error, info};

#[get("/")]
pub async fn index() -> impl Responder {
    // Serve the static HTML file instead of embedded HTML
    match std::fs::read_to_string("static/index.html") {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(_) => HttpResponse::InternalServerError().body("Could not load index page")
    }
}

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "Home Inventory Manager is running",
        "timestamp": chrono::Utc::now()
    }))
}

// Items API endpoints
#[get("/api/items")]
                    padding: 1rem 0;
                }

                .nav-section {
                    margin-bottom: 2rem;
                }

                .nav-section-title {
                    padding: 0 1.5rem 0.5rem;
                    font-size: 0.75rem;
                    font-weight: 600;
                    text-transform: uppercase;
                    color: #94a3b8;
                    letter-spacing: 0.05em;
                }

                .nav-item {
                    display: flex;
                    align-items: center;
                    padding: 0.75rem 1.5rem;
                    color: #cbd5e1;
                    text-decoration: none;
                    transition: all 0.2s ease;
                    border-left: 3px solid transparent;
                }

                .nav-item:hover {
                    background: rgba(59, 130, 246, 0.1);
                    color: #3b82f6;
                    border-left-color: #3b82f6;
                }

                .nav-item.active {
                    background: rgba(59, 130, 246, 0.15);
                    color: #3b82f6;
                    border-left-color: #3b82f6;
                }

                .nav-item i {
                    width: 20px;
                    margin-right: 0.75rem;
                }

                /* Main Content */
                .main-content {
                    margin-left: 280px;
                    min-height: 100vh;
                }

                .header {
                    background: white;
                    border-bottom: 1px solid #e2e8f0;
                    padding: 1rem 2rem;
                    position: sticky;
                    top: 0;
                    z-index: 100;
                }

                .header-content {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                }

                .page-title {
                    font-size: 1.5rem;
                    font-weight: 600;
                    color: #1e293b;
                }

                .header-actions {
                    display: flex;
                    gap: 1rem;
                    align-items: center;
                }

                .btn {
                    display: inline-flex;
                    align-items: center;
                    gap: 0.5rem;
                    padding: 0.625rem 1rem;
                    border: none;
                    border-radius: 0.5rem;
                    font-size: 0.875rem;
                    font-weight: 500;
                    cursor: pointer;
                    transition: all 0.2s ease;
                    text-decoration: none;
                }

                .btn-primary {
                    background: #3b82f6;
                    color: white;
                }

                .btn-primary:hover {
                    background: #2563eb;
                    transform: translateY(-1px);
                }

                .btn-secondary {
                    background: #f1f5f9;
                    color: #475569;
                    border: 1px solid #e2e8f0;
                }

                .btn-secondary:hover {
                    background: #e2e8f0;
                }

                /* Content Area */
                .content {
                    padding: 2rem;
                }

                .dashboard-grid {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
                    gap: 1.5rem;
                    margin-bottom: 2rem;
                }

                .stat-card {
                    background: white;
                    border-radius: 1rem;
                    padding: 1.5rem;
                    border: 1px solid #e2e8f0;
                    transition: all 0.2s ease;
                }

                .stat-card:hover {
                    transform: translateY(-2px);
                    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);
                }

                .stat-header {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    margin-bottom: 1rem;
                }

                .stat-title {
                    font-size: 0.875rem;
                    font-weight: 500;
                    color: #64748b;
                }

                .stat-icon {
                    width: 40px;
                    height: 40px;
                    border-radius: 0.5rem;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    font-size: 1.25rem;
                }

                .stat-value {
                    font-size: 2rem;
                    font-weight: 700;
                    color: #1e293b;
                    line-height: 1;
                }

                .stat-change {
                    font-size: 0.75rem;
                    margin-top: 0.5rem;
                }

                .stat-change.positive {
                    color: #059669;
                }

                .recent-section {
                    background: white;
                    border-radius: 1rem;
                    border: 1px solid #e2e8f0;
                    overflow: hidden;
                }

                .section-header {
                    padding: 1.5rem;
                    border-bottom: 1px solid #e2e8f0;
                    display: flex;
                    justify-content: between;
                    align-items: center;
                }

                .section-title {
                    font-size: 1.125rem;
                    font-weight: 600;
                    color: #1e293b;
                }

                .item-list {
                    max-height: 400px;
                    overflow-y: auto;
                }

                .item-row {
                    display: flex;
                    align-items: center;
                    padding: 1rem 1.5rem;
                    border-bottom: 1px solid #f1f5f9;
                    transition: background 0.2s ease;
                }

                .item-row:hover {
                    background: #f8fafc;
                }

                .item-info {
                    flex: 1;
                }

                .item-name {
                    font-weight: 500;
                    color: #1e293b;
                }

                .item-details {
                    font-size: 0.875rem;
                    color: #64748b;
                    margin-top: 0.25rem;
                }

                .item-price {
                    font-weight: 600;
                    color: #059669;
                }

                /* Mobile Responsive */
                @media (max-width: 768px) {
                    .sidebar {
                        transform: translateX(-100%);
                        width: 100%;
                    }

                    .sidebar.open {
                        transform: translateX(0);
                    }

                    .main-content {
                        margin-left: 0;
                    }

                    .mobile-menu-btn {
                        display: block;
                        background: none;
                        border: none;
                        font-size: 1.25rem;
                        color: #475569;
                        cursor: pointer;
                    }

                    .dashboard-grid {
                        grid-template-columns: 1fr;
                    }
                }

                .mobile-menu-btn {
                    display: none;
                }

                /* Loading state */
                .loading {
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    padding: 2rem;
                    color: #64748b;
                }

                .spinner {
                    width: 20px;
                    height: 20px;
                    border: 2px solid #e2e8f0;
                    border-top: 2px solid #3b82f6;
                    border-radius: 50%;
                    animation: spin 1s linear infinite;
                    margin-right: 0.5rem;
                }

                @keyframes spin {
                    0% { transform: rotate(0deg); }
                    100% { transform: rotate(360deg); }
                }
            </style>
        </head>
        <body>
            <!-- Sidebar -->
            <div class="sidebar" id="sidebar">
                <div class="sidebar-header">
                    <a href="/" class="logo">
                        <i class="fas fa-home"></i>
                        <span>Home Manager</span>
                    </a>
                </div>
                
                <nav class="nav-menu">
                    <div class="nav-section">
                        <div class="nav-section-title">Main</div>
                        <a href="#" class="nav-item active" data-page="dashboard">
                            <i class="fas fa-chart-pie"></i>
                            <span>Dashboard</span>
                        </a>
                        <a href="#" class="nav-item" data-page="inventories">
                            <i class="fas fa-warehouse"></i>
                            <span>Inventories</span>
                        </a>
                        <a href="#" class="nav-item" data-page="items">
                            <i class="fas fa-boxes"></i>
                            <span>All Items</span>
                        </a>
                    </div>
                    
                    <div class="nav-section">
                        <div class="nav-section-title">Manage</div>
                        <a href="#" class="nav-item" data-page="categories">
                            <i class="fas fa-tags"></i>
                            <span>Categories</span>
                        </a>
                        <a href="#" class="nav-item" data-page="locations">
                            <i class="fas fa-map-marker-alt"></i>
                            <span>Locations</span>
                        </a>
                        <a href="#" class="nav-item" data-page="reports">
                            <i class="fas fa-chart-bar"></i>
                            <span>Reports</span>
                        </a>
                    </div>

                    <div class="nav-section">
                        <div class="nav-section-title">System</div>
                        <a href="#" class="nav-item" data-page="settings">
                            <i class="fas fa-cog"></i>
                            <span>Settings</span>
                        </a>
                        <a href="/health" class="nav-item">
                            <i class="fas fa-heartbeat"></i>
                            <span>Health Check</span>
                        </a>
                    </div>
                </nav>
            </div>

            <!-- Main Content -->
            <div class="main-content">
                <header class="header">
                    <div class="header-content">
                        <div style="display: flex; align-items: center; gap: 1rem;">
                            <button class="mobile-menu-btn" onclick="toggleSidebar()">
                                <i class="fas fa-bars"></i>
                            </button>
                            <h1 class="page-title" id="page-title">Dashboard</h1>
                        </div>
                        <div class="header-actions">
                            <button class="btn btn-secondary" onclick="showQuickAdd()">
                                <i class="fas fa-plus"></i>
                                Quick Add
                            </button>
                            <button class="btn btn-primary" onclick="showAddInventory()">
                                <i class="fas fa-warehouse"></i>
                                New Inventory
                            </button>
                        </div>
                    </div>
                </header>

                <main class="content" id="main-content">
                    <div class="dashboard-grid">
                        <div class="stat-card">
                            <div class="stat-header">
                                <span class="stat-title">Total Inventories</span>
                                <div class="stat-icon" style="background: linear-gradient(135deg, #3b82f6, #1d4ed8); color: white;">
                                    <i class="fas fa-warehouse"></i>
                                </div>
                            </div>
                            <div class="stat-value" id="total-inventories">-</div>
                            <div class="stat-change positive">
                                <i class="fas fa-arrow-up"></i> Active inventories
                            </div>
                        </div>

                        <div class="stat-card">
                            <div class="stat-header">
                                <span class="stat-title">Total Items</span>
                                <div class="stat-icon" style="background: linear-gradient(135deg, #059669, #047857); color: white;">
                                    <i class="fas fa-boxes"></i>
                                </div>
                            </div>
                            <div class="stat-value" id="total-items">-</div>
                            <div class="stat-change positive">
                                <i class="fas fa-arrow-up"></i> Across all inventories
                            </div>
                        </div>

                        <div class="stat-card">
                            <div class="stat-header">
                                <span class="stat-title">Total Value</span>
                                <div class="stat-icon" style="background: linear-gradient(135deg, #dc2626, #b91c1c); color: white;">
                                    <i class="fas fa-dollar-sign"></i>
                                </div>
                            </div>
                            <div class="stat-value" id="total-value">-</div>
                            <div class="stat-change positive">
                                <i class="fas fa-arrow-up"></i> Total portfolio value
                            </div>
                        </div>
                    </div>

                    <div class="recent-section">
                        <div class="section-header">
                            <h2 class="section-title">Recent Items</h2>
                            <a href="#" class="btn btn-secondary" data-page="items">View All</a>
                        </div>
                        <div class="item-list" id="recent-items">
                            <div class="loading">
                                <div class="spinner"></div>
                                Loading recent items...
                            </div>
                        </div>
                    </div>
                </main>
            </div>

            <script>
                // Global state
                let currentPage = 'dashboard';
                let inventories = [];
                let items = [];

                // Navigation
                function setActivePage(page) {
                    currentPage = page;
                    
                    // Update nav items
                    document.querySelectorAll('.nav-item').forEach(item => {
                        item.classList.remove('active');
                        if (item.dataset.page === page) {
                            item.classList.add('active');
                        }
                    });

                    // Update page title
                    const pageTitle = document.getElementById('page-title');
                    const titles = {
                        dashboard: 'Dashboard',
                        inventories: 'Inventories',
                        items: 'All Items',
                        categories: 'Categories',
                        locations: 'Locations',
                        reports: 'Reports',
                        settings: 'Settings'
                    };
                    pageTitle.textContent = titles[page] || 'Dashboard';

                    // Load page content
                    loadPageContent(page);
                }

                // Add click handlers to nav items
                document.querySelectorAll('.nav-item[data-page]').forEach(item => {
                    item.addEventListener('click', (e) => {
                        e.preventDefault();
                        setActivePage(item.dataset.page);
                    });
                });

                // Mobile sidebar toggle
                function toggleSidebar() {
                    const sidebar = document.getElementById('sidebar');
                    sidebar.classList.toggle('open');
                }

                // API functions
                async function fetchItems() {
                    try {
                        const response = await fetch('/api/items');
                        const data = await response.json();
                        if (data.success) {
                            items = data.data;
                            return items;
                        }
                    } catch (error) {
                        console.error('Error fetching items:', error);
                    }
                    return [];
                }

                // Update dashboard stats
                function updateDashboardStats() {
                    const totalItems = items.length;
                    const totalValue = items.reduce((sum, item) => sum + (item.purchase_price || 0), 0);
                    const inventoryCount = new Set(items.map(item => item.inventory_id || 'default')).size;

                    document.getElementById('total-inventories').textContent = inventoryCount;
                    document.getElementById('total-items').textContent = totalItems;
                    document.getElementById('total-value').textContent = '$' + totalValue.toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 });
                }

                // Display recent items
                function displayRecentItems() {
                    const recentItemsContainer = document.getElementById('recent-items');
                    
                    if (items.length === 0) {
                        recentItemsContainer.innerHTML = `
                            <div style="text-align: center; padding: 2rem; color: #64748b;">
                                <i class="fas fa-box-open" style="font-size: 3rem; margin-bottom: 1rem; opacity: 0.5;"></i>
                                <p>No items found</p>
                                <button class="btn btn-primary" onclick="showQuickAdd()" style="margin-top: 1rem;">
                                    <i class="fas fa-plus"></i> Add your first item
                                </button>
                            </div>
                        `;
                        return;
                    }

                    // Sort by most recent and take first 5
                    const recentItems = items
                        .sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
                        .slice(0, 5);

                    recentItemsContainer.innerHTML = recentItems.map(item => `
                        <div class="item-row">
                            <div class="item-info">
                                <div class="item-name">${item.name}</div>
                                <div class="item-details">${item.category} • ${item.location}</div>
                            </div>
                            <div class="item-price">$${(item.purchase_price || 0).toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 })}</div>
                        </div>
                    `).join('');
                }

                // Load page content
                function loadPageContent(page) {
                    const content = document.getElementById('main-content');
                    
                    switch (page) {
                        case 'dashboard':
                            // Dashboard is already loaded, just refresh data
                            loadDashboardData();
                            break;
                        case 'items':
                            loadItemsPage();
                            break;
                        case 'inventories':
                            loadInventoriesPage();
                            break;
                        default:
                            content.innerHTML = `
                                <div style="text-align: center; padding: 4rem; color: #64748b;">
                                    <i class="fas fa-hammer" style="font-size: 4rem; margin-bottom: 2rem; opacity: 0.5;"></i>
                                    <h2>Coming Soon</h2>
                                    <p>This page is under construction.</p>
                                </div>
                            `;
                    }
                }

                // Load dashboard data
                async function loadDashboardData() {
                    await fetchItems();
                    updateDashboardStats();
                    displayRecentItems();
                }

                // Quick add modal functions
                function showQuickAdd() {
                    // TODO: Implement modal for quick add
                    alert('Quick add feature coming soon!');
                }

                function showAddInventory() {
                    // TODO: Implement modal for adding inventory
                    alert('Add inventory feature coming soon!');
                }

                // Initialize app
                document.addEventListener('DOMContentLoaded', () => {
                    loadDashboardData();
                });
            </script>
        </body>
        </html>
    "#)
                    border-radius: 4px;
                }
                .api-info h3 {
                    color: #1976d2;
                    margin-bottom: 0.5rem;
                }
                .api-endpoints {
                    list-style: none;
                    margin-left: 1rem;
                }
                .api-endpoints li {
                    color: #555;
                    margin-bottom: 0.25rem;
                    font-family: 'Courier New', monospace;
                    font-size: 0.9rem;
                }
                .status-badge {
                    display: inline-block;
                    background: #4caf50;
                    color: white;
                    padding: 0.25rem 0.5rem;
                    border-radius: 12px;
                    font-size: 0.8rem;
                    font-weight: bold;
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>🏠 Home Inventory System</h1>
                <div class="welcome-text">
                    <p>Welcome to your personal home inventory management system!</p>
                    <p>Built with Rust + Actix Web + PostgreSQL</p>
                    <span class="status-badge">Server Running</span>
                </div>
                
                <div class="actions">
                    <div class="action-card" onclick="window.location.href='/api/items'">
                        <div class="action-title">📋 View Items</div>
                        <div class="action-desc">Browse your inventory items</div>
                    </div>
                    <div class="action-card" onclick="addNewItem()">
                        <div class="action-title">➕ Add Item</div>
                        <div class="action-desc">Register a new item</div>
                    </div>
                    <div class="action-card" onclick="window.location.href='/health'">
                        <div class="action-title">❤️ Health Check</div>
                        <div class="action-desc">Check server status</div>
                    </div>
                    <div class="action-card" onclick="toggleApiInfo()">
                        <div class="action-title">🔧 API Info</div>
                        <div class="action-desc">View available endpoints</div>
                    </div>
                </div>

                <div class="api-info" id="apiInfo" style="display: none;">
                    <h3>🔌 Available API Endpoints</h3>
                    <ul class="api-endpoints">
                        <li>GET /health - Server health check</li>
                        <li>GET /api/items - List all inventory items</li>
                        <li>POST /api/items - Create new item</li>
                        <li>GET /api/items/{id} - Get specific item</li>
                        <li>PUT /api/items/{id} - Update item</li>
                        <li>DELETE /api/items/{id} - Delete item</li>
                        <li>GET /api/items/search/{query} - Search items</li>
                        <li>GET / - This dashboard</li>
                    </ul>
                </div>
            </div>

            <script>
                function toggleApiInfo() {
                    const apiInfo = document.getElementById('apiInfo');
                    apiInfo.style.display = apiInfo.style.display === 'none' ? 'block' : 'none';
                }

                function addNewItem() {
                    const name = prompt('Item name:');
                    const category = prompt('Category:');
                    const location = prompt('Location:');
                    
                    if (name && category && location) {
                        fetch('/api/items', {
                            method: 'POST',
                            headers: {
                                'Content-Type': 'application/json',
                            },
                            body: JSON.stringify({
                                name: name,
                                category: category,
                                location: location,
                                description: null,
                                purchase_date: null,
                                purchase_price: null
                            })
                        })
                        .then(response => response.json())
                        .then(data => {
                            if (data.success) {
                                alert('Item added successfully!');
                                window.location.href = '/api/items';
                            } else {
                                alert('Error adding item: ' + (data.message || 'Unknown error'));
                            }
                        })
                        .catch(error => {
                            alert('Error: ' + error.message);
                        });
                    }
                }
            </script>
        </body>
        </html>
    "#)
}

// Get all items
#[get("/api/items")]
pub async fn get_items(pool: web::Data<Pool>) -> Result<impl Responder> {
    let db_service = DatabaseService::new(pool.get_ref().clone());
    
    match db_service.get_all_items().await {
        Ok(items) => {
            info!("Successfully retrieved {} items from database", items.len());
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(items.clone()),
                message: Some(format!("Found {} items", items.len())),
            }))
        }
        Err(e) => {
            error!("Database error retrieving items: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "Failed to retrieve items".to_string(),
                details: Some(e.to_string()),
            }))
        }
    }
}

// Create new item
#[post("/api/items")]
pub async fn create_item(
    pool: web::Data<Pool>,
    item: web::Json<CreateItemRequest>,
) -> Result<impl Responder> {
    let db_service = DatabaseService::new(pool.get_ref().clone());
    let request = item.into_inner();
    
    info!("Creating new item: {}", request.name);
    
    match db_service.create_item(request).await {
        Ok(new_item) => {
            info!("Successfully created item with ID: {:?}", new_item.id);
            Ok(HttpResponse::Created().json(ApiResponse {
                success: true,
                data: Some(new_item),
                message: Some("Item created successfully".to_string()),
            }))
        }
        Err(e) => {
            error!("Database error creating item: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "Failed to create item".to_string(),
                details: Some(e.to_string()),
            }))
        }
    }
}

// Get item by ID
#[get("/api/items/{id}")]
pub async fn get_item(
    pool: web::Data<Pool>, 
    path: web::Path<i32>
) -> Result<impl Responder> {
    let db_service = DatabaseService::new(pool.get_ref().clone());
    let item_id = path.into_inner();
    
    match db_service.get_item_by_id(item_id).await {
        Ok(Some(item)) => {
            info!("Retrieved item ID: {}", item_id);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(item),
                message: None,
            }))
        }
        Ok(None) => {
            info!("Item not found: {}", item_id);
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: "Item not found".to_string(),
                details: Some(format!("No item found with ID: {}", item_id)),
            }))
        }
        Err(e) => {
            error!("Database error retrieving item {}: {}", item_id, e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "Failed to retrieve item".to_string(),
                details: Some(e.to_string()),
            }))
        }
    }
}

// Update item
#[put("/api/items/{id}")]
pub async fn update_item(
    pool: web::Data<Pool>,
    path: web::Path<i32>,
    item: web::Json<UpdateItemRequest>,
) -> Result<impl Responder> {
    let db_service = DatabaseService::new(pool.get_ref().clone());
    let item_id = path.into_inner();
    let request = item.into_inner();
    
    info!("Updating item ID: {}", item_id);
    
    match db_service.update_item(item_id, request).await {
        Ok(Some(updated_item)) => {
            info!("Successfully updated item ID: {}", item_id);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(updated_item),
                message: Some("Item updated successfully".to_string()),
            }))
        }
        Ok(None) => {
            info!("Item not found for update: {}", item_id);
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: "Item not found".to_string(),
                details: Some(format!("No item found with ID: {}", item_id)),
            }))
        }
        Err(e) => {
            error!("Database error updating item {}: {}", item_id, e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "Failed to update item".to_string(),
                details: Some(e.to_string()),
            }))
        }
    }
}

// Delete item
#[delete("/api/items/{id}")]
pub async fn delete_item(
    pool: web::Data<Pool>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    let db_service = DatabaseService::new(pool.get_ref().clone());
    let item_id = path.into_inner();
    
    info!("Deleting item ID: {}", item_id);
    
    match db_service.delete_item(item_id).await {
        Ok(true) => {
            info!("Successfully deleted item ID: {}", item_id);
            Ok(HttpResponse::Ok().json(ApiResponse::<()> {
                success: true,
                data: None,
                message: Some("Item deleted successfully".to_string()),
            }))
        }
        Ok(false) => {
            info!("Item not found for deletion: {}", item_id);
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                success: false,
                error: "Item not found".to_string(),
                details: Some(format!("No item found with ID: {}", item_id)),
            }))
        }
        Err(e) => {
            error!("Database error deleting item {}: {}", item_id, e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "Failed to delete item".to_string(),
                details: Some(e.to_string()),
            }))
        }
    }
}

// Search items
#[get("/api/items/search/{query}")]
pub async fn search_items(
    pool: web::Data<Pool>,
    path: web::Path<String>,
) -> Result<impl Responder> {
    let db_service = DatabaseService::new(pool.get_ref().clone());
    let search_query = path.into_inner();
    
    info!("Searching items with query: {}", search_query);
    
    match db_service.search_items(&search_query).await {
        Ok(items) => {
            info!("Search found {} items for query: {}", items.len(), search_query);
            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(items.clone()),
                message: Some(format!("Found {} items matching '{}'", items.len(), search_query)),
            }))
        }
        Err(e) => {
            error!("Database error searching items: {}", e);
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                success: false,
                error: "Failed to search items".to_string(),
                details: Some(e.to_string()),
            }))
        }
    }
}

pub fn init_routes() -> Scope {
    web::scope("")
        .service(index)
        .service(get_items)
        .service(create_item)
        .service(get_item)
        .service(update_item)
        .service(delete_item)
        .service(search_items)
}
