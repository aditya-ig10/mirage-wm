# Mirage-WM Rendering Pipeline: Known Issues & Limitations

## Critical Rendering Bugs

### 1. **Type Mismatch: Logical vs Physical Coordinates**
**Status:** ⚠️ PARTIALLY FIXED
**File:** `src/backend/winit.rs`, `src/dock.rs`
**Issue:** 
- Dock rectangle methods return `Rectangle<i32, Logical>` 
- Frame drawing expects `Rectangle<i32, Physical>`
- Type mismatch causes compilation errors
**Impact:** Medium - Cannot render dock app icons individually
**Workaround:** Currently only rendering dock background, individual icons skipped

```rust
// Current workaround in render_frame():
// TODO: Render dock apps icons here
// Currently skipping individual app icons - dock background visible
```

**Fix needed:**
- Convert dock rect methods to return Physical coordinates
- Or implement proper coordinate transformation during rendering

---

### 2. **Nested Wayland Client Rendering Limitations**
**Status:** 🔴 NOT FIXED (Fundamental limitation)
**File:** `src/backend/winit.rs` (render_frame function)
**Issue:**
- Compositor is nested inside GNOME Wayland (via Winit)
- Client windows don't render visible content
- Surface buffers aren't properly displayed
**Root Cause:** GNOME Wayland nesting environment constraints
**Impact:** CRITICAL - No client application window content visible

```rust
// In render_frame(), client content is collected but often empty:
let elements: Vec<WaylandSurfaceRenderElement<GlesRenderer>> = 
    render_elements_from_surface_tree(renderer, window.wl_surface(), ...);

// Elements frequently empty, fallback color used instead
if !elements.is_empty() {
    draw_render_elements(&mut frame, 1.0, &elements, &[window_rect])?;
} else {
    // FALLBACK: Draw colored rectangle instead of actual window content
    frame.draw_solid(window_rect, &[window_rect], color)?;
}
```

**Solutions:**
1. Run on native KMS/DRM backend instead of Winit (requires kernel support)
2. Use VirtualBox/QEMU with proper device passthrough
3. Run on bare metal with DRM support

---

### 3. **Decoration Rendering on Top of Content**
**Status:** ⚠️ PARTIAL - Decorations visible but may overlap incorrectly
**File:** `src/backend/winit.rs`, `src/decorations.rs`
**Issue:**
- Window decorations (title bar, buttons) render after window content
- Title bar is 32px but may not align perfectly with surface content
- No clipping applied to window content below title bar
**Impact:** Low-Medium - Decorations visible but UI feels disjointed

**Fix needed:**
- Clip window content to exclude title bar area
- Adjust layout to reserve space for decorations
- Render in correct Z-order: background → content → decorations

---

### 4. **Color Space & Gamma Issues**
**Status:** ⚠️ PARTIAL - Colors work but may be incorrect
**File:** `src/backend/winit.rs` (Color32F values)
**Issue:**
- Using sRGB color values (0.15, 0.15, 0.15) directly
- No color space conversion
- May appear washed out or too bright on some systems
**Impact:** Low - Visual only, doesn't break functionality

**Fix:**
```rust
// Current: Direct sRGB
frame.clear(Color32F::new(0.15, 0.15, 0.15, 1.0), &[screen_rect])?;

// Better: Apply gamma correction if needed
// Color32F uses linear RGB, input should be pre-gamma-corrected
```

---

### 5. **Damage Tracking Inefficiency**
**Status:** ⚠️ PARTIAL - Implemented but not optimal
**File:** `src/backend/winit.rs` (render_frame function)
**Issue:**
- All damage rects submitted every frame
- No delta/incremental updates
- Entire window redrawn even for small changes
**Impact:** Medium - Increases GPU load unnecessarily

**Current:** Every rect is added to damage_rects
**Fix:** Only add rects that actually changed from last frame

---

### 6. **Text Rendering Not Implemented**
**Status:** ❌ NOT IMPLEMENTED
**File:** `src/decorations.rs`
**Issue:**
- Window title should render text in title bar
- Currently no text rendering in compositor
- Need font loading and glyph rendering
**Impact:** Medium - Window titles not visible

**Fix needed:**
- Integrate `rusttype` or `ab_glyph` for font rendering
- Render window title text in title bar area

---

### 7. **Coordinate System Confusion**
**Status:** ⚠️ PARTIAL - Mostly working but fragile
**Files:** `src/layout.rs`, `src/backend/winit.rs`
**Issue:**
- Mixed use of (x, y) tuples and Point types
- Inconsistent coordinate origin expectations
- No clear documentation of coordinate space
**Impact:** Low-Medium - Works but error-prone for extensions

