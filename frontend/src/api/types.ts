/**
 * Request payload for creating a shortened URL
 */
export interface ShortenRequest {
  url: string;
  code?: string;
  ttl?: string;
}

/**
 * Response from the shorten API endpoint
 */
export interface ShortenResponse {
  short_url: string;
  expires_at: number;
  code: string;
  original_url: string;
}

/**
 * Error response from the API
 */
export interface ApiErrorResponse {
  error: string;
}

/**
 * Validation error details
 */
export interface ValidationError {
  field: 'url' | 'code' | 'ttl';
  message: string;
}
