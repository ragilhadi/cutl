# cutl - Self-Hosted CLI-First URL Shortener

A simple, self-hosted URL shortener with a Rust CLI client and HTTP API server.

## Features

- **Shorten URLs** - Convert long URLs into short links
- **Custom Codes** - Choose your own short code or let the server generate one
- **Expiration (TTL)** - Set how long links should last (5 minutes to 30 days)
- **HTTP API** - Simple REST API for integration
- **CLI Tool** - Easy-to-use command-line interface
- **SQLite** - Lightweight database with no external dependencies
- **Docker Support** - Multi-stage Dockerfile for easy deployment
- **Automated Releases** - Version file-based CI/CD workflow
- **Multi-Platform** - Binaries for Linux, macOS, and Windows
- **Continuous Testing** - Automated unit tests and linting

## Project Structure

```
cutl/
├── server/              # HTTP API server (axum + SQLite)
│   ├── src/
│   │   ├── main.rs      # Entry point
│   │   ├── config.rs    # Configuration management
│   │   ├── models.rs    # Data models
│   │   ├── database.rs  # Database operations
│   │   ├── handlers.rs  # HTTP handlers
│   │   └── utils.rs     # Utilities (validation, code generation)
│   ├── Dockerfile       # Multi-stage Docker build
│   └── Cargo.toml
├── cli/                 # CLI client tool
│   ├── src/
│   │   ├── main.rs      # Entry point
│   │   ├── config.rs    # Configuration
│   │   ├── client.rs    # API client
│   │   ├── output.rs    # Output formatting
│   │   └── validation.rs # Input validation
│   └── Cargo.toml
├── docker-compose.yml   # Docker Compose configuration
├── .github/
│   └── workflows/
│       ├── release.yml  # Automated release workflow
│       └── test.yml     # CI/CD test workflow
├── version/
│   └── version          # Version file for releases
├── Makefile            # Convenience commands
├── schema.sql          # Database schema reference
├── Cargo.toml          # Workspace configuration
└── README.md           # This file
```

## Installation

### Install from GitHub Release (Easiest)

#### Linux / macOS / Git Bash

Download and install the latest release with a single command:

```bash
curl -fsSL https://raw.githubusercontent.com/ragilhadi/cutl/master/install-from-release.sh | bash
```

Or download and inspect the script first:

```bash
curl -fsSL https://raw.githubusercontent.com/ragilhadi/cutl/master/install-from-release.sh -o install.sh
chmod +x install.sh
./install.sh
```

#### Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/ragilhadi/cutl/master/install-from-release.ps1 | iex
```

Or download and inspect the script first:

```powershell
Invoke-WebRequest -Uri https://raw.githubusercontent.com/ragilhadi/cutl/master/install-from-release.ps1 -OutFile install.ps1
.\install.ps1
```

The installer will:
- Detect your OS and architecture automatically
- Download the appropriate binary from the latest GitHub release
- Install to `~/.local/bin/cutl` on Linux/macOS or `%LOCALAPPDATA%\cutl\bin` on Windows
- Make the binary executable

**Supported platforms:**
- Linux (x86_64, aarch64)
- macOS (x86_64, aarch64/Apple Silicon)
- Windows (x86_64)

### Install from Source

If you prefer to build from source or need a custom build:

```bash
git clone https://github.com/ragilhadi/cutl.git
cd cutl
./install.sh
```

This will build the CLI locally and install it to `~/.local/bin`.

## Quick Start

### Prerequisites

- **For GitHub Release:** None! Just download and run
- **For Docker:** Docker and Docker Compose
- **For Source Build:** Rust 1.83 or later

### Option 1: Docker Deployment (Recommended)

Using `docker-compose`:

```bash
# Build and start the container
docker-compose up -d

# View logs
docker-compose logs -f

# Stop the container
docker-compose down
```

Or using the Makefile:

```bash
make build   # Build the Docker image
make run     # Start the container
make logs    # View logs
make stop    # Stop the container
```

### Option 2: Native Build

Build both the server and CLI:

```bash
cargo build --release
```

The compiled binaries will be at:
- `target/release/cutl-server`
- `target/release/cutl`

### Running the Server

**Using Docker:**

Edit `docker-compose.yml` to configure your environment variables, then:

```bash
docker-compose up -d
```

**Native:**

1. **Basic usage (default settings):**

```bash
./target/release/cutl-server
```

This will:
- Use `sqlite:cutl.db` as the database
- Listen on `0.0.0.0:3000`
- Use `https://cutl.my.id` as the base URL

