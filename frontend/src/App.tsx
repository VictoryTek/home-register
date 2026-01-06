import { BrowserRouter, Routes, Route, useLocation, useNavigate } from 'react-router-dom';
import { AppProvider } from '@/context/AppContext';
import { Sidebar, Toast } from '@/components';
import { InventoriesPage, InventoryDetailPage, SettingsPage } from '@/pages';
import '@/styles/index.css';

function AppContent() {
  const location = useLocation();
  const navigate = useNavigate();

  const getCurrentPage = () => {
    if (location.pathname === '/settings') return 'settings';
    if (location.pathname.startsWith('/inventory/')) return 'inventories';
    return 'inventories';
  };

  const handleNavigate = (page: string) => {
    switch (page) {
      case 'inventories':
        navigate('/');
        break;
      case 'settings':
        navigate('/settings');
        break;
      default:
        navigate('/');
    }
  };

  return (
    <>
      <Sidebar currentPage={getCurrentPage()} onNavigate={handleNavigate} />
      <main className="main-content">
        <Routes>
          <Route path="/" element={<InventoriesPage />} />
          <Route path="/inventory/:id" element={<InventoryDetailPage />} />
          <Route path="/settings" element={<SettingsPage />} />
        </Routes>
      </main>
      <Toast />
    </>
  );
}

function App() {
  return (
    <BrowserRouter>
      <AppProvider>
        <AppContent />
      </AppProvider>
    </BrowserRouter>
  );
}

export default App;
