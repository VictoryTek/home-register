-- Fix the purchase_price column type for better Rust compatibility
ALTER TABLE items ALTER COLUMN purchase_price TYPE DOUBLE PRECISION;
