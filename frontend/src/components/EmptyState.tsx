import { ReactNode } from 'react';

interface EmptyStateProps {
  icon: string;
  title: string;
  text: string;
  action?: ReactNode;
}

export function EmptyState({ icon, title, text, action }: EmptyStateProps) {
  return (
    <div className="empty-state">
      <div className="empty-state-icon">
        <i className={icon}></i>
      </div>
      <div className="empty-state-title">{title}</div>
      <div className="empty-state-text">{text}</div>
      {action}
    </div>
  );
}
