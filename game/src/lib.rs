use steadfast_core::def::engine::Application;
use steadfast_core::module::game::GameExports;
use steadfast_core::module::{init_module, Host};

struct State {}

init_module! {
    state: State,
    exports: GameExports,
    init: init,
    reload: reload,
    update: update,
    unload: unload,
    deinit: deinit,
}

#[no_mangle]
fn create_application() -> Application {
    Application { num: 17 }
}

fn init(_state: &mut State) {}

fn reload(_state: &mut State) -> GameExports {
    GameExports { create_application }
}

fn update(_host: &mut Host, _state: &mut State) {}

fn unload(_state: &mut State) {}

fn deinit(_state: &mut State) {}
