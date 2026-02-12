import { BrowserRouter, Routes, Route, useLocation, useNavigate, Navigate } from 'react-router-dom';
import { AppProvider } from '@/context/AppContext';
import { AuthProvider, useAuth } from '@/context/AuthContext';
import { Sidebar, Toast } from '@/components';
import { InventoriesPage, InventoryDetailPage, OrganizersPage, SettingsPage, SetupPage, LoginPage, RegisterPage, RecoveryPage } from '@/pages';
import '@/styles/index.css';

// Loading spinner component
function LoadingScreen() {
  return (
    <div className="loading-screen">
      <div className="loading-spinner"></div>
      <p>Loading...</p>
    </div>
  );
}

// Protected route wrapper
function ProtectedRoute({ children }: { children: React.ReactNode }) {
  const { isAuthenticated, isLoading, needsSetup } = useAuth();
  
  if (isLoading) {
    return <LoadingScreen />;
  }
  
  if (needsSetup) {
    return <Navigate to="/setup" replace />;
  }
  
  if (!isAuthenticated) {
    return <Navigate to="/login" replace />;
  }
  
  return <>{children}</>;
}

function AppContent() {
  const location = useLocation();
  const navigate = useNavigate();
  const { isAuthenticated, isLoading, needsSetup } = useAuth();

  const getCurrentPage = () => {
    if (location.pathname === '/settings') {
      return 'settings';
    }
    if (location.pathname.includes('/organizers')) {
      return 'organizers';
    }
    if (location.pathname.startsWith('/inventory/')) {
      return 'inventories';
    }
    return 'inventories';
  };

  const handleNavigate = (page: string) => {
    switch (page) {
      case 'inventories':
        navigate('/');
        break;
      case 'organizers':
        navigate('/organizers');
        break;
      case 'settings':
        navigate('/settings');
        break;
      default:
        navigate('/');
    }
  };

  // Show loading screen while checking auth
  if (isLoading) {
    return <LoadingScreen />;
  }

  // Auth pages (no sidebar)
  if (location.pathname === '/setup' || location.pathname === '/login' || location.pathname === '/register' || location.pathname === '/recover') {
    return (
      <Routes>
        <Route path="/setup" element={
          needsSetup ? <SetupPage /> : <Navigate to={isAuthenticated ? "/" : "/login"} replace />
        } />
        <Route path="/login" element={
          needsSetup ? <Navigate to="/setup" replace /> : 
          isAuthenticated ? <Navigate to="/" replace /> : <LoginPage />
        } />
        <Route path="/register" element={
          needsSetup ? <Navigate to="/setup" replace /> : 
          isAuthenticated ? <Navigate to="/" replace /> : <RegisterPage />
        } />
        <Route path="/recover" element={
          needsSetup ? <Navigate to="/setup" replace /> : 
          isAuthenticated ? <Navigate to="/" replace /> : <RecoveryPage />
        } />
      </Routes>
    );
  }

  return (
    <>
      <Sidebar 
        currentPage={getCurrentPage()} 
        onNavigate={handleNavigate}
      />
      <main className="main-content">
        <Routes>
          <Route path="/" element={
            <ProtectedRoute><InventoriesPage /></ProtectedRoute>
          } />
          <Route path="/inventory/:id" element={
            <ProtectedRoute><InventoryDetailPage /></ProtectedRoute>
          } />
          <Route path="/organizers" element={
            <ProtectedRoute><OrganizersPage /></ProtectedRoute>
          } />
          <Route path="/inventory/:id/organizers" element={
            <ProtectedRoute><OrganizersPage /></ProtectedRoute>
          } />
          <Route path="/settings" element={
            <ProtectedRoute><SettingsPage /></ProtectedRoute>
          } />
          {/* Redirect unknown routes */}
          <Route path="*" element={<Navigate to="/" replace />} />
        </Routes>
      </main>
      <Toast />
    </>
  );
}

function App() {
  return (
    <BrowserRouter>
      <AuthProvider>
        <AppProvider>
          <AppContent />
        </AppProvider>
      </AuthProvider>
    </BrowserRouter>
  );
}

export default App;
