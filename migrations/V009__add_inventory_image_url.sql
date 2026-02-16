-- Add image_url column to inventories table
ALTER TABLE inventories ADD COLUMN IF NOT EXISTS image_url TEXT;
