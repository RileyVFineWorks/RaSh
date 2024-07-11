#!/bin/bash

export MY_SHELL_CONFIG_DIR="$HOME/.rash"

if [ ! -f "$HOME/.cargo/bin/rash" ]; then
    echo "My Shell binary not found. Please install it first."
    exit 1
fi

exec "$HOME/.cargo/bin/rash" "$@"