import { createContext, useContext, useState, useEffect, useCallback, ReactNode } from 'react';
import type { User, UserSettings, SetupStatusResponse } from '@/types';
import { authApi } from '@/services/api';

// Storage keys - similar to Humidor
const TOKEN_KEY = 'home_registry_token';
const USER_KEY = 'home_registry_user';

interface AuthContextType {
  user: User | null;
  token: string | null;
  settings: UserSettings | null;
  isLoading: boolean;
  isAuthenticated: boolean;
  needsSetup: boolean | null;
  login: (username: string, password: string) => Promise<{ success: boolean; error?: string }>;
  logout: () => void;
  checkSetupStatus: () => Promise<SetupStatusResponse | null>;
  refreshUser: () => Promise<void>;
  refreshSettings: () => Promise<void>;
  updateSettings: (settings: Partial<UserSettings>) => Promise<boolean>;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [token, setToken] = useState<string | null>(null);
  const [settings, setSettings] = useState<UserSettings | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [needsSetup, setNeedsSetup] = useState<boolean | null>(null);

  // Check for existing auth on mount
  useEffect(() => {
    const initAuth = async () => {
      const storedToken = localStorage.getItem(TOKEN_KEY);
      const storedUser = localStorage.getItem(USER_KEY);

      if (storedToken && storedUser) {
        try {
          const parsedUser = JSON.parse(storedUser);
          setToken(storedToken);
          setUser(parsedUser);
          
          // Verify token is still valid by fetching profile
          const profileResult = await authApi.getProfile(storedToken);
          if (profileResult.success && profileResult.data) {
            setUser(profileResult.data);
            localStorage.setItem(USER_KEY, JSON.stringify(profileResult.data));
            
            // Fetch user settings
            const settingsResult = await authApi.getSettings(storedToken);
            if (settingsResult.success && settingsResult.data) {
              setSettings(settingsResult.data);
            }
          } else {
            // Token invalid, clear auth
            logout();
          }
        } catch (error) {
          console.error('Error restoring auth:', error);
          logout();
        }
      } else {
        // Check if setup is needed
        await checkSetupStatus();
      }
      setIsLoading(false);
    };

    initAuth();
  }, []);

  const checkSetupStatus = useCallback(async (): Promise<SetupStatusResponse | null> => {
    try {
      const result = await authApi.checkSetupStatus();
      if (result.success && result.data) {
        setNeedsSetup(result.data.needs_setup);
        return result.data;
      }
    } catch (error) {
      console.error('Error checking setup status:', error);
    }
    return null;
  }, []);

  const login = useCallback(async (username: string, password: string): Promise<{ success: boolean; error?: string }> => {
    try {
      const result = await authApi.login({ username, password });
      
      if (result.success && result.data) {
        const { token: newToken, user: newUser } = result.data;
        
        // Store auth data
        localStorage.setItem(TOKEN_KEY, newToken);
        localStorage.setItem(USER_KEY, JSON.stringify(newUser));
        
        setToken(newToken);
        setUser(newUser);
        setNeedsSetup(false);
        
        // Fetch user settings
        const settingsResult = await authApi.getSettings(newToken);
        if (settingsResult.success && settingsResult.data) {
          setSettings(settingsResult.data);
        }
        
        return { success: true };
      } else {
        return { success: false, error: result.error || 'Login failed' };
      }
    } catch (error) {
      console.error('Login error:', error);
      return { success: false, error: 'Network error. Please try again.' };
    }
  }, []);

  const logout = useCallback(() => {
    localStorage.removeItem(TOKEN_KEY);
    localStorage.removeItem(USER_KEY);
    setToken(null);
    setUser(null);
    setSettings(null);
  }, []);

  const refreshUser = useCallback(async () => {
    if (!token) return;
    
    try {
      const result = await authApi.getProfile(token);
      if (result.success && result.data) {
        setUser(result.data);
        localStorage.setItem(USER_KEY, JSON.stringify(result.data));
      }
    } catch (error) {
      console.error('Error refreshing user:', error);
    }
  }, [token]);

  const refreshSettings = useCallback(async () => {
    if (!token) return;
    
    try {
      const result = await authApi.getSettings(token);
      if (result.success && result.data) {
        setSettings(result.data);
      }
    } catch (error) {
      console.error('Error refreshing settings:', error);
    }
  }, [token]);

  const updateSettings = useCallback(async (newSettings: Partial<UserSettings>): Promise<boolean> => {
    if (!token) return false;
    
    try {
      const result = await authApi.updateSettings(token, newSettings);
      if (result.success && result.data) {
        setSettings(result.data);
        return true;
      }
    } catch (error) {
      console.error('Error updating settings:', error);
    }
    return false;
  }, [token]);

  return (
    <AuthContext.Provider
      value={{
        user,
        token,
        settings,
        isLoading,
        isAuthenticated: !!token && !!user,
        needsSetup,
        login,
        logout,
        checkSetupStatus,
        refreshUser,
        refreshSettings,
        updateSettings,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
}

// Helper function to get the current token for API calls
export function getAuthToken(): string | null {
  return localStorage.getItem(TOKEN_KEY);
}

// Helper function to get auth headers for fetch calls
export function getAuthHeaders(): Record<string, string> {
  const token = getAuthToken();
  if (token) {
    return {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    };
  }
  return {
    'Content-Type': 'application/json',
  };
}
