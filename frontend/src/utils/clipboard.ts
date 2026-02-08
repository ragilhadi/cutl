/**
 * Copies text to the clipboard using the modern Clipboard API
 * @param text - The text to copy
 * @returns Promise that resolves when copied, rejects on error
 */
export async function copyToClipboard(text: string): Promise<void> {
  if (!navigator.clipboard) {
    // Fallback for older browsers
    return fallbackCopy(text);
  }

  try {
    await navigator.clipboard.writeText(text);
  } catch (error) {
    // If modern API fails, try fallback
    return fallbackCopy(text);
  }
}

/**
 * Fallback copy method using execCommand
 * @param text - The text to copy
 */
function fallbackCopy(text: string): Promise<void> {
  return new Promise((resolve, reject) => {
    const textArea = document.createElement('textarea');
    textArea.value = text;
    textArea.style.position = 'fixed';
    textArea.style.left = '-9999px';
    textArea.style.top = '0';
    document.body.appendChild(textArea);
    textArea.focus();
    textArea.select();

    try {
      const successful = document.execCommand('copy');
      document.body.removeChild(textArea);

      if (successful) {
        resolve();
      } else {
        reject(new Error('Failed to copy to clipboard'));
      }
    } catch (error) {
      document.body.removeChild(textArea);
      reject(error);
    }
  });
}

/**
 * Gets the remaining character count for a custom code
 * @param current - Current code value
 * @param max - Maximum allowed characters (default: 32)
 * @returns Number of remaining characters
 */
export function getRemainingChars(current: string, max: number = 32): number {
  return max - current.length;
}

/**
 * Format a Unix timestamp to a localized date string
 * @param timestamp - Unix timestamp in seconds
 * @returns Formatted date string
 */
export function formatExpirationDate(timestamp: number): string {
  return new Date(timestamp * 1000).toLocaleString(undefined, {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
}

/**
 * Calculate time until expiration from now
 * @param timestamp - Unix timestamp in seconds
 * @returns Human readable time remaining (e.g., "2 days", "5 hours")
 */
export function getTimeRemaining(timestamp: number): string {
  const now = Math.floor(Date.now() / 1000);
  const diff = timestamp - now;

  if (diff <= 0) {
    return 'Expired';
  }

  const minutes = Math.floor(diff / 60);
  const hours = Math.floor(minutes / 60);
  const days = Math.floor(hours / 24);

  if (days > 0) {
    return `${days} day${days > 1 ? 's' : ''}`;
  } else if (hours > 0) {
    return `${hours} hour${hours > 1 ? 's' : ''}`;
  } else if (minutes > 0) {
    return `${minutes} minute${minutes > 1 ? 's' : ''}`;
  } else {
    return 'Less than a minute';
  }
}
