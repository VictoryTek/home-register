-- Create organizer tables for flexible categorization per inventory
-- Organizers allow users to define custom classification attributes for items

-- Organizer types table: defines custom attributes per inventory
-- input_type: 'select' for predefined options, 'text' for free-form entry
CREATE TABLE IF NOT EXISTS organizer_types (
    id SERIAL PRIMARY KEY,
    inventory_id INTEGER NOT NULL REFERENCES inventories(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    input_type VARCHAR(20) NOT NULL DEFAULT 'select' CHECK (input_type IN ('select', 'text')),
    is_required BOOLEAN NOT NULL DEFAULT false,
    display_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_organizer_name_per_inventory UNIQUE (inventory_id, name)
);

-- Organizer options table: predefined values for 'select' type organizers
CREATE TABLE IF NOT EXISTS organizer_options (
    id SERIAL PRIMARY KEY,
    organizer_type_id INTEGER NOT NULL REFERENCES organizer_types(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    display_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_option_name_per_organizer UNIQUE (organizer_type_id, name)
);

-- Item organizer values table: links items to their organizer values
-- For 'select' type: organizer_option_id is set, text_value is NULL
-- For 'text' type: text_value is set, organizer_option_id is NULL
CREATE TABLE IF NOT EXISTS item_organizer_values (
    id SERIAL PRIMARY KEY,
    item_id INTEGER NOT NULL REFERENCES items(id) ON DELETE CASCADE,
    organizer_type_id INTEGER NOT NULL REFERENCES organizer_types(id) ON DELETE CASCADE,
    organizer_option_id INTEGER REFERENCES organizer_options(id) ON DELETE SET NULL,
    text_value TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- Ensure one value per organizer type per item
    CONSTRAINT unique_item_organizer UNIQUE (item_id, organizer_type_id),
    -- Ensure either option_id OR text_value is set (but not both)
    CONSTRAINT check_value_type CHECK (
        (organizer_option_id IS NOT NULL AND text_value IS NULL) OR
        (organizer_option_id IS NULL AND text_value IS NOT NULL) OR
        (organizer_option_id IS NULL AND text_value IS NULL)
    )
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_organizer_types_inventory_id ON organizer_types(inventory_id);
CREATE INDEX IF NOT EXISTS idx_organizer_types_display_order ON organizer_types(inventory_id, display_order);
CREATE INDEX IF NOT EXISTS idx_organizer_options_type_id ON organizer_options(organizer_type_id);
CREATE INDEX IF NOT EXISTS idx_organizer_options_display_order ON organizer_options(organizer_type_id, display_order);
CREATE INDEX IF NOT EXISTS idx_item_organizer_values_item_id ON item_organizer_values(item_id);
CREATE INDEX IF NOT EXISTS idx_item_organizer_values_type_id ON item_organizer_values(organizer_type_id);
CREATE INDEX IF NOT EXISTS idx_item_organizer_values_option_id ON item_organizer_values(organizer_option_id);
