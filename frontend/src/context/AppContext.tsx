import { createContext, useContext, useState, useEffect, useCallback, ReactNode } from 'react';
import type { Theme, ToastMessage, Inventory, Item } from '@/types';
import { checkWarrantyNotifications, type WarrantyNotification } from '@/utils/notifications';
import { useAuth } from '@/context/AuthContext';

interface AppContextType {
  theme: Theme;
  toggleTheme: () => void;
  toasts: ToastMessage[];
  showToast: (message: string, type: ToastMessage['type']) => void;
  removeToast: (id: string) => void;
  currentInventoryId: number | null;
  setCurrentInventoryId: (id: number | null) => void;
  inventories: Inventory[];
  setInventories: (inventories: Inventory[]) => void;
  items: Item[];
  setItems: (items: Item[]) => void;
  warrantyNotifications: WarrantyNotification[];
  checkNotifications: () => void;
  sidebarOpen: boolean;
  toggleSidebar: () => void;
  closeSidebar: () => void;
}

const AppContext = createContext<AppContextType | undefined>(undefined);

export function AppProvider({ children }: { children: ReactNode }) {
  // CRITICAL FIX: Import useAuth to access dismissed warranties
  const { getDismissedWarranties } = useAuth();

  const [theme, setTheme] = useState<Theme>(() => {
    const saved = localStorage.getItem('theme');
    return saved ? (saved as Theme) : 'light';
  });

  const [toasts, setToasts] = useState<ToastMessage[]>([]);
  const [currentInventoryId, setCurrentInventoryId] = useState<number | null>(null);
  const [inventories, setInventories] = useState<Inventory[]>([]);
  const [items, setItems] = useState<Item[]>([]);
  const [warrantyNotifications, setWarrantyNotifications] = useState<WarrantyNotification[]>([]);
  const [sidebarOpen, setSidebarOpen] = useState(false);

  useEffect(() => {
    document.body.setAttribute('data-theme', theme);
    localStorage.setItem('theme', theme);
  }, [theme]);

  // Wrap toggleTheme in useCallback to create stable reference (prevent unnecessary re-renders)
  const toggleTheme = useCallback(() => {
    setTheme((prev) => (prev === 'light' ? 'dark' : 'light'));
  }, []);

  // CRITICAL FIX: Wrap showToast in useCallback to prevent infinite loop in components
  // that depend on this function. Without this, every context re-render creates a new
  // showToast reference, causing dependent useCallback hooks to recreate, triggering
  // useEffect hooks, leading to infinite API request loops.
  const showToast = useCallback((message: string, type: ToastMessage['type']) => {
    const id = Date.now().toString();
    setToasts((prev) => [...prev, { id, message, type }]);

    setTimeout(() => {
      removeToast(id);
    }, 3000);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []); // Empty deps - function is now stable across re-renders

  // Wrap removeToast in useCallback for consistency and to prevent unnecessary re-renders
  const removeToast = useCallback((id: string) => {
    setToasts((prev) => prev.filter((toast) => toast.id !== id));
  }, []);

  const toggleSidebar = useCallback(() => {
    setSidebarOpen((prev) => !prev);
  }, []);

  const closeSidebar = useCallback(() => {
    setSidebarOpen(false);
  }, []);

  const checkNotifications = useCallback(() => {
    // CRITICAL FIX: Pass dismissed warranties to filter at the source
    // This ensures all downstream components get pre-filtered notifications
    const dismissedWarranties = getDismissedWarranties();
    const notifications = checkWarrantyNotifications(items, dismissedWarranties);
    setWarrantyNotifications(notifications);
  }, [items, getDismissedWarranties]);

  // Auto-check notifications when items change
  useEffect(() => {
    checkNotifications();
  }, [checkNotifications]);

  return (
    <AppContext.Provider
      value={{
        theme,
        toggleTheme,
        toasts,
        showToast,
        removeToast,
        currentInventoryId,
        setCurrentInventoryId,
        inventories,
        setInventories,
        items,
        setItems,
        warrantyNotifications,
        checkNotifications,
        sidebarOpen,
        toggleSidebar,
        closeSidebar,
      }}
    >
      {children}
    </AppContext.Provider>
  );
}

// eslint-disable-next-line react-refresh/only-export-components
export function useApp() {
  const context = useContext(AppContext);
  if (context === undefined) {
    throw new Error('useApp must be used within an AppProvider');
  }
  return context;
}
