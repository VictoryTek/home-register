/**
 * Currency formatting utility
 * Formats prices according to user preferences
 */

export type CurrencyType = 'USD' | 'EUR' | 'GBP' | 'CAD' | 'AUD' | 'JPY';

interface CurrencyConfig {
  symbol: string;
  position: 'before' | 'after';
  decimals: number;
}

const CURRENCY_CONFIGS: Record<CurrencyType, CurrencyConfig> = {
  USD: { symbol: '$', position: 'before', decimals: 2 },
  EUR: { symbol: '€', position: 'after', decimals: 2 },
  GBP: { symbol: '£', position: 'before', decimals: 2 },
  CAD: { symbol: 'C$', position: 'before', decimals: 2 },
  AUD: { symbol: 'A$', position: 'before', decimals: 2 },
  JPY: { symbol: '¥', position: 'before', decimals: 0 }, // JPY typically doesn't use decimals
};

/**
 * Format a price according to the specified currency
 * @param amount - The numeric amount to format
 * @param currency - Currency code (USD, EUR, etc.)
 * @returns Formatted currency string
 */
export function formatCurrency(amount: number | undefined | null, currency: CurrencyType = 'USD'): string {
  if (amount === undefined || amount === null) return '';
  
  const config = CURRENCY_CONFIGS[currency] || CURRENCY_CONFIGS.USD;
  const formattedAmount = amount.toFixed(config.decimals);
  
  if (config.position === 'before') {
    return `${config.symbol}${formattedAmount}`;
  } else {
    return `${formattedAmount}${config.symbol}`;
  }
}

/**
 * Get just the currency symbol for a given currency
 * @param currency - Currency code
 * @returns Currency symbol
 */
export function getCurrencySymbol(currency: CurrencyType = 'USD'): string {
  return CURRENCY_CONFIGS[currency]?.symbol || '$';
}
