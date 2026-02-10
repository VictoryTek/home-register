import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { authApi } from '@/services/api';
import '@/styles/auth.css';

export function SetupPage() {
  const navigate = useNavigate();
  const [step, setStep] = useState(1);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
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

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
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
        
        // Navigate to main app
        navigate('/');
        window.location.reload(); // Refresh to update auth state
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

  return (
    <div className="auth-page">
      <div className="auth-background">
        <div className="auth-gradient-orb auth-gradient-orb-1"></div>
        <div className="auth-gradient-orb auth-gradient-orb-2"></div>
        <div className="auth-gradient-orb auth-gradient-orb-3"></div>
      </div>
      
      <div className="auth-container">
      <div className="auth-card setup-card">
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
          <div className={`progress-step ${step >= 3 ? 'active' : ''}`}>
            <div className="step-number">3</div>
            <span>Inventory</span>
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

          <div className="auth-actions">
            {step > 1 && (
              <button type="button" className="btn-secondary" onClick={handleBack}>
                Back
              </button>
            )}
            
            {step < 3 ? (
              <button type="button" className="btn-primary" onClick={handleNext}>
                Next
              </button>
            ) : (
              <button type="submit" className="btn-primary" disabled={isLoading}>
                {isLoading ? 'Creating Account...' : 'Complete Setup'}
              </button>
            )}
          </div>
        </form>
      </div>
      </div>
    </div>
  );
}
