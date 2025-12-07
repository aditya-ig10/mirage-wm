# Mirage-WM Wayland Compositor - Setup Guide

## Quick Start

### 1. Build the Compositor

```bash
cd /home/aditya/Projects/mirage-wm
cargo build --release
```

### 2. Install Session Files (Requires sudo)

```bash
# Copy session launcher script
sudo cp /home/aditya/Projects/mirage-wm/mirage-wm-session.sh /usr/local/bin/
sudo chmod +x /usr/local/bin/mirage-wm-session.sh

# Copy desktop session file for SDDM
sudo cp /home/aditya/Projects/mirage-wm/mirage-wm.desktop /usr/share/wayland-sessions/
```

### 3. Boot into Mirage-WM

1. At the SDDM login screen, click the **Session** button (bottom left)
2. Select **"Mirage-WM"** from the list
3. Enter your credentials and login
4. Mirage-WM will start as your Wayland compositor

---

## What You're Getting

### ✅ Implemented Features

- **Window Manager**: Tiling layout (Master-Stack algorithm)
- **Window Decorations**: macOS-like title bars with color buttons
  - Red close button (🔴)
  - Yellow minimize button (🟡)
  - Green maximize button (🟢)
- **Dock**: Application launcher at bottom of screen
- **Launchpad**: Full app launcher with grid view
- **Input Handling**: Pointer/keyboard routing to applications
- **Wayland Support**: Full wl_compositor, xdg_shell protocol support
- **Client Support**: Accepts real Wayland clients (Kitty, etc.)

### ⚠️ Known Limitations

See `RENDERING_BUGS.md` for detailed technical issues.

**Major Issues:**
1. **Client Window Content**: May not render properly due to nested Wayland limitations
2. **Coordinate Types**: Logical vs Physical mismatch on dock rendering
3. **Text Rendering**: Window titles don't display text (no font rendering yet)
4. **Button Actions**: Decoration buttons (close/minimize/maximize) not functional yet
5. **Dock Icons**: App icons in dock don't render individually yet

---

## System Requirements

- **OS**: Linux with Wayland support
- **Display Server**: X11 or Wayland (for running within existing session)
- **GPU**: OpenGL 3.1+ capable
- **Rust**: 1.70+ (for building)

### Dependencies

```bash
# Core dependencies
sudo pacman -S wayland wayland-protocols libxkbcommon mesa
# OR on Ubuntu/Debian:
sudo apt install libwayland-dev wayland-protocols libxkbcommon-dev libgl1-mesa-dev
```

---

## Running Standalone (Without SDDM)

### Nested Mode (inside existing Wayland session):

```bash
cd /home/aditya/Projects/mirage-wm
cargo run --release
```

This starts the compositor nested in your current Wayland session.

### Direct from TTY:

```bash
cd /home/aditya/Projects/mirage-wm
cargo run --release -- --wayland
```

**Note**: This may not work unless your system has DRM/KMS support.

---

## Configuration

### Environment Variables

```bash
# Set before running compositor:
export WAYLAND_DISPLAY=mirage-0
export QT_QPA_PLATFORM=wayland
export GDK_BACKEND=wayland
```

### Layout Configuration

Edit `src/layout.rs` to modify:
- Window tiling algorithm
- Master/stack split ratio
- Gaps between windows

### Keyboard Shortcuts

Currently none implemented. The following can be added:

```
Super+Q           - Close focused window
Super+M           - Maximize focused window
Super+N           - Minimize focused window
Super+Tab         - Cycle windows
Super+[1-9]       - Jump to workspace
Super+Shift+[1-9] - Move window to workspace
```

---

## Troubleshooting

### Compositor Won't Start

**Error**: "Failed to initialize Winit backend"
```bash
# Check graphics drivers
glxinfo | grep "OpenGL version"

# Ensure GPU supports OpenGL 3.1+
# Update drivers if needed
```

**Error**: "Failed to create Wayland socket"
```bash
# Check if port is already in use
lsof -i :wayland-0
# Kill any existing processes
pkill -f mirage-wm
```

### Black Screen / No Rendering

1. Check if compositor is running:
   ```bash
   ps aux | grep mirage-wm
   ```

2. Check logs:
   ```bash
   cargo run 2>&1 | grep -i error
   ```

3. Try running with verbose logging:
   ```bash
   RUST_LOG=debug cargo run 2>&1 | tee /tmp/mirage.log
   ```

### Applications Won't Launch

1. Verify Wayland support in application:
   ```bash
   # Launch with explicit backend
   QT_QPA_PLATFORM=wayland kitty
   ```

