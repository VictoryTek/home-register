-- Add missing columns to items table
ALTER TABLE items 
ADD COLUMN IF NOT EXISTS warranty_expiry DATE,
ADD COLUMN IF NOT EXISTS notes TEXT,
ADD COLUMN IF NOT EXISTS quantity INTEGER DEFAULT 1,
ADD COLUMN IF NOT EXISTS inventory_id INTEGER DEFAULT 1;

-- Create foreign key constraint for inventory_id
ALTER TABLE items 
ADD CONSTRAINT fk_items_inventory 
FOREIGN KEY (inventory_id) REFERENCES inventories(id) 
ON DELETE SET DEFAULT;

-- Create index for inventory_id
CREATE INDEX IF NOT EXISTS idx_items_inventory_id ON items(inventory_id);
