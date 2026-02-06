.PHONY: help build run stop logs clean rebuild shell test test-cli test-server

# Default target
help:
	@echo "cutl - URL Shortener"
	@echo ""
	@echo "Targets:"
	@echo "  build     - Build the Docker image"
	@echo "  run       - Run the container in detached mode"
	@echo "  stop      - Stop the running container"
	@echo "  logs      - Show container logs"
	@echo "  clean     - Stop and remove containers and volumes"
	@echo "  rebuild   - Rebuild and restart the container"
	@echo "  shell     - Open a shell in the running container"
	@echo "  test      - Run all tests"
	@echo "  test-cli  - Run CLI tests"
	@echo "  test-server - Run server tests"

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
