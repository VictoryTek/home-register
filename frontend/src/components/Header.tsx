import { useApp } from '@/context/AppContext';
import { useAuth } from '@/context/AuthContext';
import { UserMenu } from './UserMenu';

interface HeaderProps {
  title: string;
  subtitle: string;
  icon?: string;
}

export function Header({ title, subtitle, icon }: HeaderProps) {
  const { theme, toggleTheme, warrantyNotifications } = useApp();
  const { user, settings } = useAuth();

  const notificationCount = settings?.notifications_enabled ? warrantyNotifications.length : 0;

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
          {notificationCount > 0 && (
            <div style={{ position: 'relative', marginRight: '0.5rem' }}>
              <button
                className="theme-toggle"
                title={`${notificationCount} warranty notification${notificationCount !== 1 ? 's' : ''}`}
                style={{ position: 'relative' }}
              >
                <i className="fas fa-bell"></i>
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
              </button>
            </div>
          )}
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