**Example inconsistency:**
```rust
// Sometimes tuples:
let location = (geom.location.x, geom.location.y);

// Sometimes Points:
let pointer_pos = state.pointer_pos; // Point<f64, Logical>

// Sometimes explicit coords:
frame.draw_solid(window_rect, &[window_rect], color)?;
```

---

### 8. **Event Loop Frame Timing**
**Status:** ⚠️ PARTIAL - Fixed rate but not optimal
**File:** `src/backend/winit.rs` (main loop)
**Issue:**
- Fixed 1ms polling interval
- No vsync synchronization
- May cause stuttering or excessive CPU usage
- No frame rate limiting
**Impact:** Low - Works but not smooth

```rust
// Current: Fixed 1ms dispatch
let result = event_loop.dispatch(Some(Duration::from_millis(1)), &mut state);
```

**Fix:**
- Implement refresh rate synchronization
- Use vsync from output mode (60 Hz)
- Implement frame skipping if rendering slower than refresh

---

### 9. **Pointer Focus & Click Detection**
**Status:** ⚠️ PARTIAL - Works but no decoration click handling
**File:** `src/backend/winit.rs` (PointerButton event handler)
**Issue:**
- Click detection only sets window focus
- Window decoration button clicks not detected
- No close/minimize/maximize actions implemented
- Dock click detection not wired
**Impact:** High - UI controls don't function

**Current code:**
```rust
if event.state() == ButtonState::Pressed {
    if let Some(idx) = state.window_at(state.pointer_pos) {
        state.set_focus(Some(idx));
        // TODO: Check for decoration button clicks here
        // TODO: Implement close/minimize/maximize actions
    }
}
```

**Fix needed:**
- Detect clicks on decoration buttons
- Detect clicks on dock apps
- Implement window close/minimize/maximize
- Implement dock app launching

---

### 10. **No Error Handling in Rendering Loop**
**Status:** ⚠️ PARTIAL - Errors logged but not recovered
**File:** `src/backend/winit.rs`
**Issue:**
- Rendering errors logged but loop continues
- No recovery mechanism
- Frame skipping not implemented
- GPU state may be corrupted after error
**Impact:** Low-Medium - May cause graphical glitches

```rust
// Current: Errors logged but ignored
if let Err(err) = render_frame(&state, &mut backend) {
    error!("Rendering error: {}", err);
    // No recovery - loop continues with potentially bad state
}
```

---

## Summary Table

| Issue | Severity | Type | Fixable | Priority |
|-------|----------|------|---------|----------|
| Logical vs Physical coords | Medium | Type System | Yes | High |
| Client rendering (nested) | Critical | Fundamental | Workaround only | Medium |
| Decoration overlap | Medium | Layout | Yes | Medium |
| Color space | Low | Visual | Yes | Low |
| Damage tracking | Medium | Performance | Yes | Medium |
| Text rendering | High | Feature | Yes | High |
| Coordinate confusion | Medium | Code Quality | Yes | Medium |
| Frame timing | Low | Performance | Yes | Low |
| Click detection | High | Functionality | Yes | High |
| Error handling | Medium | Robustness | Yes | Low |

---

## Rendering Pipeline Flow

```
1. Clear screen (dark gray background)
   ├─ Call frame.clear() with screen rect
   └─ Damage rect added

2. For each window:
   ├─ Collect surface render elements
   ├─ Draw elements OR fallback color
   ├─ Draw decorations (title bar + buttons)
   │  ├─ Title bar rect
   │  ├─ Close button (red)
   │  ├─ Minimize button (yellow)
   │  └─ Maximize button (green)
   └─ Damage rects added

3. Render dock:
   ├─ Draw dock background (bottom 80px)
   ├─ TODO: Draw app icons
   └─ Damage rect added

4. Render cursor:
   ├─ Draw white square at pointer position
   └─ Damage rect added

5. Frame finish & submit:
   ├─ frame.finish()
   ├─ backend.submit(damage_rects)
   └─ Display update
```

---

## Testing Checklist

- [ ] Run on native KMS/DRM backend
- [ ] Verify client window content renders
- [ ] Test window decoration button clicks
- [ ] Test dock app launching
- [ ] Verify window close/minimize/maximize
- [ ] Check text rendering in title bars
- [ ] Profile CPU/GPU usage
- [ ] Test with multiple windows
- [ ] Verify cursor visibility
- [ ] Test keyboard input routing

---

## Recommended Fix Priority

**Phase 1 (Critical):**
1. Fix coordinate type mismatches (Logical vs Physical)
2. Implement window decoration click detection
3. Add window close/minimize/maximize actions

**Phase 2 (High Impact):**
1. Implement text rendering for titles
2. Fix dock app icon rendering
3. Wire dock app launching

**Phase 3 (Quality):**
1. Improve error handling
2. Optimize damage tracking
3. Add vsync support
