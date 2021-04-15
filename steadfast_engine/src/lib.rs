mod application;

use steadfast_core::def::engine::Application;
use steadfast_core::module::engine::EngineExports;
use steadfast_core::module::{init_module, Host};

struct State<'a> {
    application: Option<Application>,
    host: &'a Host,
}

init_module! {
    state: State,
    exports: EngineExports,
    init: init,
    reload: reload,
    update: update,
    unload: unload,
    deinit: deinit,
}

fn init(state: &mut State) {
    state.application = None;
}

fn reload(state: &mut State) -> EngineExports {
    EngineExports {}
}

fn update(host: &'static mut Host, state: &mut State) {
    state.host = host;

    if let Some(game) = &host.libgame {
        if state.application.is_none() {
            state.application = Some(((*game).create_application)());
        }
    }
}

fn unload(state: &mut State) {}

fn deinit(state: &mut State) {}
