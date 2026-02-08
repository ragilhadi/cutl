# cutl Frontend

Modern, responsive web UI for the cutl URL shortener service.

## Tech Stack

- **Build Tool:** Vite 7.2
- **Language:** TypeScript 5.9
- **Framework:** Vanilla TypeScript (lightweight, no framework dependencies)
- **Styling:** CSS with design tokens

## Development

```bash
# Install dependencies
npm install

# Start development server (runs on http://localhost:3234)
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

## Project Structure

```
frontend/
├── src/
│   ├── api/
│   │   ├── client.ts        # API client with error handling
│   │   └── types.ts         # TypeScript interfaces
│   ├── components/
│   │   ├── ShortenForm.ts   # Main form component
│   │   ├── ResultCard.ts    # Display shortened URL result
│   │   └── InstallGuide.ts  # Installation documentation
│   ├── styles/
│   │   ├── main.css         # Main stylesheet
│   │   └── themes.css       # Design tokens (colors, spacing)
│   ├── utils/
│   │   ├── validation.ts    # Form validation
│   │   └── clipboard.ts     # Copy to clipboard utility
│   ├── config.ts            # API configuration
│   ├── App.ts               # Main application entry
│   └── main.ts              # Bootstrap
├── dist/                    # Production build output
├── index.html
├── vite.config.ts
└── package.json
```

## Deployment

The frontend is a **static site** that can be deployed to any hosting platform:

### Vercel

```bash
npm run build
vercel deploy dist
```

### Netlify

```bash
npm run build
# Drag and drop the dist/ folder to Netlify dashboard
```

### GitHub Pages

```bash
npm run build
# Push dist/ folder to gh-pages branch
```

### Nginx

```nginx
server {
    listen 80;
    root /usr/share/nginx/html;
    index index.html;

    location / {
        try_files $uri $uri/ /index.html;
    }

    location /assets/ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }
}
```

## API Configuration

The frontend calls the API at `https://cutl.my.id/shorten` directly.

For local development with a local API server, edit `src/config.ts`:

```typescript
export const API_BASE_URL = 'http://localhost:8080';
export const SHORTEN_ENDPOINT = `${API_BASE_URL}/api/shorten`;
```

## Features

- URL shortening with custom codes
- Configurable expiration times (TTL)
- Real-time form validation
- Copy-to-clipboard functionality
- Tabbed installation guide
- Responsive design for mobile
- Loading states and error handling
- Smooth animations and transitions
