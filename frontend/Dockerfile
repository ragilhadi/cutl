# Build stage
FROM node:20-alpine AS builder
WORKDIR /app/frontend

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm ci

# Copy source files
COPY . .

# Build the application
RUN npm run build

# Production stage - serve with Node.js
FROM node:20-alpine

WORKDIR /app

# Install 'serve' package for serving static files
RUN npm install -g serve

# Copy built files from builder stage
COPY --from=builder /app/frontend/dist ./dist

# Expose port
EXPOSE 3234

# Serve the static files
CMD ["serve", "-s", "dist", "-l", "3234"]