2. **With environment variables:**

```bash
export DATABASE_URL="sqlite:/path/to/database.db"
export BASE_URL="https://your-domain.com"
export BIND_ADDRESS="0.0.0.0:8080"
export AUTH_TOKEN="your-secret-token"  # Optional

./target/release/cutl-server
```

3. **Using a `.env` file:**

Copy the example file:
```bash
cp server/.env.example server/.env
```

Edit `server/.env`:
```env
DATABASE_URL=sqlite:cutl.db
BASE_URL=https://cutl.my.id
BIND_ADDRESS=0.0.0.0:3000
AUTH_TOKEN=optional-secret-token
```

Then run:
```bash
./target/release/cutl-server
```

### Running the CLI

1. **Basic usage:**

```bash
./target/release/cutl https://example.com
```

Output:

```
✓ Short URL created

  Short URL: https://cutl.my.id/abc123
  Code:      abc123
  Expires:   2026-02-13 12:00:00 +00:00
```

2. **With custom TTL:**

```bash
./target/release/cutl https://example.com --ttl 3d
```

3. **With custom code:**

```bash
./target/release/cutl https://example.com --code mylink
```

4. **With both custom code and TTL:**

```bash
./target/release/cutl https://example.com --code docs --ttl 7d
```

5. **Using a custom server:**

```bash
export CUTL_SERVER="https://your-cutl-instance.com"
./target/release/cutl https://example.com
```

Or:

```bash
./target/release/cutl https://example.com --server https://your-cutl-instance.com
```

6. **With authentication:**

```bash
export CUTL_TOKEN="your-secret-token"
./target/release/cutl https://example.com
```

## API Documentation

### POST /shorten

Creates a new short link.

**Request Headers (optional):**
```
Authorization: Bearer <TOKEN>
```

**Request Body:**
```json
{
  "url": "https://example.com",
  "code": "optional_custom_code",
  "ttl": "3d"
}
```

**Response (200 OK):**
```json
{
  "code": "abc123",
  "short_url": "https://cutl.my.id/abc123",
  "expires_at": 1760000000
}
```

**Error Responses:**

- `400 Bad Request` - Invalid URL, code, or TTL
- `401 Unauthorized` - Invalid or missing auth token
- `409 Conflict` - Code already exists
- `500 Internal Server Error` - Server error

### GET /{code}

Redirects to the original URL.

**Response:**
- `302 Found` - Redirects to `original_url`
- `404 Not Found` - Link doesn't exist or has expired

## Configuration

### Server Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | SQLite database path | `sqlite:cutl.db` |
| `BASE_URL` | Base URL for short links | `https://cutl.my.id` |
| `BIND_ADDRESS` | Address to bind to | `0.0.0.0:3000` |
| `AUTH_TOKEN` | Optional bearer token for API auth | (none) |
| `RUST_LOG` | Log level (info/debug/trace) | (none) |

### CLI Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|-------|
| `CUTL_SERVER` | Server API URL | `https://cutl.my.id` |
| `CUTL_TOKEN` | Optional auth token | (none) |

**Note:** The CLI now defaults to `https://cutl.my.id` as the server. You can override this with:
- `--server` flag: `cutl https://example.com --server http://localhost:3000`
- `CUTL_SERVER` environment variable: `export CUTL_SERVER="http://localhost:3000"`

## Short Code Rules

- **Length:** 1-32 characters
- **Allowed characters:** Letters (a-z, A-Z), numbers (0-9), hyphens (-), underscores (_)
- **Pattern:** `^[a-zA-Z0-9_-]{1,32}$`

If no code is provided, the server generates a random base62 code (6-8 characters).

## TTL (Time-To-Live) Format

TTL specifies how long a link remains valid.

| Format | Description | Example |
|--------|-------------|---------|
| `5m` | Minutes | `5m` = 5 minutes |
| `1h` | Hours | `1h` = 1 hour |
| `3d` | Days | `3d` = 3 days |
| `30d` | Days | `30d` = 30 days |

**Limits:**
- Minimum: 5 minutes (300 seconds)
- Maximum: 30 days (2,592,000 seconds)
- Default: 7 days