2. Check if WAYLAND_DISPLAY is set:
   ```bash
   echo $WAYLAND_DISPLAY  # Should show "mirage-0"
   ```

3. Try launching directly:
   ```bash
   # In compositor window, open terminal with F1 (if implemented)
   # Or from another window:
   WAYLAND_DISPLAY=mirage-0 kitty
   ```

### Input Not Working

1. Keyboard not responding:
   ```bash
   # Check if keyboard device initialized
   cargo run 2>&1 | grep -i keyboard
   ```

2. Mouse not moving:
   ```bash
   # Check pointer events
   cargo run 2>&1 | grep -i pointer
   ```

---

## Development & Testing

### Building in Debug Mode

```bash
cd /home/aditya/Projects/mirage-wm
cargo build
./target/debug/mirage-wm
```

### Building Release Version

```bash
cargo build --release
./target/release/mirage-wm
```

### Running Tests

```bash
cargo test
```

### Code Organization

```
src/
├── main.rs              # Entry point
├── backend/
│   └── winit.rs         # Wayland backend & rendering
├── state/
│   └── mod.rs           # Compositor state management
├── decorations.rs       # Window decorations (title bars)
├── dock.rs              # Application dock launcher
├── launchpad.rs         # Full app launcher
├── layout.rs            # Window tiling algorithm
├── wallpaper.rs         # Wallpaper manager
└── drawing.rs           # Drawing utilities
```

---

## Contributing

### Next Steps to Complete

**Priority 1 (Critical Functionality):**
1. Fix Logical/Physical coordinate mismatch in dock rendering
2. Implement window decoration button click detection
3. Add window close/minimize/maximize actions
4. Implement dock app launching via click

**Priority 2 (Visual Polish):**
1. Add text rendering for window titles (using rusttype/ab_glyph)
2. Render individual dock app icons
3. Implement launchpad grid rendering
4. Add wallpaper background rendering

**Priority 3 (Quality):**
1. Implement proper error handling and recovery
2. Add vsync support for smooth rendering
3. Optimize damage tracking
4. Add configuration file support

---

## Session Switching

To switch back to your previous desktop:

**From SDDM:**
1. Click **Session** button before entering password
2. Select your previous environment (GNOME, KDE, etc.)

**From within Mirage-WM:**
1. Right-click on desktop (to be implemented)
2. Select "Switch Session"
3. Or just close Mirage-WM and restart system

---

## Backing Up Current Setup

Before switching to Mirage-WM, backup your current session:

```bash
# Backup current session preference
cat ~/.config/sddm/state.conf > ~/.config/sddm/state.conf.backup

# If you need to restore:
cp ~/.config/sddm/state.conf.backup ~/.config/sddm/state.conf
```

---

## Performance Tuning

### GPU Acceleration

Ensure GPU acceleration is enabled:

```bash
# Check if using GPU
glxinfo | grep "direct rendering: Yes"

# Set vsync (varies by driver)
export MESA_VER SYNC=0  # 0 = off, 1 = on
```

### CPU Usage

If compositor uses high CPU:

1. Reduce rendering frame rate (modify poll interval in winit.rs)
2. Enable damage tracking optimizations
3. Check for infinite loops in event handler

```bash
# Monitor CPU usage while running
top -p $(pgrep mirage-wm)
```

---

## Uninstalling

To remove Mirage-WM from your system:

```bash
# Remove installed files
sudo rm /usr/local/bin/mirage-wm-session.sh
sudo rm /usr/share/wayland-sessions/mirage-wm.desktop

# Keep source code (optional)
# rm -rf /home/aditya/Projects/mirage-wm

# Switch back to another session in SDDM
```

---

## Getting Help

### Useful Resources

- [Smithay Documentation](https://smithay.github.io/smithay/)
- [Wayland Protocol](https://wayland.freedesktop.org/)
- [Mirage-WM GitHub Issues](https://github.com/aditya-ig10/mirage-wm)

### Debugging Tips

```bash
# Run with verbose logging
RUST_LOG=debug ./target/debug/mirage-wm 2>&1 | tee /tmp/mirage-debug.log

# Monitor specific events
./target/debug/mirage-wm 2>&1 | grep -i "pointer\|keyboard\|surface"

# Check for memory leaks
valgrind --leak-check=full ./target/debug/mirage-wm
```

---

**Enjoy using Mirage-WM!** 🚀

For issues or questions, check RENDERING_BUGS.md for known issues and workarounds.
