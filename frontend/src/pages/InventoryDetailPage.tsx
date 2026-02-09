import { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { Header, LoadingState, EmptyState, Modal, WarrantyNotificationBanner } from '@/components';
import { inventoryApi, itemApi, organizerApi } from '@/services/api';
import { useApp } from '@/context/AppContext';
import { useAuth } from '@/context/AuthContext';
import { formatDate } from '@/utils/dateFormat';
import { formatCurrency } from '@/utils/currencyFormat';
import type { Inventory, Item, CreateItemRequest, OrganizerTypeWithOptions, SetItemOrganizerValueRequest } from '@/types';

export function InventoryDetailPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { showToast, setItems: setGlobalItems } = useApp();
  const { settings } = useAuth();
  const [loading, setLoading] = useState(true);
  const [inventory, setInventory] = useState<Inventory | null>(null);
  const [items, setItems] = useState<Item[]>([]);
  const [organizers, setOrganizers] = useState<OrganizerTypeWithOptions[]>([]);
  const [showAddItemModal, setShowAddItemModal] = useState(false);
  const [newItem, setNewItem] = useState<CreateItemRequest>({
    inventory_id: parseInt(id || '0'),
    name: '',
    description: '',
    category: '',
    location: '',
    purchase_date: undefined,
    purchase_price: undefined,
    warranty_expiry: undefined,
    quantity: 1,
  });
  const [organizerValues, setOrganizerValues] = useState<Record<number, { optionId?: number; textValue?: string }>>({});

  useEffect(() => {
    if (id) {
      loadInventoryDetail(parseInt(id));
    }
  }, [id]);

  const loadInventoryDetail = async (inventoryId: number) => {
    setLoading(true);
    try {
      const [invResult, itemsResult, organizersResult] = await Promise.all([
        inventoryApi.getById(inventoryId),
        inventoryApi.getItems(inventoryId),
        organizerApi.getByInventory(inventoryId),
      ]);

      if (invResult.success && invResult.data) {
        setInventory(invResult.data);
      } else {
        showToast('Inventory not found', 'error');
        navigate('/');
        return;
      }

      if (itemsResult.success && itemsResult.data) {
        setItems(itemsResult.data);
        setGlobalItems(itemsResult.data); // Update global items state for notifications
      }

      if (organizersResult.success && organizersResult.data) {
        setOrganizers(organizersResult.data);
      }
    } catch (error) {
      showToast('Failed to load inventory', 'error');
      navigate('/');
    } finally {
      setLoading(false);
    }
  };

  const handleAddItem = async () => {
    if (!newItem.name.trim()) {
      showToast('Please enter an item name', 'error');
      return;
    }

    // Check required organizers
    for (const org of organizers) {
      if (org.is_required) {
        const value = organizerValues[org.id!];
        if (!value || (org.input_type === 'select' && !value.optionId) || (org.input_type === 'text' && !value.textValue?.trim())) {
          showToast(`Please fill in the required field: ${org.name}`, 'error');
          return;
        }
      }
    }

    try {
      const result = await itemApi.create({
        ...newItem,
        inventory_id: parseInt(id || '0'),
      });

      if (result.success && result.data) {
        // Save organizer values if any are set
        const valuesToSave: SetItemOrganizerValueRequest[] = [];
        for (const [typeIdStr, value] of Object.entries(organizerValues)) {
          const typeId = parseInt(typeIdStr);
          if (value.optionId || value.textValue?.trim()) {
            valuesToSave.push({
              organizer_type_id: typeId,
              organizer_option_id: value.optionId,
              text_value: value.textValue?.trim() || undefined,
            });
          }
        }

        if (valuesToSave.length > 0) {
          await itemApi.setOrganizerValues(result.data.id!, { values: valuesToSave });
        }

        showToast('Item added successfully!', 'success');
        setShowAddItemModal(false);
        setNewItem({
          inventory_id: parseInt(id || '0'),
          name: '',
          description: '',
          category: '',
          location: '',
          purchase_date: undefined,
          purchase_price: undefined,
          warranty_expiry: undefined,
          quantity: 1,
        });
        setOrganizerValues({});
        loadInventoryDetail(parseInt(id || '0'));
      } else {
        showToast(result.error || 'Failed to add item', 'error');
      }
    } catch (error) {
      showToast('Failed to add item', 'error');
    }
  };

  const handleDeleteItem = async (itemId: number) => {
    if (!confirm('Are you sure you want to delete this item?')) return;

    try {
      const result = await itemApi.delete(itemId);
      if (result.success) {
        showToast('Item deleted successfully!', 'success');
        loadInventoryDetail(parseInt(id || '0'));
      } else {
        showToast(result.error || 'Failed to delete item', 'error');
      }
    } catch (error) {
      showToast('Failed to delete item', 'error');
    }
  };

  const totalValue = items.reduce(
    (sum, item) => sum + (item.purchase_price || 0) * (item.quantity || 1),
    0
  );

  if (loading) {
    return (
      <>
        <Header title="Loading..." subtitle="" icon="fas fa-warehouse" />
        <div className="content">
          <LoadingState message="Loading inventory details..." />
        </div>
      </>
    );
  }

  if (!inventory) {
    return null;
  }

  return (
    <>
      <Header
        title={inventory.name}
        subtitle={inventory.description || 'Manage and organize your inventory collections'}
        icon="fas fa-warehouse"
      />
      
      <div className="content">
        <div className="inventory-detail">
          <WarrantyNotificationBanner />
          
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '1.5rem' }}>
            <button className="btn btn-ghost" onClick={() => navigate('/')}>
              <i className="fas fa-arrow-left"></i>
              Back to Inventories
            </button>
            <div style={{ display: 'flex', gap: '0.5rem' }}>
              <button className="btn btn-secondary" onClick={() => navigate(`/inventory/${id}/organizers`)}>
                <i className="fas fa-folder-tree"></i>
                Organizers
              </button>
              <button className="btn btn-primary" onClick={() => setShowAddItemModal(true)}>
                <i className="fas fa-plus"></i>
                Add Item
              </button>
            </div>
          </div>

          <div className="stats-row" style={{ display: 'flex', gap: '1rem', marginBottom: '1.5rem' }}>
            <div className="stat-card">
              <div className="stat-icon" style={{ background: 'var(--primary-color)' }}>
                <i className="fas fa-boxes"></i>
              </div>
              <div className="stat-content">
                <div className="stat-label">Total Items</div>
                <div className="stat-value">{items.length}</div>
              </div>
            </div>
            <div className="stat-card">
              <div className="stat-icon" style={{ background: 'var(--success-color)' }}>
                <i className="fas fa-coins"></i>
              </div>
              <div className="stat-content">
                <div className="stat-label">Total Value</div>
                <div className="stat-value">{formatCurrency(totalValue, settings?.currency as any || 'USD')}</div>
              </div>
            </div>
          </div>

          <div className="inventory-detail-items">
            <h2 className="section-title">
              <i className="fas fa-boxes"></i>
              Items in this Inventory
            </h2>

            {items.length === 0 ? (
              <EmptyState
                icon="fas fa-box-open"
                title="No Items Yet"
                text="Start adding items to this inventory."
              />
            ) : (
              <div className="items-grid">
                {items.map((item) => {
                  const itemValue = (item.purchase_price || 0) * (item.quantity || 1);
                  return (
                    <div key={item.id} className="item-card">
                      <div className="item-card-header">
                        <h3 className="item-card-title">{item.name}</h3>
                        {item.category && (
                          <span className="item-card-category">{item.category}</span>
                        )}
                      </div>
                      <div className="item-card-body">
                        {item.description && (
                          <p className="item-card-description">{item.description}</p>
                        )}
                        <div className="item-card-details">
                          {item.location && (
                            <div className="detail-item">
                              <i className="fas fa-map-marker-alt"></i>
                              <span>{item.location}</span>
                            </div>
                          )}
                          {item.quantity && (
                            <div className="detail-item">
                              <i className="fas fa-boxes"></i>
                              <span>Qty: {item.quantity}</span>
                            </div>
                          )}
                          {item.purchase_date && (
                            <div className="detail-item">
                              <i className="fas fa-calendar-alt"></i>
                              <span>Purchased: {formatDate(item.purchase_date, settings?.date_format as any || 'MM/DD/YYYY')}</span>
                            </div>
                          )}
                          {item.purchase_price && (
                            <div className="detail-item">
                              <i className="fas fa-tag"></i>
                              <span>{formatCurrency(item.purchase_price, settings?.currency as any || 'USD')} ea</span>
                            </div>
                          )}
                          {itemValue > 0 && (
                            <div className="detail-item">
                              <i className="fas fa-coins"></i>
                              <span>Total: {formatCurrency(itemValue, settings?.currency as any || 'USD')}</span>
                            </div>
                          )}
                          {item.warranty_expiry && (
                            <div className="detail-item">
                              <i className="fas fa-shield-alt"></i>
                              <span>Warranty: {formatDate(item.warranty_expiry, settings?.date_format as any || 'MM/DD/YYYY')}</span>
                            </div>
                          )}
                        </div>
                      </div>
                      <div className="item-card-footer">
                        <button className="btn btn-sm btn-ghost" title="View Details">
                          <i className="fas fa-eye"></i>
                        </button>
                        <button className="btn btn-sm btn-ghost" title="Edit Item">
                          <i className="fas fa-edit"></i>
                        </button>
                        <button
                          className="btn btn-sm btn-ghost text-danger"
                          onClick={() => handleDeleteItem(item.id!)}
                          title="Delete Item"
                        >
                          <i className="fas fa-trash"></i>
                        </button>
                      </div>
                    </div>
                  );
                })}
              </div>
            )}
          </div>
        </div>
      </div>

      <Modal
        isOpen={showAddItemModal}
        onClose={() => setShowAddItemModal(false)}
        title="Add New Item"
        subtitle="Quickly add an item to your inventory"
        footer={
          <>
            <button className="btn btn-secondary" onClick={() => setShowAddItemModal(false)}>
              Cancel
            </button>
            <button className="btn btn-primary" onClick={handleAddItem}>
              <i className="fas fa-plus"></i>
              Add Item
            </button>
          </>
        }
      >
        <div className="form-group">
          <label className="form-label" htmlFor="item-name">Item Name *</label>
          <input
            type="text"
            className="form-input"
            id="item-name"
            placeholder="Enter item name"
            value={newItem.name}
            onChange={(e) => setNewItem({ ...newItem, name: e.target.value })}
          />
        </div>

        {/* Dynamic Organizer Fields */}
        {organizers.length > 0 && (
          <div className="organizer-fields">
            {organizers.map((org) => (
              <div className="form-group" key={org.id}>
                <label className="form-label" htmlFor={`organizer-${org.id}`}>
                  {org.name}{org.is_required ? ' *' : ''}
                </label>
                {org.input_type === 'select' ? (
                  <select
                    className="form-select"
                    id={`organizer-${org.id}`}
                    value={organizerValues[org.id!]?.optionId || ''}
                    onChange={(e) => setOrganizerValues({
                      ...organizerValues,
                      [org.id!]: { optionId: e.target.value ? parseInt(e.target.value) : undefined }
                    })}
                  >
                    <option value="">Select {org.name.toLowerCase()}</option>
                    {org.options.map((opt) => (
                      <option key={opt.id} value={opt.id}>{opt.name}</option>
                    ))}
                  </select>
                ) : (
                  <input
                    type="text"
                    className="form-input"
                    id={`organizer-${org.id}`}
                    placeholder={`Enter ${org.name.toLowerCase()}`}
                    value={organizerValues[org.id!]?.textValue || ''}
                    onChange={(e) => setOrganizerValues({
                      ...organizerValues,
                      [org.id!]: { textValue: e.target.value }
                    })}
                  />
                )}
              </div>
            ))}
          </div>
        )}

        {organizers.length === 0 && (
          <div className="form-row">
            <div className="form-group">
              <label className="form-label" htmlFor="item-category">Category</label>
              <input
                type="text"
                className="form-input"
                id="item-category"
                placeholder="Enter category"
                value={newItem.category}
                onChange={(e) => setNewItem({ ...newItem, category: e.target.value })}
              />
            </div>

            <div className="form-group">
              <label className="form-label" htmlFor="item-location">Location</label>
              <input
                type="text"
                className="form-input"
                id="item-location"
                placeholder="Enter location"
                value={newItem.location}
                onChange={(e) => setNewItem({ ...newItem, location: e.target.value })}
              />
            </div>
          </div>
        )}

        <div className="form-row">
          <div className="form-group">
            <label className="form-label" htmlFor="item-price">Purchase Price</label>
            <input
              type="number"
              className="form-input"
              id="item-price"
              placeholder="0.00"
              step="0.01"
              min="0"
              value={newItem.purchase_price || ''}
              onChange={(e) => setNewItem({ ...newItem, purchase_price: e.target.value ? parseFloat(e.target.value) : undefined })}
            />
          </div>

          <div className="form-group">
            <label className="form-label" htmlFor="item-quantity">Quantity</label>
            <input
              type="number"
              className="form-input"
              id="item-quantity"
              placeholder="1"
              min="1"
              value={newItem.quantity || 1}
              onChange={(e) => setNewItem({ ...newItem, quantity: parseInt(e.target.value) || 1 })}
            />
          </div>
        </div>

        <div className="form-row">
          <div className="form-group">
            <label className="form-label" htmlFor="item-purchase-date">Purchase Date</label>
            <input
              type="date"
              className="form-input"
              id="item-purchase-date"
              value={newItem.purchase_date || ''}
              onChange={(e) => setNewItem({ ...newItem, purchase_date: e.target.value })}
            />
          </div>

          <div className="form-group">
            <label className="form-label" htmlFor="item-warranty">Warranty Expiry</label>
            <input
              type="date"
              className="form-input"
              id="item-warranty"
              value={newItem.warranty_expiry || ''}
              onChange={(e) => setNewItem({ ...newItem, warranty_expiry: e.target.value })}
            />
          </div>
        </div>

        <div className="form-group">
          <label className="form-label" htmlFor="item-description">Description</label>
          <textarea
            className="form-input"
            id="item-description"
            placeholder="Optional description"
            rows={3}
            value={newItem.description}
            onChange={(e) => setNewItem({ ...newItem, description: e.target.value })}
          />
        </div>
      </Modal>
    </>
  );
}
