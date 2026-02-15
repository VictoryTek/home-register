import { useState, useEffect, useRef, useCallback } from 'react';
import { useNavigate } from 'react-router-dom';
import { Header, LoadingState, EmptyState, Modal, ConfirmModal } from '@/components';
import { inventoryApi } from '@/services/api';
import { useApp } from '@/context/AppContext';
import { useAuth } from '@/context/AuthContext';
import type { Inventory, Item } from '@/types';

export function InventoriesPage() {
  const navigate = useNavigate();
  const { showToast, inventories, setInventories, setItems } = useApp();
  const { settings, user } = useAuth();
  const [loading, setLoading] = useState(true);
  const hasAutoNavigated = useRef(false);
  const [itemCounts, setItemCounts] = useState<Record<number, number>>({});
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showEditModal, setShowEditModal] = useState(false);
  const [showDeleteModal, setShowDeleteModal] = useState(false);
  const [editingInventory, setEditingInventory] = useState<Inventory | null>(null);
  const [deletingInventory, setDeletingInventory] = useState<Inventory | null>(null);
  const [formData, setFormData] = useState({
    name: '',
    description: '',
    location: '',
    image_url: '',
  });
  const [imageOption, setImageOption] = useState<'upload' | 'url'>('url');
  const [imagePreview, setImagePreview] = useState<string>('');

  // CRITICAL FIX: Empty dependency array to prevent infinite loop
  // - setInventories and setItems are stable (guaranteed by React useState)
  // - showToast is now stable (wrapped in useCallback in AppContext)
  // - setLoading is stable (local useState)
  // Previously, including showToast/setItems/setInventories caused infinite loops
  // when AppContext re-rendered, recreating function references
  const loadInventories = useCallback(async () => {
    setLoading(true);
    try {
      const result = await inventoryApi.getAll();
      if (result.success && result.data) {
        setInventories(result.data);

        // Load item counts and all items for notification checking
        const counts: Record<number, number> = {};
        const allItems: Item[] = [];

        // Parallelize API calls instead of sequential loop to reduce rate limit pressure
        const itemsPromises = result.data.map((inv) =>
          inv.id ? inventoryApi.getItems(inv.id) : Promise.resolve({ success: false, data: null })
        );

        const itemsResults = await Promise.all(itemsPromises);

        // Explicit null check to satisfy TypeScript type narrowing in forEach callback
        itemsResults.forEach((itemsResult, index) => {
          if (!result.data) {
            return; // Type guard: ensure result.data exists
          }

          const inv = result.data[index];
          if (!inv?.id) {
            return; // Type guard: ensure inv and inv.id exist
          }

          if (itemsResult.success && itemsResult.data) {
            counts[inv.id] = itemsResult.data.length;
            allItems.push(...itemsResult.data);
          } else {
            counts[inv.id] = 0;
          }
        });

        setItemCounts(counts);
        setItems(allItems); // Update global items state for notifications
      } else {
        showToast(result.error ?? 'Failed to load inventories', 'error');
      }
    } catch {
      showToast('Failed to load inventories', 'error');
    } finally {
      setLoading(false);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []); // Empty deps - all functions used are stable

  useEffect(() => {
    void loadInventories();
  }, [loadInventories]);

  // Auto-navigate to default inventory if set
  useEffect(() => {
    if (
      !loading &&
      !hasAutoNavigated.current &&
      settings?.default_inventory_id &&
      inventories.length > 0
    ) {
      // Check if the default inventory exists
      const defaultInventory = inventories.find((inv) => inv.id === settings.default_inventory_id);
      if (defaultInventory) {
        hasAutoNavigated.current = true;
        navigate(`/inventory/${settings.default_inventory_id}`);
      }
    }
  }, [loading, settings, inventories, navigate]);

  const handleCreateInventory = async () => {
    if (!formData.name.trim()) {
      showToast('Please enter an inventory name', 'error');
      return;
    }

    try {
      const result = await inventoryApi.create(formData);
      if (result.success) {
        showToast('Inventory created successfully!', 'success');
        setShowCreateModal(false);
        resetForm();
        void loadInventories();
      } else {
        showToast(result.error ?? 'Failed to create inventory', 'error');
      }
    } catch {
      showToast('Failed to create inventory', 'error');
    }
  };

  const handleEditInventory = async () => {
    if (!formData.name.trim()) {
      showToast('Please enter an inventory name', 'error');
      return;
    }

    if (!editingInventory?.id) {
      return;
    }

    try {
      const result = await inventoryApi.update(editingInventory.id, formData);
      if (result.success) {
        showToast('Inventory updated successfully!', 'success');
        setShowEditModal(false);
        resetForm();
        void loadInventories();
      } else {
        showToast(result.error ?? 'Failed to update inventory', 'error');
      }
    } catch {
      showToast('Failed to update inventory', 'error');
    }
  };

  const handleDeleteInventory = async () => {
    if (!deletingInventory?.id) {
      return;
    }

    try {
      const result = await inventoryApi.delete(deletingInventory.id);
      if (result.success) {
        showToast('Inventory deleted successfully!', 'success');
        void loadInventories();
      } else {
        showToast(result.error ?? 'Failed to delete inventory', 'error');
      }
    } catch {
      showToast('Failed to delete inventory', 'error');
    }
  };

  const openEditModal = (e: React.MouseEvent, inventory: Inventory) => {
    e.stopPropagation();
    setEditingInventory(inventory);
    setFormData({
      name: inventory.name,
      description: inventory.description ?? '',
      location: inventory.location ?? '',
      image_url: inventory.image_url ?? '',
    });
    setImagePreview(inventory.image_url ?? '');
    setImageOption(inventory.image_url?.startsWith('data:') ? 'upload' : 'url');
    setShowEditModal(true);
  };

  const openDeleteModal = (e: React.MouseEvent, inventory: Inventory) => {
    e.stopPropagation();
    setDeletingInventory(inventory);
    setShowDeleteModal(true);
  };

  const resetForm = () => {
    setFormData({ name: '', description: '', location: '', image_url: '' });
    setImagePreview('');
    setImageOption('url');
    setEditingInventory(null);
    setDeletingInventory(null);
  };

  const handleImageUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      const reader = new FileReader();
      reader.onload = (event) => {
        const dataUrl = event.target?.result as string;
        setImagePreview(dataUrl);
        setFormData({ ...formData, image_url: dataUrl });
      };
      reader.readAsDataURL(file);
    }
  };

  const handleImageUrlChange = (url: string) => {
    setFormData({ ...formData, image_url: url });
    setImagePreview(url);
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
          <div className="page-actions">
            <button className="btn btn-primary" onClick={() => setShowCreateModal(true)}>
              <i className="fas fa-plus"></i>
              Create Inventory
            </button>
            {settings?.default_inventory_id && (
              <span
                style={{ fontSize: '0.875rem', color: 'var(--text-secondary)', marginLeft: '1rem' }}
              >
                <i className="fas fa-info-circle"></i> Default inventory is set to auto-open
              </span>
            )}
          </div>

          {loading ? (
            <LoadingState message="Loading inventories..." />
          ) : inventories.length === 0 ? (
            <EmptyState
              icon="fas fa-warehouse"
              title="No Inventories Yet"
              text="Create your first inventory to start organizing your items."
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
                      {inventory.image_url ? (
                        <img
                          src={inventory.image_url}
                          alt={inventory.name}
                          style={{
                            width: '100%',
                            height: '100%',
                            objectFit: 'cover',
                          }}
                        />
                      ) : (
                        <div
                          style={{
                            width: '100%',
                            height: '100%',
                            background:
                              'linear-gradient(135deg, var(--accent-color), var(--accent-light))',
                            display: 'flex',
                            alignItems: 'center',
                            justifyContent: 'center',
                            color: 'white',
                            fontSize: '2.5rem',
                          }}
                        >
                          <i className="fas fa-warehouse"></i>
                        </div>
                      )}
                    </div>
                  </div>
                  <div className="inventory-card-body">
                    <h3 className="inventory-card-title">{inventory.name}</h3>
                    <p className="inventory-card-description">
                      {inventory.description ?? 'No description'}
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
                        <span>{itemCounts[inventory.id ?? 0] ?? 0} items</span>
                      </div>
                      {inventory.user_id && user?.id && inventory.user_id !== user.id && (
                        <div className="stat-item" style={{ marginLeft: 'auto' }}>
                          <span className="badge badge-shared">
                            <i className="fas fa-share-alt"></i> Shared
                          </span>
                        </div>
                      )}
                    </div>
                  </div>
                  <div className="inventory-card-footer">
                    <button
                      className="btn btn-sm btn-ghost"
                      onClick={(e) => openEditModal(e, inventory)}
                      title="Edit Inventory"
                    >
                      <i className="fas fa-edit"></i>
                    </button>
                    <button
                      className="btn btn-sm btn-ghost text-danger"
                      onClick={(e) => openDeleteModal(e, inventory)}
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
          <label className="form-label" htmlFor="inventory-name">
            Inventory Name *
          </label>
          <input
            type="text"
            className="form-input"
            id="inventory-name"
            placeholder="e.g., Main House, Garage, Storage Unit"
            value={formData.name}
            onChange={(e) => setFormData({ ...formData, name: e.target.value })}
          />
        </div>
        <div className="form-group">
          <label className="form-label" htmlFor="inventory-description">
            Description
          </label>
          <textarea
            className="form-input"
            id="inventory-description"
            placeholder="Optional description"
            rows={3}
            value={formData.description}
            onChange={(e) => setFormData({ ...formData, description: e.target.value })}
          />
        </div>
        <div className="form-group">
          <label className="form-label" htmlFor="inventory-location">
            Location
          </label>
          <input
            type="text"
            className="form-input"
            id="inventory-location"
            placeholder="e.g., Main Office, Kitchen, Living Room"
            value={formData.location}
            onChange={(e) => setFormData({ ...formData, location: e.target.value })}
          />
        </div>
        <div className="form-group">
          <label className="form-label">Inventory Image</label>
          <div style={{ marginBottom: '1rem' }}>
            <button
              type="button"
              className={`btn btn-sm ${imageOption === 'url' ? 'btn-primary' : 'btn-secondary'}`}
              onClick={() => setImageOption('url')}
              style={{ marginRight: '0.5rem' }}
            >
              Image URL
            </button>
            <button
              type="button"
              className={`btn btn-sm ${imageOption === 'upload' ? 'btn-primary' : 'btn-secondary'}`}
              onClick={() => setImageOption('upload')}
            >
              Upload Image
            </button>
          </div>

          {imageOption === 'url' ? (
            <input
              type="text"
              className="form-input"
              placeholder="Enter image URL"
              value={formData.image_url}
              onChange={(e) => handleImageUrlChange(e.target.value)}
            />
          ) : (
            <div
              className="image-upload-container"
              onClick={() => document.getElementById('inventory-image-input')?.click()}
              style={{ cursor: 'pointer' }}
            >
              <input
                type="file"
                id="inventory-image-input"
                accept="image/*"
                style={{ display: 'none' }}
                onChange={handleImageUpload}
              />
              <div className="image-preview">
                {imagePreview ? (
                  <img
                    src={imagePreview}
                    alt="Preview"
                    style={{
                      maxWidth: '100%',
                      maxHeight: '120px',
                      borderRadius: 'var(--radius-md)',
                      objectFit: 'cover',
                    }}
                  />
                ) : (
                  <div className="image-placeholder">
                    <i className="fas fa-image" style={{ fontSize: '2rem', opacity: 0.6 }}></i>
                    <span>Click to upload an image</span>
                  </div>
                )}
              </div>
            </div>
          )}

          {imagePreview && (
            <div style={{ marginTop: '0.5rem', textAlign: 'center' }}>
              <button
                type="button"
                className="btn btn-sm btn-secondary"
                onClick={() => {
                  setImagePreview('');
                  setFormData({ ...formData, image_url: '' });
                }}
              >
                Clear Image
              </button>
            </div>
          )}
        </div>
      </Modal>

      {/* Edit Inventory Modal */}
      <Modal
        isOpen={showEditModal}
        onClose={() => {
          setShowEditModal(false);
          resetForm();
        }}
        title="Edit Inventory"
        subtitle="Update your inventory information"
        footer={
          <>
            <button
              className="btn btn-secondary"
              onClick={() => {
                setShowEditModal(false);
                resetForm();
              }}
            >
              Cancel
            </button>
            <button className="btn btn-primary" onClick={handleEditInventory}>
              <i className="fas fa-save"></i>
              Save Changes
            </button>
          </>
        }
      >
        <div className="form-group">
          <label className="form-label" htmlFor="edit-inventory-name">
            Inventory Name *
          </label>
          <input
            type="text"
            className="form-input"
            id="edit-inventory-name"
            placeholder="e.g., Main House, Garage, Storage Unit"
            value={formData.name}
            onChange={(e) => setFormData({ ...formData, name: e.target.value })}
          />
        </div>
        <div className="form-group">
          <label className="form-label" htmlFor="edit-inventory-description">
            Description
          </label>
          <textarea
            className="form-input"
            id="edit-inventory-description"
            placeholder="Optional description"
            rows={3}
            value={formData.description}
            onChange={(e) => setFormData({ ...formData, description: e.target.value })}
          />
        </div>
        <div className="form-group">
          <label className="form-label" htmlFor="edit-inventory-location">
            Location
          </label>
          <input
            type="text"
            className="form-input"
            id="edit-inventory-location"
            placeholder="e.g., Main Office, Kitchen, Living Room"
            value={formData.location}
            onChange={(e) => setFormData({ ...formData, location: e.target.value })}
          />
        </div>
        <div className="form-group">
          <label className="form-label">Inventory Image</label>
          <div style={{ marginBottom: '1rem' }}>
            <button
              type="button"
              className={`btn btn-sm ${imageOption === 'url' ? 'btn-primary' : 'btn-secondary'}`}
              onClick={() => setImageOption('url')}
              style={{ marginRight: '0.5rem' }}
            >
              Image URL
            </button>
            <button
              type="button"
              className={`btn btn-sm ${imageOption === 'upload' ? 'btn-primary' : 'btn-secondary'}`}
              onClick={() => setImageOption('upload')}
            >
              Upload Image
            </button>
          </div>

          {imageOption === 'url' ? (
            <input
              type="text"
              className="form-input"
              placeholder="Enter image URL"
              value={formData.image_url}
              onChange={(e) => handleImageUrlChange(e.target.value)}
            />
          ) : (
            <div
              className="image-upload-container"
              onClick={() => document.getElementById('edit-inventory-image-input')?.click()}
              style={{ cursor: 'pointer' }}
            >
              <input
                type="file"
                id="edit-inventory-image-input"
                accept="image/*"
                style={{ display: 'none' }}
                onChange={handleImageUpload}
              />
              <div className="image-preview">
                {imagePreview ? (
                  <img
                    src={imagePreview}
                    alt="Preview"
                    style={{
                      maxWidth: '100%',
                      maxHeight: '120px',
                      borderRadius: 'var(--radius-md)',
                      objectFit: 'cover',
                    }}
                  />
                ) : (
                  <div className="image-placeholder">
                    <i className="fas fa-image" style={{ fontSize: '2rem', opacity: 0.6 }}></i>
                    <span>Click to upload an image</span>
                  </div>
                )}
              </div>
            </div>
          )}

          {imagePreview && (
            <div style={{ marginTop: '0.5rem', textAlign: 'center' }}>
              <button
                type="button"
                className="btn btn-sm btn-secondary"
                onClick={() => {
                  setImagePreview('');
                  setFormData({ ...formData, image_url: '' });
                }}
              >
                Clear Image
              </button>
            </div>
          )}
        </div>
      </Modal>

      {/* Delete Confirmation Modal */}
      <ConfirmModal
        isOpen={showDeleteModal}
        onClose={() => {
          setShowDeleteModal(false);
          setDeletingInventory(null);
        }}
        onConfirm={handleDeleteInventory}
        title="Delete Inventory"
        message={`Are you sure you want to delete "${deletingInventory?.name}"? This action cannot be undone.`}
        confirmText="Delete"
        confirmButtonClass="btn-danger"
        icon="fas fa-trash"
      />
    </>
  );
}
