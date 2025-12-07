# Mirage Compositor - Wayland Preview Guide

## Quick Start

### 1. Build the Compositor
```bash
cd /home/aditya/Projects/mirage-wm
cargo build --release
```

### 2. Run the Compositor (as a Nested Window)
The compositor runs as a Wayland client inside your GNOME/KDE session:

```bash
# Run with logging to see events
RUST_LOG=info ./target/debug/mirage-wm

# Or with more detailed logging
RUST_LOG=debug ./target/debug/mirage-wm
```

### 3. Test with Wayland Clients
Once the compositor window appears, you can open Wayland clients inside it:

```bash
# Open a terminal in the nested compositor
WAYLAND_DISPLAY=wayland-1 kitty

# Or use any Wayland-compatible client
WAYLAND_DISPLAY=wayland-1 wl-clip-persistor

# List running Wayland clients
WAYLAND_DISPLAY=wayland-1 weston-info
```

## What to Look For

### Current Features (Phase 1-3)
✅ **Compositor Window**: Opens a native window in your Wayland session
✅ **Event Loop**: Processes Wayland client events
✅ **Input Handling**: Responds to mouse and keyboard input (logs in console)
✅ **Window Tracking**: Detects new windows from clients
✅ **Focus Management**: Clicks on windows update focus (watch console logs)
✅ **Tiling Layout**: Automatically positions windows (not yet visible)

### What's Missing (Will be in Phase 4+)
❌ **Visible Rendering**: Windows don't appear on screen yet
❌ **Background**: Window is blank (no color buffer)
❌ **Window Borders**: No visual window decorations
❌ **Cursor**: Pointer not rendered
❌ **Text Rendering**: No window titles visible

## Console Logging

When running with `RUST_LOG=info`, you'll see:
```
[INFO] Initialization completed, starting the main loop.
[INFO] Mirage Compositor running at 1280x800
[INFO] New XDG toplevel window!           <- Client opened a window
[INFO] Pointer absolute position (640.5, 400.2)  <- Mouse movement
[INFO] Clicked on window 0               <- Click detected
[INFO] Window focus changed to 0          <- Focus set
```

When running with `RUST_LOG=trace`, you'll also see:
```
[TRACE] Window[0]: Pos(0,0) Size(640x800) Color(Blue(0.3, 0.6, 1.0))  <- Window render op
[TRACE] Cursor: Pos(640.5, 400.2) Radius(8)                          <- Cursor render op
```

## Testing in Terminal

```bash
# Terminal 1: Run the compositor with logging
RUST_LOG=info ./target/debug/mirage-wm

# Terminal 2: Find the Wayland socket
ls -la /run/user/$UID/ | grep wayland

# Terminal 3: Open a client (if available)
WAYLAND_DISPLAY=wayland-1 weston-info
```

## Architecture Overview (Phase 1-4)

```
src/
├── main.rs              # Entry point
├── state/mod.rs         # State management + protocol handlers
├── backend/
│   └── winit.rs         # Event loop + rendering backend
├── layout.rs            # Tiling window layout
└── shell/               # XDG shell handlers (existing)
```

## Architecture Overview (Phase 1-4)

### Phase 1: Event Loop ✅
- Winit backend provides windowed Wayland compositor
- Calloop event loop processes Wayland client events
- XDG Shell protocol handlers track window creation

### Phase 2: Input Handling ✅
- PointerMotionAbsolute events track cursor position
- PointerButton events detect clicks
- Keyboard events logged (not yet routed to windows)

### Phase 3: Window Management ✅
- Tiling layout automatically positions windows
- Click-to-focus sets active window
- Windows tracked in Vec with focus tracking

### Phase 4: Rendering (In Progress) 🔄
- Framebuffer binding and frame submission working
- Render frame function prepared with window data
- Drawing module ready for implementing actual rendering
- **TODO**: Implement actual color buffer clearing and rendering

## File Structure
