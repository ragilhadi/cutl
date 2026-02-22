// ============================================================================
// API Configuration
// ============================================================================
// Choose ONE of the following configurations based on your deployment:

// OPTION 1: Production API (default)
// Frontend deployed to Vercel/Netlify/GitHub Pages calling production API
export const API_BASE_URL = 'https://cutl.my.id';
export const SHORTEN_ENDPOINT = `${API_BASE_URL}/shorten`;
export const ANALYTICS_ENDPOINT = (code: string) => `${API_BASE_URL}/analytics/${encodeURIComponent(code)}`;

// OPTION 2: Docker Compose (frontend + backend on same host)
// Uncomment this when using docker-compose:
// export const API_BASE_URL = ''; // Empty for same-origin
// export const SHORTEN_ENDPOINT = '/shorten';
// export const ANALYTICS_ENDPOINT = (code: string) => `/analytics/${encodeURIComponent(code)}`;

// OPTION 3: Local development with local backend
// Uncomment this for local development:
// export const API_BASE_URL = 'http://localhost:3233';
// export const SHORTEN_ENDPOINT = `${API_BASE_URL}/shorten`;
// export const ANALYTICS_ENDPOINT = (code: string) => `${API_BASE_URL}/analytics/${encodeURIComponent(code)}`;

// OPTION 4: Custom backend URL
// export const API_BASE_URL = 'https://your-custom-domain.com';
// export const SHORTEN_ENDPOINT = `${API_BASE_URL}/shorten`;
// export const ANALYTICS_ENDPOINT = (code: string) => `${API_BASE_URL}/analytics/${encodeURIComponent(code)}`;

