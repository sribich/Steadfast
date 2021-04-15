use crate::engine::EngineExports;
use crate::game::GameExports;

#[derive(Debug)]
pub struct Host {
    pub libgame: Option<GameExports>,
    pub libengine: Option<EngineExports>,
}

impl Default for Host {
    fn default() -> Self {
        Self {
            libgame: None,
            libengine: None,
        }
    }
}
