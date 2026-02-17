/**
 * HEIC/HEIF image conversion utilities using heic2any library.
 * Converts HEIC images to JPEG for browser compatibility.
 */

/**
 * Custom error class for HEIC conversion failures
 */
export class HeicConversionError extends Error {
  constructor(
    message: string,
    public readonly originalError?: unknown
  ) {
    super(message);
    this.name = 'HeicConversionError';
  }
}

/**
 * Convert a HEIC/HEIF image file to JPEG format.
 * Uses lazy-loaded heic2any library to minimize initial bundle size.
 * Typical conversion time: 1.5-3 seconds for 12MP iPhone photos.
 *
 * @param file - HEIC/HEIF File object to convert
 * @returns Promise resolving to JPEG File object
 * @throws HeicConversionError if conversion fails
 *
 * @example
 * try {
 *   const jpegFile = await convertHeicToJpeg(heicFile);
 *   // Process JPEG file normally
 * } catch (error) {
 *   if (error instanceof HeicConversionError) {
 *     console.error('HEIC conversion failed:', error.message);
 *   }
 * }
 */
export async function convertHeicToJpeg(file: File): Promise<File> {
  try {
    // Lazy load heic2any only when needed (code splitting for bundle size)
    const heic2any = await import('heic2any');

    // Convert HEIC to JPEG blob
    const convertedBlob = await heic2any.default({
      blob: file,
      toType: 'image/jpeg',
      quality: 0.9, // High quality before subsequent compression step
    });

    // heic2any may return Blob or Blob[] (array for Live Photos with multiple frames)
    // We take the first frame for Live Photos
    const blob = Array.isArray(convertedBlob) ? convertedBlob[0] : convertedBlob;

    if (!blob) {
      throw new HeicConversionError('Conversion resulted in invalid blob');
    }

    // Create new File object with .jpg extension
    const newName = file.name.replace(/\.heic$/i, '.jpg').replace(/\.heif$/i, '.jpg');
    return new File([blob], newName, { type: 'image/jpeg' });
  } catch (error) {
    console.error('HEIC conversion failed:', error);
    throw new HeicConversionError(
      'Failed to convert HEIC image. Please convert to JPG or PNG first.',
      error
    );
  }
}
