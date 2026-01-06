import { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import type { Theme, ToastMessage, Inventory, Item } from '@/types';

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
}

const AppContext = createContext<AppContextType | undefined>(undefined);

export function AppProvider({ children }: { children: ReactNode }) {
  const [theme, setTheme] = useState<Theme>(() => {
    const saved = localStorage.getItem('theme');
    return (saved as Theme) || 'light';
  });
  
  const [toasts, setToasts] = useState<ToastMessage[]>([]);
  const [currentInventoryId, setCurrentInventoryId] = useState<number | null>(null);
  const [inventories, setInventories] = useState<Inventory[]>([]);
  const [items, setItems] = useState<Item[]>([]);

  useEffect(() => {
    document.body.setAttribute('data-theme', theme);
    localStorage.setItem('theme', theme);
  }, [theme]);

  const toggleTheme = () => {
    setTheme(prev => prev === 'light' ? 'dark' : 'light');
  };

  const showToast = (message: string, type: ToastMessage['type']) => {
    const id = Date.now().toString();
    setToasts(prev => [...prev, { id, message, type }]);
    
    setTimeout(() => {
      removeToast(id);
    }, 3000);
  };

  const removeToast = (id: string) => {
    setToasts(prev => prev.filter(toast => toast.id !== id));
  };

  return (
    <AppContext.Provider value={{
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
    }}>
      {children}
    </AppContext.Provider>
  );
}

export function useApp() {
  const context = useContext(AppContext);
  if (context === undefined) {
    throw new Error('useApp must be used within an AppProvider');
  }
  return context;
}
