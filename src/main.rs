mod backend;
mod state;
mod layout;
mod decorations;
mod wallpaper;
mod dock;
mod launchpad;

use backend::winit::run_winit_backend;
use state::MirageState;

fn main() {
    run_winit_backend::<MirageState>();
}
