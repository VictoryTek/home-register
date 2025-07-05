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

-- Insert sample data
INSERT INTO items (name, description, category, location, purchase_date, purchase_price, quantity, inventory_id, notes) VALUES
('MacBook Pro', 'Work laptop, 16-inch', 'Electronics', 'Home Office', '2024-01-15', 2499.99, 1, 1, 'Primary work computer'),
('Kitchen Aid Mixer', 'Stand mixer, red color', 'Appliances', 'Kitchen', '2023-12-10', 299.99, 1, 2, 'Great for baking'),
('Persian Rug', '8x10 feet, handwoven', 'Furniture', 'Living Room', '2023-06-20', 1800.00, 1, 3, 'Beautiful centerpiece'),
('Wireless Headphones', 'Sony WH-1000XM4, noise canceling', 'Electronics', 'Home Office', '2024-03-22', 349.99, 1, 1, 'Excellent sound quality'),
('Coffee Machine', 'Espresso machine with milk frother', 'Appliances', 'Kitchen', '2024-02-05', 599.99, 1, 2, 'Daily use, works perfectly');
