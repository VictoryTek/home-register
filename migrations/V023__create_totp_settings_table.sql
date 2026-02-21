-- Create TOTP authenticator settings table
-- Supports three modes: 2fa_only, recovery_only, both

CREATE TABLE IF NOT EXISTS user_totp_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,

    -- TOTP secret (encrypted at rest via application-level encryption)
    -- Base32-encoded, typically 32 characters (160 bits of entropy)
    totp_secret_encrypted TEXT NOT NULL,

    -- Mode configuration
    totp_mode VARCHAR(20) NOT NULL DEFAULT 'both'
        CHECK (totp_mode IN ('2fa_only', 'recovery_only', 'both')),

    -- Status
    is_enabled BOOLEAN NOT NULL DEFAULT false,

    -- Setup tracking
    -- is_verified: true after user successfully enters their first TOTP code
    is_verified BOOLEAN NOT NULL DEFAULT false,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ,

    -- Rate limiting tracking
    failed_attempts INT NOT NULL DEFAULT 0,
    last_failed_at TIMESTAMPTZ
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_user_totp_settings_user_id ON user_totp_settings(user_id);
CREATE INDEX IF NOT EXISTS idx_user_totp_settings_is_enabled ON user_totp_settings(is_enabled);
