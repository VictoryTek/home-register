-- Migration: Add 'image' as a valid organizer input type
-- This allows organizer types to accept image uploads
-- Image values are stored as URL paths in the existing text_value column

-- Step 1: Drop the existing CHECK constraint on input_type
ALTER TABLE organizer_types DROP CONSTRAINT IF EXISTS organizer_types_input_type_check;

-- Step 2: Add updated CHECK constraint that includes 'image'
ALTER TABLE organizer_types 
ADD CONSTRAINT organizer_types_input_type_check 
CHECK (input_type IN ('select', 'text', 'image'));

-- Note: No changes needed to item_organizer_values table.
-- Image organizer values store the image URL path in the existing text_value column,
-- similar to how 'text' type organizers store their values.
