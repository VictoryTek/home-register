/**
 * Date formatting utility
 * Formats dates according to user preferences
 */

export type DateFormatType = 'MM/DD/YYYY' | 'DD/MM/YYYY' | 'YYYY-MM-DD' | 'DD.MM.YYYY';

/**
 * Format a date string according to the specified format
 * @param dateString - ISO date string from the API
 * @param format - Desired date format
 * @returns Formatted date string
 */
export function formatDate(dateString: string | undefined | null, format: DateFormatType = 'MM/DD/YYYY'): string {
  if (!dateString) return '';
  
  try {
    const date = new Date(dateString);
    
    // Check if date is valid
    if (isNaN(date.getTime())) return '';
    
    const day = date.getDate().toString().padStart(2, '0');
    const month = (date.getMonth() + 1).toString().padStart(2, '0');
    const year = date.getFullYear();
    
    switch (format) {
      case 'MM/DD/YYYY':
        return `${month}/${day}/${year}`;
      case 'DD/MM/YYYY':
        return `${day}/${month}/${year}`;
      case 'YYYY-MM-DD':
        return `${year}-${month}-${day}`;
      case 'DD.MM.YYYY':
        return `${day}.${month}.${year}`;
      default:
        return `${month}/${day}/${year}`;
    }
  } catch (error) {
    console.error('Error formatting date:', error);
    return '';
  }
}

/**
 * Hook to get the date formatter with user's preferred format
 * Usage: const { formatDate } = useDateFormat();
 */
export function createDateFormatter(format: DateFormatType = 'MM/DD/YYYY') {
  return (dateString: string | undefined | null) => formatDate(dateString, format);
}
