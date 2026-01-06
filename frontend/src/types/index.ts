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

export type Page = 'inventories' | 'categories' | 'tags' | 'settings' | 'inventory-detail';

export interface ToastMessage {
  id: string;
  message: string;
  type: 'success' | 'error' | 'warning' | 'info';
}
