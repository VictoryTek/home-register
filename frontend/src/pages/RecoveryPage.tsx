import { useState } from 'react';
import { useNavigate, Link } from 'react-router-dom';
import { authApi } from '@/services/api';
import '@/styles/auth.css';

export function RecoveryPage() {
  const navigate = useNavigate();
  const [step, setStep] = useState<'input' | 'success'>('input');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [showPassword, setShowPassword] = useState(false);
  const [remainingCodes, setRemainingCodes] = useState<number | null>(null);

  const [formData, setFormData] = useState({
    username: '',
    recoveryCode: '',
    newPassword: '',
    confirmPassword: '',
  });

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setFormData((prev) => ({ ...prev, [name]: value }));
    setError(null);
  };

  // Format recovery code with dashes as user types
  const handleRecoveryCodeChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    let value = e.target.value.toUpperCase().replace(/[^A-Z0-9]/g, '');
    // Add dashes at positions 4 and 8
    if (value.length > 4) {
      value = value.slice(0, 4) + '-' + value.slice(4);
    }
    if (value.length > 9) {
      value = value.slice(0, 9) + '-' + value.slice(9);
    }
    // Limit to 14 characters (XXXX-XXXX-XXXX)
    value = value.slice(0, 14);
    setFormData((prev) => ({ ...prev, recoveryCode: value }));
    setError(null);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!formData.username.trim()) {
      setError('Please enter your username');
      return;
    }

    if (!formData.recoveryCode || formData.recoveryCode.length < 14) {
      setError('Please enter a valid recovery code (format: XXXX-XXXX-XXXX)');
      return;
    }

    if (!formData.newPassword) {
      setError('Please enter a new password');
      return;
    }

    if (formData.newPassword.length < 8) {
      setError('Password must be at least 8 characters long');
      return;
    }

    if (formData.newPassword !== formData.confirmPassword) {
      setError('Passwords do not match');
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      const result = await authApi.useRecoveryCode(
        formData.username,
        formData.recoveryCode,
        formData.newPassword
      );

      if (result.success && result.data) {
        setRemainingCodes(result.data.remaining_codes);
        setStep('success');
      } else {
        setError(result.error ?? 'Failed to reset password. Please check your credentials.');
      }
    } catch {
      setError('An error occurred. Please try again.');
    }

    setIsLoading(false);
  };

  if (step === 'success') {
    return (
      <div className="auth-page">
        <div className="auth-background">
          <div className="auth-gradient-orb auth-gradient-orb-1"></div>
          <div className="auth-gradient-orb auth-gradient-orb-2"></div>
          <div className="auth-gradient-orb auth-gradient-orb-3"></div>
        </div>

        <div className="auth-container">
          <div className="auth-card">
            <div className="auth-header">
              <div className="auth-logo">
                <img src="/logo_full.png" alt="Home Registry" className="auth-logo-img" />
              </div>
              <div
                style={{
                  width: '60px',
                  height: '60px',
                  borderRadius: '50%',
                  background: 'var(--success-bg, #d4edda)',
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center',
                  margin: '1rem auto',
                }}
              >
                <i
                  className="fas fa-check"
                  style={{ fontSize: '24px', color: 'var(--success, #28a745)' }}
                ></i>
              </div>
              <h1 className="auth-title">Password Reset!</h1>
              <p className="auth-subtitle">Your password has been successfully changed.</p>
            </div>

            <div
              style={{
                padding: '1rem',
                background: 'var(--surface-alt, #f8f9fa)',
                borderRadius: '8px',
                marginBottom: '1.5rem',
                textAlign: 'center',
              }}
            >
              <p style={{ margin: 0, fontSize: '0.9rem' }}>
                You have <strong>{remainingCodes}</strong> recovery code
                {remainingCodes !== 1 ? 's' : ''} remaining.
                {remainingCodes !== null && remainingCodes <= 2 && (
                  <span
                    style={{
                      display: 'block',
                      marginTop: '0.5rem',
                      color: 'var(--warning, #ffc107)',
                    }}
                  >
                    ⚠️ Consider generating new recovery codes in Settings.
                  </span>
                )}
              </p>
            </div>

            <button onClick={() => navigate('/login')} className="auth-submit-btn">
              <i className="fas fa-sign-in-alt"></i>
              Go to Login
            </button>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="auth-page">
      <div className="auth-background">
        <div className="auth-gradient-orb auth-gradient-orb-1"></div>
        <div className="auth-gradient-orb auth-gradient-orb-2"></div>
        <div className="auth-gradient-orb auth-gradient-orb-3"></div>
      </div>

      <div className="auth-container">
        <div className="auth-card">
          <div className="auth-header">
            <div className="auth-logo">
              <img src="/logo_full.png" alt="Home Registry" className="auth-logo-img" />
            </div>
            <h1 className="auth-title">Account Recovery</h1>
            <p className="auth-subtitle">Use a recovery code to reset your password</p>
          </div>

          {error && (
            <div className="auth-error">
              <i className="fas fa-exclamation-circle"></i>
              <span>{error}</span>
            </div>
          )}

          <form onSubmit={handleSubmit} className="auth-form">
            <div className="form-group">
              <label htmlFor="username">
                <i className="fas fa-user"></i>
                Username
              </label>
              <input
                type="text"
                id="username"
                name="username"
                value={formData.username}
                onChange={handleInputChange}
                placeholder="Enter your username"
                autoFocus
                autoComplete="username"
              />
            </div>

            <div className="form-group">
              <label htmlFor="recoveryCode">
                <i className="fas fa-key"></i>
                Recovery Code
              </label>
              <input
                type="text"
                id="recoveryCode"
                name="recoveryCode"
                value={formData.recoveryCode}
                onChange={handleRecoveryCodeChange}
                placeholder="XXXX-XXXX-XXXX"
                style={{ fontFamily: 'monospace', letterSpacing: '0.1em' }}
                autoComplete="off"
              />
            </div>

            <div className="form-group">
              <label htmlFor="newPassword">
                <i className="fas fa-lock"></i>
                New Password
              </label>
              <div className="input-with-icon">
                <input
                  type={showPassword ? 'text' : 'password'}
                  id="newPassword"
                  name="newPassword"
                  value={formData.newPassword}
                  onChange={handleInputChange}
                  placeholder="Enter new password"
                  autoComplete="new-password"
                />
                <button
                  type="button"
                  className="password-toggle"
                  onClick={() => setShowPassword(!showPassword)}
                  tabIndex={-1}
                >
                  <i className={`fas ${showPassword ? 'fa-eye-slash' : 'fa-eye'}`}></i>
                </button>
              </div>
            </div>

            <div className="form-group">
              <label htmlFor="confirmPassword">
                <i className="fas fa-lock"></i>
                Confirm New Password
              </label>
              <input
                type={showPassword ? 'text' : 'password'}
                id="confirmPassword"
                name="confirmPassword"
                value={formData.confirmPassword}
                onChange={handleInputChange}
                placeholder="Confirm new password"
                autoComplete="new-password"
              />
            </div>

            <button type="submit" className="auth-submit-btn" disabled={isLoading}>
              {isLoading ? (
                <>
                  <span className="btn-spinner"></span>
                  Resetting Password...
                </>
              ) : (
                <>
                  <i className="fas fa-key"></i>
                  Reset Password
                </>
              )}
            </button>
          </form>

          <div className="auth-footer">
            <p>
              <Link to="/login" className="auth-link">
                Back to Login
              </Link>
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