## Database Schema

```sql
CREATE TABLE links (
    code TEXT PRIMARY KEY,
    original_url TEXT NOT NULL,
    expires_at INTEGER NOT NULL,
    created_at INTEGER NOT NULL
);

CREATE INDEX idx_links_expires_at ON links(expires_at);
```

## Security

### URL Validation

- Must start with `http://` or `https://`
- Cannot point to `localhost` or `127.0.0.1`
- URL format is validated before storage

### Authentication (Optional)

To enable API authentication, set the `AUTH_TOKEN` environment variable on the server.

**Docker:**
```yaml
environment:
  - AUTH_TOKEN=your-secret-token
```

**Native:**
```bash
export AUTH_TOKEN="your-secret-token"
```

**CLI:**
```bash
export CUTL_TOKEN="your-secret-token"
```

The CLI will automatically include the `Authorization: Bearer <TOKEN>` header.

## Deployment

### Docker Deployment

The multi-stage Dockerfile creates a minimal production image:

1. **Build stage:** Compiles the Rust application
2. **Runtime stage:** Creates a minimal Debian image with just the binary

**Features:**
- Non-root user (`cutl`)
- Persistent volume for SQLite database
- Health checks
- Small image size
- Automatic permission handling via entrypoint script

**Important:** The Docker setup includes fixes for SQLite database permissions:
- Container runs as root initially
- Entrypoint script creates `/data` directory with proper ownership
- Switches to `cutl` user before running the server
- Database file is created with correct permissions automatically

### Using systemd (Linux)

Create `/etc/systemd/system/cutl.service`:

```ini
[Unit]
Description=cutl URL Shortener
After=network.target

[Service]
Type=simple
User=cutl
WorkingDirectory=/opt/cutl
Environment="DATABASE_URL=sqlite:/opt/cutl/cutl.db"
Environment="BASE_URL=https://cutl.my.id"
Environment="BIND_ADDRESS=0.0.0.0:3000"
Environment="AUTH_TOKEN=your-secret-token"
ExecStart=/opt/cutl/cutl-server
Restart=always

[Install]
WantedBy=multi-user.target
```

Then:

```bash
sudo systemctl daemon-reload
sudo systemctl enable cutl
sudo systemctl start cutl
```

### Using with a Reverse Proxy (nginx)

```nginx
server {
    listen 80;
    server_name cutl.my.id;

    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

## Examples

### Using curl

```bash
# Create a short link
curl -X POST https://cutl.my.id/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com/very/long/path", "ttl": "7d"}'
```

### Using with authentication

```bash
curl -X POST https://cutl.my.id/shorten \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer your-token" \
  -d '{"url": "https://example.com"}'
```

## Development

### Running tests

```bash
cargo test --workspace
```

### Checking code

```bash
cargo clippy --workspace
```

### Formatting code

```bash
cargo fmt --all
```

### CI/CD Workflows

This project uses GitHub Actions for automated testing and releases:

#### Test Workflow (`.github/workflows/test.yml`)
Runs on every push and pull request to `master`:
- ✅ Code formatting check (`cargo fmt --check`)
- ✅ Linting with Clippy (`cargo clippy`)
- ✅ Unit tests for CLI and Server
- ✅ Build verification
- ✅ Cargo caching for faster builds

#### Release Workflow (`.github/workflows/release.yml`)
Automatically creates releases when `version/version` file changes:
- ✅ Reads version from `version/version` file
- ✅ Checks for duplicate tags
- ✅ Creates git tag automatically
- ✅ Builds binaries for all platforms (Linux, macOS, Windows)
- ✅ Publishes GitHub release with binaries

### Creating a Release

Simply update the version file and push to `master`:

```bash
echo "v1.0.1" > version/version
git add version/version
git commit -m "Release v1.0.1"
git push origin master
```

GitHub Actions will automatically:
1. Create a git tag
2. Build binaries for all platforms
3. Create a GitHub release
4. Upload all binaries

See `RELEASE.md` and `VERSION_WORKFLOW.md` for detailed release documentation.

### Building for different targets

```bash
# For Linux x86_64
cargo build --release --target x86_64-unknown-linux-gnu

# For macOS ARM64 (Apple Silicon)
cargo build --release --target aarch64-apple-darwin

# For Windows
cargo build --release --target x86_64-pc-windows-gnu
```

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
