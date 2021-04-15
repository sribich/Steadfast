mod host;
mod modules;

pub use crate::host::*;
pub use crate::modules::*;

use libloading::Library;
use notify::{watcher, RecommendedWatcher, Watcher};
use std::fmt::Debug;
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;
use thiserror::Error;

#[cfg(windows)]
type Symbol<T> = libloading::os::windows::Symbol<T>;
#[cfg(not(windows))]
type Symbol<T> = libloading::unix::windows::Symbol<T>;

/// An opaque pointer to a module's state.
///
/// Model state is allocated on the [`Host`] so its'
/// memory is not freed when we reload the module.
pub struct State {
    _private: [u8; 0],
}

pub struct Module<VTable: Debug> {
    path: Box<Path>,
    pub symbols: Option<Symbols<VTable>>,
    pub state: Vec<u64>,
    watcher: RecommendedWatcher,
    rx: Receiver<notify::DebouncedEvent>,
}

pub struct ModuleAPI<VTable: Debug> {
    pub size: fn() -> usize,
    pub init: fn(*mut ()),
    pub reload: fn(*mut ()) -> VTable,
    pub update: fn(&mut Host, *mut ()),
    pub unload: fn(*mut ()),
    pub deinit: fn(*mut ()),
}

#[derive(Debug)]
pub struct Symbols<VTable: Debug> {
    pub lib: Library,
    pub api: Symbol<*mut ModuleAPI<VTable>>,
    // /// The path from which the library was loaded. This _will_ be a
    // /// temporary file in windows.
    // loaded_path: PathBuf,

    // /// The original location of the file. This is the file we watch
    // /// to know when to reload.
    // original_path: PathBuf,
}

impl<VTable: Debug> Module<VTable> {
    /// Creates a new library that can be reloaded at runtime
    ///
    /// [`path`] must be a dynamic library containing a `__MODULE`
    /// symbol, created using the [`init_module!`] macro.
    ///
    pub fn new(path: &'static Path) -> Result<Self, Error> {
        let symbols = Self::load(path)?;
        let size = (unsafe { &**symbols.api }.size)();

        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(1))?;
        watcher.watch(path.parent().unwrap(), notify::RecursiveMode::NonRecursive)?;

        let mut module = Module {
            path: path.with_extension("dll").into_boxed_path(),
            state: vec![],
            symbols: None,
            watcher,
            rx,
        };

        module.resize_state(size);

        (unsafe { &**symbols.api }.init)(Self::get_state(&mut module.state));

        Ok(module)
    }

    pub fn reload(&mut self) -> Result<Option<&Symbols<VTable>>, Error> {
        let mut reload = false;

        while let Ok(event) = self.rx.try_recv() {
            use notify::DebouncedEvent::*;

            match event {
                NoticeWrite(ref path) | Write(ref path) | Create(ref path) => {
                    if path.canonicalize()? == *self.path.canonicalize()? {
                        reload = true;
                    }
                }
                _ => (),
            }
        }

        if reload || self.symbols.is_none() {
            Ok(self.do_reload()?)
        } else {
            Ok(None)
        }
    }

    pub fn do_reload(&mut self) -> Result<Option<&Symbols<VTable>>, Error> {
        if let Some(Symbols { ref mut api, .. }) = self.symbols {
            (unsafe { &***api }.unload)(Self::get_state(&mut self.state));
        }

        self.symbols = None;

        let symbols = Self::load(&self.path)?;

        self.resize_state((unsafe { &**symbols.api }.size)());

        // TODO: Load module vtable
        (unsafe { &**symbols.api }.reload)(Self::get_state(&mut self.state));
        self.symbols = Some(symbols);

        if let Some(symbols) = &self.symbols {
            Ok(Some(symbols))
        } else {
            Ok(None)
        }
    }

    pub fn update(&mut self, host: &mut Host) -> () {
        if let Some(Symbols { ref mut api, .. }) = self.symbols {
            (unsafe { &***api }.update)(host, Self::get_state(&mut self.state));
        }
    }

    fn resize_state(&mut self, size: usize) {
        self.state.resize((size + 7) / 8, 0);
    }

    pub fn get_state(buffer: &mut Vec<u64>) -> *mut () {
        buffer.as_mut_ptr() as *mut ()
    }

    #[cfg(windows)]
    fn load(path: &Path) -> Result<Symbols<VTable>, Error> {
        let path_extension = path.with_extension("dll");
        let path_copied = path_extension.with_extension("copy.dll");

        std::fs::copy(&path_extension, &path_copied)?;

        Symbols::new(&path_copied)
    }

    #[cfg(not(windows))]
    fn load(path: &'static Path) {
        Symbols::new(path)
    }
}

