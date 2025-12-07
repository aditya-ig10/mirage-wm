#!/bin/bash

# Mirage-WM Installation Script
# This script installs Mirage-WM as a bootable desktop environment in SDDM

set -e

echo "=== Mirage-WM Installation ==="
echo ""

MIRAGE_DIR="/home/aditya/Projects/mirage-wm"
BINARY_DEBUG="$MIRAGE_DIR/target/debug/mirage-wm"
BINARY_RELEASE="$MIRAGE_DIR/target/release/mirage-wm"
SESSION_SCRIPT="$MIRAGE_DIR/mirage-wm-session.sh"
DESKTOP_ENTRY="$MIRAGE_DIR/mirage-wm.desktop"

echo "Checking files..."

if [ ! -f "$SESSION_SCRIPT" ]; then
    echo "✗ ERROR: Session script not found at $SESSION_SCRIPT"
    exit 1
else
    echo "✓ Session script found"
fi

if [ ! -f "$BINARY_DEBUG" ] && [ ! -f "$BINARY_RELEASE" ]; then
    echo "✗ ERROR: Mirage-WM binary not found. Please run 'cargo build' first"
    exit 1
else
    echo "✓ Mirage-WM binary found"
fi

if [ ! -f "$DESKTOP_ENTRY" ]; then
    echo "✗ ERROR: Desktop entry not found at $DESKTOP_ENTRY"
    exit 1
else
    echo "✓ Desktop entry found"
fi

echo ""
echo "Installing files to system..."

# Need sudo for system directories
if ! command -v sudo &> /dev/null; then
    echo "✗ ERROR: sudo not found. Please install sudo and try again."
    exit 1
fi

# Install session script to /usr/local/bin
echo "Installing session script..."
sudo cp "$SESSION_SCRIPT" /usr/local/bin/mirage-wm-session.sh
sudo chmod +x /usr/local/bin/mirage-wm-session.sh
echo "✓ Installed to /usr/local/bin/mirage-wm-session.sh"

# Install desktop entry
echo "Installing desktop entry..."
sudo cp "$DESKTOP_ENTRY" /usr/share/wayland-sessions/mirage-wm.desktop
echo "✓ Installed to /usr/share/wayland-sessions/mirage-wm.desktop"

echo ""
echo "=== Installation Complete ==="
echo ""
echo "Next steps:"
echo "  1. Log out of your current session"
echo "  2. At the SDDM login screen:"
echo "     - Look at the bottom right for 'Session' button"
echo "     - Click it and select 'Mirage-WM'"
echo "     - Enter your credentials"
echo "     - You should now boot into Mirage-WM"
echo ""
echo "If you see a black screen:"
echo "  1. Press Ctrl+Alt+F2 to switch to console"
echo "  2. Run: tail ~/.local/share/mirage-wm-session.log"
echo "  3. Check for errors"
echo "  4. Run diagnostic: /home/aditya/Projects/mirage-wm/diagnose.sh"
echo ""
echo "To verify installation:"
echo "  $ ls /usr/share/wayland-sessions/mirage-wm.desktop"
echo "  $ /usr/local/bin/mirage-wm-session.sh --version"
echo ""
