-- Update inventory sharing to use 4-tier permission system
-- Permission levels (per-inventory):
--   view: View shared inventory and its items
--   edit_items: View + Edit item details (not add/remove)
--   edit_inventory: Edit Items + Edit inventory details, add/remove items
-- 
-- For "All Access" (user-to-user): see user_access_grants table below

-- First, migrate existing permission values
-- 'view' stays 'view'
-- 'edit' becomes 'edit_items'
-- 'full' becomes 'edit_inventory'
UPDATE inventory_shares SET permission_level = 'edit_items' WHERE permission_level = 'edit';
UPDATE inventory_shares SET permission_level = 'edit_inventory' WHERE permission_level = 'full';

-- Drop and recreate the constraint with new values
ALTER TABLE inventory_shares DROP CONSTRAINT IF EXISTS inventory_shares_permission_level_check;
ALTER TABLE inventory_shares ADD CONSTRAINT inventory_shares_permission_level_check 
    CHECK (permission_level IN ('view', 'edit_items', 'edit_inventory'));

-- Create user access grants table for "All Access" relationships
-- This grants a user full access to ALL inventories of another user (like a shared account)
CREATE TABLE IF NOT EXISTS user_access_grants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    grantor_user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    grantee_user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_user_access_grant UNIQUE (grantor_user_id, grantee_user_id),
    CONSTRAINT no_self_grant CHECK (grantor_user_id != grantee_user_id)
);

-- Create indexes for user access grants
CREATE INDEX IF NOT EXISTS idx_user_access_grants_grantor ON user_access_grants(grantor_user_id);
CREATE INDEX IF NOT EXISTS idx_user_access_grants_grantee ON user_access_grants(grantee_user_id);

-- Add comment to explain the table
COMMENT ON TABLE user_access_grants IS 'Grants a grantee full access to all inventories owned by the grantor (All Access tier)';
COMMENT ON COLUMN user_access_grants.grantor_user_id IS 'User granting access to their inventories';
COMMENT ON COLUMN user_access_grants.grantee_user_id IS 'User receiving access to all grantor inventories';
