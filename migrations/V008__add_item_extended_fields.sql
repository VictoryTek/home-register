-- Add additional fields to items table for extended data
ALTER TABLE items ADD COLUMN IF NOT EXISTS image_url TEXT;
ALTER TABLE items ADD COLUMN IF NOT EXISTS purchase_link TEXT;
ALTER TABLE items ADD COLUMN IF NOT EXISTS warranty_info TEXT;
ALTER TABLE items ADD COLUMN IF NOT EXISTS condition VARCHAR(50); -- new, used, refurbished, etc.
ALTER TABLE items ADD COLUMN IF NOT EXISTS serial_number VARCHAR(255);
ALTER TABLE items ADD COLUMN IF NOT EXISTS manufacturer VARCHAR(255);
ALTER TABLE items ADD COLUMN IF NOT EXISTS model VARCHAR(255);

-- Create index for serial number lookups
CREATE INDEX IF NOT EXISTS idx_items_serial_number ON items(serial_number) WHERE serial_number IS NOT NULL;
