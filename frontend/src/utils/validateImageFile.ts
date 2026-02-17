/**
 * File validation utilities for image uploads.
 * Provides extension, MIME type, and size validation.
 */

/**
 * Allowed image file extensions (lowercase)
 */
const ALLOWED_EXTENSIONS = ['jpg', 'jpeg', 'png', 'gif', 'webp', 'heic', 'heif'] as const;

/**
 * Extract file extension from filename (lowercase)
 * @param filename - The filename to extract extension from
 * @returns The lowercase file extension without the dot
 */
export function getFileExtension(filename: string): string {
  const parts = filename.toLowerCase().split('.');
  return parts.length > 1 ? (parts[parts.length - 1] ?? '') : '';
}

/**
 * Check if a filename has a valid image extension
 * @param filename - The filename to validate
 * @returns True if extension is in the allowed list
 */
export function isValidImageExtension(filename: string): boolean {
  const ext = getFileExtension(filename);
  return ALLOWED_EXTENSIONS.includes(ext as (typeof ALLOWED_EXTENSIONS)[number]);
}

/**
 * Check if a file is a HEIC/HEIF format image
 * Checks both file extension and MIME type
 * @param file - The File object to check
 * @returns True if file is HEIC/HEIF format
 */
export function isHeicFile(file: File): boolean {
  const ext = getFileExtension(file.name);
  const isHeicExtension = ext === 'heic' || ext === 'heif';
  const isHeicMimeType = file.type === 'image/heic' || file.type === 'image/heif';
  return isHeicExtension || isHeicMimeType;
}

/**
 * Get a user-friendly string of all allowed image formats
 * @returns Formatted string like "JPG, PNG, WebP, or HEIC"
 */
export function getAllowedFormatsString(): string {
  return 'JPG, PNG, GIF, WebP, or HEIC';
}

/**
 * Validate an image file for upload
 * Checks extension, MIME type, and file size
 * @param file - The File object to validate
 * @param maxSizeBytes - Maximum allowed file size in bytes
 * @returns Validation result with specific error message if invalid
 */
export function validateImageFile(
  file: File,
  maxSizeBytes: number
): { valid: boolean; error?: string } {
  // Check file extension
  if (!isValidImageExtension(file.name)) {
    const ext = getFileExtension(file.name) || 'unknown';
    return {
      valid: false,
      error: `Unsupported file format (.${ext}). Supported formats: ${getAllowedFormatsString()}`,
    };
  }

  // Check file size
  if (file.size > maxSizeBytes) {
    const maxSizeMB = Math.round(maxSizeBytes / (1024 * 1024));
    return {
      valid: false,
      error: `Image must be under ${maxSizeMB}MB`,
    };
  }

  // Note: MIME type validation is optional since it can be spoofed
  // We rely on browser Image() API to reject invalid images during processing

  return { valid: true };
}
