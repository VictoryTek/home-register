import { useEffect, useRef, useCallback } from 'react';

interface ImageLightboxProps {
  isOpen: boolean;
  onClose: () => void;
  imageUrl: string;
  altText: string;
}

export function ImageLightbox({ isOpen, onClose, imageUrl, altText }: ImageLightboxProps) {
  const closeButtonRef = useRef<HTMLButtonElement>(null);
  const previousFocusRef = useRef<HTMLElement | null>(null);

  const handleKeyDown = useCallback(
    (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        onClose();
      }
      // Focus trap: keep focus within the lightbox
      if (e.key === 'Tab') {
        e.preventDefault();
        closeButtonRef.current?.focus();
      }
    },
    [onClose]
  );

  useEffect(() => {
    if (isOpen) {
      previousFocusRef.current = document.activeElement as HTMLElement;
      document.addEventListener('keydown', handleKeyDown);
      // Focus close button after a short delay (for transition)
      const timer = setTimeout(() => {
        closeButtonRef.current?.focus();
      }, 100);
      // Prevent body scroll
      document.body.style.overflow = 'hidden';
      return () => {
        document.removeEventListener('keydown', handleKeyDown);
        clearTimeout(timer);
        document.body.style.overflow = '';
      };
    }
    // Restore focus on close
    previousFocusRef.current?.focus();
    return undefined;
  }, [isOpen, handleKeyDown]);

  if (!isOpen) {
    return null;
  }

  return (
    <div
      className="lightbox-overlay active"
      onClick={onClose}
      role="dialog"
      aria-modal="true"
      aria-label="Image viewer"
    >
      <div className="lightbox-content" onClick={(e) => e.stopPropagation()}>
        <img src={imageUrl} alt={altText} className="lightbox-image" />
      </div>
      <button
        ref={closeButtonRef}
        className="lightbox-close"
        onClick={onClose}
        aria-label="Close image viewer"
        type="button"
      >
        <i className="fas fa-times"></i>
      </button>
    </div>
  );
}
