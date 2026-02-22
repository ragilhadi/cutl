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

// ── Analytics ─────────────────────────────────────────────────────

/** A count grouped by a string value (country or referrer) */
export interface CountStat {
  value: string | null;
  count: number;
}

/** Daily visit count */
export interface DailyStat {
  /** Date in "YYYY-MM-DD" format */
  date: string;
  count: number;
}

/** A single visit record */
export interface VisitRow {
  visited_at: number;
  ip: string | null;
  country: string | null;
  city: string | null;
  user_agent: string | null;
  referer: string | null;
}

/** Response from GET /analytics/{code} */
export interface AnalyticsResponse {
  code: string;
  original_url: string;
  created_at: number;
  expires_at: number;
  total_visits: number;
  countries: CountStat[];
  referers: CountStat[];
  daily: DailyStat[];
  recent_visits: VisitRow[];
}
