import { useState, useEffect, useCallback } from 'react';
import { useApp } from '@/context/AppContext';
import { useAuth } from '@/context/AuthContext';
import { authApi } from '@/services/api';
import { Modal, ConfirmModal } from '@/components';
import type { User, CreateUserRequest, UpdateUserRequest } from '@/types';

export function UserManagement() {
  const { showToast } = useApp();
  const { user: currentUser } = useAuth();
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState(true);
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showEditModal, setShowEditModal] = useState(false);
  const [showDeleteModal, setShowDeleteModal] = useState(false);
  const [editingUser, setEditingUser] = useState<User | null>(null);
  const [deletingUser, setDeletingUser] = useState<User | null>(null);
  const [formData, setFormData] = useState({
    username: '',
    full_name: '',
    password: '',
    is_admin: false,
    is_active: true,
  });

  const loadUsers = useCallback(async () => {
    setLoading(true);
    try {
      const result = await authApi.getAllUsers();
      if (result.success && result.data) {
        setUsers(result.data);
      } else {
        showToast(result.error ?? 'Failed to load users', 'error');
      }
    } catch {
      showToast('Failed to load users', 'error');
    } finally {
      setLoading(false);
    }
  }, [showToast]);

  useEffect(() => {
    void loadUsers();
  }, [loadUsers]);

  const openCreateModal = () => {
    setFormData({
      username: '',
      full_name: '',
      password: '',
      is_admin: false,
      is_active: true,
    });
    setShowCreateModal(true);
  };

  const openEditModal = (user: User) => {
    setEditingUser(user);
    setFormData({
      username: user.username,
      full_name: user.full_name,
      password: '',
      is_admin: user.is_admin,
      is_active: user.is_active,
    });
    setShowEditModal(true);
  };

  const openDeleteModal = (user: User) => {
    setDeletingUser(user);
    setShowDeleteModal(true);
  };

  const handleCreateUser = async () => {
    if (!formData.username.trim() || !formData.full_name.trim() || !formData.password) {
      showToast('Please fill in all required fields', 'error');
      return;
    }

    if (formData.password.length < 8) {
      showToast('Password must be at least 8 characters', 'error');
      return;
    }

    try {
      const createData: CreateUserRequest = {
        username: formData.username,
        full_name: formData.full_name,
        password: formData.password,
        is_admin: formData.is_admin,
        is_active: formData.is_active,
      };

      const result = await authApi.createUser(createData);
      if (result.success) {
        showToast('User created successfully!', 'success');
        setShowCreateModal(false);
        void loadUsers();
      } else {
        showToast(result.error ?? 'Failed to create user', 'error');
      }
    } catch {
      showToast('Failed to create user', 'error');
    }
  };

  const handleUpdateUser = async () => {
    if (!editingUser) {return;}

    if (!formData.full_name.trim()) {
      showToast('Please fill in all required fields', 'error');
      return;
    }

    if (formData.password && formData.password.length < 8) {
      showToast('Password must be at least 8 characters', 'error');
      return;
    }

    try {
      const updateData: UpdateUserRequest = {
        full_name: formData.full_name,
        is_admin: formData.is_admin,
        is_active: formData.is_active,
      };

      if (formData.password) {
        updateData.password = formData.password;
      }

      const result = await authApi.updateUser(editingUser.id, updateData);
      if (result.success) {
        showToast('User updated successfully!', 'success');
        setShowEditModal(false);
        void loadUsers();
      } else {
        showToast(result.error ?? 'Failed to update user', 'error');
      }
    } catch {
      showToast('Failed to update user', 'error');
    }
  };

  const handleDeleteUser = async () => {
    if (!deletingUser) {return;}

    try {
      const result = await authApi.deleteUser(deletingUser.id);
      if (result.success) {
        showToast('User deleted successfully!', 'success');
        setShowDeleteModal(false);
        setDeletingUser(null);
        void loadUsers();
      } else {
        showToast(result.error ?? 'Failed to delete user', 'error');
      }
    } catch {
      showToast('Failed to delete user', 'error');
    }
  };

  if (loading) {
    return (
      <div style={{ padding: '2rem', textAlign: 'center' }}>
        <div className="spinner-small"></div>
        <p style={{ marginTop: '1rem', color: 'var(--text-secondary)' }}>Loading users...</p>
      </div>
    );
  }

  return (
    <div>
      <div style={{ marginBottom: '1rem' }}>
        <button className="btn btn-primary btn-sm" onClick={openCreateModal}>
          <i className="fas fa-plus"></i>
          Add User
        </button>
      </div>

      <div className="settings-group">
        {users.map((user) => (
          <div key={user.id} className="setting-item" style={{ alignItems: 'flex-start' }}>
            <div className="setting-info" style={{ flex: 1 }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '0.75rem', marginBottom: '0.5rem' }}>
                <div>
                  <div style={{ fontSize: '1rem', fontWeight: 600, color: 'var(--text-primary)' }}>
                    {user.full_name}
                  </div>
                  <div style={{ fontSize: '0.875rem', color: 'var(--text-secondary)' }}>
                    @{user.username}
                  </div>
                </div>
              </div>
              <div style={{ display: 'flex', gap: '0.5rem', marginTop: '0.5rem' }}>
                {user.is_admin && (
                  <span className="badge badge-primary">
                    <i className="fas fa-shield-alt"></i> Administrator
                  </span>
                )}
                <span className={`badge ${user.is_active ? 'badge-success' : 'badge-danger'}`}>
                  <i className={`fas fa-${user.is_active ? 'check-circle' : 'times-circle'}`}></i>
                  {user.is_active ? 'Active' : 'Inactive'}
                </span>
                {user.id === currentUser?.id && (
                  <span className="badge badge-info">
                    <i className="fas fa-user"></i> You
                  </span>
                )}
              </div>
            </div>
            <div style={{ display: 'flex', gap: '0.5rem' }}>
              <button
                className="btn btn-sm btn-ghost"
                onClick={() => openEditModal(user)}
                title="Edit User"
              >
                <i className="fas fa-edit"></i>
              </button>
              <button
                className="btn btn-sm btn-ghost btn-danger"
                onClick={() => openDeleteModal(user)}
                disabled={user.id === currentUser?.id}
                title={user.id === currentUser?.id ? 'Cannot delete yourself' : 'Delete User'}
              >
                <i className="fas fa-trash"></i>
              </button>
            </div>
          </div>
        ))}
      </div>

      {/* Create User Modal */}
      <Modal
        isOpen={showCreateModal}
        onClose={() => setShowCreateModal(false)}
        title="Create New User"
        subtitle="Add a new user to the system"
        footer={
          <>
            <button className="btn btn-secondary" onClick={() => setShowCreateModal(false)}>
              Cancel
            </button>
            <button className="btn btn-success" onClick={handleCreateUser}>
              <i className="fas fa-user-plus"></i>
              Create User
            </button>
          </>
        }
      >
        <div className="form-group">
          <label className="form-label" htmlFor="create-username">Username *</label>
          <input
            type="text"
            className="form-input"
            id="create-username"
            value={formData.username}
            onChange={(e) => setFormData({ ...formData, username: e.target.value })}
            autoFocus
          />
        </div>

        <div className="form-group">
          <label className="form-label" htmlFor="create-full-name">Full Name *</label>
          <input
            type="text"
            className="form-input"
            id="create-full-name"
            value={formData.full_name}
            onChange={(e) => setFormData({ ...formData, full_name: e.target.value })}
          />
        </div>

        <div className="form-group">
          <label className="form-label" htmlFor="create-password">Password *</label>
          <input
            type="password"
            className="form-input"
            id="create-password"
            value={formData.password}
            onChange={(e) => setFormData({ ...formData, password: e.target.value })}
            placeholder="Minimum 8 characters"
          />
        </div>

        <div className="form-group">
          <label className="form-checkbox">
            <input
              type="checkbox"
              checked={formData.is_admin}
              onChange={(e) => setFormData({ ...formData, is_admin: e.target.checked })}
            />
            <span className="form-checkbox-label">Administrator</span>
          </label>
          <p className="form-hint">
            Administrators have full access to all features, including user management
          </p>
        </div>

        <div className="form-group">
          <label className="form-checkbox">
            <input
              type="checkbox"
              checked={formData.is_active}
              onChange={(e) => setFormData({ ...formData, is_active: e.target.checked })}
            />
            <span className="form-checkbox-label">Account Active</span>
          </label>
          <p className="form-hint">
            Inactive users cannot log in
          </p>
        </div>
      </Modal>

      {/* Edit User Modal */}
      <Modal
        isOpen={showEditModal}
        onClose={() => setShowEditModal(false)}
        title="Edit User"
        subtitle={`Editing ${editingUser?.username}`}
        footer={
          <>
            <button className="btn btn-secondary" onClick={() => setShowEditModal(false)}>
              Cancel
            </button>
            <button className="btn btn-primary" onClick={handleUpdateUser}>
              <i className="fas fa-save"></i>
              Save Changes
            </button>
          </>
        }
      >
        <div className="form-group">
          <label className="form-label">Username</label>
          <input
            type="text"
            className="form-input"
            value={formData.username}
            disabled
            style={{ background: 'var(--bg-secondary)', color: 'var(--text-tertiary)', cursor: 'not-allowed' }}
          />
          <p className="form-hint">Username cannot be changed</p>
        </div>

        <div className="form-group">
          <label className="form-label" htmlFor="edit-full-name">Full Name *</label>
          <input
            type="text"
            className="form-input"
            id="edit-full-name"
            value={formData.full_name}
            onChange={(e) => setFormData({ ...formData, full_name: e.target.value })}
          />
        </div>

        <div className="form-group">
          <label className="form-label" htmlFor="edit-password">New Password</label>
          <input
            type="password"
            className="form-input"
            id="edit-password"
            value={formData.password}
            onChange={(e) => setFormData({ ...formData, password: e.target.value })}
            placeholder="Leave blank to keep current password"
          />
        </div>

        <div className="form-group">
          <label className="form-checkbox">
            <input
              type="checkbox"
              checked={formData.is_admin}
              onChange={(e) => setFormData({ ...formData, is_admin: e.target.checked })}
              disabled={editingUser?.id === currentUser?.id}
            />
            <span className="form-checkbox-label">Administrator</span>
          </label>
          {editingUser?.id === currentUser?.id && (
            <p className="form-hint" style={{ color: 'var(--warning-color)' }}>
              You cannot change your own admin status
            </p>
          )}
        </div>

        <div className="form-group">
          <label className="form-checkbox">
            <input
              type="checkbox"
              checked={formData.is_active}
              onChange={(e) => setFormData({ ...formData, is_active: e.target.checked })}
              disabled={editingUser?.id === currentUser?.id}
            />
            <span className="form-checkbox-label">Account Active</span>
          </label>
          {editingUser?.id === currentUser?.id && (
            <p className="form-hint" style={{ color: 'var(--warning-color)' }}>
              You cannot deactivate your own account
            </p>
          )}
        </div>
      </Modal>

      {/* Delete Confirmation Modal */}
      <ConfirmModal
        isOpen={showDeleteModal}
        onClose={() => {
          setShowDeleteModal(false);
          setDeletingUser(null);
        }}
        onConfirm={handleDeleteUser}
        title="Delete User"
        message={`Are you sure you want to delete ${deletingUser?.full_name} (@${deletingUser?.username})? This action cannot be undone.`}
        confirmText="Delete User"
        confirmButtonClass="btn-danger"
      />
    </div>
  );
}
