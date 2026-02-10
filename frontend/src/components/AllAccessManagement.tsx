import { useState, useEffect } from 'react';
import { ConfirmModal } from '@/components';
import { useApp } from '@/context/AppContext';
import type { UserAccessGrantWithUsers, CreateUserAccessGrantRequest, User } from '@/types';
import { authApi } from '@/services/api';

export function AllAccessManagement() {
  const { showToast } = useApp();
  const [grantsGiven, setGrantsGiven] = useState<UserAccessGrantWithUsers[]>([]);
  const [grantsReceived, setGrantsReceived] = useState<UserAccessGrantWithUsers[]>([]);
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState(false);
  const [showAddForm, setShowAddForm] = useState(false);
  const [newGrant, setNewGrant] = useState<CreateUserAccessGrantRequest>({
    grantee_username: '',
  });
  const [grantToRevoke, setGrantToRevoke] = useState<UserAccessGrantWithUsers | null>(null);

  useEffect(() => {
    loadGrants();
    loadUsers();
  }, []);

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

  const loadGrants = async () => {
    setLoading(true);
    try {
      const [givenResult, receivedResult] = await Promise.all([
        authApi.getMyAccessGrants(),
        authApi.getReceivedAccessGrants(),
      ]);

      if (givenResult.success && givenResult.data) {
        setGrantsGiven(givenResult.data);
      }
      if (receivedResult.success && receivedResult.data) {
        setGrantsReceived(receivedResult.data);
      }
    } catch (error) {
      showToast('Failed to load access grants', 'error');
    } finally {
      setLoading(false);
    }
  };

  const handleGrantAccess = async () => {
    if (!newGrant.grantee_username.trim()) {
      showToast('Please select a user', 'error');
      return;
    }

    try {
      const result = await authApi.createAccessGrant(newGrant);
      if (result.success) {
        showToast('All Access granted successfully', 'success');
        setNewGrant({ grantee_username: '' });
        setShowAddForm(false);
        loadGrants();
      } else {
        showToast(result.error || 'Failed to grant access', 'error');
      }
    } catch (error) {
      showToast('Failed to grant access', 'error');
    }
  };

  const handleRevokeAccess = async () => {
    if (!grantToRevoke) return;

    try {
      const result = await authApi.revokeAccessGrant(grantToRevoke.id);
      if (result.success) {
        showToast('All Access revoked successfully', 'success');
        setGrantToRevoke(null);
        loadGrants();
      } else {
        showToast(result.error || 'Failed to revoke access', 'error');
      }
    } catch (error) {
      showToast('Failed to revoke access', 'error');
    }
  };

  return (
    <>
      <div className="all-access-management">
        <div className="settings-section">
          <h2 className="section-title">All Access Grants</h2>
          <p className="section-description">
            Grant users full access to <strong>all your inventories</strong> - they can view, edit, delete, and manage
            sharing as if they own them.
          </p>

          {/* Grant Access Section */}
          <div style={{ marginBottom: '2rem' }}>
            <h3 style={{ fontSize: '1.1rem', marginBottom: '1rem' }}>Users I've Granted Access To</h3>
            {!showAddForm ? (
              <button
                className="btn btn-primary"
                onClick={() => setShowAddForm(true)}
                style={{ marginBottom: '1rem' }}
              >
                <span>➕</span> Grant All Access
              </button>
            ) : (
              <div className="card" style={{ marginBottom: '1rem', padding: '1rem' }}>
                <h4 style={{ fontSize: '1rem', marginBottom: '1rem' }}>Grant All Access</h4>
                <div className="form-group">
                  <label className="form-label">Username</label>
                  <select
                    className="form-select"
                    value={newGrant.grantee_username}
                    onChange={(e) => setNewGrant({ grantee_username: e.target.value })}
                  >
                    <option value="">Select a user...</option>
                    {users
                      .filter(user => !grantsGiven.some(grant => grant.grantee.username === user.username))
                      .map((user) => (
                        <option key={user.id} value={user.username}>
                          {user.full_name} (@{user.username})
                        </option>
                      ))}
                  </select>
                  <p style={{ fontSize: '0.875rem', color: 'var(--text-secondary)', marginTop: '0.5rem' }}>
                    This user will have full access to all your current and future inventories.
                  </p>
                </div>
                <div style={{ display: 'flex', gap: '0.5rem' }}>
                  <button className="btn btn-primary" onClick={handleGrantAccess}>
                    Grant Access
                  </button>
                  <button className="btn btn-secondary" onClick={() => setShowAddForm(false)}>
                    Cancel
                  </button>
                </div>
              </div>
            )}

            {loading ? (
              <p style={{ textAlign: 'center', padding: '2rem', color: 'var(--text-secondary)' }}>Loading...</p>
            ) : grantsGiven.length === 0 ? (
              <p style={{ textAlign: 'center', padding: '2rem', color: 'var(--text-secondary)' }}>
                You haven't granted All Access to anyone yet.
              </p>
            ) : (
              <div style={{ display: 'flex', flexDirection: 'column', gap: '0.75rem' }}>
                {grantsGiven.map((grant) => (
                  <div
                    key={grant.id}
                    className="card"
                    style={{
                      padding: '1rem',
                      display: 'flex',
                      justifyContent: 'space-between',
                      alignItems: 'center',
                    }}
                  >
                    <div>
                      <div style={{ fontWeight: '500', marginBottom: '0.25rem' }}>
                        {grant.grantee.full_name}
                      </div>
                      <div style={{ fontSize: '0.875rem', color: 'var(--text-secondary)' }}>
                        @{grant.grantee.username}
                      </div>
                      <div style={{ fontSize: '0.75rem', color: 'var(--text-secondary)', marginTop: '0.5rem' }}>
                        Granted on {new Date(grant.created_at).toLocaleDateString()}
                      </div>
                    </div>
                    <button
                      className="btn btn-danger"
                      onClick={() => setGrantToRevoke(grant)}
                      style={{ fontSize: '0.875rem' }}
                    >
                      Revoke
                    </button>
                  </div>
                ))}
              </div>
            )}
          </div>

          {/* Received Access Section */}
          <div>
            <h3 style={{ fontSize: '1.1rem', marginBottom: '1rem' }}>Users Who've Granted Me Access</h3>
            {loading ? (
              <p style={{ textAlign: 'center', padding: '2rem', color: 'var(--text-secondary)' }}>Loading...</p>
            ) : grantsReceived.length === 0 ? (
              <p style={{ textAlign: 'center', padding: '2rem', color: 'var(--text-secondary)' }}>
                No one has granted you All Access to their inventories.
              </p>
            ) : (
              <div style={{ display: 'flex', flexDirection: 'column', gap: '0.75rem' }}>
                {grantsReceived.map((grant) => (
                  <div
                    key={grant.id}
                    className="card"
                    style={{
                      padding: '1rem',
                      display: 'flex',
                      justifyContent: 'space-between',
                      alignItems: 'center',
                    }}
                  >
                    <div>
                      <div style={{ fontWeight: '500', marginBottom: '0.25rem' }}>
                        {grant.grantor.full_name}
                      </div>
                      <div style={{ fontSize: '0.875rem', color: 'var(--text-secondary)' }}>
                        @{grant.grantor.username}
                      </div>
                      <div style={{ fontSize: '0.75rem', color: 'var(--text-secondary)', marginTop: '0.5rem' }}>
                        Granted on {new Date(grant.created_at).toLocaleDateString()}
                      </div>
                    </div>
                    <div
                      style={{
                        padding: '0.5rem 1rem',
                        background: 'var(--success-bg)',
                        color: 'var(--success)',
                        borderRadius: '8px',
                        fontSize: '0.875rem',
                        fontWeight: '500',
                      }}
                    >
                      ✓ Full Access
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>

          <div
            style={{
              marginTop: '1.5rem',
              padding: '1rem',
              background: 'var(--warning-bg)',
              border: '1px solid var(--warning)',
              borderRadius: '8px',
            }}
          >
            <p style={{ fontSize: '0.875rem', color: 'var(--text-primary)', margin: 0 }}>
              <strong>⚠️ Important:</strong> Users with All Access can do everything you can do with your inventories,
              including deleting them and managing shares. Only grant this to people you fully trust.
            </p>
          </div>
        </div>
      </div>

      <ConfirmModal
        isOpen={!!grantToRevoke}
        onClose={() => setGrantToRevoke(null)}
        onConfirm={handleRevokeAccess}
        title="Revoke All Access"
        message={`Are you sure you want to revoke ${grantToRevoke?.grantee.full_name}'s access to all your inventories? They will immediately lose all access.`}
        confirmText="Revoke Access"
      />
    </>
  );
}
