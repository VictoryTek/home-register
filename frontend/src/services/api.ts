import type { 
  ApiResponse, 
  Inventory, 
  Item, 
  CreateInventoryRequest, 
  CreateItemRequest, 
  UpdateItemRequest 
} from '@/types';

const API_BASE = '/api';

async function handleResponse<T>(response: Response): Promise<ApiResponse<T>> {
  const data = await response.json();
  return data as ApiResponse<T>;
}

// Inventory API
export const inventoryApi = {
  async getAll(): Promise<ApiResponse<Inventory[]>> {
    const response = await fetch(`${API_BASE}/inventories`);
    return handleResponse<Inventory[]>(response);
  },

  async getById(id: number): Promise<ApiResponse<Inventory>> {
    const response = await fetch(`${API_BASE}/inventories/${id}`);
    return handleResponse<Inventory>(response);
  },

  async create(data: CreateInventoryRequest): Promise<ApiResponse<Inventory>> {
    const response = await fetch(`${API_BASE}/inventories`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    });
    return handleResponse<Inventory>(response);
  },

  async getItems(inventoryId: number): Promise<ApiResponse<Item[]>> {
    const response = await fetch(`${API_BASE}/inventories/${inventoryId}/items`);
    return handleResponse<Item[]>(response);
  },

  async delete(id: number): Promise<ApiResponse<void>> {
    const response = await fetch(`${API_BASE}/inventories/${id}`, {
      method: 'DELETE',
    });
    return handleResponse<void>(response);
  },
};

// Items API
export const itemApi = {
  async getAll(): Promise<ApiResponse<Item[]>> {
    const response = await fetch(`${API_BASE}/items`);
    return handleResponse<Item[]>(response);
  },

  async getById(id: number): Promise<ApiResponse<Item>> {
    const response = await fetch(`${API_BASE}/items/${id}`);
    return handleResponse<Item>(response);
  },

  async create(data: CreateItemRequest): Promise<ApiResponse<Item>> {
    const response = await fetch(`${API_BASE}/items`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    });
    return handleResponse<Item>(response);
  },

  async update(id: number, data: UpdateItemRequest): Promise<ApiResponse<Item>> {
    const response = await fetch(`${API_BASE}/items/${id}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    });
    return handleResponse<Item>(response);
  },

  async delete(id: number): Promise<ApiResponse<boolean>> {
    const response = await fetch(`${API_BASE}/items/${id}`, {
      method: 'DELETE',
    });
    return handleResponse<boolean>(response);
  },

  async search(query: string): Promise<ApiResponse<Item[]>> {
    const response = await fetch(`${API_BASE}/items/search/${encodeURIComponent(query)}`);
    return handleResponse<Item[]>(response);
  },
};

// Health check
export async function checkHealth(): Promise<ApiResponse<{ status: string; message: string }>> {
  const response = await fetch(`${API_BASE}/health`);
  return handleResponse<{ status: string; message: string }>(response);
}
