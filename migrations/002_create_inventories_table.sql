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

-- Update existing items to use default inventory
UPDATE items SET inventory_id = 1 WHERE inventory_id IS NULL;

-- Make inventory_id NOT NULL
ALTER TABLE items ALTER COLUMN inventory_id SET NOT NULL;

-- Add foreign key constraint (using DO block to check if exists)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'items_inventory_id_fkey'
    ) THEN
        ALTER TABLE items ADD CONSTRAINT items_inventory_id_fkey 
        FOREIGN KEY (inventory_id) REFERENCES inventories(id);
    END IF;
END $$;
