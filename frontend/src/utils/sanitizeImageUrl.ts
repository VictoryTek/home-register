/**
 * Sanitizes and validates an image URL to prevent XSS attacks.
 *
 * Allowed URL types:
 * - HTTPS URLs (secure image hosting)
 * - HTTP URLs (local development, less secure)
 * - Data URLs with image MIME types (base64-encoded images)
 *
 * Blocked URL types:
 * - javascript: protocol (XSS vector)
 * - data: URLs with non-image MIME types (XSS vector)
 * - file: protocol (local file disclosure)
 * - vbscript:, about:, etc. (various XSS vectors)
 *
 * @param url - The URL string to sanitize
 * @returns The sanitized URL if valid, null if invalid/dangerous
 */
export function sanitizeImageUrl(url: string): string | null {
  // Empty URLs are valid (used for clearing images)
  if (!url || url.trim() === '') {
    return '';
  }

  const trimmedUrl = url.trim();

  // Validate data URLs specifically for image content
  if (trimmedUrl.toLowerCase().startsWith('data:')) {
    // Only allow data URLs with image MIME types
    const imageDataUrlPattern = /^data:image\/(png|jpeg|jpg|gif|webp|svg\+xml|bmp|x-icon);/i;
    if (imageDataUrlPattern.test(trimmedUrl)) {
      return trimmedUrl; // Valid data URL with image MIME type
    }
    // Reject data URLs with non-image MIME types or malformed format
    console.warn('Rejected non-image data URL');
    return null;
  }

  // For non-data URLs, use URL constructor to parse and validate
  try {
    const parsedUrl = new URL(trimmedUrl);

    // Safelist: Only allow HTTP and HTTPS protocols
    if (parsedUrl.protocol === 'http:' || parsedUrl.protocol === 'https:') {
      return trimmedUrl; // Valid HTTP/HTTPS URL
    }

    // Reject any other protocol (javascript:, file:, vbscript:, etc.)
    console.warn(`Rejected URL with dangerous protocol: ${parsedUrl.protocol}`);
    return null;
  } catch (error) {
    // Invalid URL format (URL constructor throws TypeError)
    console.warn('Invalid URL format:', error);
    return null;
  }
}

/**
 * Type guard to check if a URL is safe for image display
 *
 * @param url - The URL to check
 * @returns true if URL is safe, false otherwise
 */
export function isValidImageUrl(url: string): boolean {
  return sanitizeImageUrl(url) !== null;
}
