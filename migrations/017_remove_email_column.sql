-- Remove email column from users table
ALTER TABLE users DROP COLUMN IF EXISTS email;

-- Drop email index if it exists
DROP INDEX IF EXISTS idx_users_email;
