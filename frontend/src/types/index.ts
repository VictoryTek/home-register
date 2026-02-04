// API Response types
export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  message?: string;
  error?: string;
}

export interface ErrorResponse {
  success: false;
  error: string;
  message?: string;
}

// Core domain types
export interface Inventory {
  id?: number;
  name: string;
  description?: string;
  location?: string;
  image_url?: string;
  created_at?: string;
  updated_at?: string;
}

export interface Item {
  id?: number;
  inventory_id: number;
  name: string;
  description?: string;
  category?: string;
  location?: string;
  purchase_date?: string;
  purchase_price?: number;
  warranty_expiry?: string;
  notes?: string;
  quantity?: number;
  created_at?: string;
  updated_at?: string;
}

export interface Category {
  id?: number;
  name: string;
  description?: string;
  color?: string;
  icon?: string;
  customFields?: CustomField[];
  created_at?: string;
  updated_at?: string;
}

export interface Tag {
  id?: number;
  name: string;
  description?: string;
  color?: string;
  categoryId?: number;
  created_at?: string;
  updated_at?: string;
}

export interface CustomField {
  id: string;
  name: string;
  type: 'text' | 'number' | 'date' | 'textarea' | 'select' | 'checkbox';
  required?: boolean;
  options?: string[];
}

export interface CustomFieldValue {
  id?: number;
  item_id: number;
  custom_field_id: number;
  value?: string;
  created_at?: string;
  updated_at?: string;
}

// Request types
export interface CreateInventoryRequest {
  name: string;
  description?: string;
  location?: string;
  image_url?: string;
}

export interface UpdateInventoryRequest {
  name?: string;
  description?: string;
  location?: string;
  image_url?: string;
}

export interface CreateItemRequest {
  inventory_id?: number;
  name: string;
  description?: string;
  category?: string;
  location?: string;
  purchase_date?: string;
  purchase_price?: number;
  warranty_expiry?: string;
  notes?: string;
  quantity?: number;
}

export interface UpdateItemRequest {
  name?: string;
  description?: string;
  category?: string;
  location?: string;
  purchase_date?: string;
  purchase_price?: number;
  warranty_expiry?: string;
  notes?: string;
  quantity?: number;
  inventory_id?: number;
}

// UI state types
export type Theme = 'light' | 'dark';

export type Page = 'inventories' | 'categories' | 'tags' | 'settings' | 'inventory-detail' | 'organizers';

export interface ToastMessage {
  id: string;
  message: string;
  type: 'success' | 'error' | 'warning' | 'info';
}

// Organizer types
export interface OrganizerType {
  id?: number;
  inventory_id: number;
  name: string;
  input_type: 'select' | 'text';
  is_required: boolean;
  display_order: number;
  created_at?: string;
  updated_at?: string;
}

export interface OrganizerOption {
  id?: number;
  organizer_type_id: number;
  name: string;
  display_order: number;
  created_at?: string;
  updated_at?: string;
}

export interface OrganizerTypeWithOptions extends OrganizerType {
  options: OrganizerOption[];
}

export interface ItemOrganizerValue {
  id?: number;
  item_id: number;
  organizer_type_id: number;
  organizer_option_id?: number;
  text_value?: string;
  created_at?: string;
  updated_at?: string;
}

export interface ItemOrganizerValueWithDetails {
  organizer_type_id: number;
  organizer_type_name: string;
  input_type: 'select' | 'text';
  is_required: boolean;
  value?: string;
  organizer_option_id?: number;
  text_value?: string;
}

// Organizer request types
export interface CreateOrganizerTypeRequest {
  name: string;
  input_type?: 'select' | 'text';
  is_required?: boolean;
  display_order?: number;
}

export interface UpdateOrganizerTypeRequest {
  name?: string;
  input_type?: 'select' | 'text';
  is_required?: boolean;
  display_order?: number;
}

export interface CreateOrganizerOptionRequest {
  name: string;
  display_order?: number;
}

export interface UpdateOrganizerOptionRequest {
  name?: string;
  display_order?: number;
}

export interface SetItemOrganizerValueRequest {
  organizer_type_id: number;
  organizer_option_id?: number;
  text_value?: string;
}

export interface SetItemOrganizerValuesRequest {
  values: SetItemOrganizerValueRequest[];
}
