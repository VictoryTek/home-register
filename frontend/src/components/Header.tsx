import { useApp } from '@/context/AppContext';
import { useAuth } from '@/context/AuthContext';

interface HeaderProps {
  title: string;
  subtitle: string;
  icon?: string;
}

// Helper to get user initials
function getInitials(name: string): string {
  if (!name) return '?';
  const parts = name.trim().split(/\s+/);
  if (parts.length >= 2) {
    return (parts[0][0] + parts[parts.length - 1][0]).toUpperCase();
  }
  return name.substring(0, 2).toUpperCase();
}

export function Header({ title, subtitle, icon }: HeaderProps) {
  const { theme, toggleTheme } = useApp();
  const { user, logout } = useAuth();

  return (
    <header className="header">
      <div className="header-content">
        <div className="page-title-section">
          <h1 className="page-title">
            {icon && <i className={icon}></i>}
            {title}
          </h1>
          <p className="page-subtitle">{subtitle}</p>
        </div>
        <div className="header-actions">
          <button 
            className="theme-toggle" 
            onClick={toggleTheme}
            title={`Switch to ${theme === 'light' ? 'dark' : 'light'} mode`}
          >
            <i className={`fas fa-${theme === 'light' ? 'sun' : 'moon'}`}></i>
          </button>
          {user && (
            <div className="header-user">
              <div className="user-info">
                <div className="user-avatar">
                  {getInitials(user.full_name || user.username)}
                </div>
                <div className="user-details">
                  <span className="user-name">{user.full_name || user.username}</span>
                  <span className="user-role">{user.is_admin ? 'Admin' : 'User'}</span>
                </div>
              </div>
              <button className="logout-btn" onClick={logout} title="Sign out">
                <i className="fas fa-sign-out-alt"></i>
              </button>
            </div>
          )}
        </div>
      </div>
    </header>
  );
}
