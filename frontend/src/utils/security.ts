/**
 * Security utilities for XSS prevention and safe content handling
 */

/**
 * Escapes HTML special characters to prevent XSS attacks
 * Use this when inserting user-controlled content into HTML strings
 */
export function escapeHtml(unsafe: string): string {
  return unsafe
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#039;');
}

/**
 * Creates a safe print window with escaped content
 * Use this instead of document.write() with user content
 */
export function createSafePrintWindow(
  title: string,
  contentBuilder: (doc: Document) => void
): void {
  const printWindow = window.open('', '', 'width=800,height=600');
  if (!printWindow) {
    return;
  }

  // Build document structure safely
  printWindow.document.title = title;
  
  // Call the content builder to populate the document body
  contentBuilder(printWindow.document);
  
  printWindow.document.close();
  printWindow.focus();
  printWindow.print();
  printWindow.close();
}
