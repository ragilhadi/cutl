.PHONY: help build run stop logs clean rebuild shell test test-cli test-server fmt fmt-check clippy ci build-cli build-server build-all

# Default target
help:
	@echo "cutl - URL Shortener"
	@echo ""
	@echo "Docker Targets:"
	@echo "  build       - Build the Docker image"
	@echo "  run         - Run the container in detached mode"
	@echo "  stop        - Stop the running container"
	@echo "  logs        - Show container logs"
	@echo "  clean       - Stop and remove containers and volumes"
	@echo "  rebuild     - Rebuild and restart the container"
	@echo "  shell       - Open a shell in the running container"
	@echo ""
	@echo "Test Targets:"
	@echo "  test        - Run all tests"
	@echo "  test-cli    - Run CLI tests"
	@echo "  test-server - Run server tests"
	@echo ""
	@echo "CI/CD Targets:"
	@echo "  fmt         - Format all code"
	@echo "  fmt-check   - Check code formatting"
	@echo "  clippy      - Run clippy linter"
	@echo "  ci          - Run all CI checks (fmt, clippy, test)"
	@echo ""
	@echo "Build Targets:"
	@echo "  build-cli   - Build CLI binary"
	@echo "  build-server - Build server binary"
	@echo "  build-all   - Build entire workspace"

# Build the Docker image
build:
	docker-compose build

# Run the container
run:
	docker-compose up -d

# Stop the container
stop:
	docker-compose down

# Show logs
logs:
	docker-compose logs -f cutl-server

# Clean everything
clean: stop
	docker-compose down -v
	docker system prune -f

# Rebuild and restart
rebuild:
	docker-compose down
	docker-compose build --no-cache
	docker-compose up -d

# Open shell in container
shell:
	docker-compose exec cutl-server /bin/bash

# Run all tests
test: test-cli test-server

# Run CLI tests
test-cli:
	@echo "Running CLI tests..."
	cargo test --manifest-path cli/Cargo.toml

# Run server tests
test-server:
	@echo "Running server tests..."
	cargo test --manifest-path server/Cargo.toml

# Format all code
fmt:
	@echo "Formatting code..."
	cargo fmt --all

# Check code formatting
fmt-check:
	@echo "Checking code formatting..."
	cargo fmt --all -- --check

# Run clippy linter
clippy:
	@echo "Running clippy..."
	cargo clippy --workspace --all-targets --all-features -- -D warnings

# Build CLI binary
build-cli:
	@echo "Building CLI..."
	cargo build -p cutl --verbose

# Build server binary
build-server:
	@echo "Building server..."
	cargo build -p cutl-server --verbose

# Build entire workspace
build-all:
	@echo "Building workspace..."
	cargo build --workspace --verbose

# Run all CI checks (same as GitHub Actions test workflow)
ci: fmt-check clippy test build-all
	@echo "✅ All CI checks passed!"
