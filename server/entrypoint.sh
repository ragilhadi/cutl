#!/bin/sh
set -e

# Ensure /data directory exists with proper permissions
# This runs as root before switching to the cutl user
if [ "$(id -u)" = "0" ]; then
    mkdir -p /data
    # Create an empty database file if it doesn't exist
    touch /data/cutl.db
    # Set ownership to cutl user
    chown -R cutl:cutl /data
    # Make directory and file writable
    chmod 775 /data
    chmod 664 /data/cutl.db
fi

# Run the server as the cutl user
exec gosu cutl "$@"
