#!/bin/sh
set -e

BINARY="gdcleaner"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/gdcleaner"
CONFIG_FILE="config.toml"

echo "==> Checking dependencies..."
if ! command -v cargo > /dev/null 2>&1; then
    echo "Error: cargo not found. Install Rust from https://rustup.rs"
    exit 1
fi

echo "==> Building $BINARY (release)..."
cargo build --release

echo "==> Installing binary to $INSTALL_DIR..."
mkdir -p "$INSTALL_DIR"
cp "target/release/$BINARY" "$INSTALL_DIR/$BINARY"
chmod +x "$INSTALL_DIR/$BINARY"

echo "==> Installing config to $CONFIG_DIR..."
mkdir -p "$CONFIG_DIR"
if [ ! -f "$CONFIG_DIR/$CONFIG_FILE" ]; then
    cp "$CONFIG_FILE" "$CONFIG_DIR/$CONFIG_FILE"
    echo "    Config installed."
else
    echo "    Config already exists, skipping (not overwriting)."
fi

echo ""
echo "Done! $BINARY installed to $INSTALL_DIR/$BINARY"

if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    echo ""
    echo "Warning: $INSTALL_DIR is not in your PATH."
    echo "Add this to your shell config:"
    echo ""
    echo "  fish:  fish_add_path $INSTALL_DIR"
    echo "  bash:  export PATH=\"\$PATH:$INSTALL_DIR\""
    echo "  zsh:   export PATH=\"\$PATH:$INSTALL_DIR\""
fi
