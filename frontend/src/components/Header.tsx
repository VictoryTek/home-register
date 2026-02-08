import { useApp } from '@/context/AppContext';
import { useAuth } from '@/context/AuthContext';
import { UserMenu } from './UserMenu';

interface HeaderProps {
  title: string;
  subtitle: string;
  icon?: string;
}

export function Header({ title, subtitle, icon }: HeaderProps) {
  const { theme, toggleTheme } = useApp();
  const { user } = useAuth();

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
          {user && <UserMenu />}
        </div>
      </div>
    </header>
  );
}
