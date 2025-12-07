use smithay::{
    reexports::wayland_server::{DisplayHandle, protocol::wl_surface::WlSurface},
    wayland::{
        compositor::{CompositorState, CompositorHandler},
        shell::xdg::{XdgShellHandler, XdgShellState, ToplevelSurface},
        output::OutputHandler,
        buffer::BufferHandler,
    },
    input::{SeatHandler, SeatState, pointer::PointerHandle, keyboard::KeyboardHandle},
    output::Output,
    utils::{Point, Logical},
};
use crate::layout::TilingLayout;

pub struct MirageState {
    pub compositor: CompositorState,
    pub xdg_shell: XdgShellState,
    pub output: Option<Output>,
    pub windows: Vec<ToplevelSurface>,
    pub pointer_pos: Point<f64, Logical>,
    pub focused_window: Option<usize>,
    pub layout: TilingLayout,
    pub seat_state: SeatState<Self>,
    pub pointer: Option<PointerHandle<Self>>,
    pub keyboard: Option<KeyboardHandle<Self>>,
}

impl MirageState {
    pub fn new(display_handle: &DisplayHandle) -> Self {
        let compositor = CompositorState::new::<Self>(display_handle);
        let xdg_shell = XdgShellState::new::<Self>(display_handle);
        let seat_state = SeatState::new();

        Self { 
            compositor, 
            xdg_shell,
            output: None,
            windows: Vec::new(),
            pointer_pos: Point::from((0.0, 0.0)),
            focused_window: None,
            layout: TilingLayout::new(1280, 800),
            seat_state,
            pointer: None,
            keyboard: None,
        }
    }

    pub fn initialize_seat(&mut self, display_handle: &DisplayHandle) {
        let mut seat = self.seat_state.new_wl_seat(display_handle, "default");

        // Add pointer device
        self.pointer = Some(seat.add_pointer());

        // Add keyboard device
        self.keyboard = seat.add_keyboard(Default::default(), 200, 200).ok();
    }

    /// Find which window index is at the given position
    pub fn window_at(&self, pos: Point<f64, Logical>) -> Option<usize> {
        // Windows are stacked - iterate in reverse to find topmost window
        for idx in (0..self.windows.len()).rev() {
            let geom = self.layout.calculate_geometry(idx, self.windows.len());
            if geom.contains_point(pos) {
                return Some(idx);
            }
        }
        None
    }

    /// Set focus to a specific window
    pub fn set_focus(&mut self, idx: Option<usize>) {
        if let Some(idx) = idx {
            if idx < self.windows.len() {
                self.focused_window = Some(idx);
                tracing::info!("Window focus changed to {}", idx);
            }
        } else {
            self.focused_window = None;
            tracing::info!("Window focus cleared");
        }
    }
}

impl CompositorHandler for MirageState {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor
    }

    fn client_compositor_state<'a>(
        &self,
        _client: &'a smithay::reexports::wayland_server::Client,
    ) -> &'a smithay::wayland::compositor::CompositorClientState {
        todo!()
    }

    fn commit(&mut self, _surface: &WlSurface) {
        // TODO: Handle surface commit
    }
}

impl OutputHandler for MirageState {}

impl XdgShellHandler for MirageState {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        tracing::info!("New XDG toplevel window!");
        self.windows.push(surface);
    }

    fn new_popup(
        &mut self,
        _popup: smithay::wayland::shell::xdg::PopupSurface,
        _positioner: smithay::wayland::shell::xdg::PositionerState,
    ) {
        println!("New XDG popup!");
    }

    fn move_request(
        &mut self,
        _surface: smithay::wayland::shell::xdg::ToplevelSurface,
        _seat: smithay::reexports::wayland_server::protocol::wl_seat::WlSeat,
        _serial: smithay::utils::Serial,
    ) {}

    fn resize_request(
        &mut self,
        _surface: smithay::wayland::shell::xdg::ToplevelSurface,
        _seat: smithay::reexports::wayland_server::protocol::wl_seat::WlSeat,
        _serial: smithay::utils::Serial,
        _edges: smithay::reexports::wayland_protocols::xdg::shell::server::xdg_toplevel::ResizeEdge,
    ) {}

    fn grab(
        &mut self,
        _popup: smithay::wayland::shell::xdg::PopupSurface,
        _seat: smithay::reexports::wayland_server::protocol::wl_seat::WlSeat,
        _serial: smithay::utils::Serial,
    ) {
        // TODO: Implement grab handling
    }

    fn reposition_request(
        &mut self,
        _popup: smithay::wayland::shell::xdg::PopupSurface,
        _positioner: smithay::wayland::shell::xdg::PositionerState,
        _token: u32,
    ) {
        // TODO: Implement reposition handling
    }
}

impl SeatHandler for MirageState {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;
    type TouchFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }
}

impl BufferHandler for MirageState {
    fn buffer_destroyed(&mut self, _buffer: &smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer) {
        // Buffer was destroyed - no action needed for now
    }
}

smithay::delegate_compositor!(MirageState);
smithay::delegate_xdg_shell!(MirageState);
smithay::delegate_seat!(MirageState);
smithay::delegate_output!(MirageState);
