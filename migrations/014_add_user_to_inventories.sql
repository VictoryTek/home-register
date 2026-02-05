-- Add user_id to inventories table (nullable initially for existing data)
ALTER TABLE inventories ADD COLUMN IF NOT EXISTS user_id UUID REFERENCES users(id) ON DELETE CASCADE;

-- Create index for user_id lookups
CREATE INDEX IF NOT EXISTS idx_inventories_user_id ON inventories(user_id);

-- Create inventory shares table for sharing inventories between users
CREATE TABLE IF NOT EXISTS inventory_shares (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    inventory_id INT NOT NULL REFERENCES inventories(id) ON DELETE CASCADE,
    shared_with_user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    shared_by_user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    permission_level VARCHAR(20) NOT NULL CHECK (permission_level IN ('view', 'edit', 'full')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_inventory_share UNIQUE (inventory_id, shared_with_user_id)
);

-- Create indexes for inventory shares
CREATE INDEX IF NOT EXISTS idx_inventory_shares_inventory_id ON inventory_shares(inventory_id);
CREATE INDEX IF NOT EXISTS idx_inventory_shares_shared_with ON inventory_shares(shared_with_user_id);
CREATE INDEX IF NOT EXISTS idx_inventory_shares_shared_by ON inventory_shares(shared_by_user_id);
