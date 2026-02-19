import { useState, useRef, useCallback } from 'react';
import { imageApi } from '@/services/api';

interface ImageOrganizerInputProps {
  value?: string;
  onChange: (url: string | undefined) => void;
  itemName: string;
  required?: boolean;
}

const MAX_FILE_SIZE = 5 * 1024 * 1024; // 5 MB
const ACCEPTED_TYPES = ['image/jpeg', 'image/png', 'image/gif', 'image/webp'];

export function ImageOrganizerInput({
  value,
  onChange,
  itemName,
  required,
}: ImageOrganizerInputProps) {
  const [uploading, setUploading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [dragging, setDragging] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleUpload = useCallback(
    async (file: File) => {
      setError(null);

      // Client-side validations
      if (!ACCEPTED_TYPES.includes(file.type)) {
        setError('Only JPG, PNG, GIF, and WebP images are allowed.');
        return;
      }

      if (file.size > MAX_FILE_SIZE) {
        setError(`File is too large. Maximum size is ${MAX_FILE_SIZE / 1024 / 1024} MB.`);
        return;
      }

      setUploading(true);
      try {
        const result = await imageApi.upload(file);
        if (result.success && result.data) {
          onChange(result.data.url);
        } else {
          setError(result.error ?? 'Upload failed');
        }
      } catch {
        setError('Failed to upload image');
      } finally {
        setUploading(false);
      }
    },
    [onChange]
  );

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      void handleUpload(file);
    }
    // Reset the input so the same file can be re-selected
    if (fileInputRef.current) {
      fileInputRef.current.value = '';
    }
  };

  const handleRemove = useCallback(() => {
    // Extract filename from URL to delete from server
    if (value) {
      const filename = value.split('/').pop();
      if (filename) {
        // Fire-and-forget delete - don't block the UI
        void imageApi.delete(filename);
      }
    }
    onChange(undefined);
    setError(null);
  }, [value, onChange]);

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setDragging(true);
  };

  const handleDragLeave = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setDragging(false);
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setDragging(false);

    const file = e.dataTransfer.files[0];
    if (file) {
      void handleUpload(file);
    }
  };

  return (
    <div className="image-organizer-container">
      {/* Preview existing image */}
      {value && (
        <div className="image-upload-preview">
          <img src={value} alt={itemName} />
          <button
            type="button"
            className="image-upload-remove"
            onClick={handleRemove}
            aria-label="Remove image"
            title="Remove image"
          >
            <i className="fas fa-times"></i>
          </button>
        </div>
      )}

      {/* Upload drop zone */}
      {!value && (
        <div
          className={`image-upload-dropzone${dragging ? ' dragging' : ''}`}
          onClick={() => fileInputRef.current?.click()}
          onDragOver={handleDragOver}
          onDragLeave={handleDragLeave}
          onDrop={handleDrop}
          role="button"
          tabIndex={0}
          onKeyDown={(e) => {
            if (e.key === 'Enter' || e.key === ' ') {
              e.preventDefault();
              fileInputRef.current?.click();
            }
          }}
          aria-label={`Upload image for ${itemName}`}
        >
          {uploading ? (
            <>
              <i className="fas fa-spinner fa-spin"></i>
              <p>Uploading...</p>
            </>
          ) : (
            <>
              <i className="fas fa-camera"></i>
              <p>Click to upload or drag & drop</p>
              <p className="form-hint-upload">
                JPG, PNG, GIF, WebP • Max {MAX_FILE_SIZE / 1024 / 1024}MB
              </p>
            </>
          )}
        </div>
      )}

      {/* Hidden file input */}
      <input
        ref={fileInputRef}
        type="file"
        accept="image/jpeg,image/png,image/gif,image/webp"
        style={{ display: 'none' }}
        onChange={handleFileChange}
        required={required && !value}
      />

      {/* Error message */}
      {error && <p className="form-error">{error}</p>}
    </div>
  );
}
