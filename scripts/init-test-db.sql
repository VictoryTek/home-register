-- Create test database for integration tests
-- This script is automatically executed when the PostgreSQL container initializes
-- It creates the home_inventory_test database if it doesn't exist

-- Check if database exists and create if it doesn't
SELECT 'CREATE DATABASE home_inventory_test'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'home_inventory_test')\gexec

-- Grant appropriate permissions
\c home_inventory_test
GRANT ALL PRIVILEGES ON DATABASE home_inventory_test TO postgres;
