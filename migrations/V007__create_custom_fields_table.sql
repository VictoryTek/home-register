-- Create custom_fields table for category-specific fields
CREATE TABLE IF NOT EXISTS custom_fields (
    id SERIAL PRIMARY KEY,
    category_id INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    field_type VARCHAR(50) NOT NULL DEFAULT 'text', -- text, number, date, boolean, select
    options TEXT, -- JSON array for select field options
    required BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE,
    UNIQUE(category_id, name)
);

-- Create item_custom_values table for storing custom field values
CREATE TABLE IF NOT EXISTS item_custom_values (
    id SERIAL PRIMARY KEY,
    item_id INTEGER NOT NULL,
    custom_field_id INTEGER NOT NULL,
    value TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE,
    FOREIGN KEY (custom_field_id) REFERENCES custom_fields(id) ON DELETE CASCADE,
    UNIQUE(item_id, custom_field_id)
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_custom_fields_category_id ON custom_fields(category_id);
CREATE INDEX IF NOT EXISTS idx_item_custom_values_item_id ON item_custom_values(item_id);
CREATE INDEX IF NOT EXISTS idx_item_custom_values_field_id ON item_custom_values(custom_field_id);
