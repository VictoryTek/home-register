export function LoadingState({ message = 'Loading...' }: { message?: string }) {
  return (
    <div className="loading-state">
      <div className="loading-state-content">
        <div className="loading-emoji">⏳</div>
        <div style={{ color: 'var(--text-secondary)' }}>{message}</div>
      </div>
    </div>
  );
}
