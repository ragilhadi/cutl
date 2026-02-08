# Docker Deployment

## Quick Start

```bash
# Start backend only
docker-compose up -d cutl-server

# Start frontend + backend
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

## Services

### Backend (cutl-server)
- **Port**: 3233
- **Container**: `cutl-server`
- **Database**: SQLite persisted in `cutl-data` volume

### Frontend (cutl-frontend)
- **Port**: 3234
- **Container**: `cutl-frontend`
- **Purpose**: Serves static web UI

**Note**: The frontend is independent and calls the API at the URL configured in `frontend/src/config.ts`.

## Frontend Deployment Options

The frontend is a **static site** that can be deployed anywhere:

### Option 1: Docker (Current Setup)
```bash
docker-compose up -d cutl-frontend
# Access at http://localhost:3234
```

### Option 2: Deploy to Vercel
```bash
cd frontend
npm run build
vercel deploy dist
```

### Option 3: Deploy to Netlify
```bash
cd frontend
npm run build
# Drag and drop dist/ folder to Netlify
```

### Option 4: GitHub Pages
```bash
cd frontend
npm run build
# Push dist/ contents to gh-pages branch
```

## API Configuration

The frontend API endpoint is configured in `frontend/src/config.ts`:

```typescript
// For production (calling https://cutl.my.id)
export const API_BASE_URL = 'https://cutl.my.id';

// For local development
// export const API_BASE_URL = 'http://localhost:3233';
```

**Important**: Edit this file BEFORE building if you want to change the API endpoint.

```bash
cd frontend
# Edit src/config.ts
npm run build
docker-compose up -d --build cutl-frontend
```

## Standalone Frontend Docker

Build and run just the frontend:

```bash
cd frontend
docker build -t cutl-frontend .
docker run -p 3234:80 cutl-frontend
```

## Building Images

```bash
# Build all
docker-compose build

# Build specific service
docker-compose build cutl-frontend
docker-compose build cutl-server

# Rebuild and start
docker-compose up -d --build
```

## Troubleshooting

### Frontend can't reach backend
Edit `frontend/src/config.ts` to point to the correct backend URL, then rebuild:
```bash
cd frontend
npm run build
docker-compose up -d --build cutl-frontend
```

### View logs
```bash
docker-compose logs -f cutl-frontend
docker-compose logs -f cutl-server
```

### Reset everything
```bash
docker-compose down -v  # Removes volumes too!
docker-compose up -d --build
```
