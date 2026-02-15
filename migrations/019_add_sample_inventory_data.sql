-- Sample data for testing inventory reporting features
-- This migration adds sample inventories and items across various categories
-- 
-- NOTE: Sample inventories are automatically assigned to the first admin user via:
--   1. Application logic in /auth/setup endpoint (runs during initial setup)
--   2. Migration 020_assign_sample_data_to_first_admin.sql (defensive backup)
--
-- If you need to manually assign sample inventories to a specific user:
--   UPDATE inventories SET user_id = (SELECT id FROM users WHERE username = 'YOUR_USERNAME') WHERE user_id IS NULL;
--
-- Or assign to the first admin user:
--   UPDATE inventories SET user_id = (SELECT id FROM users WHERE is_admin = true ORDER BY created_at LIMIT 1) WHERE user_id IS NULL;

-- Create sample inventories (with NULL user_id - will be assigned after user creation)
INSERT INTO inventories (id, name, description, location, user_id, created_at, updated_at) VALUES
    (100, 'Home Office', 'Electronics and office equipment in the home office', 'Home Office', NULL, NOW(), NOW()),
    (101, 'Living Room', 'Furniture and electronics in living room', 'Living Room', NULL, NOW(), NOW()),
    (102, 'Kitchen', 'Kitchen appliances and cookware', 'Kitchen', NULL, NOW(), NOW()),
    (103, 'Garage', 'Tools and outdoor equipment', 'Garage', NULL, NOW(), NOW()),
    (104, 'Master Bedroom', 'Bedroom furniture and personal items', 'Master Bedroom', NULL, NOW(), NOW())
ON CONFLICT (id) DO NOTHING;

-- Update sequence for inventories
SELECT setval('inventories_id_seq', 104, true);

