use crate::defs::engine::Application;
use crate::vtable;

vtable! {
    struct GameVTable {
        create_application: () -> Application,
    }
}
