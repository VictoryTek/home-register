import type { 
  ApiResponse, 
  Inventory, 
  Item, 
  CreateInventoryRequest,
  UpdateInventoryRequest,
  CreateItemRequest, 
  UpdateItemRequest,
  OrganizerTypeWithOptions,
  OrganizerType,
  OrganizerOption,
  CreateOrganizerTypeRequest,
  UpdateOrganizerTypeRequest,
  CreateOrganizerOptionRequest,
  UpdateOrganizerOptionRequest,
  ItemOrganizerValueWithDetails,
  ItemOrganizerValue,
  SetItemOrganizerValuesRequest,
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

  async update(id: number, data: UpdateInventoryRequest): Promise<ApiResponse<Inventory>> {
    const response = await fetch(`${API_BASE}/inventories/${id}`, {
      method: 'PUT',
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

  async getOrganizerValues(itemId: number): Promise<ApiResponse<ItemOrganizerValueWithDetails[]>> {
    const response = await fetch(`${API_BASE}/items/${itemId}/organizer-values`);
    return handleResponse<ItemOrganizerValueWithDetails[]>(response);
  },

  async setOrganizerValues(itemId: number, data: SetItemOrganizerValuesRequest): Promise<ApiResponse<ItemOrganizerValue[]>> {
    const response = await fetch(`${API_BASE}/items/${itemId}/organizer-values`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    });
    return handleResponse<ItemOrganizerValue[]>(response);
  },
};

// Organizer API
export const organizerApi = {
  // Organizer Types
  async getByInventory(inventoryId: number): Promise<ApiResponse<OrganizerTypeWithOptions[]>> {
    const response = await fetch(`${API_BASE}/inventories/${inventoryId}/organizers`);
    return handleResponse<OrganizerTypeWithOptions[]>(response);
  },

  async createType(inventoryId: number, data: CreateOrganizerTypeRequest): Promise<ApiResponse<OrganizerType>> {
    const response = await fetch(`${API_BASE}/inventories/${inventoryId}/organizers`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    });
    return handleResponse<OrganizerType>(response);
  },

  async getType(id: number): Promise<ApiResponse<OrganizerType>> {
    const response = await fetch(`${API_BASE}/organizers/${id}`);
    return handleResponse<OrganizerType>(response);
  },

  async updateType(id: number, data: UpdateOrganizerTypeRequest): Promise<ApiResponse<OrganizerType>> {
    const response = await fetch(`${API_BASE}/organizers/${id}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    });
    return handleResponse<OrganizerType>(response);
  },

  async deleteType(id: number): Promise<ApiResponse<void>> {
    const response = await fetch(`${API_BASE}/organizers/${id}`, {
      method: 'DELETE',
    });
    return handleResponse<void>(response);
  },

  // Organizer Options
  async getOptions(organizerTypeId: number): Promise<ApiResponse<OrganizerOption[]>> {
    const response = await fetch(`${API_BASE}/organizers/${organizerTypeId}/options`);
    return handleResponse<OrganizerOption[]>(response);
  },

  async createOption(organizerTypeId: number, data: CreateOrganizerOptionRequest): Promise<ApiResponse<OrganizerOption>> {
    const response = await fetch(`${API_BASE}/organizers/${organizerTypeId}/options`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    });
    return handleResponse<OrganizerOption>(response);
  },

  async updateOption(optionId: number, data: UpdateOrganizerOptionRequest): Promise<ApiResponse<OrganizerOption>> {
    const response = await fetch(`${API_BASE}/organizer-options/${optionId}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    });
    return handleResponse<OrganizerOption>(response);
  },

  async deleteOption(optionId: number): Promise<ApiResponse<void>> {
    const response = await fetch(`${API_BASE}/organizer-options/${optionId}`, {
      method: 'DELETE',
    });
    return handleResponse<void>(response);
  },
};

// Health check
export async function checkHealth(): Promise<ApiResponse<{ status: string; message: string }>> {
  const response = await fetch(`${API_BASE}/health`);
  return handleResponse<{ status: string; message: string }>(response);
}