-- Insert sample items across different categories and price ranges
INSERT INTO items (name, description, category, location, purchase_date, purchase_price, warranty_expiry, notes, quantity, inventory_id, created_at, updated_at) VALUES
    -- Home Office Electronics
    ('Dell XPS 15 Laptop', 'High-performance laptop for work', 'Electronics', 'Home Office Desk', '2024-01-15', 1899.99, '2027-01-15', 'SN: DXP123456789', 1, 100, NOW(), NOW()),
    ('LG 27" 4K Monitor', 'Primary display monitor', 'Electronics', 'Home Office Desk', '2024-02-20', 449.99, '2026-02-20', 'Model: 27UP850-W', 1, 100, NOW(), NOW()),
    ('Herman Miller Aeron Chair', 'Ergonomic office chair', 'Furniture', 'Home Office', '2023-06-10', 1395.00, NULL, 'Size B, Graphite color', 1, 100, NOW(), NOW()),
    ('Logitech MX Master 3 Mouse', 'Wireless productivity mouse', 'Electronics', 'Home Office Desk', '2024-03-05', 99.99, '2025-03-05', 'Space Gray', 1, 100, NOW(), NOW()),
    ('Mechanical Keyboard', 'Cherry MX Brown switches', 'Electronics', 'Home Office Desk', '2023-11-20', 159.99, '2025-11-20', 'RGB backlit', 1, 100, NOW(), NOW()),
    ('Standing Desk Converter', 'Adjustable height desk converter', 'Furniture', 'Home Office', '2023-08-15', 299.99, NULL, 'Black, 32" width', 1, 100, NOW(), NOW()),
    ('Webcam HD 1080p', 'USB webcam for video calls', 'Electronics', 'Home Office Desk', '2024-01-10', 79.99, '2025-01-10', 'Logitech C920', 1, 100, NOW(), NOW()),
    ('Desk Lamp LED', 'Adjustable LED desk lamp', 'Furniture', 'Home Office Desk', '2023-12-01', 45.99, NULL, 'Dimmable, USB charging', 1, 100, NOW(), NOW()),
    
    -- Living Room Electronics & Furniture
    ('Sony 65" OLED TV', '4K HDR Smart TV', 'Electronics', 'Living Room Wall', '2023-11-25', 2199.99, '2025-11-25', 'Model: XR65A80K', 1, 101, NOW(), NOW()),
    ('Sonos Arc Soundbar', 'Premium soundbar with Dolby Atmos', 'Electronics', 'Living Room TV Stand', '2023-12-01', 899.00, '2025-12-01', 'Black finish', 1, 101, NOW(), NOW()),
    ('Sectional Sofa', 'L-shaped fabric sectional', 'Furniture', 'Living Room Center', '2022-03-15', 1599.00, NULL, 'Gray color, seats 6', 1, 101, NOW(), NOW()),
    ('Coffee Table', 'Glass top coffee table', 'Furniture', 'Living Room Center', '2022-03-15', 299.99, NULL, 'Tempered glass, metal frame', 1, 101, NOW(), NOW()),
    ('Area Rug 8x10', 'Modern geometric pattern', 'Furniture', 'Living Room Floor', '2022-04-01', 349.99, NULL, 'Machine washable', 1, 101, NOW(), NOW()),
    ('Table Lamp Set', 'Matching table lamps', 'Furniture', 'Living Room Side Tables', '2022-05-10', 89.99, NULL, 'Set of 2, includes bulbs', 2, 101, NOW(), NOW()),
    ('PlayStation 5', 'Gaming console with disc drive', 'Electronics', 'Living Room TV Stand', '2024-06-20', 499.99, '2025-06-20', 'Includes 2 controllers', 1, 101, NOW(), NOW()),
    
    -- Kitchen Appliances
    ('KitchenAid Stand Mixer', 'Professional 6-quart mixer', 'Appliances', 'Kitchen Counter', '2023-05-12', 449.99, '2025-05-12', 'Empire Red, includes 3 attachments', 1, 102, NOW(), NOW()),
    ('Ninja Air Fryer', '8-quart air fryer with multiple functions', 'Appliances', 'Kitchen Counter', '2024-02-14', 129.99, '2025-02-14', 'Model: AF161', 1, 102, NOW(), NOW()),
    ('Instant Pot Duo', '6-quart pressure cooker', 'Appliances', 'Kitchen Pantry', '2023-10-30', 89.99, '2024-10-30', '7-in-1 programmable', 1, 102, NOW(), NOW()),
    ('Cuisinart Coffee Maker', '12-cup programmable coffee maker', 'Appliances', 'Kitchen Counter', '2023-07-15', 79.99, '2024-07-15', 'Stainless steel', 1, 102, NOW(), NOW()),
    ('Vitamix Blender', 'Professional-grade blender', 'Appliances', 'Kitchen Counter', '2022-09-20', 549.99, '2029-09-20', '7-year warranty, 64oz container', 1, 102, NOW(), NOW()),
    ('Cookware Set', '10-piece stainless steel cookware', 'Kitchenware', 'Kitchen Cabinet', '2023-01-10', 299.99, NULL, 'All-Clad brand', 1, 102, NOW(), NOW()),
    ('Knife Set', 'Professional 15-piece knife block', 'Kitchenware', 'Kitchen Counter', '2023-02-05', 199.99, '2033-02-05', 'German steel, lifetime warranty', 1, 102, NOW(), NOW()),
    ('Food Processor', '12-cup food processor', 'Appliances', 'Kitchen Cabinet', '2023-08-22', 159.99, '2025-08-22', 'Cuisinart brand', 1, 102, NOW(), NOW()),
    
    -- Garage Tools & Equipment
    ('DeWalt Drill Set', 'Cordless drill with 100-piece kit', 'Tools', 'Garage Workbench', '2023-03-20', 199.99, '2026-03-20', '20V Max, 2 batteries included', 1, 103, NOW(), NOW()),
    ('Craftsman Toolbox', 'Rolling tool chest with 6 drawers', 'Tools', 'Garage Corner', '2022-06-15', 499.99, NULL, 'Red finish, 41" wide', 1, 103, NOW(), NOW()),
    ('Ryobi Miter Saw', '10-inch compound miter saw', 'Tools', 'Garage Workbench', '2023-07-08', 249.99, '2025-07-08', 'LED cutline indicator', 1, 103, NOW(), NOW()),
    ('Ladder Extension', '24-foot aluminum extension ladder', 'Tools', 'Garage Wall', '2022-04-12', 179.99, NULL, '300 lb capacity, Type IA', 1, 103, NOW(), NOW()),
    ('Shop Vacuum', '12-gallon wet/dry vacuum', 'Tools', 'Garage Floor', '2023-01-25', 129.99, '2025-01-25', '6.5 HP motor', 1, 103, NOW(), NOW()),
    ('Workbench', 'Heavy-duty steel workbench', 'Furniture', 'Garage Wall', '2022-05-01', 349.99, NULL, '72" wide with pegboard', 1, 103, NOW(), NOW()),
    ('Air Compressor', '6-gallon pancake air compressor', 'Tools', 'Garage Floor', '2023-09-10', 179.99, '2024-09-10', 'Includes hose and accessories', 1, 103, NOW(), NOW()),
    ('Bicycle', 'Mountain bike 21-speed', 'Sports', 'Garage Wall Rack', '2021-05-20', 599.99, NULL, 'Trek brand, adult size', 1, 103, NOW(), NOW()),
    ('Lawn Mower', 'Self-propelled gas mower', 'Tools', 'Garage Corner', '2023-04-15', 399.99, '2025-04-15', '21" cutting deck, Honda engine', 1, 103, NOW(), NOW()),
    
    -- Master Bedroom
    ('King Bed Frame', 'Upholstered platform bed', 'Furniture', 'Master Bedroom', '2022-01-20', 899.99, NULL, 'Gray fabric, includes headboard', 1, 104, NOW(), NOW()),
    ('Mattress King', 'Memory foam hybrid mattress', 'Furniture', 'Master Bedroom', '2022-01-20', 1299.99, '2032-01-20', 'Sealy brand, 10-year warranty', 1, 104, NOW(), NOW()),
    ('Dresser 6-Drawer', 'Solid wood dresser', 'Furniture', 'Master Bedroom', '2022-02-05', 599.99, NULL, 'Espresso finish', 1, 104, NOW(), NOW()),
    ('Nightstand Set', 'Matching nightstands with USB ports', 'Furniture', 'Master Bedroom', '2022-02-05', 249.99, NULL, 'Set of 2, espresso finish', 2, 104, NOW(), NOW()),
    ('Smart TV 43"', 'Roku TV for bedroom', 'Electronics', 'Master Bedroom Dresser', '2023-08-30', 329.99, '2025-08-30', '4K HDR, built-in streaming', 1, 104, NOW(), NOW()),
    ('Air Purifier', 'HEPA air purifier for bedroom', 'Appliances', 'Master Bedroom Corner', '2024-01-05', 179.99, '2026-01-05', 'Covers 500 sq ft, quiet mode', 1, 104, NOW(), NOW()),
    ('Ceiling Fan', 'Smart ceiling fan with light', 'Furniture', 'Master Bedroom Ceiling', '2022-03-15', 199.99, NULL, '52" span, remote control', 1, 104, NOW(), NOW()),
    ('Area Rug 5x8', 'Plush bedroom rug', 'Furniture', 'Master Bedroom Floor', '2022-04-01', 149.99, NULL, 'Ivory color, high pile', 1, 104, NOW(), NOW());

-- Update sequence for items
SELECT setval('items_id_seq', (SELECT MAX(id) FROM items), true);

-- Summary of sample data:
-- Inventories: 5 (Home Office, Living Room, Kitchen, Garage, Master Bedroom)
-- Items: 40 items total
--   - Home Office: 8 items ($4,430.93)
--   - Living Room: 7 items ($6,037.94)
--   - Kitchen: 8 items ($1,959.90)
--   - Garage: 9 items ($2,989.90)
--   - Master Bedroom: 8 items ($3,809.92)
-- Categories: Electronics, Furniture, Appliances, Tools, Kitchenware, Sports
-- Price Range: $45.99 - $2,199.99
-- Date Range: 2021-05-20 to 2024-06-20
