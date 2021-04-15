use steadfast_core::def::engine::Application;
use steadfast_core::def::game::GameVTable;
use steadfast_core::def::Host;
use steadfast_core::module::init_module;

struct State {
    counter: u32,
}

init_module! {
    host: Host,
    state: State,
    vtable: GameVTable,
    init: init,
    reload: reload,
    update: update,
    unload: unload,
    deinit: deinit,
}

#[no_mangle]
fn create_application() -> Application {
    Application { num: 5 }
}

fn init(host: &mut Host, state: &mut State) {}

fn reload(host: &mut Host, state: &mut State) -> GameVTable {
    GameVTable { create_application }
}

fn update(host: &mut Host, state: &mut State) {}

fn unload(host: &mut Host, state: &mut State) {}

fn deinit(host: &mut Host, state: &mut State) {}
