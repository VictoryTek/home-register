import { useState, useEffect, useCallback } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { useApp } from '@/context/AppContext';
import { organizerApi, inventoryApi } from '@/services/api';
import { Header, Modal, ConfirmModal, LoadingState, EmptyState } from '@/components';
import type { 
  OrganizerTypeWithOptions, 
  OrganizerOption,
  CreateOrganizerTypeRequest,
  CreateOrganizerOptionRequest,
  Inventory 
} from '@/types';

export function OrganizersPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { showToast } = useApp();
  const inventoryId = id ? parseInt(id, 10) : null;
  
  const [inventories, setInventories] = useState<Inventory[]>([]);
  const [inventory, setInventory] = useState<Inventory | null>(null);
  const [organizers, setOrganizers] = useState<OrganizerTypeWithOptions[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  
  // Modal states
  const [showTypeModal, setShowTypeModal] = useState(false);
  const [showOptionModal, setShowOptionModal] = useState(false);
  const [showDeleteTypeModal, setShowDeleteTypeModal] = useState(false);
  const [showDeleteOptionModal, setShowDeleteOptionModal] = useState(false);
  
  // Edit states
  const [editingType, setEditingType] = useState<OrganizerTypeWithOptions | null>(null);
  const [editingOption, setEditingOption] = useState<OrganizerOption | null>(null);
  const [selectedTypeForOption, setSelectedTypeForOption] = useState<OrganizerTypeWithOptions | null>(null);
  const [typeToDelete, setTypeToDelete] = useState<OrganizerTypeWithOptions | null>(null);
  const [optionToDelete, setOptionToDelete] = useState<{ option: OrganizerOption; type: OrganizerTypeWithOptions } | null>(null);
  
  // Form states
  const [typeName, setTypeName] = useState('');
  const [typeInputType, setTypeInputType] = useState<'select' | 'text'>('select');
  const [typeIsRequired, setTypeIsRequired] = useState(false);
  const [optionName, setOptionName] = useState('');

  const loadData = useCallback(async () => {
    if (!inventoryId) {return;}
    setLoading(true);
    setError(null);
    try {
      const [invResult, orgResult] = await Promise.all([
        inventoryApi.getById(inventoryId),
        organizerApi.getByInventory(inventoryId),
      ]);
      if (invResult.success && invResult.data) {
        setInventory(invResult.data);
      } else {
        setError('Inventory not found');
      }
      if (orgResult.success && orgResult.data) {
        setOrganizers(orgResult.data);
      }
    } catch {
      setError('Failed to load organizers');
    } finally {
      setLoading(false);
    }
  }, [inventoryId]);

  const loadInventories = useCallback(async () => {
    setLoading(true);
    try {
      const res = await inventoryApi.getAll();
      if (res.success && res.data) {
        setInventories(res.data);
      }
    } catch {
      setError('Failed to load inventories');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    if (inventoryId) {
      void loadData();
    } else {
      void loadInventories();
    }
  }, [inventoryId, loadData, loadInventories]);

  const loadData = async () => {
    if (!inventoryId) {return;}
    setLoading(true);
    setError(null);
    try {
      const [inventoryRes, organizersRes] = await Promise.all([
        inventoryApi.getById(inventoryId),
        organizerApi.getByInventory(inventoryId),
      ]);
      
      if (inventoryRes.success && inventoryRes.data) {
        setInventory(inventoryRes.data);
      } else {
        setError('Inventory not found');
        return;
      }
      
      if (organizersRes.success && organizersRes.data) {
        setOrganizers(organizersRes.data);
      }
    } catch (err) {
      setError('Failed to load organizers');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const openCreateTypeModal = () => {
    setEditingType(null);
    setTypeName('');
    setTypeInputType('select');
    setTypeIsRequired(false);
    setShowTypeModal(true);
  };

  const openEditTypeModal = (type: OrganizerTypeWithOptions) => {
    setEditingType(type);
    setTypeName(type.name);
    setTypeInputType(type.input_type);
    setTypeIsRequired(type.is_required);
    setShowTypeModal(true);
  };

  const openCreateOptionModal = (type: OrganizerTypeWithOptions) => {
    setSelectedTypeForOption(type);
    setEditingOption(null);
    setOptionName('');
    setShowOptionModal(true);
  };

  const openEditOptionModal = (option: OrganizerOption, type: OrganizerTypeWithOptions) => {
    setSelectedTypeForOption(type);
    setEditingOption(option);
    setOptionName(option.name);
    setShowOptionModal(true);
  };

  const handleSaveType = async () => {
    if (!typeName.trim()) {
      showToast('Please enter a name for the organizer', 'error');
      return;
    }

    try {
      if (editingType?.id) {
        // Update existing type
        const res = await organizerApi.updateType(editingType.id, {
          name: typeName,
          input_type: typeInputType,
          is_required: typeIsRequired,
        });
        if (res.success) {
          showToast('Organizer updated successfully', 'success');
          void loadData();
        } else {
          showToast(res.error ?? 'Failed to update organizer', 'error');
        }
      } else {
        // Create new type
        if (!inventoryId) {
          return;
        }
        const data: CreateOrganizerTypeRequest = {
          name: typeName,
          input_type: typeInputType,
          is_required: typeIsRequired,
        };
        const res = await organizerApi.createType(inventoryId, data);
        if (res.success) {
          showToast('Organizer created successfully', 'success');
          void loadData();
        } else {
          showToast(res.error ?? 'Failed to create organizer', 'error');
        }
      }
      setShowTypeModal(false);
    } catch (err) {
      showToast('An error occurred', 'error');
      console.error(err);
    }
  };

  const handleSaveOption = async () => {
    if (!optionName.trim() || !selectedTypeForOption) {
      showToast('Please enter a name for the option', 'error');
      return;
    }

    try {
      if (editingOption?.id) {
        // Update existing option
        const res = await organizerApi.updateOption(editingOption.id, { name: optionName });
        if (res.success) {
          showToast('Option updated successfully', 'success');
          void loadData();
        } else {
          showToast(res.error ?? 'Failed to update option', 'error');
        }
      } else {
        // Create new option
        const data: CreateOrganizerOptionRequest = { name: optionName };
        const res = await organizerApi.createOption(selectedTypeForOption.id, data);
        if (res.success) {
          showToast('Option created successfully', 'success');
          void loadData();
        } else {
          showToast(res.error ?? 'Failed to create option', 'error');
        }
      }
      setShowOptionModal(false);
    } catch (err) {
      showToast('An error occurred', 'error');
      console.error(err);
    }
  };

  const confirmDeleteType = (type: OrganizerTypeWithOptions) => {
    setTypeToDelete(type);
    setShowDeleteTypeModal(true);
  };

  const handleDeleteType = async () => {
    if (!typeToDelete?.id) {
      return;
    }
    
    try {
      const res = await organizerApi.deleteType(typeToDelete.id);
      if (res.success) {
        showToast('Organizer deleted successfully', 'success');
        void loadData();
      } else {
        showToast(res.error ?? 'Failed to delete organizer', 'error');
      }
    } catch (err) {
      showToast('An error occurred', 'error');
      console.error(err);
    } finally {
      setShowDeleteTypeModal(false);
      setTypeToDelete(null);
    }
  };

  const confirmDeleteOption = (option: OrganizerOption, type: OrganizerTypeWithOptions) => {
    setOptionToDelete({ option, type });
    setShowDeleteOptionModal(true);
  };

  const handleDeleteOption = async () => {
    if (!optionToDelete?.option.id) {
      return;
    }
    
    try {
      const res = await organizerApi.deleteOption(optionToDelete.option.id);
      if (res.success) {
        showToast('Option deleted successfully', 'success');
        void loadData();
      } else {
        showToast(res.error ?? 'Failed to delete option', 'error');
      }
    } catch (err) {
      showToast('An error occurred', 'error');
      console.error(err);
    } finally {
      setShowDeleteOptionModal(false);
      setOptionToDelete(null);
    }
  };

  if (loading) {
    return <LoadingState message="Loading organizers..." />;
  }

  // If no inventory ID, show inventory selection
  if (!inventoryId) {
    return (
      <>
        <Header
          title="Organizers"
          subtitle="Select an inventory to manage its organizers"
          icon="fas fa-folder-tree"
        />
        <div className="content">
          {inventories.length === 0 ? (
            <EmptyState
              icon="fas fa-warehouse"
              title="No Inventories"
              text="Create an inventory first to manage organizers."
              action={
                <button className="btn btn-secondary" onClick={() => navigate('/')}>
                  <i className="fas fa-arrow-left"></i>
                  Go to Inventories
                </button>
              }
            />
          ) : (
            <div className="organizers-inventory-list">
              {inventories.map((inv) => (
                <div
                  key={inv.id}
                  className="organizer-inventory-item"
                  onClick={() => navigate(`/inventory/${inv.id}/organizers`)}
                >
                  <div className="organizer-inventory-icon">
                    <i className="fas fa-warehouse"></i>
                  </div>
                  <div className="organizer-inventory-info">
                    <h3>{inv.name}</h3>
                    {inv.description && <p>{inv.description}</p>}
                  </div>
                  <div className="organizer-inventory-action">
                    <i className="fas fa-chevron-right"></i>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </>
    );
  }

  if (error || !inventory) {
    return (
      <div className="page-container">
        <EmptyState
          icon="fas fa-exclamation-triangle"
          title="Error"
          text={error ?? 'Failed to load inventory'}
          action={
            <button className="btn btn-primary" onClick={() => navigate('/')}>
              Go Back
            </button>
          }
        />
      </div>
    );
  }

  return (
    <>
      <Header
        title="Organizers"
        subtitle={inventory.name}
        icon="fas fa-folder-tree"
      />
      
      <div className="content">
        <button className="btn btn-icon" onClick={() => navigate(`/inventory/${inventoryId}`)} title="Back to Inventory">
          <i className="fas fa-arrow-left"></i>
        </button>
        
        <div className="page-actions">
          <button className="btn btn-primary" onClick={openCreateTypeModal}>
            <i className="fas fa-plus"></i>
            Add Organizer
          </button>
        </div>

        {/* Organizers List */}
        {organizers.length === 0 ? (
          <EmptyState
            icon="fas fa-folder-open"
            title="No Organizers Yet"
            text="Create organizers to classify and categorize items in this inventory."
          />
        ) : (
          <div className="organizers-grid">
          {[...organizers].sort((a, b) => a.name.localeCompare(b.name)).map((organizer) => (
            <div key={organizer.id} className="organizer-card">
              <div className="organizer-header">
                <div className="organizer-info">
                  <h3>{organizer.name}</h3>
                  <div className="organizer-meta">
                    <span className={`badge badge-${organizer.input_type === 'select' ? 'primary' : 'secondary'}`}>
                      {organizer.input_type === 'select' ? 'Dropdown' : 'Text Input'}
                    </span>
                    {organizer.is_required && (
                      <span className="badge badge-warning">Required</span>
                    )}
                  </div>
                </div>
                <div className="organizer-actions">
                  <button
                    className="btn btn-icon btn-ghost"
                    onClick={() => openEditTypeModal(organizer)}
                    title="Edit"
                  >
                    <i className="fas fa-edit"></i>
                  </button>
                  <button
                    className="btn btn-icon btn-ghost btn-danger"
                    onClick={() => confirmDeleteType(organizer)}
                    title="Delete"
                  >
                    <i className="fas fa-trash"></i>
                  </button>
                </div>
              </div>

              {organizer.input_type === 'select' && (
                <div className="organizer-options">
                  <div className="options-header">
                    <span className="options-count">
                      {organizer.options.length} option{organizer.options.length !== 1 ? 's' : ''}
                    </span>
                    <button
                      className="btn btn-sm btn-ghost"
                      onClick={() => openCreateOptionModal(organizer)}
                    >
                      <i className="fas fa-plus"></i>
                      <span>Add Option</span>
                    </button>
                  </div>
                  
                  {organizer.options.length > 0 ? (
                    <ul className="options-list">
                      {organizer.options.map((option) => (
                        <li key={option.id} className="option-item">
                          <span className="option-name">{option.name}</span>
                          <div className="option-actions">
                            <button
                              className="btn btn-icon btn-sm btn-ghost"
                              onClick={() => openEditOptionModal(option, organizer)}
                              title="Edit"
                            >
                              <i className="fas fa-edit"></i>
                            </button>
                            <button
                              className="btn btn-icon btn-sm btn-ghost btn-danger"
                              onClick={() => confirmDeleteOption(option, organizer)}
                              title="Delete"
                            >
                              <i className="fas fa-times"></i>
                            </button>
                          </div>
                        </li>
                      ))}
                    </ul>
                  ) : (
                    <p className="options-empty">No options defined yet</p>
                  )}
                </div>
              )}

              {organizer.input_type === 'text' && (
                <div className="organizer-text-info">
                  <i className="fas fa-keyboard"></i>
                  <span>Users will enter free-form text</span>
                </div>
              )}
            </div>
          ))}
        </div>
      )}

      {/* Create/Edit Type Modal */}
      <Modal
        isOpen={showTypeModal}
        onClose={() => setShowTypeModal(false)}
        title={editingType ? 'Edit Organizer' : 'Create Organizer'}
        footer={
          <>
            <button className="btn btn-secondary" onClick={() => setShowTypeModal(false)}>
              Cancel
            </button>
            <button className="btn btn-primary" onClick={handleSaveType}>
              <i className={`fas fa-${editingType ? 'save' : 'plus'}`}></i>
              {editingType ? 'Save Changes' : 'Create Organizer'}
            </button>
          </>
        }
      >
        <div className="form-group">
          <label className="form-label" htmlFor="typeName">Organizer Name *</label>
          <input
            className="form-input"
            id="typeName"
            type="text"
            value={typeName}
            onChange={(e) => setTypeName(e.target.value)}
            placeholder="e.g., Brand, Color, Condition, Room"
            autoFocus
          />
        </div>

        <div className="form-group">
          <label className="form-label" htmlFor="typeInputType">Input Type</label>
          <select
            className="form-select"
            id="typeInputType"
            value={typeInputType}
            onChange={(e) => setTypeInputType(e.target.value as 'select' | 'text')}
          >
            <option value="select">Dropdown (Select from options)</option>
            <option value="text">Text Input (Free-form entry)</option>
          </select>
          <p className="form-hint">
            {typeInputType === 'select'
              ? 'Users will select from predefined options you create after saving.'
              : 'Users can enter any text value (e.g., serial numbers, custom notes).'}
          </p>
        </div>

        <div className="form-group">
          <label className="form-checkbox">
            <input
              type="checkbox"
              checked={typeIsRequired}
              onChange={(e) => setTypeIsRequired(e.target.checked)}
            />
            <span className="form-checkbox-label">Required when adding items</span>
          </label>
          <p className="form-hint">
            If checked, users must provide a value for this organizer when creating items.
          </p>
        </div>
      </Modal>

      {/* Create/Edit Option Modal */}
      <Modal
        isOpen={showOptionModal}
        onClose={() => setShowOptionModal(false)}
        title={editingOption ? 'Edit Option' : 'Add Option'}
        subtitle={selectedTypeForOption ? `Adding to "${selectedTypeForOption.name}"` : undefined}
        footer={
          <>
            <button className="btn btn-secondary" onClick={() => setShowOptionModal(false)}>
              Cancel
            </button>
            <button className="btn btn-primary" onClick={handleSaveOption}>
              <i className={`fas fa-${editingOption ? 'save' : 'plus'}`}></i>
              {editingOption ? 'Save Changes' : 'Add Option'}
            </button>
          </>
        }
      >
        <div className="form-group">
          <label className="form-label" htmlFor="optionName">Option Name *</label>
          <input
            className="form-input"
            id="optionName"
            type="text"
            value={optionName}
            onChange={(e) => setOptionName(e.target.value)}
            placeholder="Enter option value"
            autoFocus
          />
          <p className="form-hint">
            This will appear as a selectable choice in the dropdown.
          </p>
        </div>
      </Modal>

      {/* Delete Type Confirmation */}
      <ConfirmModal
        isOpen={showDeleteTypeModal}
        onClose={() => setShowDeleteTypeModal(false)}
        onConfirm={handleDeleteType}
        title="Delete Organizer"
        message={`Are you sure you want to delete "${typeToDelete?.name}"? This will also remove all options and item values associated with this organizer.`}
        confirmText="Delete"
      />

      {/* Delete Option Confirmation */}
      <ConfirmModal
        isOpen={showDeleteOptionModal}
        onClose={() => setShowDeleteOptionModal(false)}
        onConfirm={handleDeleteOption}
        title="Delete Option"
        message={`Are you sure you want to delete "${optionToDelete?.option.name}" from "${optionToDelete?.type.name}"?`}
        confirmText="Delete"
      />
      </div>
    </>
  );
}
