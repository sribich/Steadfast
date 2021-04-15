use crate::exports;
use steadfast_defs::engine::Application;

exports! {
    struct GameExports {
        create_application: () -> Application,
    }
}
