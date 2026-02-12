/**
 * Notifications utility
 * Checks for warranty expirations and other alerts
 */

import type { Item } from '@/types';

export interface WarrantyNotification {
  id: number;
  itemName: string;
  inventoryId: number;
  warrantyExpiry: string;
  daysUntilExpiry: number;
  status: 'expired' | 'expiring-soon' | 'expiring-this-month';
}

/**
 * Check items for warranty expiry notifications
 * @param items - Array of items to check
 * @param daysThreshold - Days before expiry to consider "expiring soon" (default 30)
 * @returns Array of warranty notifications
 */
export function checkWarrantyNotifications(
  items: Item[],
  daysThreshold = 30
): WarrantyNotification[] {
  const notifications: WarrantyNotification[] = [];
  const now = new Date();
  now.setHours(0, 0, 0, 0); // Set to start of day for accurate comparison

  items.forEach((item) => {
    if (!item.warranty_expiry || !item.id) {
      return;
    }

    const expiryDate = new Date(item.warranty_expiry);
    expiryDate.setHours(0, 0, 0, 0);

    const diffTime = expiryDate.getTime() - now.getTime();
    const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));

    // Determine status based on days until expiry
    let status: WarrantyNotification['status'];
    
    if (diffDays < 0) {
      status = 'expired';
    } else if (diffDays <= 7) {
      status = 'expiring-soon';
    } else if (diffDays <= daysThreshold) {
      status = 'expiring-this-month';
    } else {
      return; // Not expiring soon enough to notify
    }

    notifications.push({
      id: item.id,
      itemName: item.name,
      inventoryId: item.inventory_id,
      warrantyExpiry: item.warranty_expiry,
      daysUntilExpiry: diffDays,
      status,
    });
  });

  // Sort by urgency: expired first, then by days until expiry
  return notifications.sort((a, b) => {
    if (a.status === 'expired' && b.status !== 'expired') {return -1;}
    if (a.status !== 'expired' && b.status === 'expired') {return 1;}
    return a.daysUntilExpiry - b.daysUntilExpiry;
  });
}

/**
 * Get notification message for a warranty notification
 */
export function getNotificationMessage(notification: WarrantyNotification): string {
  const { itemName, daysUntilExpiry, status } = notification;

  if (status === 'expired') {
    const daysExpired = Math.abs(daysUntilExpiry);
    return `${itemName} warranty expired ${daysExpired === 1 ? '1 day' : `${daysExpired} days`} ago`;
  } else if (status === 'expiring-soon') {
    return `${itemName} warranty expires in ${daysUntilExpiry === 1 ? '1 day' : `${daysUntilExpiry} days`}`;
  } else {
    return `${itemName} warranty expires in ${daysUntilExpiry} days`;
  }
}
