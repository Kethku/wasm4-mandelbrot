mod app;
#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;

pub use wasm4::*;
use app::State;

static mut STATE: Option<State> = None;

#[no_mangle]
unsafe fn start() {
    STATE = Some(State::new());
}

#[no_mangle]
unsafe fn update() {
    if let Some(state) = STATE.as_mut() {
        state.update();
        state.draw();
    }
}
