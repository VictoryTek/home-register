import { useState, useRef, useEffect } from 'react';
import { useAuth } from '@/context/AuthContext';
import { ChangePasswordModal } from './ChangePasswordModal';
import { EditProfileModal } from './EditProfileModal';

// Helper to get user initials
function getInitials(name: string): string {
  if (!name) return '?';
  const parts = name.trim().split(/\s+/);
  const first = parts[0];
  const last = parts[parts.length - 1];
  if (parts.length >= 2 && first && last) {
    return ((first[0] ?? '') + (last[0] ?? '')).toUpperCase();
  }
  return name.substring(0, 2).toUpperCase();
}

export function UserMenu() {
  const { user, logout } = useAuth();
  const [isOpen, setIsOpen] = useState(false);
  const [showPasswordModal, setShowPasswordModal] = useState(false);
  const [showProfileModal, setShowProfileModal] = useState(false);
  const menuRef = useRef<HTMLDivElement>(null);

  // Close dropdown when clicking outside
  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    }

    if (isOpen) {
      document.addEventListener('mousedown', handleClickOutside);
      return () => document.removeEventListener('mousedown', handleClickOutside);
    }
    return undefined;
  }, [isOpen]);

  // Close on escape key
  useEffect(() => {
    function handleEscape(event: KeyboardEvent) {
      if (event.key === 'Escape') {
        setIsOpen(false);
      }
    }

    if (isOpen) {
      document.addEventListener('keydown', handleEscape);
      return () => document.removeEventListener('keydown', handleEscape);
    }
    return undefined;
  }, [isOpen]);

  if (!user) return null;

  const handleMenuItemClick = (action: () => void) => {
    setIsOpen(false);
    action();
  };

  return (
    <>
      <div className="user-menu" ref={menuRef}>
        <button 
          className="user-menu-trigger"
          onClick={() => setIsOpen(!isOpen)}
          aria-expanded={isOpen}
          aria-haspopup="true"
        >
          <div className="user-avatar">
            {getInitials(user.full_name || user.username)}
          </div>
          <div className="user-details">
            <span className="user-name">{user.full_name || user.username}</span>
            <span className="user-role">{user.is_admin ? 'Admin' : 'User'}</span>
          </div>
          <i className={`fas fa-chevron-${isOpen ? 'up' : 'down'} menu-chevron`}></i>
        </button>

        {isOpen && (
          <div className="user-menu-dropdown">
            <div className="menu-header">
              <div className="menu-user-avatar">
                {getInitials(user.full_name || user.username)}
              </div>
              <div className="menu-user-info">
                <span className="menu-user-name">{user.full_name || user.username}</span>
                <span className="menu-user-email">@{user.username}</span>
              </div>
            </div>
            
            <div className="menu-divider"></div>
            
            <div className="menu-section">
              <span className="menu-section-title">Account</span>
              <button 
                className="menu-item"
                onClick={() => handleMenuItemClick(() => setShowProfileModal(true))}
              >
                <i className="fas fa-user-edit"></i>
                <span>Edit Profile</span>
              </button>
              <button 
                className="menu-item"
                onClick={() => handleMenuItemClick(() => setShowPasswordModal(true))}
              >
                <i className="fas fa-key"></i>
                <span>Change Password</span>
              </button>
            </div>
            
            <div className="menu-divider"></div>
            
            <button 
              className="menu-item menu-item-danger"
              onClick={() => handleMenuItemClick(logout)}
            >
              <i className="fas fa-sign-out-alt"></i>
              <span>Sign Out</span>
            </button>
          </div>
        )}
      </div>

      {showPasswordModal && (
        <ChangePasswordModal onClose={() => setShowPasswordModal(false)} />
      )}

      {showProfileModal && (
        <EditProfileModal onClose={() => setShowProfileModal(false)} />
      )}
    </>
  );
}
