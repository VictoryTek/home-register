-- Fix foreign key constraints to use CASCADE delete
-- This ensures that deleting an inventory also deletes all related items

-- Drop old constraint if it exists
ALTER TABLE items DROP CONSTRAINT IF EXISTS fk_items_inventory;
ALTER TABLE items DROP CONSTRAINT IF EXISTS items_inventory_id_fkey;

-- Add new constraint with CASCADE
ALTER TABLE items 
ADD CONSTRAINT items_inventory_id_fkey 
FOREIGN KEY (inventory_id) 
REFERENCES inventories(id) 
ON DELETE CASCADE;
