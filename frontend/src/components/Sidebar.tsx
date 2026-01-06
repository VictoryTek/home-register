import { useApp } from '@/context/AppContext';

interface SidebarProps {
  currentPage: string;
  onNavigate: (page: string) => void;
}

export function Sidebar({ currentPage, onNavigate }: SidebarProps) {
  return (
    <aside className="sidebar">
      <div className="sidebar-header">
        <a href="/" className="logo">
          <img src="/logo_full.png" alt="Home Register" />
        </a>
      </div>
      
      <nav className="nav-menu">
        <div className="nav-section">
          <div className="nav-section-title">Overview</div>
          <button
            className={`nav-item ${currentPage === 'inventories' ? 'active' : ''}`}
            onClick={() => onNavigate('inventories')}
          >
            <i className="fas fa-warehouse"></i>
            <span>Inventories</span>
          </button>
        </div>

        <div className="nav-section system-section">
          <div className="nav-section-title">System</div>
          <button
            className={`nav-item ${currentPage === 'settings' ? 'active' : ''}`}
            onClick={() => onNavigate('settings')}
          >
            <i className="fas fa-cog"></i>
            <span>Settings</span>
          </button>
        </div>
      </nav>
    </aside>
  );
}
