-- Create user settings table for user preferences
CREATE TABLE IF NOT EXISTS user_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    theme VARCHAR(20) DEFAULT 'light',
    default_inventory_id INT REFERENCES inventories(id) ON DELETE SET NULL,
    items_per_page INT DEFAULT 25,
    date_format VARCHAR(20) DEFAULT 'YYYY-MM-DD',
    currency VARCHAR(10) DEFAULT 'USD',
    notifications_enabled BOOLEAN DEFAULT true,
    settings_json JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index for user_id lookups
CREATE INDEX IF NOT EXISTS idx_user_settings_user_id ON user_settings(user_id);
