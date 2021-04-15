#[macro_export]
macro_rules! steadfast_entry {
    () => {
        fn main() {
            use steadfast_core::log::error;
            use steadfast_core::module::engine::EngineExports;
            use steadfast_core::module::game::GameExports;
            use steadfast_core::module::load_modules;
            use steadfast_runtime::log::init_logger;

            init_logger();

            load_modules! {
                libgame   => GameExports,
                libengine => EngineExports,
            }

            let mut module_manager = ModuleManager::new();

            loop {
                #[cfg(debug_assertions)]
                module_manager.reload();

                std::thread::sleep(std::time::Duration::from_millis(1000));
            }
        }
    };
}
