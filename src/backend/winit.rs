use std::time::Duration;

use smithay::{
    backend::winit::{self, WinitEvent},
    backend::renderer::{
        gles::GlesRenderer,
        element::{
            surface::{render_elements_from_surface_tree, WaylandSurfaceRenderElement},
            Kind,
        },
        utils::draw_render_elements,
        Color32F,
    },
    reexports::{
        calloop::EventLoop,
        winit::platform::pump_events::PumpStatus,
    },
    output::{Mode, Output, PhysicalProperties, Subpixel},
    utils::Transform,
};
use tracing::{error, info};

use crate::state::MirageState;

pub const OUTPUT_NAME: &str = "winit";

// Global serial counter for input events
static SERIAL_COUNTER: smithay::utils::SerialCounter = smithay::utils::SerialCounter::new();

pub fn run_winit_backend<S: 'static>() {
    let mut event_loop = EventLoop::try_new().unwrap();
    let display: smithay::reexports::wayland_server::Display<MirageState> = 
        smithay::reexports::wayland_server::Display::new().unwrap();
    let mut display_handle = display.handle();

    let (mut backend, mut winit) = match winit::init::<GlesRenderer>() {
        Ok(ret) => ret,
        Err(err) => {
            error!("Failed to initialize Winit backend: {}", err);
            return;
        }
    };

    let size = backend.window_size();

    let mode = Mode {
        size,
        refresh: 60_000,
    };
    let output = Output::new(
        OUTPUT_NAME.to_string(),
        PhysicalProperties {
            size: (0, 0).into(),
            subpixel: Subpixel::Unknown,
            make: "Mirage".into(),
            model: "Winit".into(),
        },
    );
    output.create_global::<MirageState>(&display_handle);
    output.change_current_state(Some(mode), Some(Transform::Normal), None, Some((0, 0).into()));
    output.set_preferred(mode);

    let mut state = MirageState::new(&display_handle);
    state.output = Some(output.clone());
    state.initialize_seat(&display_handle);

    info!("Initialization completed, starting the main loop.");
    info!("Mirage Compositor running at {}x{}", size.w, size.h);

    loop {
        let status = winit.dispatch_new_events(|event| match event {
            WinitEvent::Resized { size, .. } => {
                if let Some(output) = &state.output {
                    let mode = Mode {
                        size,
                        refresh: 60_000,
                    };
                    output.change_current_state(Some(mode), None, None, None);
                    output.set_preferred(mode);
                    state.layout.update_screen_size(size.w, size.h);
                    info!("Output resized to {}x{}", size.w, size.h);
                }
            }
            WinitEvent::Input(input_event) => {
                // Process input events and route to devices
                use smithay::backend::input::{InputEvent, AbsolutePositionEvent, PointerButtonEvent, ButtonState, KeyboardKeyEvent, Event};
                use smithay::input::pointer::MotionEvent;
                match input_event {
                    InputEvent::PointerMotion { event: _ } => {
                        // Regular pointer motion (not used in nested compositor)
                        info!("Pointer motion event");
                    }
                    InputEvent::PointerMotionAbsolute { event } => {
                        // Absolute positioning from Winit - route to pointer device
                        let size = backend.window_size();
                        let x = event.x_transformed(size.w);
                        let y = event.y_transformed(size.h);
                        state.pointer_pos.x = x;
                        state.pointer_pos.y = y;
                        info!("Pointer absolute position ({:.1}, {:.1})", x, y);
                        
                        // Route to pointer device - clone to avoid borrow checker issues
                        if let Some(pointer) = state.pointer.clone() {
                            // Update pointer focus based on window under cursor
                            let focus = if let Some(idx) = state.window_at(state.pointer_pos) {
                                state.windows.get(idx).map(|s| s.wl_surface().clone())
                            } else {
                                None
                            };
                            
                            let pointer_pos = state.pointer_pos;
                            let motion_event = MotionEvent {
                                location: pointer_pos,
                                serial: SERIAL_COUNTER.next_serial(),
                                time: event.time_msec(),
                            };
                            
                            pointer.motion(&mut state, focus.map(|s| (s, pointer_pos)), &motion_event);
                            pointer.frame(&mut state);
                        }
                    }
                    InputEvent::PointerButton { event } => {
                        // Mouse click - check which window is under cursor and set focus
                        if event.state() == ButtonState::Pressed {
                            if let Some(idx) = state.window_at(state.pointer_pos) {
                                state.set_focus(Some(idx));
                                info!("Clicked on window {}", idx);
                            } else {
                                state.set_focus(None);
                                info!("Clicked on empty space");
                            }
                        }
                        
                        // Route button event to pointer device - clone to avoid borrow checker issues
                        if let Some(pointer) = state.pointer.clone() {
                            use smithay::input::pointer::ButtonEvent as PointerButtonEvent;
                            let button_event = PointerButtonEvent {
                                button: event.button_code(),
                                state: event.state(),
                                serial: SERIAL_COUNTER.next_serial(),
                                time: event.time_msec(),
                            };
                            pointer.button(&mut state, &button_event);
                            pointer.frame(&mut state);
                        }
                    }
                    InputEvent::PointerAxis { event: _ } => {
                        // Mouse scroll - route to pointer device
                        info!("Scroll event");
                        // TODO: Implement axis scrolling
                    }
                    InputEvent::Keyboard { event } => {
                        // Keyboard input - route to keyboard device
                        info!("Keyboard event");
                        if let Some(keyboard) = state.keyboard.clone() {
                            use smithay::input::keyboard::FilterResult;
                            keyboard.input::<(), _>(
                                &mut state,
                                event.key_code(),
                                event.state(),
                                SERIAL_COUNTER.next_serial(),
                                event.time_msec(),
                                |_, _, _| FilterResult::Forward, // Forward all keys to clients
                            );
                        }
                    }
                    _ => {}
                }
            }
            _ => (),
        });

        if let PumpStatus::Exit(_) = status {
            info!("Received exit event, shutting down...");
            break;
        }

        // Render a frame
        if let Err(err) = render_frame(&state, &mut backend) {
            error!("Rendering error: {}", err);
        }

        let result = event_loop.dispatch(Some(Duration::from_millis(1)), &mut state);
        if result.is_err() {
            info!("Event loop error, shutting down...");
            break;
        } else {
            display_handle.flush_clients().ok();
        }
    }

    info!("Mirage Compositor shutdown complete.");
}

