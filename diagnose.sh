#!/bin/bash

# Mirage-WM Diagnostic Script
# Use this to debug issues with the compositor

echo "=== Mirage-WM Diagnostic Report ==="
echo "Generated: $(date)"
echo ""

echo "=== System Information ==="
echo "Username: $(whoami)"
echo "UID: $(id -u)"
echo "Hostname: $(hostname)"
echo "Kernel: $(uname -r)"
echo "Desktop: $XDG_CURRENT_DESKTOP"
echo "Session: $XDG_SESSION_TYPE"
echo ""

echo "=== Environment Variables ==="
echo "DISPLAY: ${DISPLAY:-not set}"
echo "WAYLAND_DISPLAY: ${WAYLAND_DISPLAY:-not set}"
echo "XDG_RUNTIME_DIR: ${XDG_RUNTIME_DIR:-not set}"
echo "HOME: $HOME"
echo ""

echo "=== Graphics Information ==="
echo "GPU Vendor:"
lspci | grep -i vga || echo "  (Not found)"
echo ""
echo "OpenGL Support:"
glxinfo 2>/dev/null | grep "OpenGL version" || echo "  glxinfo not available"
echo ""

echo "=== Wayland Support ==="
echo "Available Wayland sessions:"
ls -1 /usr/share/wayland-sessions/ | grep -i mirage || echo "  No Mirage sessions found"
echo ""

echo "=== Binary Check ==="
if [ -f /home/aditya/Projects/mirage-wm/target/debug/mirage-wm ]; then
    echo "✓ Debug binary found"
    ls -lh /home/aditya/Projects/mirage-wm/target/debug/mirage-wm
else
    echo "✗ Debug binary NOT found"
fi

if [ -f /home/aditya/Projects/mirage-wm/target/release/mirage-wm ]; then
    echo "✓ Release binary found"
    ls -lh /home/aditya/Projects/mirage-wm/target/release/mirage-wm
else
    echo "✗ Release binary NOT found"
fi
echo ""

echo "=== Session Script Check ==="
if [ -f /home/aditya/Projects/mirage-wm/mirage-wm-session.sh ]; then
    echo "✓ Session script found"
    ls -lh /home/aditya/Projects/mirage-wm/mirage-wm-session.sh
    echo "  Executable: $(test -x /home/aditya/Projects/mirage-wm/mirage-wm-session.sh && echo 'Yes' || echo 'No')"
else
    echo "✗ Session script NOT found"
fi
echo ""

echo "=== Desktop Entry Check ==="
if [ -f /usr/share/wayland-sessions/mirage-wm.desktop ]; then
    echo "✓ Desktop entry found at /usr/share/wayland-sessions/mirage-wm.desktop"
    cat /usr/share/wayland-sessions/mirage-wm.desktop
else
    echo "✗ Desktop entry NOT found"
fi
echo ""

echo "=== Log Files ==="
if [ -f ~/.local/share/mirage-wm-session.log ]; then
    echo "Recent session log:"
    tail -20 ~/.local/share/mirage-wm-session.log
else
    echo "No session log file found yet"
fi
echo ""

echo "=== Network/Sockets ==="
echo "Wayland sockets:"
ls -la /tmp/wayland-* /run/user/$(id -u)/wayland-* 2>/dev/null | head -5 || echo "  None found"
echo ""

echo "=== Recommendations ==="
echo ""
echo "If you see a black screen:"
echo "  1. Check if compositor is running: ps aux | grep mirage-wm"
echo "  2. Check logs: tail -f ~/.local/share/mirage-wm-session.log"
echo "  3. Run in debug mode: RUST_LOG=debug /home/aditya/Projects/mirage-wm/target/debug/mirage-wm"
echo "  4. Check GPU: glxinfo | grep OpenGL"
echo "  5. Verify Wayland: echo \$WAYLAND_DISPLAY"
echo ""

echo "=== To Test Compositor ==="
echo ""
echo "Option 1: Test in current session (nested)"
echo "  $ /home/aditya/Projects/mirage-wm/target/debug/mirage-wm"
echo ""
echo "Option 2: Boot from SDDM"
echo "  1. At login screen, click 'Session'"
echo "  2. Select 'Mirage-WM'"
echo "  3. Enter credentials"
echo "  4. Check logs: tail ~/.local/share/mirage-wm-session.log"
echo ""

echo "=== Debugging Tips ==="
echo ""
echo "Enable verbose logging:"
echo "  export RUST_LOG=debug"
echo "  /home/aditya/Projects/mirage-wm/target/debug/mirage-wm"
echo ""
echo "Check process status:"
echo "  ps aux | grep mirage-wm"
echo ""
echo "Monitor system logs:"
echo "  journalctl -f -u mirage-wm"
echo ""
