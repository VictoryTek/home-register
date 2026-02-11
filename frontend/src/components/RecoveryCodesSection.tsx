import { useState, useEffect, useRef } from 'react';
import { useApp } from '@/context/AppContext';
import type { RecoveryCodesStatus } from '@/types';
import { authApi } from '@/services/api';
import { escapeHtml } from '@/utils/security';

interface RecoveryCodesSectionProps {
  onCodesGenerated?: () => void;
}

export function RecoveryCodesSection({ onCodesGenerated }: RecoveryCodesSectionProps) {
  const { showToast } = useApp();
  const [status, setStatus] = useState<RecoveryCodesStatus | null>(null);
  const [codes, setCodes] = useState<string[] | null>(null);
  const [loading, setLoading] = useState(false);
  const [generating, setGenerating] = useState(false);
  const [confirming, setConfirming] = useState(false);
  const [showConfirmDialog, setShowConfirmDialog] = useState(false);
  const codesRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    loadStatus();
  }, []);

  const loadStatus = async () => {
    setLoading(true);
    try {
      const result = await authApi.getRecoveryCodesStatus();
      if (result.success && result.data) {
        setStatus(result.data);
      }
    } catch (error) {
      console.error('Error loading recovery codes status:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleGenerate = async () => {
    // Show confirmation if user already has codes
    if (status?.has_codes) {
      setShowConfirmDialog(true);
      return;
    }
    await generateCodes();
  };

  const generateCodes = async () => {
    setGenerating(true);
    setShowConfirmDialog(false);
    try {
      const result = await authApi.generateRecoveryCodes();
      if (result.success && result.data) {
        setCodes(result.data.codes);
        showToast('Recovery codes generated! Save them now.', 'success');
        // Reload status (codes are unconfirmed)
        await loadStatus();
        onCodesGenerated?.();
      } else {
        showToast(result.error || 'Failed to generate recovery codes', 'error');
      }
    } catch (error) {
      showToast('Failed to generate recovery codes', 'error');
    } finally {
      setGenerating(false);
    }
  };

  const handleConfirmSaved = async () => {
    setConfirming(true);
    try {
      const result = await authApi.confirmRecoveryCodes();
      if (result.success) {
        showToast('Recovery codes confirmed!', 'success');
        setCodes(null); // Hide codes after confirmation
        await loadStatus();
      } else {
        showToast(result.error || 'Failed to confirm recovery codes', 'error');
      }
    } catch (error) {
      showToast('Failed to confirm recovery codes', 'error');
    } finally {
      setConfirming(false);
    }
  };

  const handleCopyAll = async () => {
    if (!codes) return;
    try {
      await navigator.clipboard.writeText(codes.join('\n'));
      showToast('Codes copied to clipboard!', 'success');
    } catch (error) {
      showToast('Failed to copy codes', 'error');
    }
  };

  const handleDownload = () => {
    if (!codes) return;
    const content = [
      'Home Registry - Recovery Codes',
      '================================',
      '',
      'IMPORTANT: Keep these codes safe!',
      'Each code can only be used once.',
      '',
      ...codes.map((code, i) => `${i + 1}. ${code}`),
      '',
      `Generated: ${new Date().toLocaleString()}`,
    ].join('\n');

    const blob = new Blob([content], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'home-registry-recovery-codes.txt';
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
    showToast('Recovery codes downloaded!', 'success');
  };

  const handlePrint = () => {
    if (!codes) return;
    
    const printWindow = window.open('', '_blank');
    if (!printWindow) return;
    
    // Build document safely without using innerHTML
    const doc = printWindow.document;
    doc.open();
    
    // Create safe HTML with escaped content - build from data, not DOM
    const safeCodes = codes.map(code => `<div class="code">${escapeHtml(code)}</div>`).join('');
    
    doc.write(`
      <html>
        <head>
          <title>Recovery Codes - Home Registry</title>
          <style>
            body { font-family: monospace; padding: 20px; }
            h2 { margin-bottom: 10px; }
            .warning { color: #dc3545; font-weight: bold; margin-bottom: 20px; }
            .code { 
              font-size: 18px; 
              padding: 8px 16px; 
              margin: 4px 0;
              background: #f5f5f5;
              border-radius: 4px;
            }
          </style>
        </head>
        <body>
          <h2>Home Registry - Recovery Codes</h2>
          <p class="warning">IMPORTANT: Keep these codes safe! Each code can only be used once.</p>
          <div class="codes">${safeCodes}</div>
          <p style="margin-top: 20px; color: #666;">Generated: ${escapeHtml(new Date().toLocaleString())}</p>
        </body>
      </html>
    `);
    doc.close();
    printWindow.print();
  };

  if (loading) {
    return (
      <div className="recovery-codes-section">
        <p style={{ textAlign: 'center', padding: '1rem', color: 'var(--text-secondary)' }}>
          Loading...
        </p>
      </div>
    );
  }

  return (
    <div className="recovery-codes-section">
      {/* Status display */}
      {!codes && status && (
        <div className="recovery-codes-status" style={{ marginBottom: '1.5rem' }}>
          {status.has_codes ? (
            <div className="status-card" style={{ 
              padding: '1rem', 
              background: status.codes_confirmed ? 'var(--success-bg, #d4edda)' : 'var(--warning-bg, #fff3cd)',
              borderRadius: '8px',
              border: `1px solid ${status.codes_confirmed ? 'var(--success-border, #c3e6cb)' : 'var(--warning-border, #ffeeba)'}`,
            }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', marginBottom: '0.5rem' }}>
                <i className={`fas ${status.codes_confirmed ? 'fa-check-circle' : 'fa-exclamation-triangle'}`} 
                   style={{ color: status.codes_confirmed ? 'var(--success, #28a745)' : 'var(--warning, #ffc107)' }}></i>
                <strong>{status.codes_confirmed ? 'Recovery codes active' : 'Recovery codes pending confirmation'}</strong>
              </div>
              <p style={{ margin: 0, fontSize: '0.9rem', color: 'var(--text-secondary)' }}>
                You have <strong>{status.unused_count}</strong> unused recovery code{status.unused_count !== 1 ? 's' : ''}.
                {status.generated_at && (
                  <span> Generated on {new Date(status.generated_at).toLocaleDateString()}.</span>
                )}
              </p>
              {!status.codes_confirmed && (
                <p style={{ margin: '0.5rem 0 0', fontSize: '0.85rem', color: 'var(--warning-dark, #856404)' }}>
                  ⚠️ Please generate new codes and confirm you've saved them.
                </p>
              )}
            </div>
          ) : (
            <div className="status-card" style={{ 
              padding: '1rem', 
              background: 'var(--danger-bg, #f8d7da)',
              borderRadius: '8px',
              border: '1px solid var(--danger-border, #f5c6cb)',
            }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', marginBottom: '0.5rem' }}>
                <i className="fas fa-exclamation-circle" style={{ color: 'var(--danger, #dc3545)' }}></i>
                <strong>No recovery codes</strong>
              </div>
              <p style={{ margin: 0, fontSize: '0.9rem', color: 'var(--text-secondary)' }}>
                Generate recovery codes to ensure you can regain access to your account if you forget your password.
              </p>
            </div>
          )}
        </div>
      )}

      {/* Generate button */}
      {!codes && (
        <button 
          className="btn btn-primary" 
          onClick={handleGenerate}
          disabled={generating}
          style={{ marginBottom: '1rem' }}
        >
          {generating ? (
            <>
              <span className="spinner-small"></span>
              Generating...
            </>
          ) : (
            <>
              <i className="fas fa-key"></i>
              {status?.has_codes ? 'Regenerate Recovery Codes' : 'Generate Recovery Codes'}
            </>
          )}
        </button>
      )}

      {/* Confirmation dialog for regenerating */}
      {showConfirmDialog && (
        <div className="modal-overlay" onClick={() => setShowConfirmDialog(false)}>
          <div className="modal" onClick={e => e.stopPropagation()} style={{ maxWidth: '400px' }}>
            <div className="modal-header">
              <h3>Regenerate Recovery Codes?</h3>
              <button className="modal-close" onClick={() => setShowConfirmDialog(false)}>
                <i className="fas fa-times"></i>
              </button>
            </div>
            <div className="modal-body">
              <p>
                <strong>Warning:</strong> This will invalidate all your existing recovery codes.
                Make sure you no longer need your old codes before proceeding.
              </p>
            </div>
            <div className="modal-footer">
              <button className="btn btn-secondary" onClick={() => setShowConfirmDialog(false)}>
                Cancel
              </button>
              <button className="btn btn-danger" onClick={generateCodes} disabled={generating}>
                {generating ? 'Generating...' : 'Regenerate'}
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Display generated codes */}
      {codes && (
        <div className="recovery-codes-display">
          <div className="alert alert-warning" style={{ marginBottom: '1rem' }}>
            <i className="fas fa-exclamation-triangle"></i>
            <div>
              <strong>Save these codes now!</strong>
              <p style={{ margin: '0.5rem 0 0', fontSize: '0.9rem' }}>
                You won't be able to see these codes again. Store them somewhere safe like a password manager.
              </p>
            </div>
          </div>

          <div ref={codesRef} className="codes-grid" style={{
            display: 'grid',
            gridTemplateColumns: 'repeat(2, 1fr)',
            gap: '0.5rem',
            marginBottom: '1rem',
          }}>
            {codes.map((code, index) => (
              <div key={index} className="code" style={{
                fontFamily: 'monospace',
                fontSize: '1rem',
                padding: '0.75rem',
                background: 'var(--surface-alt, #f8f9fa)',
                borderRadius: '4px',
                textAlign: 'center',
                userSelect: 'all',
              }}>
                {code}
              </div>
            ))}
          </div>

          <div className="codes-actions" style={{ display: 'flex', gap: '0.5rem', flexWrap: 'wrap', marginBottom: '1.5rem' }}>
            <button className="btn btn-secondary btn-sm" onClick={handleCopyAll}>
              <i className="fas fa-copy"></i> Copy All
            </button>
            <button className="btn btn-secondary btn-sm" onClick={handleDownload}>
              <i className="fas fa-download"></i> Download
            </button>
            <button className="btn btn-secondary btn-sm" onClick={handlePrint}>
              <i className="fas fa-print"></i> Print
            </button>
          </div>

          <div style={{ 
            padding: '1rem', 
            background: 'var(--surface-alt, #f8f9fa)', 
            borderRadius: '8px',
            border: '1px solid var(--border, #dee2e6)',
          }}>
            <label style={{ display: 'flex', alignItems: 'flex-start', gap: '0.75rem', cursor: 'pointer' }}>
              <input 
                type="checkbox" 
                onChange={(e) => e.target.checked && handleConfirmSaved()}
                disabled={confirming}
                style={{ marginTop: '0.25rem' }}
              />
              <span>
                <strong>I have saved these recovery codes</strong>
                <p style={{ margin: '0.25rem 0 0', fontSize: '0.85rem', color: 'var(--text-secondary)' }}>
                  I understand that I won't be able to see these codes again and that each code can only be used once.
                </p>
              </span>
            </label>
          </div>
        </div>
      )}

      {/* Help text */}
      <div style={{ marginTop: '1.5rem', fontSize: '0.85rem', color: 'var(--text-secondary)' }}>
        <h4 style={{ fontSize: '0.95rem', marginBottom: '0.5rem', color: 'var(--text-primary)' }}>
          <i className="fas fa-info-circle"></i> How recovery codes work
        </h4>
        <ul style={{ margin: 0, paddingLeft: '1.25rem' }}>
          <li>Each code can only be used once to reset your password</li>
          <li>Store them securely (password manager, safe, printed copy)</li>
          <li>If you run low on codes, generate new ones</li>
          <li>Regenerating codes invalidates all previous codes</li>
        </ul>
      </div>
    </div>
  );
}
