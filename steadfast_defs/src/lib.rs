mod defs;
pub use crate::defs::*;
use crate::game::GameVTable;

pub struct Host {
    pub libgame: Option<GameVTable>,
}

impl Default for Host {
    fn default() -> Self {
        Self { libgame: None }
    }
}

#[macro_export]
macro_rules! vtable {
    (struct $name:ident {
        $($func_name:ident: ($($param_name:ident: $param_type:ty),*) -> $func_ret:ty,)+
    }) => {
        use crate::Host;
        use steadfast_modules::Symbols;

        pub struct $name {
            $(
                pub $func_name: fn($($param_name: $param_type)*) -> $func_ret,
            )+
        }

        impl $name {
            pub fn new(symbols: &Symbols<Host, Self>) -> Self {
                Self {
                    $(
                        $func_name: unsafe { std::mem::transmute(symbols.lib.get::<fn($($param_name: $param_type)*) -> $func_ret>(stringify!($func_name).as_bytes()).unwrap()) },
                    )+
                }
            }

            pub fn members() -> Vec<&'static str> {
                vec![$(stringify!($func_name)),*]
            }
        }
    }
}
