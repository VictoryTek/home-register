import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { Header, LoadingState, EmptyState, Modal } from '@/components';
import { inventoryApi } from '@/services/api';
import { useApp } from '@/context/AppContext';
import type { Inventory } from '@/types';

export function InventoriesPage() {
  const navigate = useNavigate();
  const { showToast, inventories, setInventories } = useApp();
  const [loading, setLoading] = useState(true);
  const [itemCounts, setItemCounts] = useState<Record<number, number>>({});
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [newInventory, setNewInventory] = useState({ name: '', description: '' });

  useEffect(() => {
    loadInventories();
  }, []);

  const loadInventories = async () => {
    setLoading(true);
    try {
      const result = await inventoryApi.getAll();
      if (result.success && result.data) {
        setInventories(result.data);
        
        // Load item counts for each inventory
        const counts: Record<number, number> = {};
        for (const inv of result.data) {
          if (inv.id) {
            const itemsResult = await inventoryApi.getItems(inv.id);
            counts[inv.id] = itemsResult.success && itemsResult.data ? itemsResult.data.length : 0;
          }
        }
        setItemCounts(counts);
      } else {
        showToast(result.error || 'Failed to load inventories', 'error');
      }
    } catch (error) {
      showToast('Failed to load inventories', 'error');
    } finally {
      setLoading(false);
    }
  };

  const handleCreateInventory = async () => {
    if (!newInventory.name.trim()) {
      showToast('Please enter an inventory name', 'error');
      return;
    }

    try {
      const result = await inventoryApi.create(newInventory);
      if (result.success) {
        showToast('Inventory created successfully!', 'success');
        setShowCreateModal(false);
        setNewInventory({ name: '', description: '' });
        loadInventories();
      } else {
        showToast(result.error || 'Failed to create inventory', 'error');
      }
    } catch (error) {
      showToast('Failed to create inventory', 'error');
    }
  };

  const handleDeleteInventory = async (e: React.MouseEvent, id: number) => {
    e.stopPropagation();
    if (!confirm('Are you sure you want to delete this inventory?')) return;

    try {
      const result = await inventoryApi.delete(id);
      if (result.success) {
        showToast('Inventory deleted successfully!', 'success');
        loadInventories();
      } else {
        showToast(result.error || 'Failed to delete inventory', 'error');
      }
    } catch (error) {
      showToast('Failed to delete inventory', 'error');
    }
  };

  return (
    <>
      <Header
        title="Your Inventories"
        subtitle="Manage and organize your inventory collections"
        icon="fas fa-warehouse"
      />
      
      <div className="content">
        <div className="inventories-container">
          <div style={{ display: 'flex', justifyContent: 'flex-end', marginBottom: '1.5rem' }}>
            <button className="btn btn-primary" onClick={() => setShowCreateModal(true)}>
              <i className="fas fa-plus"></i>
              Create Inventory
            </button>
          </div>

          {loading ? (
            <LoadingState message="Loading inventories..." />
          ) : inventories.length === 0 ? (
            <EmptyState
              icon="fas fa-warehouse"
              title="No Inventories Yet"
              text="Create your first inventory to start organizing your items."
              action={
                <button className="btn btn-primary" onClick={() => setShowCreateModal(true)}>
                  <i className="fas fa-plus"></i>
                  Create Your First Inventory
                </button>
              }
            />
          ) : (
            <div className="inventories-grid">
              {inventories.map((inventory) => (
                <div
                  key={inventory.id}
                  className="inventory-card"
                  onClick={() => navigate(`/inventory/${inventory.id}`)}
                >
                  <div className="inventory-card-header">
                    <div className="inventory-card-image">
                      <div style={{
                        width: '100%',
                        height: '100%',
                        background: 'linear-gradient(135deg, var(--primary-color), var(--primary-light))',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                        color: 'white',
                        fontSize: '2.5rem'
                      }}>
                        <i className="fas fa-warehouse"></i>
                      </div>
                    </div>
                  </div>
                  <div className="inventory-card-body">
                    <h3 className="inventory-card-title">{inventory.name}</h3>
                    <p className="inventory-card-description">
                      {inventory.description || 'No description'}
                    </p>
                    {inventory.location && (
                      <p className="inventory-card-location">
                        <i className="fas fa-map-marker-alt"></i>
                        {inventory.location}
                      </p>
                    )}
                    <div className="inventory-card-stats">
                      <div className="stat-item">
                        <i className="fas fa-box"></i>
                        <span>{itemCounts[inventory.id!] || 0} items</span>
                      </div>
                    </div>
                  </div>
                  <div className="inventory-card-footer">
                    <button
                      className="btn btn-sm btn-ghost"
                      onClick={(e) => { e.stopPropagation(); /* TODO: Edit */ }}
                      title="Edit Inventory"
                    >
                      <i className="fas fa-edit"></i>
                    </button>
                    <button
                      className="btn btn-sm btn-ghost text-danger"
                      onClick={(e) => handleDeleteInventory(e, inventory.id!)}
                      title="Delete Inventory"
                    >
                      <i className="fas fa-trash"></i>
                    </button>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>

      <Modal
        isOpen={showCreateModal}
        onClose={() => setShowCreateModal(false)}
        title="Create New Inventory"
        subtitle="Set up a new inventory space"
        footer={
          <>
            <button className="btn btn-secondary" onClick={() => setShowCreateModal(false)}>
              Cancel
            </button>
            <button className="btn btn-success" onClick={handleCreateInventory}>
              <i className="fas fa-warehouse"></i>
              Create Inventory
            </button>
          </>
        }
      >
        <div className="form-group">
          <label className="form-label" htmlFor="inventory-name">Inventory Name *</label>
          <input
            type="text"
            className="form-input"
            id="inventory-name"
            placeholder="e.g., Main House, Garage, Storage Unit"
            value={newInventory.name}
            onChange={(e) => setNewInventory({ ...newInventory, name: e.target.value })}
          />
        </div>
        <div className="form-group">
          <label className="form-label" htmlFor="inventory-description">Description</label>
          <textarea
            className="form-input"
            id="inventory-description"
            placeholder="Optional description"
            rows={3}
            value={newInventory.description}
            onChange={(e) => setNewInventory({ ...newInventory, description: e.target.value })}
          />
        </div>
      </Modal>
    </>
  );
}
