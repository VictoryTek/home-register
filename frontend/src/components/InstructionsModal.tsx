import { useState, useEffect } from 'react';
import { createPortal } from 'react-dom';
import { useAuth } from '@/context/AuthContext';
import { useApp } from '@/context/AppContext';

export function InstructionsModal() {
  const { settings, updateSettings } = useAuth();
  const { showToast } = useApp();
  const [isChecked, setIsChecked] = useState(false);
  const [isSaving, setIsSaving] = useState(false);

  // Don't show if settings not loaded yet or already acknowledged
  const isAcknowledged = settings?.settings_json.instructionsAcknowledged === true;

  // Prevent body scroll when modal is open
  useEffect(() => {
    if (!settings || isAcknowledged) {
      return undefined;
    }
    document.body.style.overflow = 'hidden';
    return () => {
      document.body.style.overflow = '';
    };
  }, [settings, isAcknowledged]);

  if (!settings || isAcknowledged) {
    return null;
  }

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
          <div className="instructions-welcome-icon">
            <i className="fas fa-home"></i>
          </div>
          <h2 className="modal-title" id="instructions-title">
            Welcome to Home Registry!
          </h2>
          <p className="modal-subtitle">Here&apos;s a quick guide to help you get started</p>
        </div>

        <div className="modal-body instructions-modal-body">
          <div className="instructions-section">
            <div className="instructions-icon">
              <i className="fas fa-boxes-stacked"></i>
            </div>
            <div className="instructions-text">
              <h3>Inventories</h3>
              <p>
                Think of inventories as containers for your belongings. You might create one for
                each room in your home (Kitchen, Garage, Bedroom) or for different categories
                (Electronics, Tools, Documents). Create as many as you need!
              </p>
            </div>
          </div>

          <div className="instructions-section">
            <div className="instructions-icon">
              <i className="fas fa-cube"></i>
            </div>
            <div className="instructions-text">
              <h3>Items</h3>
              <p>
                Inside each inventory, you can add items — the actual things you own. For each item,
                you can record details like its name, description, purchase date, price, warranty
                expiration, and more. The more details you add, the more useful your registry
                becomes!
              </p>
            </div>
          </div>

          <div className="instructions-section">
            <div className="instructions-icon">
              <i className="fas fa-tags"></i>
            </div>
            <div className="instructions-text">
              <h3>Organizers</h3>
              <p>
                Organizers let you create custom fields to track your items within an inventory. For
                example, you could create an organizer for &quot;Serial Number&quot;, or
                &quot;Condition&quot; with options like &quot;New,&quot; &quot;Good,&quot; or
                &quot;Needs Repair.&quot; Each inventory created automatically generates its own set
                of organizers. You should create your custom organizers for the inventory before
                adding items.
              </p>
            </div>
          </div>

          <div className="instructions-section">
            <div className="instructions-icon">
              <i className="fas fa-bell"></i>
            </div>
            <div className="instructions-text">
              <h3>Warranty Notifications</h3>
              <p>
                When you add warranty expiration dates to your items, Home Registry will
                automatically notify you when warranties are about to expire or have already
                expired. Never miss a warranty claim again!
              </p>
            </div>
          </div>

          <div className="instructions-section">
            <div className="instructions-icon">
              <i className="fas fa-share-nodes"></i>
            </div>
            <div className="instructions-text">
              <h3>Sharing</h3>
              <p>
                You can share your inventories with other users in your household. Choose whether
                they can just view your inventory or also make changes to it. Great for families
                managing a home together!
              </p>
            </div>
          </div>

          <div className="instructions-section">
            <div className="instructions-icon">
              <i className="fas fa-chart-bar"></i>
            </div>
            <div className="instructions-text">
              <h3>Reports</h3>
              <p>
                Generate reports to see the total value of your belongings, breakdowns by category,
                and other useful statistics. Perfect for insurance purposes or simply keeping track
                of what you own.
              </p>
            </div>
          </div>

          <div className="instructions-section">
            <div className="instructions-icon">
              <i className="fas fa-database"></i>
            </div>
            <div className="instructions-text">
              <h3>Backups</h3>
              <p>
                Regularly back up your data to keep it safe. You can create backups from the
                Settings page and restore them at any time. Your data is important — protect it!
              </p>
            </div>
          </div>

          <div className="instructions-tip">
            <i className="fas fa-lightbulb"></i>
            <p>
              <strong>Pro tip:</strong> Start by creating your first inventory (like &quot;Living
              Room&quot; or &quot;Kitchen&quot;), then add organizers for fields you want to track,
              then add a few items to get familiar with how everything works. You can always edit or
              reorganize later!
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
            <span>I have read and understand these instructions</span>
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
