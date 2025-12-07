# Black Screen Issue - Troubleshooting Guide

## Problem: Black Screen When Booting Mirage-WM from SDDM

When you select "Mirage-WM" at the SDDM login screen and log in, you get a completely black screen with no UI visible.

---

## Root Causes & Solutions

### 1. **Session Files Not Installed** ✓ FIXED

**Symptom**: Desktop entry not showing in SDDM session list

**Solution**:
```bash
# Run the installation script
/home/aditya/Projects/mirage-wm/install.sh
```

This copies the necessary files to:
- `/usr/share/wayland-sessions/mirage-wm.desktop` - Session definition
- `/usr/local/bin/mirage-wm-session.sh` - Session launcher script

**Verify**:
```bash
ls /usr/share/wayland-sessions/mirage-wm.desktop
# Should show: -rw-r--r-- 1 root root 235 ... mirage-wm.desktop
```

---

### 2. **Compositor Not Starting**

**Symptom**: Screen is black, no cursor, appears frozen

**Diagnosis**:
```bash
# Check if process is running
ps aux | grep mirage-wm

# If nothing, check the logs
tail -f ~/.local/share/mirage-wm-session.log
```

**Common Errors**:

#### Error: "Failed to initialize Winit backend"
- Graphics driver not properly installed
- OpenGL not supported
- GPU not recognized

**Fix**:
```bash
# Verify GPU and OpenGL
glxinfo | grep -i "opengl\|nvidia\|amd\|intel"

# Should show: OpenGL version 3.1 or higher

# Update drivers
sudo pacman -Syu mesa  # Arch
sudo apt update && sudo apt upgrade  # Ubuntu/Debian
```

#### Error: "Failed to create Wayland socket"
- Another compositor is running
- Socket permission issue

**Fix**:
```bash
# Kill any existing compositors
killall mirage-wm mutter kwin gnome-shell kwin_wayland

# Clean up sockets
rm -f /tmp/mirage-* /run/user/$(id -u)/mirage-*

# Try again
/home/aditya/Projects/mirage-wm/target/debug/mirage-wm
```

---

### 3. **No Visible Output (Graphics Issue)**

**Symptom**: Compositor runs but screen stays black or shows artifacts

**Cause**: The compositor is rendering but nothing is visible. This could be because:
1. No windows are open (compositor shows empty desktop)
2. OpenGL rendering not working properly
3. Wrong output device

**Solution**:

Step 1: Test with explicit rendering
```bash
# Run in debug mode with verbose output
RUST_LOG=debug /home/aditya/Projects/mirage-wm/target/debug/mirage-wm 2>&1 | tee /tmp/mirage-debug.log

# Look for lines like:
# "Mirage Compositor running at 1920x1080"
# "Rendering frame at 1920x1080"
# "No windows open - rendering test indicator"
```

Step 2: If you see "No windows open - rendering test indicator", the compositor is working but just needs an application to open.

**To test opening an application**:
- Press keys (should trigger keyboard events in logs)
- Try opening a terminal: `WAYLAND_DISPLAY=mirage-0 kitty &`

Step 3: If you don't see any rendering messages:
```bash
# Check OpenGL
glxinfo | head -10

# If it shows "client GLX vendor string: (null)", your GPU may not support the nested Wayland backend
# This is a known limitation with GNOME Wayland
```

---

### 4. **Winit Backend Not Initializing**

**Root Issue**: Winit (the window system) can't create a window because:
- Running in headless mode (no display)
- Running via remote SSH
- Display environment not set up properly

**Symptoms**:
- Immediate crash with no output
- "Failed to initialize Winit backend"
- Black screen forever

**Solutions**:

#### Option A: Check Display Environment
```bash
# When running from SDDM, these should be set:
echo "DISPLAY: $DISPLAY"
echo "WAYLAND_DISPLAY: $WAYLAND_DISPLAY"
echo "XDG_RUNTIME_DIR: $XDG_RUNTIME_DIR"

# If empty, the session environment wasn't loaded properly
```

#### Option B: Run Nested in Your Current Session
```bash
# This works better for debugging
/home/aditya/Projects/mirage-wm/target/debug/mirage-wm
```

This runs the compositor nested in your current Wayland session, which is much more reliable than booting into it from SDDM.

---

### 5. **Known Rendering Issues**

See `RENDERING_BUGS.md` for detailed technical issues:

**Major Issues**:
1. **Client Window Content Not Rendering**: Applications may show up as colored rectangles instead of actual content
2. **Text Not Showing**: Window titles don't display text
3. **Buttons Not Working**: Decoration buttons (close/minimize) aren't functional yet
4. **Logical vs Physical Coordinates**: Some UI elements may be positioned incorrectly

These are known limitations, not bugs in the startup process.

---

## Step-by-Step Debugging

If you still see a black screen after installation:

### Step 1: Verify Installation
```bash
/home/aditya/Projects/mirage-wm/diagnose.sh
```

