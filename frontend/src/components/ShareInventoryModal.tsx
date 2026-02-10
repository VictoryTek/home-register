import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { Modal, ConfirmModal } from '@/components';
import { useApp } from '@/context/AppContext';
import type { InventoryShare, CreateInventoryShareRequest, PermissionLevel, User, TransferOwnershipRequest } from '@/types';
import { authApi } from '@/services/api';

interface ShareInventoryModalProps {
  isOpen: boolean;
  onClose: () => void;
  inventoryId: number;
  inventoryName: string;
}

const PERMISSION_LABELS: Record<PermissionLevel, { label: string; description: string }> = {
  view: {
    label: 'View Only',
    description: 'Can view inventory and items',
  },
  edit_items: {
    label: 'Edit Items',
    description: 'Can view and edit item details (cannot add/remove items)',
  },
  edit_inventory: {
    label: 'Edit Inventory',
    description: 'Can view, edit items, add/remove items, and edit inventory details',
  },
};

export function ShareInventoryModal({ isOpen, onClose, inventoryId, inventoryName }: ShareInventoryModalProps) {
  const { showToast } = useApp();
  const navigate = useNavigate();
  const [shares, setShares] = useState<InventoryShare[]>([]);
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState(false);
  const [showAddForm, setShowAddForm] = useState(false);
  const [newShare, setNewShare] = useState<CreateInventoryShareRequest>({
    shared_with_username: '',
    permission_level: 'view',
  });
  const [shareToDelete, setShareToDelete] = useState<InventoryShare | null>(null);
  const [editingShareId, setEditingShareId] = useState<string | null>(null);
  
  // Transfer ownership state
  const [showTransferForm, setShowTransferForm] = useState(false);
  const [transferRequest, setTransferRequest] = useState<TransferOwnershipRequest>({
    new_owner_username: '',
  });
  const [showTransferConfirm, setShowTransferConfirm] = useState(false);
  const [isTransferring, setIsTransferring] = useState(false);

  useEffect(() => {
    if (isOpen) {
      loadShares();
      loadUsers();
    }
  }, [isOpen, inventoryId]);

  const loadUsers = async () => {
    try {
      const result = await authApi.getAllUsers();
      if (result.success && result.data) {
        setUsers(result.data);
      } else {
        showToast(result.error || 'Failed to load users', 'error');
      }
    } catch (error) {
      showToast('Failed to load users', 'error');
    }
  };

  const loadShares = async () => {
    setLoading(true);
    try {
      const result = await authApi.getInventoryShares(inventoryId);
      if (result.success && result.data) {
        setShares(result.data);
      } else {
        showToast(result.error || 'Failed to load shares', 'error');
      }
    } catch (error) {
      showToast('Failed to load shares', 'error');
    } finally {
      setLoading(false);
    }
  };

  const handleAddShare = async () => {
    if (!newShare.shared_with_username.trim()) {
      showToast('Please select a user', 'error');
      return;
    }

    try {
      const result = await authApi.shareInventory(inventoryId, newShare);
      if (result.success) {
        showToast('Inventory shared successfully', 'success');
        setNewShare({ shared_with_username: '', permission_level: 'view' });
        setShowAddForm(false);
        loadShares();
      } else {
        showToast(result.error || 'Failed to share inventory', 'error');
      }
    } catch (error) {
      showToast('Failed to share inventory', 'error');
    }
  };

  const handleUpdatePermission = async (shareId: string, newPermission: PermissionLevel) => {
    try {
      const result = await authApi.updateInventoryShare(shareId, { permission_level: newPermission });
      if (result.success) {
        showToast('Permission updated successfully', 'success');
        setEditingShareId(null);
        loadShares();
      } else {
        showToast(result.error || 'Failed to update permission', 'error');
      }
    } catch (error) {
      showToast('Failed to update permission', 'error');
    }
  };

  const handleDeleteShare = async () => {
    if (!shareToDelete) return;

    try {
      const result = await authApi.removeInventoryShare(shareToDelete.id);
      if (result.success) {
        showToast('Share removed successfully', 'success');
        setShareToDelete(null);
        loadShares();
      } else {
        showToast(result.error || 'Failed to remove share', 'error');
      }
    } catch (error) {
      showToast('Failed to remove share', 'error');
    }
  };

  const handleTransferOwnership = async () => {
    if (!transferRequest.new_owner_username) {
      showToast('Please select a user', 'error');
      return;
    }

    setIsTransferring(true);
    try {
      const result = await authApi.transferOwnership(inventoryId, transferRequest);
      if (result.success && result.data) {
        showToast(
          `Ownership transferred to ${result.data.new_owner.full_name}. ${result.data.items_transferred} items transferred.`,
          'success'
        );
        setShowTransferConfirm(false);
        setShowTransferForm(false);
        setTransferRequest({ new_owner_username: '' });
        onClose();
        // Navigate away since user no longer has access
        navigate('/');
      } else {
        showToast(result.error || 'Failed to transfer ownership', 'error');
      }
    } catch (error) {
      showToast('Failed to transfer ownership', 'error');
    } finally {
      setIsTransferring(false);
    }
  };

  return (
    <>
      <Modal
        isOpen={isOpen}
        onClose={onClose}
        title={`Share "${inventoryName}"`}
        subtitle="Manage who can access this inventory"
        maxWidth="700px"
      >
        <div className="share-modal-content">
          {/* Add Share Section */}
          {!showAddForm ? (
            <button
              className="btn btn-primary"
              onClick={() => setShowAddForm(true)}
              style={{ marginBottom: '1.5rem' }}
            >
              <span>➕</span> Share with User
            </button>
          ) : (
            <div className="share-add-form card" style={{ marginBottom: '1.5rem', padding: '1rem' }}>
              <h3 style={{ fontSize: '1rem', marginBottom: '1rem' }}>Share with User</h3>
              <div className="form-group">
                <label className="form-label">Username</label>
                <select
                  className="form-select"
                  value={newShare.shared_with_username}
                  onChange={(e) => setNewShare({ ...newShare, shared_with_username: e.target.value })}
                >
                  <option value="">Select a user...</option>
                  {users
                    .filter(user => !shares.some(share => share.shared_with_user.username === user.username))
                    .map((user) => (
                      <option key={user.id} value={user.username}>
                        {user.full_name} (@{user.username})
                      </option>
                    ))}
                </select>
              </div>
              <div className="form-group">
                <label className="form-label">Permission Level</label>
                <select
                  className="form-select"
                  value={newShare.permission_level}
                  onChange={(e) => setNewShare({ ...newShare, permission_level: e.target.value as PermissionLevel })}
                >
                  {Object.entries(PERMISSION_LABELS).map(([value, { label, description }]) => (
                    <option key={value} value={value}>
                      {label} - {description}
                    </option>
                  ))}
                </select>
              </div>
              <div style={{ display: 'flex', gap: '0.5rem' }}>
                <button className="btn btn-primary" onClick={handleAddShare}>
                  Share
                </button>
                <button className="btn btn-secondary" onClick={() => setShowAddForm(false)}>
                  Cancel
                </button>
              </div>
            </div>
          )}

          {/* Existing Shares List */}
          <div className="shares-list">
            <h3 style={{ fontSize: '1rem', marginBottom: '1rem', color: 'var(--text-secondary)' }}>
              Current Shares ({shares.length})
            </h3>
            {loading ? (
              <p style={{ textAlign: 'center', padding: '2rem', color: 'var(--text-secondary)' }}>
                Loading shares...
              </p>
            ) : shares.length === 0 ? (
              <p style={{ textAlign: 'center', padding: '2rem', color: 'var(--text-secondary)' }}>
                This inventory is not shared with anyone yet.
              </p>
            ) : (
              <div style={{ display: 'flex', flexDirection: 'column', gap: '0.75rem' }}>
                {shares.map((share) => (
                  <div
                    key={share.id}
                    className="card"
                    style={{
                      padding: '1rem',
                      display: 'flex',
                      justifyContent: 'space-between',
                      alignItems: 'center',
                    }}
                  >
                    <div style={{ flex: 1 }}>
                      <div style={{ fontWeight: '500', marginBottom: '0.25rem' }}>
                        {share.shared_with_user.full_name}
                      </div>
                      <div style={{ fontSize: '0.875rem', color: 'var(--text-secondary)' }}>
                        @{share.shared_with_user.username}
                      </div>
                      {editingShareId === share.id ? (
                        <div style={{ marginTop: '0.75rem' }}>
                          <select
                            className="form-select"
                            value={share.permission_level}
                            onChange={(e) => handleUpdatePermission(share.id, e.target.value as PermissionLevel)}
                            style={{ maxWidth: '300px' }}
                          >
                            {Object.entries(PERMISSION_LABELS).map(([value, { label, description }]) => (
                              <option key={value} value={value}>
                                {label} - {description}
                              </option>
                            ))}
                          </select>
                          <button
                            className="btn btn-secondary"
                            onClick={() => setEditingShareId(null)}
                            style={{ marginTop: '0.5rem', fontSize: '0.875rem' }}
                          >
                            Cancel
                          </button>
                        </div>
                      ) : (
                        <div style={{ marginTop: '0.5rem', fontSize: '0.875rem' }}>
                          <span
                            style={{
                              display: 'inline-block',
                              padding: '0.25rem 0.75rem',
                              borderRadius: '12px',
                              background: 'var(--bg-secondary)',
                              color: 'var(--text-primary)',
                            }}
                          >
                            {PERMISSION_LABELS[share.permission_level].label}
                          </span>
                        </div>
                      )}
                    </div>
                    <div style={{ display: 'flex', gap: '0.5rem' }}>
                      {editingShareId !== share.id && (
                        <button
                          className="btn btn-icon"
                          onClick={() => setEditingShareId(share.id)}
                          title="Edit permission"
                        >
                          ✏️
                        </button>
                      )}
                      <button
                        className="btn btn-icon"
                        onClick={() => setShareToDelete(share)}
                        title="Remove share"
                      >
                        🗑️
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>

          <div style={{ marginTop: '1.5rem', padding: '1rem', background: 'var(--bg-secondary)', borderRadius: '8px' }}>
            <p style={{ fontSize: '0.875rem', color: 'var(--text-secondary)', margin: 0 }}>
              <strong>💡 Tip:</strong> Looking to give someone full access to all your inventories? Use the "All
              Access" feature in Settings instead.
            </p>
          </div>

          {/* Transfer Ownership Section */}
          <div style={{ marginTop: '2rem', borderTop: '1px solid var(--border-color)', paddingTop: '1.5rem' }}>
            <h3 style={{ fontSize: '1rem', marginBottom: '0.5rem', color: 'var(--danger-color)' }}>
              ⚠️ Transfer Ownership
            </h3>
            <p style={{ fontSize: '0.875rem', color: 'var(--text-secondary)', marginBottom: '1rem' }}>
              Permanently transfer this inventory to another user. <strong>You will lose all access</strong> after the transfer.
            </p>
            
            {!showTransferForm ? (
              <button
                className="btn"
                onClick={() => setShowTransferForm(true)}
                style={{ 
                  background: 'transparent',
                  border: '1px solid var(--danger-color)',
                  color: 'var(--danger-color)',
                }}
              >
                <span>🔄</span> Transfer Ownership
              </button>
            ) : (
              <div className="card" style={{ padding: '1rem', borderColor: 'var(--danger-color)' }}>
                <div className="form-group">
                  <label className="form-label">Transfer to User</label>
                  <select
                    className="form-select"
                    value={transferRequest.new_owner_username}
                    onChange={(e) => setTransferRequest({ new_owner_username: e.target.value })}
                  >
                    <option value="">Select a user...</option>
                    {users.map((user) => (
                      <option key={user.id} value={user.username}>
                        {user.full_name} (@{user.username})
                      </option>
                    ))}
                  </select>
                </div>
                
                <div style={{ 
                  padding: '0.75rem', 
                  background: 'var(--warning-bg)', 
                  borderRadius: '6px',
                  marginBottom: '1rem',
                  border: '1px solid var(--warning-color)'
                }}>
                  <p style={{ fontSize: '0.875rem', color: 'var(--warning-color)', margin: 0 }}>
                    <strong>⚠️ Warning:</strong> This action is <strong>irreversible</strong>. You will:
                  </p>
                  <ul style={{ fontSize: '0.875rem', color: 'var(--warning-color)', margin: '0.5rem 0 0 1.25rem', padding: 0 }}>
                    <li>Lose ownership of "{inventoryName}"</li>
                    <li>Lose all access to this inventory</li>
                    <li>All existing shares will be removed</li>
                  </ul>
                </div>
                
                <div style={{ display: 'flex', gap: '0.5rem' }}>
                  <button 
                    className="btn"
                    onClick={() => setShowTransferConfirm(true)}
                    disabled={!transferRequest.new_owner_username}
                    style={{ 
                      background: 'var(--danger-color)',
                      color: 'white',
                      border: 'none',
                    }}
                  >
                    Transfer Ownership
                  </button>
                  <button 
                    className="btn btn-secondary" 
                    onClick={() => {
                      setShowTransferForm(false);
                      setTransferRequest({ new_owner_username: '' });
                    }}
                  >
                    Cancel
                  </button>
                </div>
              </div>
            )}
          </div>
        </div>
      </Modal>

      <ConfirmModal
        isOpen={!!shareToDelete}
        onClose={() => setShareToDelete(null)}
        onConfirm={handleDeleteShare}
        title="Remove Share"
        message={`Are you sure you want to stop sharing this inventory with ${shareToDelete?.shared_with_user.full_name}?`}
        confirmText="Remove"
      />

      <ConfirmModal
        isOpen={showTransferConfirm}
        onClose={() => setShowTransferConfirm(false)}
        onConfirm={handleTransferOwnership}
        title="⚠️ Confirm Ownership Transfer"
        message={`Are you absolutely sure you want to transfer ownership of "${inventoryName}" to @${transferRequest.new_owner_username}? This action cannot be undone and you will lose all access to this inventory.`}
        confirmText={isTransferring ? "Transferring..." : "Yes, Transfer Ownership"}
        confirmButtonClass="btn-danger"
      />
    </>
  );
}
