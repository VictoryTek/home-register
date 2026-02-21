import { useState, useEffect } from 'react';
import { useApp } from '@/context/AppContext';
import { authApi } from '@/services/api';
import type { TotpStatusResponse, TotpSetupResponse, TotpMode } from '@/types';

const MODE_LABELS: Record<TotpMode, { label: string; description: string }> = {
  '2fa_only': {
    label: '2FA Only',
    description: 'Require authenticator code at every login',
  },
  recovery_only: {
    label: 'Recovery Only',
    description: 'Use authenticator to recover your account if you lose your password',
  },
  both: {
    label: 'Both (Recommended)',
    description: 'Require at login AND use for account recovery',
  },
};

type SetupStep = 'idle' | 'scanning' | 'verifying' | 'success';

export function TotpSettings() {
  const { showToast } = useApp();
  const [status, setStatus] = useState<TotpStatusResponse | null>(null);
  const [loading, setLoading] = useState(true);
  const [setupStep, setSetupStep] = useState<SetupStep>('idle');
  const [setupData, setSetupData] = useState<TotpSetupResponse | null>(null);
  const [verifyCode, setVerifyCode] = useState('');
  const [selectedMode, setSelectedMode] = useState<TotpMode>('both');
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [showDisableConfirm, setShowDisableConfirm] = useState(false);
  const [disablePassword, setDisablePassword] = useState('');
  const [showDisablePassword, setShowDisablePassword] = useState(false);
  const [changingMode, setChangingMode] = useState(false);
  const [showSecretKey, setShowSecretKey] = useState(false);

  useEffect(() => {
    void loadStatus();
  }, []);

  const loadStatus = async () => {
    setLoading(true);
    try {
      const result = await authApi.getTotpStatus();
      if (result.success && result.data) {
        setStatus(result.data);
        if (result.data.mode) {
          setSelectedMode(result.data.mode);
        }
      }
    } catch (error) {
      console.error('Error loading TOTP status:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleStartSetup = async () => {
    setIsSubmitting(true);
    try {
      const result = await authApi.setupTotp();
      if (result.success && result.data) {
        setSetupData(result.data);
        setSetupStep('scanning');
        setSelectedMode('both');
        setVerifyCode('');
      } else {
        showToast(result.error ?? 'Failed to start authenticator setup', 'error');
      }
    } catch {
      showToast('Failed to start authenticator setup', 'error');
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleVerifyCode = (e: React.ChangeEvent<HTMLInputElement>) => {
    // Only allow digits, max 6
    const value = e.target.value.replace(/\D/g, '').slice(0, 6);
    setVerifyCode(value);
  };

  const handleVerifySetup = async (e: React.FormEvent) => {
    e.preventDefault();

    if (verifyCode.length !== 6) {
      showToast('Please enter a 6-digit code', 'error');
      return;
    }

    setIsSubmitting(true);
    try {
      const result = await authApi.verifyTotpSetup(verifyCode, selectedMode);
      if (result.success && result.data) {
        setSetupStep('success');
        showToast('Authenticator enabled successfully!', 'success');
        void loadStatus();
      } else {
        showToast(result.error ?? 'Invalid verification code. Please try again.', 'error');
      }
    } catch {
      showToast('Failed to verify code', 'error');
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleChangeMode = async (newMode: TotpMode) => {
    setChangingMode(true);
    try {
      const result = await authApi.changeTotpMode(newMode);
      if (result.success) {
        setSelectedMode(newMode);
        showToast(`Mode changed to ${MODE_LABELS[newMode].label}`, 'success');
        void loadStatus();
      } else {
        showToast(result.error ?? 'Failed to change mode', 'error');
      }
    } catch {
      showToast('Failed to change mode', 'error');
    } finally {
      setChangingMode(false);
    }
  };

  const handleDisableTotp = async () => {
    if (!disablePassword) {
      showToast('Please enter your password', 'error');
      return;
    }

    setIsSubmitting(true);
    try {
      const result = await authApi.disableTotp(disablePassword);
      if (result.success) {
        showToast('Authenticator disabled successfully', 'success');
        setShowDisableConfirm(false);
        setDisablePassword('');
        setSetupStep('idle');
        setSetupData(null);
        void loadStatus();
      } else {
        showToast(result.error ?? 'Failed to disable authenticator', 'error');
      }
    } catch {
      showToast('Failed to disable authenticator', 'error');
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleCopySecret = async () => {
    if (!setupData?.secret) {
      return;
    }
    try {
      await navigator.clipboard.writeText(setupData.secret);
      showToast('Secret key copied to clipboard', 'success');
    } catch {
      showToast('Failed to copy to clipboard', 'error');
    }
  };

  const handleCancelSetup = () => {
    setSetupStep('idle');
    setSetupData(null);
    setVerifyCode('');
  };

  if (loading) {
    return (
      <div className="totp-settings-container">
        <div style={{ textAlign: 'center', padding: '2rem', color: 'var(--text-secondary)' }}>
          <span className="spinner-small"></span> Loading authenticator status...
        </div>
      </div>
    );
  }

  // Success state after setup
  if (setupStep === 'success') {
    return (
      <div className="totp-settings-container">
        <div className="totp-success">
          <div className="totp-success-icon">
            <i className="fas fa-check-circle"></i>
          </div>
          <h3>Authenticator Enabled!</h3>
          <p>Your account is now protected with two-factor authentication.</p>
          <button
            className="btn btn-primary"
            onClick={() => {
              setSetupStep('idle');
              setSetupData(null);
            }}
          >
            Done
          </button>
        </div>
      </div>
    );
  }

  // Setup flow - scanning QR code + verifying
  if (setupStep === 'scanning' && setupData) {
    return (
      <div className="totp-settings-container">
        <div className="totp-setup-flow">
          <h3 className="totp-setup-title">
            <i className="fas fa-mobile-alt"></i>
            Set Up Authenticator
          </h3>
          <p className="totp-setup-description">
            Scan the QR code below with your authenticator app (Google Authenticator, Authy, etc.)
          </p>

          {/* QR Code */}
          <div className="totp-qr-container">
            {setupData.qr_code_data_uri.startsWith('data:image/') ? (
              <img
                src={setupData.qr_code_data_uri}
                alt="TOTP QR Code - scan with your authenticator app"
                className="totp-qr-image"
              />
            ) : (
              <p style={{ color: 'var(--text-secondary)', textAlign: 'center', padding: '1rem' }}>
                Unable to display QR code. Please use manual entry below.
              </p>
            )}
          </div>

          {/* Manual entry */}
          <div className="totp-manual-entry">
            <button
              type="button"
              className="totp-manual-toggle"
              onClick={() => setShowSecretKey(!showSecretKey)}
            >
              <i className={`fas fa-chevron-${showSecretKey ? 'up' : 'down'}`}></i>
              Can&apos;t scan? Enter key manually
            </button>
            {showSecretKey && (
              <div className="totp-secret-display">
                <code className="totp-secret-code">{setupData.secret}</code>
                <button
                  type="button"
                  className="btn btn-secondary btn-sm totp-copy-btn"
                  onClick={handleCopySecret}
                  aria-label="Copy secret key to clipboard"
                >
                  <i className="fas fa-copy"></i>
                  Copy
                </button>
                <p
                  className="totp-secret-warning"
                  style={{
                    margin: '0.75rem 0 0',
                    padding: '0.5rem 0.75rem',
                    fontSize: '0.85rem',
                    color: 'var(--warning-text, #856404)',
                    background: 'var(--warning-bg, #fff3cd)',
                    borderRadius: '6px',
                    display: 'flex',
                    alignItems: 'center',
                    gap: '0.5rem',
                  }}
                >
                  <i className="fas fa-exclamation-triangle"></i>
                  Keep this secret safe — anyone with this key can generate codes for your account.
                </p>
              </div>
            )}
          </div>

          {/* Verification form */}
          <form onSubmit={handleVerifySetup} className="totp-verify-form">
            <div className="totp-code-input-group">
              <label htmlFor="totp-verify-code">
                <i className="fas fa-key"></i>
                Enter the 6-digit code from your app
              </label>
              <input
                type="text"
                id="totp-verify-code"
                value={verifyCode}
                onChange={handleVerifyCode}
                placeholder="000000"
                className="totp-code-input"
                autoComplete="one-time-code"
                inputMode="numeric"
                maxLength={6}
                autoFocus
                aria-label="Enter 6-digit verification code from your authenticator app"
              />
            </div>

            {/* Mode selection */}
            <div className="totp-mode-selection">
              <label className="totp-mode-label">Authentication Mode</label>
              {(Object.keys(MODE_LABELS) as TotpMode[]).map((mode) => (
                <label key={mode} className="totp-mode-option">
                  <input
                    type="radio"
                    name="totp-mode"
                    value={mode}
                    checked={selectedMode === mode}
                    onChange={() => setSelectedMode(mode)}
                  />
                  <div className="totp-mode-content">
                    <span className="totp-mode-name">{MODE_LABELS[mode].label}</span>
                    <span className="totp-mode-desc">{MODE_LABELS[mode].description}</span>
                  </div>
                </label>
              ))}
            </div>

            <div className="totp-setup-actions">
              <button
                type="button"
                className="btn btn-secondary"
                onClick={handleCancelSetup}
                disabled={isSubmitting}
              >
                Cancel
              </button>
              <button
                type="submit"
                className="btn btn-primary"
                disabled={isSubmitting || verifyCode.length !== 6}
              >
                {isSubmitting ? (
                  <>
                    <span className="spinner-small"></span>
                    Verifying...
                  </>
                ) : (
                  <>
                    <i className="fas fa-shield-alt"></i>
                    Verify &amp; Enable
                  </>
                )}
              </button>
            </div>
          </form>
        </div>
      </div>
    );
  }

  // Disable confirmation dialog
  if (showDisableConfirm) {
    return (
      <div className="totp-settings-container">
        <div className="totp-disable-confirm">
          <h3 className="totp-disable-title">
            <i className="fas fa-exclamation-triangle"></i>
            Disable Authenticator
          </h3>
          <p className="totp-disable-warning">
            This will remove two-factor authentication from your account. Enter your password to
            confirm.
          </p>

          <div className="totp-disable-form">
            <div className="totp-password-group">
              <label htmlFor="disable-password">
                <i className="fas fa-lock"></i>
                Password
              </label>
              <div className="input-with-icon-settings">
                <input
                  type={showDisablePassword ? 'text' : 'password'}
                  id="disable-password"
                  value={disablePassword}
                  onChange={(e) => setDisablePassword(e.target.value)}
                  placeholder="Enter your password"
                  autoComplete="current-password"
                />
                <button
                  type="button"
                  className="password-toggle-settings"
                  onClick={() => setShowDisablePassword(!showDisablePassword)}
                  tabIndex={-1}
                >
                  <i className={`fas ${showDisablePassword ? 'fa-eye-slash' : 'fa-eye'}`}></i>
                </button>
              </div>
            </div>

            <div className="totp-setup-actions">
              <button
                type="button"
                className="btn btn-secondary"
                onClick={() => {
                  setShowDisableConfirm(false);
                  setDisablePassword('');
                }}
                disabled={isSubmitting}
              >
                Cancel
              </button>
              <button
                type="button"
                className="btn btn-danger"
                onClick={handleDisableTotp}
                disabled={isSubmitting || !disablePassword}
              >
                {isSubmitting ? (
                  <>
                    <span className="spinner-small"></span>
                    Disabling...
                  </>
                ) : (
                  <>
                    <i className="fas fa-trash-alt"></i>
                    Disable 2FA
                  </>
                )}
              </button>
            </div>
          </div>
        </div>
      </div>
    );
  }

  // Main state - TOTP enabled
  if (status?.is_enabled) {
    return (
      <div className="totp-settings-container">
        <div className="totp-enabled-status">
          <div className="totp-status-badge totp-status-enabled">
            <i className="fas fa-shield-alt"></i>
            <span>Authenticator Enabled</span>
          </div>

          {status.last_used_at && (
            <p className="totp-last-used">
              Last used: {new Date(status.last_used_at).toLocaleDateString()}
            </p>
          )}

          {/* Current mode + change option */}
          <div className="totp-mode-current">
            <label className="totp-mode-label">Current Mode</label>
            <div className="totp-mode-options-list">
              {(Object.keys(MODE_LABELS) as TotpMode[]).map((mode) => (
                <label
                  key={mode}
                  className={`totp-mode-option ${status.mode === mode ? 'active' : ''}`}
                >
                  <input
                    type="radio"
                    name="totp-mode-change"
                    value={mode}
                    checked={status.mode === mode}
                    onChange={() => handleChangeMode(mode)}
                    disabled={changingMode}
                  />
                  <div className="totp-mode-content">
                    <span className="totp-mode-name">{MODE_LABELS[mode].label}</span>
                    <span className="totp-mode-desc">{MODE_LABELS[mode].description}</span>
                  </div>
                </label>
              ))}
            </div>
          </div>

          <button
            className="btn btn-danger totp-disable-btn"
            onClick={() => setShowDisableConfirm(true)}
            aria-label="Disable two-factor authentication"
          >
            <i className="fas fa-times-circle"></i>
            Disable Authenticator
          </button>
        </div>
      </div>
    );
  }

  // Main state - TOTP not enabled
  return (
    <div className="totp-settings-container">
      <div className="totp-not-enabled">
        <div className="totp-status-badge totp-status-disabled">
          <i className="fas fa-shield-alt"></i>
          <span>Authenticator Not Set Up</span>
        </div>
        <p className="totp-info-text">
          Add an extra layer of security to your account by requiring an authenticator code when
          signing in. Works with apps like Google Authenticator, Authy, or any TOTP-compatible app.
        </p>
        <button className="btn btn-primary" onClick={handleStartSetup} disabled={isSubmitting}>
          {isSubmitting ? (
            <>
              <span className="spinner-small"></span>
              Setting up...
            </>
          ) : (
            <>
              <i className="fas fa-plus-circle"></i>
              Enable Authenticator
            </>
          )}
        </button>
      </div>
    </div>
  );
}
