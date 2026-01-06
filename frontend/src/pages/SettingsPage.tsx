import { Header, EmptyState } from '@/components';

export function SettingsPage() {
  return (
    <>
      <Header
        title="Settings"
        subtitle="Configure your inventory system"
        icon="fas fa-cog"
      />
      
      <div className="content">
        <EmptyState
          icon="fas fa-hammer"
          title="Coming Soon"
          text="This feature is under development and will be available soon."
        />
      </div>
    </>
  );
}
