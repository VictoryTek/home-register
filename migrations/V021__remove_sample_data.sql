-- Remove sample inventories and items for production release
-- Sample data was useful for initial development but should not exist in production
-- Inventories 100-104 and their associated items are removed
-- This migration is idempotent and safe to run multiple times

-- Remove items belonging to sample inventories
DELETE FROM items WHERE inventory_id BETWEEN 100 AND 104;

-- Remove sample inventories
DELETE FROM inventories WHERE id BETWEEN 100 AND 104;

-- Reset sequences to highest real user data ID (excluding sample range)
-- COALESCE ensures we don't error on empty tables
-- GREATEST ensures sequences never go backward
SELECT setval(
    'inventories_id_seq', 
    GREATEST(
        COALESCE((SELECT MAX(id) FROM inventories WHERE id < 100), 1),
        COALESCE((SELECT last_value FROM inventories_id_seq), 1)
    ), 
    true
);

SELECT setval(
    'items_id_seq', 
    GREATEST(
        COALESCE((SELECT MAX(id) FROM items WHERE EXISTS (
            SELECT 1 FROM inventories WHERE items.inventory_id = inventories.id AND inventories.id < 100
        )), 1),
        COALESCE((SELECT last_value FROM items_id_seq), 1)
    ), 
    true
);

-- Log completion
DO $$ 
BEGIN
    RAISE NOTICE 'Migration 021: Sample data removed successfully';
END $$;
