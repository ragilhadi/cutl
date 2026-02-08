import type { ValidationError } from '../api/types';

/**
 * Validates a URL string
 * @param url - The URL to validate
 * @returns Error message if invalid, null if valid
 */
export function validateUrl(url: string): string | null {
  if (!url || url.trim() === '') {
    return 'URL is required';
  }

  try {
    const urlObj = new URL(url);

    // Ensure protocol is http or https
    if (urlObj.protocol !== 'http:' && urlObj.protocol !== 'https:') {
      return 'URL must use HTTP or HTTPS protocol';
    }

    return null;
  } catch {
    return 'Please enter a valid URL (e.g., https://example.com)';
  }
}

/**
 * Validates a custom code
 * @param code - The custom code to validate
 * @returns Error message if invalid, null if valid
 */
export function validateCode(code: string): string | null {
  if (!code || code.trim() === '') {
    return null; // Optional field
  }

  const trimmedCode = code.trim();

  if (trimmedCode.length < 1) {
    return 'Custom code must be at least 1 character';
  }

  if (trimmedCode.length > 32) {
    return 'Custom code must be 32 characters or less';
  }

  // Allow alphanumeric, hyphens, and underscores
  const validPattern = /^[a-zA-Z0-9_-]+$/;
  if (!validPattern.test(trimmedCode)) {
    return 'Custom code can only contain letters, numbers, hyphens, and underscores';
  }

  return null;
}

/**
 * Validates a TTL (time to live) string
 * @param ttl - The TTL string to validate (e.g., "7d", "1h", "30m")
 * @returns Error message if invalid, null if valid
 */
export function validateTtl(ttl: string): string | null {
  if (!ttl || ttl.trim() === '') {
    return null; // Optional field
  }

  const trimmedTtl = ttl.trim();
  const ttlPattern = /^(\d+)([smhd])$/;
  const match = trimmedTtl.match(ttlPattern);

  if (!match) {
    return 'TTL must be in format: number + unit (s/m/h/d). Examples: 5m, 1h, 3d, 30d';
  }

  const value = parseInt(match[1], 10);
  const unit = match[2];

  // Convert to minutes for validation
  const unitToMinutes: Record<string, number> = {
    's': 1 / 60,
    'm': 1,
    'h': 60,
    'd': 60 * 24,
  };

  const minutes = value * unitToMinutes[unit];

  // Minimum: 5 minutes
  if (minutes < 5) {
    return 'TTL must be at least 5 minutes';
  }

  // Maximum: 30 days
  const maxMinutes = 30 * 24 * 60;
  if (minutes > maxMinutes) {
    return 'TTL must be 30 days or less';
  }

  return null;
}

/**
 * Validates all form fields
 * @param url - The URL to validate
 * @param code - The custom code to validate
 * @param ttl - The TTL to validate
 * @returns Array of validation errors (empty if all valid)
 */
export function validateForm(
  url: string,
  code: string,
  ttl: string
): ValidationError[] {
  const errors: ValidationError[] = [];

  const urlError = validateUrl(url);
  if (urlError) {
    errors.push({ field: 'url', message: urlError });
  }

  const codeError = validateCode(code);
  if (codeError) {
    errors.push({ field: 'code', message: codeError });
  }

  const ttlError = validateTtl(ttl);
  if (ttlError) {
    errors.push({ field: 'ttl', message: ttlError });
  }

  return errors;
}
