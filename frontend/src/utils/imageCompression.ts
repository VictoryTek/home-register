/**
 * Compress an image file to reduce upload size.
 * Uses canvas API to resize large images and compress to JPEG.
 * Typical result: iPhone photos go from 5-10MB → 200-500KB.
 *
 * @param file - The image File object to compress
 * @param maxDimension - Maximum width or height in pixels (default: 1920)
 * @param quality - JPEG compression quality 0-1 (default: 0.85)
 * @returns A compressed base64 data URL string
 */
export function compressImage(file: File, maxDimension = 1920, quality = 0.85): Promise<string> {
  return new Promise((resolve, reject) => {
    const img = new Image();
    const canvas = document.createElement('canvas');
    const ctx = canvas.getContext('2d');

    if (!ctx) {
      reject(new Error('Could not get canvas 2D context'));
      return;
    }

    img.onload = () => {
      let { width, height } = img;

      // Scale down if exceeds max dimension
      if (width > maxDimension || height > maxDimension) {
        if (width > height) {
          height = Math.round((height * maxDimension) / width);
          width = maxDimension;
        } else {
          width = Math.round((width * maxDimension) / height);
          height = maxDimension;
        }
      }

      canvas.width = width;
      canvas.height = height;
      ctx.drawImage(img, 0, 0, width, height);

      // Convert to JPEG with specified quality
      const dataUrl = canvas.toDataURL('image/jpeg', quality);

      // Revoke the object URL to free memory
      URL.revokeObjectURL(img.src);

      resolve(dataUrl);
    };

    img.onerror = () => {
      URL.revokeObjectURL(img.src);
      reject(new Error('Failed to load image for compression'));
    };

    img.src = URL.createObjectURL(file);
  });
}
