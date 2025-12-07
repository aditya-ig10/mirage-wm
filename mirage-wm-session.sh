#!/bin/bash

# Mirage-WM Wayland Compositor Session Launcher
# This starts the Mirage-WM compositor directly without Mutter

export XDG_CURRENT_DESKTOP=MirageWM
export XDG_SESSION_TYPE=wayland

# Set Wayland environment variables for Qt/GTK apps
export QT_QPA_PLATFORM=wayland
export GDK_BACKEND=wayland
export CLUTTER_BACKEND=wayland

# Enable debug logging
export RUST_LOG=info

# Ensure XDG directories exist
mkdir -p ~/.config ~/.local/share ~/.cache

# Clean up any stale sockets
rm -f /tmp/mirage-* /run/user/$(id -u)/mirage-* 2>/dev/null || true

# Log file for debugging
LOG_FILE="${HOME}/.local/share/mirage-wm-session.log"
mkdir -p "$(dirname "$LOG_FILE")"

echo "[$(date)] Starting Mirage-WM Wayland Compositor..." | tee -a "$LOG_FILE"
echo "[$(date)] User: $(whoami), UID: $(id -u)" | tee -a "$LOG_FILE"
echo "[$(date)] XDG_RUNTIME_DIR: $XDG_RUNTIME_DIR" | tee -a "$LOG_FILE"

# Start the Mirage-WM compositor
# The compositor handles its own event loop and stays in foreground
if [ -f /home/aditya/Projects/mirage-wm/target/debug/mirage-wm ]; then
    echo "[$(date)] Starting from: /home/aditya/Projects/mirage-wm/target/debug/mirage-wm" | tee -a "$LOG_FILE"
    exec /home/aditya/Projects/mirage-wm/target/debug/mirage-wm 2>&1 | tee -a "$LOG_FILE"
elif [ -f /home/aditya/Projects/mirage-wm/target/release/mirage-wm ]; then
    echo "[$(date)] Starting from: /home/aditya/Projects/mirage-wm/target/release/mirage-wm" | tee -a "$LOG_FILE"
    exec /home/aditya/Projects/mirage-wm/target/release/mirage-wm 2>&1 | tee -a "$LOG_FILE"
else
    echo "[$(date)] ERROR: mirage-wm binary not found!" | tee -a "$LOG_FILE"
    sleep 5
    exit 1
fi
