import { useEffect, useState } from 'react';
import { useApp } from '@/context/AppContext';

export function Toast() {
  const { toasts, removeToast } = useApp();

  return (
    <>
      {toasts.map((toast, index) => (
        <ToastItem
          key={toast.id}
          id={toast.id}
          message={toast.message}
          type={toast.type}
          index={index}
          onRemove={removeToast}
        />
      ))}
    </>
  );
}

interface ToastItemProps {
  id: string;
  message: string;
  type: 'success' | 'error' | 'warning' | 'info';
  index: number;
  onRemove: (id: string) => void;
}

function ToastItem({ id, message, type, index, onRemove }: ToastItemProps) {
  const [visible, setVisible] = useState(false);

  useEffect(() => {
    // Trigger animation
    setTimeout(() => setVisible(true), 100);
    
    // Auto remove
    const timeout = setTimeout(() => {
      setVisible(false);
      setTimeout(() => onRemove(id), 300);
    }, 2700);

    return () => clearTimeout(timeout);
  }, [id, onRemove]);

  const icons = {
    success: 'check-circle',
    error: 'exclamation-triangle',
    warning: 'exclamation-circle',
    info: 'info-circle',
  };

  return (
    <div
      className={`toast toast-${type} ${visible ? 'visible' : ''}`}
      style={{ top: `${2 + index * 4.5}rem` }}
    >
      <i className={`fas fa-${icons[type]} toast-icon`}></i>
      <span className="toast-message">{message}</span>
    </div>
  );
}