impl<VTable: Debug> Symbols<VTable>
where
    VTable: Debug,
{
    fn new(path: &Path) -> Result<Self, Error> {
        unsafe {
            let library = Library::new(path)?;
            let api = library
                .get::<*mut ModuleAPI<VTable>>(b"__MODULE")?
                .into_raw();

            Ok(Symbols { lib: library, api })
        }
    }
}

impl<VTable: Debug> Drop for Module<VTable> {
    fn drop(&mut self) {
        if let Some(Symbols { ref mut api, .. }) = self.symbols {
            unsafe {
                ((***api).deinit)(Self::get_state(&mut self.state));
            }
        }
    }
}

///
/// # Lifecycle
///
///   - `init` gets called on the initial application load. This is where
///     module state should be initialized.
///   - `reload` gets called on library load, including the first. Module
///     specific reload functionality may reside here, but this function
///     MUST always return a vtable of its' exports
///   - `update` is called when another library is reloaded
///   - `unload` gets called on library unload
///   - `deinit` gets called when the application is shutting down
///
#[macro_export]
macro_rules! init_module {
    (
        state: $state:ty,
        exports: $exports:ty,
        init: $init:ident,
        reload: $reload:ident,
        update: $update:ident,
        unload: $unload:ident,
        deinit: $deinit:ident,
    ) => {
        fn cast<'a>(opaque_state: *mut ()) -> &'a mut $state {
            unsafe { &mut *(opaque_state as *mut $state) }
        }

        fn __init_module(opaque_state: *mut ()) {
            $init(cast(opaque_state))
        }

        fn __reload_module(opaque_state: *mut ()) -> $exports {
            $reload(cast(opaque_state))
        }

        fn __update_module(host: &mut Host, opaque_state: *mut ()) {
            $update(host, cast(opaque_state))
        }

        fn __unload_module(opaque_state: *mut ()) {
            $unload(cast(opaque_state))
        }

        fn __deinit_module(opaque_state: *mut ()) {
            $deinit(cast(opaque_state))
        }

        #[no_mangle]
        pub static __MODULE: steadfast_core::module::ModuleAPI<$exports> =
            steadfast_core::module::ModuleAPI {
                size: std::mem::size_of::<$state>,
                init: __init_module,
                reload: __reload_module,
                update: __update_module,
                unload: __unload_module,
                deinit: __deinit_module,
            };
    };
}

#[macro_export]
macro_rules! load_modules {
    ($($libname:ident => $exports:ident,)*) => {
        use steadfast_core::module::{Host, Module, Symbols};
        use std::path::Path;

        struct ModuleManager {
            host: Host,
            $(
                $libname: Module<$exports>,
            )*
        }

        impl ModuleManager {
            pub fn new() -> Self {
                Self {
                    host: Host::default(),
                    $(
                        $libname: Module::new(
                            Path::new(concat!("../target/debug/", stringify!($libname)))
                        ).expect(concat!("Failed to load library ", stringify!($libname))),
                    )*
                }
            }

            pub fn reload(&mut self) -> () {
                let mut reloaded = false;
                $(
                    if let Ok(vtable) = self.$libname.reload() {
                        if let Some(symbols) = vtable {
                            reloaded = true;
                            self.host.$libname = Some($exports::new(symbols));
                        }
                    }
                )*

                if (reloaded) {
                    $(
                        self.$libname.update(&mut self.host);
                    )*
                }
            }
        }
    }
}

//  //

#[derive(Debug, Error)]
pub enum Error {
    #[error("An error occurred while trying to load")]
    Io(#[from] std::io::Error),

    #[error("An error occurred while creating the filesystem watcher")]
    Watch(#[from] notify::Error),

    #[error("An error occurred while attempting to load the library")]
    Library(#[from] libloading::Error),
}

#[cfg(test)]
mod tests {}
