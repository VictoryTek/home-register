-- Create inventories table
CREATE TABLE IF NOT EXISTS inventories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    location VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Add inventory_id to items table
ALTER TABLE items ADD COLUMN IF NOT EXISTS inventory_id INTEGER DEFAULT 1;

-- Insert default inventory for existing items
INSERT INTO inventories (id, name, description) VALUES (1, 'Main Inventory', 'Default inventory for existing items') ON CONFLICT (id) DO NOTHING;

-- Update existing items to use default inventory
UPDATE items SET inventory_id = 1 WHERE inventory_id IS NULL;

-- Make inventory_id NOT NULL and add foreign key
ALTER TABLE items ALTER COLUMN inventory_id SET NOT NULL;
ALTER TABLE items ADD CONSTRAINT IF NOT EXISTS items_inventory_id_fkey FOREIGN KEY (inventory_id) REFERENCES inventories(id);

-- Insert some sample inventories
INSERT INTO inventories (name, description, location) VALUES 
('Home Office', 'Electronics and office equipment', 'Main Office'),
('Kitchen', 'Appliances and kitchenware', 'Kitchen'),
('Living Room', 'Furniture and entertainment', 'Living Room')
ON CONFLICT DO NOTHING;