fn render_frame(
    state: &MirageState,
    backend: &mut smithay::backend::winit::WinitGraphicsBackend<GlesRenderer>,
) -> Result<(), Box<dyn std::error::Error>> {
    use smithay::utils::{Rectangle, Transform};
    use smithay::backend::renderer::{Renderer, Frame};
    
    let size = backend.window_size();
    let mut damage_rects = Vec::new();
    
    // First, collect all render elements BEFORE creating the frame
    // We need to bind renderer, but NOT create frame yet
    let all_elements = {
        let (renderer, _) = backend.bind()?;
        
        let mut all_window_elements = Vec::new();
        
        // Collect render elements for each window
        for (idx, window) in state.windows.iter().enumerate() {
            let geom = state.layout.calculate_geometry(idx, state.windows.len());
            let location = (geom.location.x, geom.location.y);
            
            let elements: Vec<WaylandSurfaceRenderElement<GlesRenderer>> = 
                render_elements_from_surface_tree(
                    renderer,
                    window.wl_surface(),
                    location,
                    1.0, // scale
                    1.0, // alpha
                    Kind::Unspecified,
                );
            
            all_window_elements.push((idx, geom, elements));
        }
        
        all_window_elements
    };
    
    // Now bind again and render
    {
        let (renderer, mut target) = backend.bind()?;
        let mut frame = renderer.render(&mut target, size, Transform::Normal)?;
        
        // Clear to background color (dark gray)
        let screen_rect = Rectangle::from_loc_and_size((0, 0), (size.w, size.h));
        frame.clear(Color32F::new(0.15, 0.15, 0.15, 1.0), &[screen_rect])?;
        
        // Render each window's collected elements
        for (idx, geom, elements) in all_elements {
            let window_rect = Rectangle::from_loc_and_size(
                (geom.location.x, geom.location.y),
                (geom.size.w, geom.size.h),
            );
            
            // Draw elements if any exist
            if !elements.is_empty() {
                draw_render_elements(&mut frame, 1.0, &elements, &[window_rect])?;
                damage_rects.push(window_rect);
                info!("Rendered window {} with surfaces at ({},{}) size({}x{})", 
                      idx, geom.location.x, geom.location.y, geom.size.w, geom.size.h);
            } else {
                // Fallback: draw colored rectangle if no surface content
                let color = if Some(idx) == state.focused_window {
                    Color32F::new(0.2, 0.5, 0.9, 1.0) // Blue for focused
                } else {
                    Color32F::new(0.3, 0.3, 0.3, 1.0) // Dark gray for unfocused
                };
                
                frame.draw_solid(window_rect, &[window_rect], color)?;
                damage_rects.push(window_rect);
            }
        }
        
        // Render cursor as a small white square
        let cursor_x = state.pointer_pos.x as i32;
        let cursor_y = state.pointer_pos.y as i32;
        let cursor_size = 10;
        
        let cursor_rect = Rectangle::from_loc_and_size(
            (cursor_x - cursor_size / 2, cursor_y - cursor_size / 2),
            (cursor_size, cursor_size),
        );
        
        // Only render cursor if it's within bounds
        if cursor_rect.loc.x >= 0 && cursor_rect.loc.y >= 0 
            && (cursor_rect.loc.x + cursor_rect.size.w) <= size.w
            && (cursor_rect.loc.y + cursor_rect.size.h) <= size.h {
            frame.draw_solid(cursor_rect, &[cursor_rect], Color32F::new(1.0, 1.0, 1.0, 1.0))?;
            damage_rects.push(cursor_rect);
        }
        
        // Finish frame rendering
        frame.finish()?;
    }
    
    // Submit the frame for display with damage information
    backend.submit(Some(&damage_rects))?;
    
    Ok(())
}

