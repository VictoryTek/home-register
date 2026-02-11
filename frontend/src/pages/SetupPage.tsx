import { useState, useRef } from 'react';
import { useNavigate } from 'react-router-dom';
import { authApi } from '@/services/api';
import { escapeHtml } from '@/utils/security';
import '@/styles/auth.css';

export function SetupPage() {
  const navigate = useNavigate();
  const [step, setStep] = useState(1);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [recoveryCodes, setRecoveryCodes] = useState<string[] | null>(null);
  const [codesConfirmed, setCodesConfirmed] = useState(false);
  const codesRef = useRef<HTMLDivElement>(null);
  
  const [formData, setFormData] = useState({
    username: '',
    full_name: '',
    password: '',
    confirmPassword: '',
    inventory_name: '',
  });

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({ ...prev, [name]: value }));
    setError(null);
  };

  const validateStep1 = () => {
    if (!formData.username.trim()) {
      setError('Username is required');
      return false;
    }
    if (formData.username.length < 3) {
      setError('Username must be at least 3 characters');
      return false;
    }
    if (!formData.full_name.trim()) {
      setError('Full name is required');
      return false;
    }
    return true;
  };

  const validateStep2 = () => {
    if (!formData.password) {
      setError('Password is required');
      return false;
    }
    if (formData.password.length < 8) {
      setError('Password must be at least 8 characters');
      return false;
    }
    if (formData.password !== formData.confirmPassword) {
      setError('Passwords do not match');
      return false;
    }
    return true;
  };

  const handleNext = () => {
    if (step === 1 && validateStep1()) {
      setStep(2);
    } else if (step === 2 && validateStep2()) {
      setStep(3);
    }
  };

  const handleBack = () => {
    setError(null);
    setStep(prev => Math.max(1, prev - 1));
  };

  const handleStep3Submit = async () => {
    setIsLoading(true);
    setError(null);

    await completeSetup();
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    // Prevent form submission - use button handlers instead
  };

  const completeSetup = async () => {
    setIsLoading(true);
    setError(null);

    try {
      const result = await authApi.setup({
        username: formData.username,
        full_name: formData.full_name,
        password: formData.password,
        inventory_name: formData.inventory_name || undefined,
      });

      if (result.success && result.data) {
        // Store auth data
        localStorage.setItem('home_registry_token', result.data.token);
        localStorage.setItem('home_registry_user', JSON.stringify(result.data.user));
        
        // Generate recovery codes
        const codesResponse = await authApi.generateRecoveryCodes();
        if (codesResponse.success && codesResponse.data) {
          setRecoveryCodes(codesResponse.data.codes);
          // Move to recovery codes step
          setStep(4);
        } else {
          setError(codesResponse.error || 'Failed to generate recovery codes');
        }
      } else {
        setError(result.error || 'Setup failed. Please try again.');
      }
    } catch (err) {
      console.error('Setup error:', err);
      setError('Network error. Please check your connection and try again.');
    } finally {
      setIsLoading(false);
    }
  };

  const handleCompleteSetup = async () => {
    if (!codesConfirmed) {
      setError('Please confirm you have saved your recovery codes');
      return;
    }

    try {
      setIsLoading(true);
      await authApi.confirmRecoveryCodes();
      
      // Redirect to app
      navigate('/');
      window.location.reload();
    } catch (err: any) {
      setError('Failed to complete setup');
      setIsLoading(false);
    }
  };

  const downloadCodes = () => {
    if (!recoveryCodes) return;
    
    const content = `Home Registry Recovery Codes\n\nUsername: ${formData.username}\nGenerated: ${new Date().toLocaleString()}\n\nSave these codes in a secure location. Each code can only be used once.\n\n${recoveryCodes.join('\n')}\n`;
    const blob = new Blob([content], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `recovery-codes-${formData.username}.txt`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  const printCodes = () => {
    if (!recoveryCodes) return;
    
    const printWindow = window.open('', '', 'width=800,height=600');
    if (!printWindow) return;
    
    // Build document safely without using document.write with user content
    const doc = printWindow.document;
    doc.open();
    
    // Create safe HTML with escaped user content
    const safeUsername = escapeHtml(formData.username);
    const safeCodes = recoveryCodes.map(code => `<div class="code">${escapeHtml(code)}</div>`).join('');
    
    doc.write(`
      <html>
        <head>
          <title>Recovery Codes</title>
          <style>
            body { font-family: Arial, sans-serif; padding: 20px; }
            h1 { font-size: 24px; margin-bottom: 10px; }
            .info { margin-bottom: 20px; color: #666; }
            .codes { display: grid; grid-template-columns: repeat(2, 1fr); gap: 10px; }
            .code { font-family: monospace; font-size: 14px; padding: 8px; border: 1px solid #ddd; }
            .warning { margin-top: 20px; padding: 10px; background: #fff3cd; border: 1px solid #ffc107; }
          </style>
        </head>
        <body>
          <h1>Home Registry Recovery Codes</h1>
          <div class="info">
            <p><strong>Username:</strong> ${safeUsername}</p>
            <p><strong>Generated:</strong> ${escapeHtml(new Date().toLocaleString())}</p>
          </div>
          <div class="codes">
            ${safeCodes}
          </div>
          <div class="warning">
            <strong>Important:</strong> Each code can only be used once. Store these in a secure location.
          </div>
        </body>
      </html>
    `);
    doc.close();
    printWindow.focus();
    printWindow.print();
    printWindow.close();
  };

  const copyCodes = async () => {
    if (!recoveryCodes) return;
    
    try {
      await navigator.clipboard.writeText(recoveryCodes.join('\n'));
    } catch (err) {
      console.error('Failed to copy codes:', err);
    }
  };

  return (
    <div className="auth-page">
      <div className="auth-background">
        <div className="auth-gradient-orb auth-gradient-orb-1"></div>
        <div className="auth-gradient-orb auth-gradient-orb-2"></div>
        <div className="auth-gradient-orb auth-gradient-orb-3"></div>
      </div>
      
      <div className="auth-container">
      <div className={`auth-card setup-card ${step === 4 ? 'wide' : ''}`}>
        <div className="auth-header">
          <div className="auth-logo">
            <img src="/logo_full.png" alt="Home Registry" className="auth-logo-img" />
          </div>
          <p className="auth-subtitle">Welcome! Let's set up your account.</p>
        </div>

        {/* Progress indicator */}
        <div className="setup-progress">
          <div className={`progress-step ${step >= 1 ? 'active' : ''} ${step > 1 ? 'completed' : ''}`}>
            <div className="step-number">1</div>
            <span>Account</span>
          </div>
          <div className="progress-line"></div>
          <div className={`progress-step ${step >= 2 ? 'active' : ''} ${step > 2 ? 'completed' : ''}`}>
            <div className="step-number">2</div>
            <span>Security</span>
          </div>
          <div className="progress-line"></div>
          <div className={`progress-step ${step >= 3 ? 'active' : ''} ${step > 3 ? 'completed' : ''}`}>
            <div className="step-number">3</div>
            <span>Inventory</span>
          </div>
          <div className="progress-line"></div>
          <div className={`progress-step ${step >= 4 ? 'active' : ''}`}>
            <div className="step-number">4</div>
            <span>Recovery</span>
          </div>
        </div>

        {error && (
          <div className="auth-error">
            <span className="error-icon">⚠️</span>
            {error}
          </div>
        )}

        <form onSubmit={handleSubmit}>
          {/* Step 1: Account Details */}
          {step === 1 && (
            <div className="auth-step">
              <h2>Create Admin Account</h2>
              <p className="step-description">This will be the administrator account for your Home Registry.</p>
              
              <div className="form-group">
                <label htmlFor="full_name">Full Name</label>
                <input
                  type="text"
                  id="full_name"
                  name="full_name"
                  value={formData.full_name}
                  onChange={handleInputChange}
                  placeholder="Enter your full name"
                  autoFocus
                />
              </div>

              <div className="form-group">
                <label htmlFor="username">Username</label>
                <input
                  type="text"
                  id="username"
                  name="username"
                  value={formData.username}
                  onChange={handleInputChange}
                  placeholder="Enter username"
                />
              </div>
            </div>
          )}

          {/* Step 2: Password */}
          {step === 2 && (
            <div className="auth-step">
              <h2>Set Password</h2>
              <p className="step-description">Choose a strong password to secure your account.</p>
              
              <div className="form-group">
                <label htmlFor="password">Password</label>
                <input
                  type="password"
                  id="password"
                  name="password"
                  value={formData.password}
                  onChange={handleInputChange}
                  placeholder="Enter password (min 8 characters)"
                  autoFocus
                />
              </div>

              <div className="form-group">
                <label htmlFor="confirmPassword">Confirm Password</label>
                <input
                  type="password"
                  id="confirmPassword"
                  name="confirmPassword"
                  value={formData.confirmPassword}
                  onChange={handleInputChange}
                  placeholder="Confirm your password"
                />
              </div>
            </div>
          )}

          {/* Step 3: First Inventory */}
          {step === 3 && (
            <div className="auth-step">
              <h2>Create First Inventory</h2>
              <p className="step-description">Optionally create your first inventory to get started.</p>
              
              <div className="form-group">
                <label htmlFor="inventory_name">Inventory Name (Optional)</label>
                <input
                  type="text"
                  id="inventory_name"
                  name="inventory_name"
                  value={formData.inventory_name}
                  onChange={handleInputChange}
                  placeholder="e.g., Home, Office, Garage"
                  autoFocus
                />
                <p className="form-hint">You can skip this and create inventories later.</p>
              </div>
            </div>
          )}

          {/* Step 4: Recovery Codes */}
          {step === 4 && recoveryCodes && (
            <div className="auth-step">
              <h2 style={{ marginBottom: '0.5rem' }}>Save Your Recovery Codes</h2>
              <p className="step-description" style={{ marginBottom: '0.75rem', fontSize: '0.9rem' }}>
                These codes can be used to recover your account if you forget your password. Each code can only be used once.
              </p>
              
              <div className="recovery-codes-display" ref={codesRef}>
                <div className="codes-grid">
                  {recoveryCodes.map((code, index) => (
                    <div key={index} className="code-item">
                      {code}
                    </div>
                  ))}
                </div>
              </div>

              <div className="recovery-actions">
                <button type="button" className="btn-secondary" onClick={downloadCodes}>
                  📥 Download
                </button>
                <button type="button" className="btn-secondary" onClick={copyCodes}>
                  📋 Copy All
                </button>
                <button type="button" className="btn-secondary" onClick={printCodes}>
                  🖨️ Print
                </button>
              </div>

              <div className="form-group" style={{ marginBottom: '0.75rem' }}>
                <label className="checkbox-label">
                  <input
                    type="checkbox"
                    checked={codesConfirmed}
                    onChange={(e) => setCodesConfirmed(e.target.checked)}
                  />
                  <span>I have saved these recovery codes in a secure location</span>
                </label>
              </div>

              <div className="auth-warning">
                <strong>⚠️ Important:</strong> You will not be able to see these codes again. 
                Make sure to save them before continuing.
              </div>
            </div>
          )}

          <div className="auth-actions">
            {step > 1 && step < 4 && (
              <button type="button" className="btn-secondary" onClick={handleBack}>
                Back
              </button>
            )}
            
            {step < 3 ? (
              <button type="button" className="btn-primary" onClick={handleNext}>
                Next
              </button>
            ) : step === 3 ? (
              <button type="button" className="btn-primary" onClick={handleStep3Submit} disabled={isLoading}>
                {isLoading ? 'Creating Account...' : 'Complete Setup'}
              </button>
            ) : (
              <button 
                type="button" 
                className="btn-primary" 
                onClick={handleCompleteSetup}
                disabled={isLoading || !codesConfirmed}
              >
                {isLoading ? 'Finishing...' : 'Finish Setup'}
              </button>
            )}
          </div>
        </form>
      </div>
      </div>
    </div>
  );
}
