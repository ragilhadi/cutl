import { SHORTEN_ENDPOINT } from '../config';
import type { ShortenRequest, ShortenResponse } from './types';

/**
 * API error class for handling failed requests
 */
export class ApiError extends Error {
  statusCode?: number;
  response?: unknown;

  constructor(
    message: string,
    statusCode?: number,
    response?: unknown
  ) {
    super(message);
    this.name = 'ApiError';
    this.statusCode = statusCode;
    this.response = response;
  }
}

/**
 * Creates a shortened URL using the cutl API
 * @param request - The shorten request payload
 * @returns Promise with the shorten response containing short_url and expires_at
 * @throws ApiError if the request fails
 */
export async function shortenUrl(request: ShortenRequest): Promise<ShortenResponse> {
  try {
    const response = await fetch(SHORTEN_ENDPOINT, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(request),
    });

    const data = await response.json();

    if (!response.ok) {
      const errorMessage = typeof data === 'object' && data !== null && 'error' in data
        ? (data as { error: string }).error
        : 'Failed to create short link';
      throw new ApiError(errorMessage, response.status, data);
    }

    return data as ShortenResponse;
  } catch (error) {
    if (error instanceof ApiError) {
      throw error;
    }

    // Network errors or other issues
    throw new ApiError(
      error instanceof Error ? error.message : 'Network error occurred'
    );
  }
}
