/**
 * Type definitions for heic2any library
 * heic2any converts HEIC/HEIF images to other formats (JPEG, PNG, GIF, WebP)
 * @see https://github.com/alexcorvi/heic2any
 */

declare module 'heic2any' {
  /**
   * Options for HEIC to other format conversion
   */
  interface Heic2anyOptions {
    /**
     * The HEIC/HEIF Blob or File to convert
     */
    blob: Blob | File;

    /**
     * Target image format MIME type
     */
    toType: 'image/jpeg' | 'image/png' | 'image/gif' | 'image/webp';

    /**
     * Quality of the output image (0-1)
     * Only applies to lossy formats (JPEG, WebP)
     * @default 0.5
     */
    quality?: number;

    /**
     * Whether to convert all frames (for animated/Live Photos)
     * If true, returns array of Blobs
     * @default false
     */
    multiple?: boolean;
  }

  /**
   * Convert HEIC/HEIF image(s) to another format
   * @param options - Conversion options
   * @returns Single Blob or array of Blobs (if multiple frames/Live Photo)
   */
  function heic2any(options: Heic2anyOptions): Promise<Blob | Blob[]>;

  export default heic2any;
}
