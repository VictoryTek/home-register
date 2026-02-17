import { useNavigate } from 'react-router-dom';
import { useApp } from '@/context/AppContext';
import { useAuth } from '@/context/AuthContext';
import { UserMenu } from './UserMenu';

interface HeaderProps {
  title: string;
  subtitle: string;
  icon?: string;
}

export function Header({ title, subtitle, icon }: HeaderProps) {
  const navigate = useNavigate();
  const { theme, toggleTheme, warrantyNotifications, toggleSidebar, sidebarOpen } = useApp();
  const { user, settings } = useAuth();

  // RECOMMENDED FIX: Removed redundant filtering - AppContext now filters at source
  // warrantyNotifications already excludes dismissed notifications
  const notificationCount = settings?.notifications_enabled ? warrantyNotifications.length : 0;

  return (
    <header className="header">
      <div className="header-content">
        {/* Hamburger menu button — visible only on mobile via CSS */}
        <button
          className="mobile-menu-toggle"
          onClick={toggleSidebar}
          aria-label="Toggle navigation menu"
          aria-expanded={sidebarOpen}
          type="button"
        >
          <i className="fas fa-bars"></i>
        </button>

        <div className="page-title-section">
          <h1 className="page-title">
            {icon && <i className={icon}></i>}
            {title}
          </h1>
          <p className="page-subtitle">{subtitle}</p>
        </div>
        <div className="header-actions">
          <div style={{ position: 'relative' }}>
            <button
              className="theme-toggle"
              onClick={() => navigate('/notifications')}
              title={
                notificationCount > 0
                  ? `${notificationCount} warranty notification${notificationCount !== 1 ? 's' : ''}`
                  : 'Notifications'
              }
              style={{ position: 'relative', cursor: 'pointer' }}
            >
              <i className="fas fa-bell"></i>
              {notificationCount > 0 && (
                <span
                  style={{
                    position: 'absolute',
                    top: '-4px',
                    right: '-4px',
                    background: 'var(--danger-color)',
                    color: 'white',
                    borderRadius: '50%',
                    padding: '2px 6px',
                    fontSize: '0.65rem',
                    fontWeight: 'bold',
                    minWidth: '18px',
                    height: '18px',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    border: '2px solid var(--bg-primary)',
                  }}
                >
                  {notificationCount > 99 ? '99+' : notificationCount}
                </span>
              )}
            </button>
          </div>
          <button
            className="theme-toggle"
            onClick={toggleTheme}
            title={`Switch to ${theme === 'light' ? 'dark' : 'light'} mode`}
          >
            <i className={`fas fa-${theme === 'light' ? 'sun' : 'moon'}`}></i>
          </button>
          {user && <UserMenu />}
        </div>
      </div>
    </header>
  );
}
