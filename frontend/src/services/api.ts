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
  // Auth types
  User,
  UserSettings,
  SetupStatusResponse,
  LoginRequest,
  LoginResponse,
  InitialSetupRequest,
  UpdateProfileRequest,
  ChangePasswordRequest,
  UpdateUserSettingsRequest,
  CreateUserRequest,
  UpdateUserRequest,
} from '@/types';

const API_BASE = '/api';
const TOKEN_KEY = 'home_registry_token';

// Get auth token from localStorage
function getToken(): string | null {
  return localStorage.getItem(TOKEN_KEY);
}

// Get headers with optional auth token
function getHeaders(includeAuth = true): Record<string, string> {
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
  };
  
  if (includeAuth) {
    const token = getToken();
    if (token) {
      headers['Authorization'] = `Bearer ${token}`;
    }
  }
  
  return headers;
}

async function handleResponse<T>(response: Response): Promise<ApiResponse<T>> {
  // Handle 401 Unauthorized - redirect to login
  if (response.status === 401) {
    localStorage.removeItem(TOKEN_KEY);
    localStorage.removeItem('home_registry_user');
    // Only redirect if not already on login/setup page
    if (!window.location.pathname.includes('/login') && !window.location.pathname.includes('/setup')) {
      window.location.href = '/login';
    }
  }
  
  // Check if response is JSON
  const contentType = response.headers.get('content-type');
  if (!contentType || !contentType.includes('application/json')) {
    // Not JSON - probably an HTML error page
    const text = await response.text();
    console.error('Received non-JSON response:', text.substring(0, 200));
    return {
      success: false,
      error: `Server error (${response.status}): Expected JSON but received ${contentType || 'unknown content type'}`,
      data: undefined as any,
    };
  }
  
  try {
    const data = await response.json();
    return data as ApiResponse<T>;
  } catch (error) {
    console.error('Failed to parse JSON response:', error);
    return {
      success: false,
      error: 'Invalid JSON response from server',
      data: undefined as any,
    };
  }
}

