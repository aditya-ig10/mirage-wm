mod backend;
mod state;
mod layout;

use backend::winit::run_winit_backend;
use state::MirageState;

fn main() {
    run_winit_backend::<MirageState>();
}
