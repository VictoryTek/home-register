import { useState, useEffect, useCallback } from 'react';
import { useApp } from '@/context/AppContext';
import { useAuth } from '@/context/AuthContext';
import { backupApi } from '@/services/api';
import { ConfirmModal } from '@/components';
import type { BackupInfo } from '@/types';

function formatBackupDate(dateString: string): string {
  const date = new Date(dateString);
  return date.toLocaleString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
}

export function BackupRestoreSection() {
  const { showToast } = useApp();
  const { user } = useAuth();
  const [backups, setBackups] = useState<BackupInfo[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isCreating, setIsCreating] = useState(false);
  const [isUploading, setIsUploading] = useState(false);
  const [isRestoring, setIsRestoring] = useState(false);
  const [restoreTarget, setRestoreTarget] = useState<string | null>(null);
  const [deleteTarget, setDeleteTarget] = useState<string | null>(null);

  // Load backups on mount
  const loadBackups = useCallback(async () => {
    setIsLoading(true);
    try {
      const result = await backupApi.list();
      if (result.success && result.data) {
        setBackups(result.data);
      } else {
        console.error('Failed to load backups:', result.error);
      }
    } catch (error) {
      console.error('Error loading backups:', error);
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    void loadBackups();
  }, [loadBackups]);

  // Create backup handler
  const handleCreateBackup = async () => {
    setIsCreating(true);
    try {
      const result = await backupApi.create();
      if (result.success && result.data) {
        showToast(`Backup created: ${result.data.name}`, 'success');
        await loadBackups();
      } else {
        showToast(result.error ?? 'Failed to create backup', 'error');
      }
    } catch (error) {
      console.error('Error creating backup:', error);
      showToast('An error occurred while creating backup', 'error');
    } finally {
      setIsCreating(false);
    }
  };

  // Upload backup handler
  const handleUploadBackup = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) {
      return;
    }

    // Reset the input so the same file can be uploaded again
    e.target.value = '';

    if (!file.name.endsWith('.json')) {
      showToast('Please select a .json backup file', 'error');
      return;
    }

    setIsUploading(true);
    try {
      const result = await backupApi.upload(file);
      if (result.success && result.data) {
        showToast(`Backup uploaded: ${result.data.name}`, 'success');
        await loadBackups();
      } else {
        showToast(result.error ?? 'Failed to upload backup', 'error');
      }
    } catch (error) {
      console.error('Error uploading backup:', error);
      showToast('An error occurred while uploading backup', 'error');
    } finally {
      setIsUploading(false);
    }
  };

  // Download backup handler
  const handleDownloadBackup = async (filename: string) => {
    try {
      await backupApi.download(filename);
      showToast('Backup download started', 'success');
    } catch (error) {
      console.error('Error downloading backup:', error);
      showToast('Failed to download backup', 'error');
    }
  };

  // Restore confirm handler
  const handleRestoreConfirm = async () => {
    if (!restoreTarget) {
      return;
    }

    setIsRestoring(true);
    const target = restoreTarget;
    setRestoreTarget(null);

    try {
      const result = await backupApi.restore(target);
      if (result.success) {
        showToast(result.message ?? 'Backup restored successfully', 'success');
        await loadBackups();
      } else {
        showToast(result.error ?? 'Failed to restore backup', 'error');
      }
    } catch (error) {
      console.error('Error restoring backup:', error);
      showToast('An error occurred while restoring backup', 'error');
    } finally {
      setIsRestoring(false);
    }
  };

  // Delete confirm handler
  const handleDeleteConfirm = async () => {
    if (!deleteTarget) {
      return;
    }

    const target = deleteTarget;
    setDeleteTarget(null);

    try {
      const result = await backupApi.delete(target);
      if (result.success) {
        showToast(`Backup deleted: ${target}`, 'success');
        await loadBackups();
      } else {
        showToast(result.error ?? 'Failed to delete backup', 'error');
      }
    } catch (error) {
      console.error('Error deleting backup:', error);
      showToast('An error occurred while deleting backup', 'error');
    }
  };

  // Only show for admin users
  if (!user?.is_admin) {
    return null;
  }

  return (
    <>
      {/* Backup Actions */}
      <div style={{ display: 'flex', gap: '0.75rem', marginBottom: '1rem' }}>
        <button
          className="btn btn-primary"
          onClick={handleCreateBackup}
          disabled={isCreating || isRestoring}
        >
          {isCreating ? (
            <>
              <span className="spinner-small"></span> Creating...
            </>
          ) : (
            <>
              <i className="fas fa-plus-circle"></i> Create Backup
            </>
          )}
        </button>
        <label
          className="btn btn-secondary"
          style={{ cursor: isUploading || isRestoring ? 'not-allowed' : 'pointer' }}
        >
          {isUploading ? (
            <>
              <span className="spinner-small"></span> Uploading...
            </>
          ) : (
            <>
              <i className="fas fa-upload"></i> Upload Backup
            </>
          )}
          <input
            type="file"
            accept=".json"
            style={{ display: 'none' }}
            onChange={handleUploadBackup}
            disabled={isUploading || isRestoring}
          />
        </label>
      </div>

      {/* Restoring indicator */}
      {isRestoring && (
        <div
          style={{
            padding: '0.75rem 1rem',
            marginBottom: '1rem',
            background: 'var(--warning-bg, #fff3cd)',
            border: '1px solid var(--warning-border, #ffc107)',
            borderRadius: '0.375rem',
            display: 'flex',
            alignItems: 'center',
            gap: '0.5rem',
          }}
        >
          <span className="spinner-small"></span>
          <span>Restoring backup... This may take a moment. Do not close this page.</span>
        </div>
      )}

      {/* Backups Table */}
      <div style={{ overflowX: 'auto' }}>
        <table className="settings-table" style={{ width: '100%' }}>
          <thead>
            <tr>
              <th>Backup Name</th>
              <th>Date Created</th>
              <th>Size</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {isLoading ? (
              <tr>
                <td colSpan={4} style={{ textAlign: 'center', padding: '2rem' }}>
                  <span className="spinner-small"></span> Loading backups...
                </td>
              </tr>
            ) : backups.length === 0 ? (
              <tr>
                <td colSpan={4} style={{ textAlign: 'center', padding: '2rem', color: '#888' }}>
                  No backups found. Create your first backup above.
                </td>
              </tr>
            ) : (
              backups.map((backup) => (
                <tr key={backup.name}>
                  <td style={{ fontFamily: 'monospace', fontSize: '0.85em' }}>{backup.name}</td>
                  <td>{formatBackupDate(backup.date)}</td>
                  <td>{backup.size}</td>
                  <td>
                    <div style={{ display: 'flex', gap: '0.25rem' }}>
                      <button
                        className="btn btn-sm btn-secondary"
                        onClick={() => handleDownloadBackup(backup.name)}
                        title="Download"
                        disabled={isRestoring}
                      >
                        <i className="fas fa-download"></i>
                      </button>
                      <button
                        className="btn btn-sm btn-secondary"
                        onClick={() => setRestoreTarget(backup.name)}
                        title="Restore"
                        disabled={isRestoring}
                      >
                        <i className="fas fa-undo"></i>
                      </button>
                      <button
                        className="btn btn-sm btn-danger"
                        onClick={() => setDeleteTarget(backup.name)}
                        title="Delete"
                        disabled={isRestoring}
                      >
                        <i className="fas fa-trash"></i>
                      </button>
                    </div>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>

      {/* Restore Confirmation Modal */}
      <ConfirmModal
        isOpen={restoreTarget !== null}
        onClose={() => setRestoreTarget(null)}
        onConfirm={handleRestoreConfirm}
        title="Confirm Backup Restore"
        message={`This will replace ALL current data with the backup "${restoreTarget ?? ''}". An automatic backup will be created first. This action cannot be undone.`}
        confirmText="Restore Backup"
        confirmButtonClass="btn-danger"
        icon="fas fa-exclamation-triangle"
      />

      {/* Delete Confirmation Modal */}
      <ConfirmModal
        isOpen={deleteTarget !== null}
        onClose={() => setDeleteTarget(null)}
        onConfirm={handleDeleteConfirm}
        title="Delete Backup"
        message={`Are you sure you want to delete backup "${deleteTarget ?? ''}"? This cannot be undone.`}
        confirmText="Delete"
        confirmButtonClass="btn-danger"
        icon="fas fa-trash"
      />
    </>
  );
}
