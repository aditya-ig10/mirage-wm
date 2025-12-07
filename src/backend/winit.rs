use std::time::Duration;

use smithay::{
    backend::winit::{self, WinitEvent},
    backend::renderer::gles::GlesRenderer,
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
                // Process input events - for now just log them
                use smithay::backend::input::{InputEvent, AbsolutePositionEvent};
                match input_event {
                    InputEvent::PointerMotion { event: _ } => {
                        // TODO: Track pointer position for rendering
                        info!("Pointer motion event");
                    }
                    InputEvent::PointerMotionAbsolute { event } => {
                        // Absolute positioning - use trait methods to get position
                        let size = backend.window_size();
                        let x = event.x_transformed(size.w);
                        let y = event.y_transformed(size.h);
                        state.pointer_pos.x = x;
                        state.pointer_pos.y = y;
                        info!("Pointer absolute position ({:.1}, {:.1})", x, y);
                    }
                    InputEvent::PointerButton { event } => {
                        // Mouse click - check which window is under cursor and set focus
                        use smithay::backend::input::{PointerButtonEvent, ButtonState};
                        if event.state() == ButtonState::Pressed {
                            if let Some(idx) = state.window_at(state.pointer_pos) {
                                state.set_focus(Some(idx));
                                info!("Clicked on window {}", idx);
                            } else {
                                state.set_focus(None);
                                info!("Clicked on empty space");
                            }
                        }
                    }
                    InputEvent::PointerAxis { event: _ } => {
                        // Mouse scroll
                        info!("Scroll event");
                    }
                    InputEvent::Keyboard { event: _ } => {
                        // Keyboard input
                        info!("Keyboard event");
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
    let size = backend.window_size();
    
    // Build a list of drawable elements
    #[derive(Debug)]
    struct DrawOp {
        op_type: String,
        details: String,
    }
    
    let mut draw_ops = Vec::new();
    
    // Background
    draw_ops.push(DrawOp {
        op_type: "Background".to_string(),
        details: format!("Color(0.2, 0.2, 0.2) Size({}x{})", size.w, size.h),
    });
    
    // Windows
    for (idx, _window) in state.windows.iter().enumerate() {
        let geom = state.layout.calculate_geometry(idx, state.windows.len());
        let color_name = if Some(idx) == state.focused_window {
            "Blue(0.3, 0.6, 1.0)"
        } else {
            "Gray(0.4, 0.4, 0.4)"
        };
        
        draw_ops.push(DrawOp {
            op_type: format!("Window[{}]", idx),
            details: format!(
                "Pos({},{}) Size({}x{}) Color({})",
                geom.location.x, geom.location.y, geom.size.w, geom.size.h, color_name
            ),
        });
    }
    
    // Cursor
    draw_ops.push(DrawOp {
        op_type: "Cursor".to_string(),
        details: format!("Pos({:.1}, {:.1}) Radius(8)", state.pointer_pos.x, state.pointer_pos.y),
    });
    
    // Log draw operations for debugging
    for draw_op in &draw_ops {
        tracing::trace!("{}: {}", draw_op.op_type, draw_op.details);
    }
    
    // Execute rendering
    {
        let (_renderer, _frame) = backend.bind()?;
        // TODO: Render draw_ops to frame
        // This requires proper RenderElement implementation
        // For now we prepare the data and log it
    }
    
    // Submit frame
    backend.submit(None)?;
    
    Ok(())
}

