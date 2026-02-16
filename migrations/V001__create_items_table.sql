-- Create the items table for home inventory
CREATE TABLE IF NOT EXISTS items (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    category VARCHAR(100),
    location VARCHAR(100),
    purchase_date DATE,
    purchase_price DECIMAL(10, 2),
    warranty_expiry DATE,
    notes TEXT,
    quantity INTEGER DEFAULT 1,
    inventory_id INTEGER DEFAULT 1,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create index for common queries
CREATE INDEX IF NOT EXISTS idx_items_category ON items(category);
CREATE INDEX IF NOT EXISTS idx_items_location ON items(location);
CREATE INDEX IF NOT EXISTS idx_items_name ON items(name);