Look for:
- ✓ Debug binary found
- ✓ Session script found  
- ✓ Desktop entry found (in /usr/share/wayland-sessions/)

### Step 2: Test in Nested Mode First
```bash
# This is much easier to debug
cd /home/aditya/Projects/mirage-wm
cargo build
./target/debug/mirage-wm

# You should see:
# - ATTENTION: default value of option vblank_mode overridden by environment
# - [timestamp] Mirage Compositor running at WIDTHxHEIGHT
# - A window should appear with your compositor
```

### Step 3: Check Logs from SDDM Boot
```bash
# After logging in and seeing black screen, switch to console:
# Ctrl+Alt+F2 (or F3, F4, etc.)

# Check the session log:
tail -f ~/.local/share/mirage-wm-session.log

# Look for error messages
grep -i "error\|failed" ~/.local/share/mirage-wm-session.log
```

### Step 4: Run Manually with Debug Output
```bash
# Switch to console: Ctrl+Alt+F2
# Kill the stuck session
pkill -f mirage-wm

# Run manually with output
RUST_LOG=debug /home/aditya/Projects/mirage-wm/target/debug/mirage-wm 2>&1

# Watch the output for errors
```

### Step 5: Check System Logs
```bash
# System journal logs
journalctl -xe --since "30 minutes ago" | grep -i mirage

# Or look for GPU-related errors
journalctl -xe | grep -i "gpu\|dri\|mesa"
```

---

## Workarounds & Alternatives

### If Black Screen Persists

**Option 1: Use Nested Mode**
```bash
# Don't boot into Mirage-WM from SDDM
# Instead, run nested in your current GNOME/KDE session:

/home/aditya/Projects/mirage-wm/target/debug/mirage-wm

# This gives you a window with the compositor inside it
# Much better for development and debugging
```

**Option 2: Boot to TTY and Start Manually**
```bash
# If SDDM itself is failing:

# 1. Edit SDDM config
sudo nano /etc/sddm.conf.d/kde_settings.conf

# 2. Change Session from "plasmawayland" to "mirage-wm"
# 3. Or don't change it, and just login to GNOME, then:

WAYLAND_DISPLAY=mirage-0 /home/aditya/Projects/mirage-wm/target/debug/mirage-wm
```

**Option 3: Use X11 Session as Fallback**
```bash
# If all Wayland options fail, use X11:
# At SDDM, click Session and choose "GNOME" or "Plasma"
# You can still test Mirage-WM later when issues are fixed
```

---

## Prevention Checklist

Before booting into Mirage-WM, verify:

- [ ] Desktop entry installed: `ls /usr/share/wayland-sessions/mirage-wm.desktop`
- [ ] Session script executable: `ls -x /usr/local/bin/mirage-wm-session.sh`
- [ ] Binary compiled: `ls /home/aditya/Projects/mirage-wm/target/debug/mirage-wm`
- [ ] GPU drivers working: `glxinfo | grep OpenGL`
- [ ] OpenGL 3.1+: Verify output shows version number
- [ ] Wayland support: `echo $WAYLAND_DISPLAY` (should not be empty)
- [ ] No stale processes: `pkill -9 mirage-wm`

---

## Getting Help

### Useful Commands for Debugging

```bash
# Full diagnostic
/home/aditya/Projects/mirage-wm/diagnose.sh

# Watch logs in real-time
tail -f ~/.local/share/mirage-wm-session.log

# Run with maximum verbosity
RUST_LOG=trace /home/aditya/Projects/mirage-wm/target/debug/mirage-wm 2>&1 | tee /tmp/mirage-trace.log

# Monitor running processes
watch -n 1 'ps aux | grep mirage'

# Check system resources
top -p $(pgrep mirage-wm)

# List open files/sockets
lsof -p $(pgrep mirage-wm)
```

### Files to Check

- `/home/aditya/Projects/mirage-wm/RENDERING_BUGS.md` - Known rendering issues
- `/home/aditya/Projects/mirage-wm/SETUP.md` - Full setup guide
- `~/.local/share/mirage-wm-session.log` - Session log from SDDM boot
- `/tmp/mirage-debug.log` - Debug log from manual run

---

## Expected Behavior

### When It's Working

1. **Login Screen**: Select Mirage-WM session
2. **Boot**: You see a solid gray/blue screen
3. **Visual Indicator**: You should see a blue rectangle in the center (test indicator)
4. **Dock**: Dark bar at the bottom of screen
5. **Cursor**: Small white square follows your mouse

### What You Won't See Yet

- Window content (apps show as colored rectangles)
- Window titles (no text rendering)
- Dock app icons (renders but icons not visible)
- Functional buttons (decoration buttons don't work)

These are known limitations documented in `RENDERING_BUGS.md`.

---

## Next Steps After Getting It Working

1. Open applications: `WAYLAND_DISPLAY=mirage-0 kitty`
2. Test window management
3. Report any crashes or issues
4. Contribute fixes!

---

**Still stuck?** Check the logs, run diagnostics, and test in nested mode first.
