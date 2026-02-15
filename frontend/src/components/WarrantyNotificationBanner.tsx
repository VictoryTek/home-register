import { useNavigate } from 'react-router-dom';
import { useApp } from '@/context/AppContext';
import { useAuth } from '@/context/AuthContext';
import { getNotificationMessage } from '@/utils/notifications';

export function WarrantyNotificationBanner() {
  const { warrantyNotifications } = useApp();
  const { settings } = useAuth();
  const navigate = useNavigate();

  // RECOMMENDED FIX: Removed redundant filtering - AppContext now filters at source
  // warrantyNotifications already excludes dismissed notifications
  const activeNotifications = warrantyNotifications;

  // Don't show if notifications are disabled or there are no active notifications
  if (!settings?.notifications_enabled || activeNotifications.length === 0) {
    return null;
  }

  // Group by status
  const expired = activeNotifications.filter((n) => n.status === 'expired');
  const expiringSoon = activeNotifications.filter((n) => n.status === 'expiring-soon');
  const expiringThisMonth = activeNotifications.filter((n) => n.status === 'expiring-this-month');

  const handleNotificationClick = (notification: (typeof activeNotifications)[0]) => {
    navigate(`/inventory/${notification.inventoryId}`, {
      state: { openItemId: notification.id },
    });
  };

  return (
    <div
      style={{
        background: 'var(--bg-secondary)',
        borderRadius: 'var(--radius-lg)',
        padding: '1.25rem',
        marginBottom: '1.5rem',
        border: '1px solid var(--border-color)',
      }}
    >
      <div
        style={{
          display: 'flex',
          alignItems: 'center',
          gap: '0.75rem',
          marginBottom: '1rem',
        }}
      >
        <i
          className="fas fa-bell"
          style={{ color: 'var(--warning-color)', fontSize: '1.25rem' }}
        ></i>
        <h3
          style={{
            margin: 0,
            fontSize: '1.125rem',
            fontWeight: 600,
            color: 'var(--text-primary)',
          }}
        >
          Warranty Notifications ({activeNotifications.length})
        </h3>
      </div>

      <div style={{ display: 'flex', flexDirection: 'column', gap: '0.75rem' }}>
        {expired.length > 0 && (
          <div>
            <div
              style={{
                fontSize: '0.875rem',
                fontWeight: 600,
                color: 'var(--danger-color)',
                marginBottom: '0.5rem',
              }}
            >
              <i className="fas fa-exclamation-circle"></i> Expired Warranties ({expired.length})
            </div>
            {expired.slice(0, 3).map((notification) => (
              <div
                key={notification.id}
                onClick={() => handleNotificationClick(notification)}
                style={{
                  padding: '0.75rem',
                  background: 'var(--bg-primary)',
                  borderRadius: 'var(--radius-md)',
                  marginBottom: '0.5rem',
                  cursor: 'pointer',
                  border: '1px solid var(--danger-color)',
                  transition: 'all 0.2s ease',
                }}
                onMouseEnter={(e) => {
                  e.currentTarget.style.borderColor = 'var(--danger-dark)';
                  e.currentTarget.style.transform = 'translateX(4px)';
                }}
                onMouseLeave={(e) => {
                  e.currentTarget.style.borderColor = 'var(--danger-color)';
                  e.currentTarget.style.transform = 'translateX(0)';
                }}
              >
                <div style={{ fontSize: '0.875rem', color: 'var(--text-primary)' }}>
                  {getNotificationMessage(notification)}
                </div>
              </div>
            ))}
            {expired.length > 3 && (
              <div
                style={{
                  fontSize: '0.8125rem',
                  color: 'var(--text-secondary)',
                  marginLeft: '0.75rem',
                }}
              >
                +{expired.length - 3} more expired
              </div>
            )}
          </div>
        )}

        {expiringSoon.length > 0 && (
          <div>
            <div
              style={{
                fontSize: '0.875rem',
                fontWeight: 600,
                color: 'var(--warning-color)',
                marginBottom: '0.5rem',
              }}
            >
              <i className="fas fa-exclamation-triangle"></i> Expiring Soon ({expiringSoon.length})
            </div>
            {expiringSoon.slice(0, 3).map((notification) => (
              <div
                key={notification.id}
                onClick={() => handleNotificationClick(notification)}
                style={{
                  padding: '0.75rem',
                  background: 'var(--bg-primary)',
                  borderRadius: 'var(--radius-md)',
                  marginBottom: '0.5rem',
                  cursor: 'pointer',
                  border: '1px solid var(--warning-color)',
                  transition: 'all 0.2s ease',
                }}
                onMouseEnter={(e) => {
                  e.currentTarget.style.borderColor = 'var(--accent-dark)';
                  e.currentTarget.style.transform = 'translateX(4px)';
                }}
                onMouseLeave={(e) => {
                  e.currentTarget.style.borderColor = 'var(--warning-color)';
                  e.currentTarget.style.transform = 'translateX(0)';
                }}
              >
                <div style={{ fontSize: '0.875rem', color: 'var(--text-primary)' }}>
                  {getNotificationMessage(notification)}
                </div>
              </div>
            ))}
            {expiringSoon.length > 3 && (
              <div
                style={{
                  fontSize: '0.8125rem',
                  color: 'var(--text-secondary)',
                  marginLeft: '0.75rem',
                }}
              >
                +{expiringSoon.length - 3} more expiring soon
              </div>
            )}
          </div>
        )}

        {expiringThisMonth.length > 0 && expired.length === 0 && expiringSoon.length === 0 && (
          <div>
            <div
              style={{
                fontSize: '0.875rem',
                fontWeight: 600,
                color: 'var(--info-color)',
                marginBottom: '0.5rem',
              }}
            >
              <i className="fas fa-info-circle"></i> Expiring This Month ({expiringThisMonth.length}
              )
            </div>
            {expiringThisMonth.slice(0, 3).map((notification) => (
              <div
                key={notification.id}
                onClick={() => handleNotificationClick(notification)}
                style={{
                  padding: '0.75rem',
                  background: 'var(--bg-primary)',
                  borderRadius: 'var(--radius-md)',
                  marginBottom: '0.5rem',
                  cursor: 'pointer',
                  border: '1px solid var(--info-color)',
                  transition: 'all 0.2s ease',
                }}
                onMouseEnter={(e) => {
                  e.currentTarget.style.borderColor = 'var(--primary-dark)';
                  e.currentTarget.style.transform = 'translateX(4px)';
                }}
                onMouseLeave={(e) => {
                  e.currentTarget.style.borderColor = 'var(--info-color)';
                  e.currentTarget.style.transform = 'translateX(0)';
                }}
              >
                <div style={{ fontSize: '0.875rem', color: 'var(--text-primary)' }}>
                  {getNotificationMessage(notification)}
                </div>
              </div>
            ))}
            {expiringThisMonth.length > 3 && (
              <div
                style={{
                  fontSize: '0.8125rem',
                  color: 'var(--text-secondary)',
                  marginLeft: '0.75rem',
                }}
              >
                +{expiringThisMonth.length - 3} more expiring this month
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