// Inventory API
export const inventoryApi = {
  async getAll(): Promise<ApiResponse<Inventory[]>> {
    const response = await fetch(`${API_BASE}/inventories`, {
      headers: getHeaders(),
    });
    return handleResponse<Inventory[]>(response);
  },

  async getById(id: number): Promise<ApiResponse<Inventory>> {
    const response = await fetch(`${API_BASE}/inventories/${id}`, {
      headers: getHeaders(),
    });
    return handleResponse<Inventory>(response);
  },

  async create(data: CreateInventoryRequest): Promise<ApiResponse<Inventory>> {
    const response = await fetch(`${API_BASE}/inventories`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse<Inventory>(response);
  },

  async update(id: number, data: UpdateInventoryRequest): Promise<ApiResponse<Inventory>> {
    const response = await fetch(`${API_BASE}/inventories/${id}`, {
      method: 'PUT',
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse<Inventory>(response);
  },

  async getItems(inventoryId: number): Promise<ApiResponse<Item[]>> {
    const response = await fetch(`${API_BASE}/inventories/${inventoryId}/items`, {
      headers: getHeaders(),
    });
    return handleResponse<Item[]>(response);
  },

  async delete(id: number): Promise<ApiResponse<void>> {
    const response = await fetch(`${API_BASE}/inventories/${id}`, {
      method: 'DELETE',
      headers: getHeaders(),
    });
    return handleResponse<void>(response);
  },
};

// Items API
export const itemApi = {
  async getAll(): Promise<ApiResponse<Item[]>> {
    const response = await fetch(`${API_BASE}/items`, {
      headers: getHeaders(),
    });
    return handleResponse<Item[]>(response);
  },

  async getById(id: number): Promise<ApiResponse<Item>> {
    const response = await fetch(`${API_BASE}/items/${id}`, {
      headers: getHeaders(),
    });
    return handleResponse<Item>(response);
  },

  async create(data: CreateItemRequest): Promise<ApiResponse<Item>> {
    const response = await fetch(`${API_BASE}/items`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse<Item>(response);
  },

  async update(id: number, data: UpdateItemRequest): Promise<ApiResponse<Item>> {
    const response = await fetch(`${API_BASE}/items/${id}`, {
      method: 'PUT',
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse<Item>(response);
  },

  async delete(id: number): Promise<ApiResponse<boolean>> {
    const response = await fetch(`${API_BASE}/items/${id}`, {
      method: 'DELETE',
      headers: getHeaders(),
    });
    return handleResponse<boolean>(response);
  },

  async search(query: string): Promise<ApiResponse<Item[]>> {
    const response = await fetch(`${API_BASE}/items/search/${encodeURIComponent(query)}`, {
      headers: getHeaders(),
    });
    return handleResponse<Item[]>(response);
  },

  async getOrganizerValues(itemId: number): Promise<ApiResponse<ItemOrganizerValueWithDetails[]>> {
    const response = await fetch(`${API_BASE}/items/${itemId}/organizer-values`, {
      headers: getHeaders(),
    });
    return handleResponse<ItemOrganizerValueWithDetails[]>(response);
  },

  async setOrganizerValues(itemId: number, data: SetItemOrganizerValuesRequest): Promise<ApiResponse<ItemOrganizerValue[]>> {
    const response = await fetch(`${API_BASE}/items/${itemId}/organizer-values`, {
      method: 'PUT',
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse<ItemOrganizerValue[]>(response);
  },
};

// Organizer API
export const organizerApi = {
  // Organizer Types
  async getByInventory(inventoryId: number): Promise<ApiResponse<OrganizerTypeWithOptions[]>> {
    const response = await fetch(`${API_BASE}/inventories/${inventoryId}/organizers`, {
      headers: getHeaders(),
    });
    return handleResponse<OrganizerTypeWithOptions[]>(response);
  },

  async createType(inventoryId: number, data: CreateOrganizerTypeRequest): Promise<ApiResponse<OrganizerType>> {
    const response = await fetch(`${API_BASE}/inventories/${inventoryId}/organizers`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse<OrganizerType>(response);
  },

  async getType(id: number): Promise<ApiResponse<OrganizerType>> {
    const response = await fetch(`${API_BASE}/organizers/${id}`, {
      headers: getHeaders(),
    });
    return handleResponse<OrganizerType>(response);
  },

  async updateType(id: number, data: UpdateOrganizerTypeRequest): Promise<ApiResponse<OrganizerType>> {
    const response = await fetch(`${API_BASE}/organizers/${id}`, {
      method: 'PUT',
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse<OrganizerType>(response);
  },

  async deleteType(id: number): Promise<ApiResponse<void>> {
    const response = await fetch(`${API_BASE}/organizers/${id}`, {
      method: 'DELETE',
      headers: getHeaders(),
    });
    return handleResponse<void>(response);
  },

  // Organizer Options
  async getOptions(organizerTypeId: number): Promise<ApiResponse<OrganizerOption[]>> {
    const response = await fetch(`${API_BASE}/organizers/${organizerTypeId}/options`, {
      headers: getHeaders(),
    });
    return handleResponse<OrganizerOption[]>(response);
  },

  async createOption(organizerTypeId: number, data: CreateOrganizerOptionRequest): Promise<ApiResponse<OrganizerOption>> {
    const response = await fetch(`${API_BASE}/organizers/${organizerTypeId}/options`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse<OrganizerOption>(response);
  },

  async updateOption(optionId: number, data: UpdateOrganizerOptionRequest): Promise<ApiResponse<OrganizerOption>> {
    const response = await fetch(`${API_BASE}/organizer-options/${optionId}`, {
      method: 'PUT',
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse<OrganizerOption>(response);
  },

  async deleteOption(optionId: number): Promise<ApiResponse<void>> {
    const response = await fetch(`${API_BASE}/organizer-options/${optionId}`, {
      method: 'DELETE',
      headers: getHeaders(),
    });
    return handleResponse<void>(response);
  },
};

// Health check
export async function checkHealth(): Promise<ApiResponse<{ status: string; message: string }>> {
  const response = await fetch(`${API_BASE}/health`);
  return handleResponse<{ status: string; message: string }>(response);
}

// ==================== Authentication API ====================

export const authApi = {
  // Check if initial setup is needed
  async checkSetupStatus(): Promise<ApiResponse<SetupStatusResponse>> {
    const response = await fetch(`${API_BASE}/auth/setup/status`);
    return handleResponse<SetupStatusResponse>(response);
  },

  // Initial setup - create first admin user
  async setup(data: InitialSetupRequest): Promise<ApiResponse<LoginResponse>> {
    const response = await fetch(`${API_BASE}/auth/setup`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    });
    return handleResponse<LoginResponse>(response);
  },

  // Login
  async login(data: LoginRequest): Promise<ApiResponse<LoginResponse>> {
    const response = await fetch(`${API_BASE}/auth/login`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    });
    return handleResponse<LoginResponse>(response);
  },

  // Register new user (after initial setup)
  async register(data: { username: string; email: string; full_name: string; password: string }): Promise<ApiResponse<LoginResponse>> {
    const response = await fetch(`${API_BASE}/auth/register`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    });
    return handleResponse<LoginResponse>(response);
  },

  // Get current user profile
  async getProfile(token?: string): Promise<ApiResponse<User>> {
    const headers: Record<string, string> = { 'Content-Type': 'application/json' };
    if (token) {
      headers['Authorization'] = `Bearer ${token}`;
    } else {
      const storedToken = getToken();
      if (storedToken) {
        headers['Authorization'] = `Bearer ${storedToken}`;
      }
    }
    
    const response = await fetch(`${API_BASE}/auth/me`, {
      headers,
    });
    return handleResponse<User>(response);
  },

  // Update current user profile
  async updateProfile(data: UpdateProfileRequest): Promise<ApiResponse<User>> {
    const response = await fetch(`${API_BASE}/auth/me`, {
      method: 'PUT',
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse<User>(response);
  },

  // Change password
  async changePassword(data: ChangePasswordRequest): Promise<ApiResponse<{ message: string }>> {
    const response = await fetch(`${API_BASE}/auth/password`, {
      method: 'PUT',
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse<{ message: string }>(response);
  },

  // Get user settings
  async getSettings(token?: string): Promise<ApiResponse<UserSettings>> {
    const headers: Record<string, string> = { 'Content-Type': 'application/json' };
    if (token) {
      headers['Authorization'] = `Bearer ${token}`;
    } else {
      const storedToken = getToken();
      if (storedToken) {
        headers['Authorization'] = `Bearer ${storedToken}`;
      }
    }
    
    const response = await fetch(`${API_BASE}/auth/settings`, {
      headers,
    });
    return handleResponse<UserSettings>(response);
  },

  // Update user settings
  async updateSettings(token: string, data: Partial<UpdateUserSettingsRequest>): Promise<ApiResponse<UserSettings>> {
    const response = await fetch(`${API_BASE}/auth/settings`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${token}`,
      },
      body: JSON.stringify(data),
    });
    return handleResponse<UserSettings>(response);
  },

  // Admin: Get all users
  async getAllUsers(): Promise<ApiResponse<User[]>> {
    const response = await fetch(`${API_BASE}/admin/users`, {
      headers: getHeaders(),
    });
    return handleResponse<User[]>(response);
  },

  // Admin: Get specific user
  async getUser(userId: string): Promise<ApiResponse<User>> {
    const response = await fetch(`${API_BASE}/admin/users/${userId}`, {
      headers: getHeaders(),
    });
    return handleResponse<User>(response);
  },

  // Admin: Create new user
  async createUser(data: CreateUserRequest): Promise<ApiResponse<User>> {
    const response = await fetch(`${API_BASE}/admin/users`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse<User>(response);
  },

  // Admin: Update user
  async updateUser(userId: string, data: UpdateUserRequest): Promise<ApiResponse<User>> {
    const response = await fetch(`${API_BASE}/admin/users/${userId}`, {
      method: 'PUT',
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    return handleResponse<User>(response);
  },

  // Admin: Delete user
  async deleteUser(userId: string): Promise<ApiResponse<{ message: string }>> {
    const response = await fetch(`${API_BASE}/admin/users/${userId}`, {
      method: 'DELETE',
      headers: getHeaders(),
    });
    return handleResponse<{ message: string }>(response);
  },
};
