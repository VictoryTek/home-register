import { useState, useEffect } from 'react';
import { createPortal } from 'react-dom';
import { useAuth } from '@/context/AuthContext';
import { useApp } from '@/context/AppContext';

export function InstructionsModal() {
  const { settings, updateSettings } = useAuth();
  const { showToast } = useApp();
  const [isChecked, setIsChecked] = useState(false);
  const [isSaving, setIsSaving] = useState(false);
  const [sessionDismissed, setSessionDismissed] = useState(() => {
    return sessionStorage.getItem('home_registry_instructions_dismissed') === 'true';
  });

  // Don't show if settings not loaded yet or already acknowledged
  const isAcknowledged = settings?.settings_json.instructionsAcknowledged === true;

  // Prevent body scroll when modal is open
  useEffect(() => {
    if (!settings || isAcknowledged || sessionDismissed) {
      return undefined;
    }
    document.body.style.overflow = 'hidden';
    return () => {
      document.body.style.overflow = '';
    };
  }, [settings, isAcknowledged, sessionDismissed]);

  if (!settings || isAcknowledged || sessionDismissed) {
    return null;
  }

  const handleSessionDismiss = () => {
    sessionStorage.setItem('home_registry_instructions_dismissed', 'true');
    setSessionDismissed(true);
  };

  const handleConfirm = async () => {
    if (!isChecked) {
      return;
    }

    setIsSaving(true);
    const success = await updateSettings({
      settings_json: {
        ...settings.settings_json,
        instructionsAcknowledged: true,
      },
    });

    if (!success) {
      showToast('Failed to save. Please try again.', 'error');
    }
    setIsSaving(false);
  };

  const modalContent = (
    <div
      className="modal-overlay active instructions-modal-overlay"
      role="dialog"
      aria-modal="true"
      aria-labelledby="instructions-title"
    >
      <div
        className="modal-content instructions-modal"
        style={{ maxWidth: '650px' }}
        onClick={(e) => e.stopPropagation()}
      >
        <div className="modal-header instructions-modal-header">
          <button
            className="instructions-close-btn"
            onClick={handleSessionDismiss}
            aria-label="Close"
            title="Close (will appear again next login)"
          >
            <i className="fas fa-times"></i>
          </button>
          <div className="instructions-welcome-icon">
            <i className="fas fa-home"></i>
          </div>
          <h2 className="modal-title" id="instructions-title">
            Welcome to Home Registry!
          </h2>
          <p className="modal-subtitle">Get up and running in 3 simple steps</p>
        </div>

        <div className="modal-body instructions-modal-body">
          <div className="instructions-steps">
            <div className="instructions-step">
              <div className="step-number-badge">1</div>
              <div className="instructions-step-icon">
                <i className="fas fa-boxes-stacked"></i>
              </div>
              <div className="instructions-text">
                <h3>Create Your First Inventory</h3>
                <p>
                  Head to the Inventories page and click <strong>Add Inventory</strong>. Name it
                  after a room or category &mdash; like &quot;Kitchen&quot; or
                  &quot;Electronics.&quot;
                </p>
              </div>
            </div>

            <div className="instructions-step-connector">
              <i className="fas fa-arrow-down"></i>
            </div>

            <div className="instructions-step">
              <div className="step-number-badge">2</div>
              <div className="instructions-step-icon">
                <i className="fas fa-tags"></i>
              </div>
              <div className="instructions-text">
                <h3>Set Up Organizers</h3>
                <p>
                  Before adding items, go to your inventory&apos;s <strong>Organizers</strong> tab.
                  Add custom fields you want to track &mdash; like &quot;Serial Number,&quot;
                  &quot;Condition,&quot; or &quot;Location.&quot;
                </p>
              </div>
            </div>

            <div className="instructions-step-connector">
              <i className="fas fa-arrow-down"></i>
            </div>

            <div className="instructions-step">
              <div className="step-number-badge">3</div>
              <div className="instructions-step-icon">
                <i className="fas fa-cube"></i>
              </div>
              <div className="instructions-text">
                <h3>Add Items</h3>
                <p>
                  Click <strong>Add Item</strong> inside your inventory. Fill in the details &mdash;
                  name, price, warranty date, and any custom organizer fields you created.
                </p>
              </div>
            </div>
          </div>

          <div className="instructions-tip">
            <i className="fas fa-lightbulb"></i>
            <p>
              <strong>Tip:</strong> You can always edit, reorganize, or add more inventories and
              organizers later. Start small and build from there!
            </p>
          </div>
        </div>

        <div className="modal-footer instructions-modal-footer">
          <label className="instructions-checkbox-label">
            <input
              type="checkbox"
              checked={isChecked}
              onChange={(e) => setIsChecked(e.target.checked)}
              className="instructions-checkbox"
            />
            <span>I have read this and don&apos;t want to see it again</span>
          </label>
          <button
            className="btn btn-primary instructions-confirm-btn"
            disabled={!isChecked || isSaving}
            onClick={handleConfirm}
          >
            {isSaving ? (
              <>
                <span className="btn-spinner"></span>
                Saving...
              </>
            ) : (
              <>
                <i className="fas fa-rocket"></i>
                Get Started
              </>
            )}
          </button>
        </div>
      </div>
    </div>
  );

  return createPortal(modalContent, document.body);
}
