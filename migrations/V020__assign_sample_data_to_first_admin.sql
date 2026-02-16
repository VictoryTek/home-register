-- Auto-assign sample inventories (with NULL user_id) to the first admin user
-- This migration is idempotent and safe to run multiple times
-- It only assigns inventories that still have NULL user_id when an admin exists
--
-- This serves as a defensive backup to the application-level assignment in initial_setup()
-- Catches cases where:
-- - Admin was created via direct SQL instead of /auth/setup endpoint
-- - Application-level assignment failed for any reason
-- - Database was restored from backup before assignment ran

UPDATE inventories 
SET user_id = (
    SELECT id 
    FROM users 
    WHERE is_admin = true 
    ORDER BY created_at 
    LIMIT 1
),
updated_at = NOW()
WHERE user_id IS NULL 
  AND EXISTS (SELECT 1 FROM users WHERE is_admin = true);

-- Log result for audit trail
DO $$ 
DECLARE 
    assigned_count INT;
BEGIN
    GET DIAGNOSTICS assigned_count = ROW_COUNT;
    IF assigned_count > 0 THEN
        RAISE NOTICE 'Migration 020: Assigned % sample inventories to first admin user', assigned_count;
    ELSE
        RAISE NOTICE 'Migration 020: No sample inventories needed assignment (already assigned or no admin exists)';
    END IF;
END $$;
