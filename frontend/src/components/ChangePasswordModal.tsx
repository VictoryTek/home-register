import { useState } from 'react';
import { Modal } from './Modal';
import { useApp } from '@/context/AppContext';
import { authApi } from '@/services/api';

interface ChangePasswordModalProps {
  onClose: () => void;
}

export function ChangePasswordModal({ onClose }: ChangePasswordModalProps) {
  const { showToast } = useApp();
  const [form, setForm] = useState({
    currentPassword: '',
    newPassword: '',
    confirmPassword: '',
  });
  const [isLoading, setIsLoading] = useState(false);
  const [showCurrentPassword, setShowCurrentPassword] = useState(false);
  const [showNewPassword, setShowNewPassword] = useState(false);
  const [showConfirmPassword, setShowConfirmPassword] = useState(false);
  const [errors, setErrors] = useState<Record<string, string>>({});

  const validateForm = (): boolean => {
    const newErrors: Record<string, string> = {};

    if (!form.currentPassword) {
      newErrors.currentPassword = 'Current password is required';
    }

    if (!form.newPassword) {
      newErrors.newPassword = 'New password is required';
    } else if (form.newPassword.length < 8) {
      newErrors.newPassword = 'Password must be at least 8 characters';
    }

    if (!form.confirmPassword) {
      newErrors.confirmPassword = 'Please confirm your new password';
    } else if (form.newPassword !== form.confirmPassword) {
      newErrors.confirmPassword = 'Passwords do not match';
    }

    if (form.currentPassword && form.newPassword && form.currentPassword === form.newPassword) {
      newErrors.newPassword = 'New password must be different from current password';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!validateForm()) return;

    setIsLoading(true);
    try {
      const result = await authApi.changePassword({
        current_password: form.currentPassword,
        new_password: form.newPassword,
      });

      if (result.success) {
        showToast('Password changed successfully', 'success');
        onClose();
      } else {
        const errorMsg = result.error || 'Failed to change password';
        if (errorMsg.toLowerCase().includes('current password') || errorMsg.toLowerCase().includes('incorrect')) {
          setErrors({ currentPassword: 'Current password is incorrect' });
        } else {
          showToast(errorMsg, 'error');
        }
      }
    } catch (error) {
      console.error('Error changing password:', error);
      showToast('An unexpected error occurred', 'error');
    } finally {
      setIsLoading(false);
    }
  };

  const handleInputChange = (field: keyof typeof form) => (
    e: React.ChangeEvent<HTMLInputElement>
  ) => {
    setForm(prev => ({ ...prev, [field]: e.target.value }));
    // Clear error when user starts typing
    if (errors[field]) {
      setErrors(prev => {
        const next = { ...prev };
        delete next[field];
        return next;
      });
    }
  };

  return (
    <Modal
      isOpen={true}
      onClose={onClose}
      title="Change Password"
      subtitle="Enter your current password and choose a new one"
      footer={
        <div className="modal-actions">
          <button 
            type="button" 
            className="btn btn-secondary" 
            onClick={onClose}
            disabled={isLoading}
          >
            Cancel
          </button>
          <button 
            type="submit" 
            className="btn btn-primary"
            form="change-password-form"
            disabled={isLoading}
          >
            {isLoading ? (
              <>
                <span className="spinner-small"></span>
                Changing...
              </>
            ) : (
              <>
                <i className="fas fa-check"></i>
                Change Password
              </>
            )}
          </button>
        </div>
      }
    >
      <form id="change-password-form" onSubmit={handleSubmit} className="settings-form">
        <div className={`form-group ${errors.currentPassword ? 'has-error' : ''}`}>
          <label htmlFor="currentPassword">Current Password</label>
          <div className="password-input-wrapper">
            <input
              type={showCurrentPassword ? 'text' : 'password'}
              id="currentPassword"
              value={form.currentPassword}
              onChange={handleInputChange('currentPassword')}
              placeholder="Enter your current password"
              autoComplete="current-password"
              disabled={isLoading}
            />
            <button
              type="button"
              className="password-toggle"
              onClick={() => setShowCurrentPassword(!showCurrentPassword)}
              tabIndex={-1}
            >
              <i className={`fas fa-eye${showCurrentPassword ? '-slash' : ''}`}></i>
            </button>
          </div>
          {errors.currentPassword && (
            <span className="form-error">{errors.currentPassword}</span>
          )}
        </div>

        <div className={`form-group ${errors.newPassword ? 'has-error' : ''}`}>
          <label htmlFor="newPassword">New Password</label>
          <div className="password-input-wrapper">
            <input
              type={showNewPassword ? 'text' : 'password'}
              id="newPassword"
              value={form.newPassword}
              onChange={handleInputChange('newPassword')}
              placeholder="Enter your new password"
              autoComplete="new-password"
              disabled={isLoading}
            />
            <button
              type="button"
              className="password-toggle"
              onClick={() => setShowNewPassword(!showNewPassword)}
              tabIndex={-1}
            >
              <i className={`fas fa-eye${showNewPassword ? '-slash' : ''}`}></i>
            </button>
          </div>
          {errors.newPassword && (
            <span className="form-error">{errors.newPassword}</span>
          )}
          <span className="form-hint">Must be at least 8 characters</span>
        </div>

        <div className={`form-group ${errors.confirmPassword ? 'has-error' : ''}`}>
          <label htmlFor="confirmPassword">Confirm New Password</label>
          <div className="password-input-wrapper">
            <input
              type={showConfirmPassword ? 'text' : 'password'}
              id="confirmPassword"
              value={form.confirmPassword}
              onChange={handleInputChange('confirmPassword')}
              placeholder="Confirm your new password"
              autoComplete="new-password"
              disabled={isLoading}
            />
            <button
              type="button"
              className="password-toggle"
              onClick={() => setShowConfirmPassword(!showConfirmPassword)}
              tabIndex={-1}
            >
              <i className={`fas fa-eye${showConfirmPassword ? '-slash' : ''}`}></i>
            </button>
          </div>
          {errors.confirmPassword && (
            <span className="form-error">{errors.confirmPassword}</span>
          )}
        </div>
      </form>
    </Modal>
  );
}
