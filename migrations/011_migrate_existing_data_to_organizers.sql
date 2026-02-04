-- Migration: Migrate existing category and location data to organizers
-- This creates organizer types and options from existing item data

-- This migration needs to be run AFTER 010_create_organizers_tables.sql
-- It creates organizer types for existing inventories based on item data

-- Function to migrate existing data
DO $$
DECLARE
    inv_record RECORD;
    cat_organizer_id INTEGER;
    loc_organizer_id INTEGER;
    opt_record RECORD;
    new_option_id INTEGER;
    item_record RECORD;
BEGIN
    -- Loop through each inventory
    FOR inv_record IN SELECT id FROM inventories LOOP
        
        -- Check if any items in this inventory have category data
        IF EXISTS (
            SELECT 1 FROM items 
            WHERE inventory_id = inv_record.id 
            AND category IS NOT NULL 
            AND category != ''
        ) THEN
            -- Create "Category" organizer type for this inventory
            INSERT INTO organizer_types (inventory_id, name, input_type, is_required, display_order)
            VALUES (inv_record.id, 'Category', 'select', false, 1)
            ON CONFLICT (inventory_id, name) DO NOTHING
            RETURNING id INTO cat_organizer_id;
            
            -- If insert happened, migrate options and values
            IF cat_organizer_id IS NOT NULL THEN
                -- Create options from unique category values
                FOR opt_record IN 
                    SELECT DISTINCT category as name 
                    FROM items 
                    WHERE inventory_id = inv_record.id 
                    AND category IS NOT NULL 
                    AND category != ''
                    ORDER BY category
                LOOP
                    INSERT INTO organizer_options (organizer_type_id, name, display_order)
                    VALUES (cat_organizer_id, opt_record.name, 0)
                    ON CONFLICT (organizer_type_id, name) DO NOTHING
                    RETURNING id INTO new_option_id;
                    
                    -- Link items with this category to the new option
                    IF new_option_id IS NOT NULL THEN
                        INSERT INTO item_organizer_values (item_id, organizer_type_id, organizer_option_id)
                        SELECT id, cat_organizer_id, new_option_id
                        FROM items
                        WHERE inventory_id = inv_record.id AND category = opt_record.name
                        ON CONFLICT (item_id, organizer_type_id) DO NOTHING;
                    END IF;
                END LOOP;
            END IF;
        END IF;
        
        -- Check if any items in this inventory have location data
        IF EXISTS (
            SELECT 1 FROM items 
            WHERE inventory_id = inv_record.id 
            AND location IS NOT NULL 
            AND location != ''
        ) THEN
            -- Create "Location" organizer type for this inventory
            INSERT INTO organizer_types (inventory_id, name, input_type, is_required, display_order)
            VALUES (inv_record.id, 'Location', 'select', false, 2)
            ON CONFLICT (inventory_id, name) DO NOTHING
            RETURNING id INTO loc_organizer_id;
            
            -- If insert happened, migrate options and values
            IF loc_organizer_id IS NOT NULL THEN
                -- Create options from unique location values
                FOR opt_record IN 
                    SELECT DISTINCT location as name 
                    FROM items 
                    WHERE inventory_id = inv_record.id 
                    AND location IS NOT NULL 
                    AND location != ''
                    ORDER BY location
                LOOP
                    INSERT INTO organizer_options (organizer_type_id, name, display_order)
                    VALUES (loc_organizer_id, opt_record.name, 0)
                    ON CONFLICT (organizer_type_id, name) DO NOTHING
                    RETURNING id INTO new_option_id;
                    
                    -- Link items with this location to the new option
                    IF new_option_id IS NOT NULL THEN
                        INSERT INTO item_organizer_values (item_id, organizer_type_id, organizer_option_id)
                        SELECT id, loc_organizer_id, new_option_id
                        FROM items
                        WHERE inventory_id = inv_record.id AND location = opt_record.name
                        ON CONFLICT (item_id, organizer_type_id) DO NOTHING;
                    END IF;
                END LOOP;
            END IF;
        END IF;
        
    END LOOP;
    
    RAISE NOTICE 'Migration complete: Existing category and location data migrated to organizers';
END $$;
