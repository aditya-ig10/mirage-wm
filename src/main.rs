use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use calloop::EventLoop;
use smithay::{
    reexports::wayland_server::{
        backend::{ClientData, ClientId, DisconnectReason},
        Display,
    },
    wayland::socket::ListeningSocketSource,
};
use tracing::info;
use tracing_subscriber::EnvFilter;

/// Global compositor state (will hold seats, windows, etc. later)
struct MirageState {
    // Later: window list, focus info, outputs, config, etc.
}

// Per-client metadata (can be empty for now)
struct MirageClientData;

impl ClientData for MirageClientData {
    fn initialized(&self, _client_id: ClientId) {
        // info!("Client connected: {:?}", client_id);
    }

    fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) {
        // info!("Client disconnected: {:?} ({:?})", client_id, reason);
    }
}

fn init_logging() {
    let filter = EnvFilter::from_default_env()
        .add_directive("mirage_wm=trace".parse().unwrap())
        .add_directive("smithay=info".parse().unwrap());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();
}

fn main() -> Result<()> {
    init_logging();

    // 1. Wayland display (wrapped in Rc<RefCell<...>> so we can share it)
    let display: Rc<RefCell<Display<MirageState>>> =
        Rc::new(RefCell::new(Display::new()?));

    // 2. Event loop (user state = MirageState)
    let mut event_loop: EventLoop<MirageState> = EventLoop::try_new()?;

    // 3. Global compositor state
    let mut state = MirageState {};

    // 4. Create listening socket for clients (auto picks wayland-N)
    //    This is separate from Display so we don't hit borrow issues.
    let listening_socket = ListeningSocketSource::new_auto()?;
    let socket_name = listening_socket.socket_name().to_string_lossy().into_owned();

    // 5. Insert socket into event loop: new clients -> insert_client into Display
    {
        let display_for_loop = display.clone();
        event_loop.handle().insert_source(
            listening_socket,
            move |client_stream, _meta, _state: &mut MirageState| {
                let mut display = display_for_loop
                    .borrow_mut(); // only one mutable borrow at a time
                let mut dh = display.handle();
                dh.insert_client(client_stream, Arc::new(MirageClientData));
            },
        )?;
    }

    info!("Mirage compositor starting on WAYLAND_DISPLAY={}", socket_name);
    println!("Mirage compositor running on WAYLAND_DISPLAY={}", socket_name);
    println!("(Right now it's just an empty loop, no windows or outputs yet.)");

    // 6. Main loop (no backend yet, just Wayland I/O)
    loop {
        // Drive calloop (timers, sources, etc.)
        event_loop.dispatch(Some(Duration::from_millis(16)), &mut state)?;

        // Process client requests and send events
        {
            let mut display = display.borrow_mut();
            display.dispatch_clients(&mut state)?;
            display.flush_clients()?;
        }
    }
}
