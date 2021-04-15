use libloading::Library;
use notify::{watcher, RecommendedWatcher, Watcher};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;
use thiserror::Error;

#[cfg(windows)]
type Symbol<T> = libloading::os::windows::Symbol<T>;
#[cfg(not(windows))]
type Symbol<T> = libloading::unix::windows::Symbol<T>;

pub struct State {
    _private: [u8; 0],
}

pub struct Module<Host, VTable> {
    path: Box<Path>,
    symbols: Option<Symbols<Host, VTable>>,
    pub host: Host,
    state: Vec<u64>,
    watcher: RecommendedWatcher,
    rx: Receiver<notify::DebouncedEvent>,
}

pub struct ModuleAPI<Host, VTable> {
    pub size: fn() -> usize,
    pub init: fn(&mut Host, *mut ()),
    pub reload: fn(&mut Host, *mut ()) -> VTable,
    pub update: fn(&mut Host, *mut ()),
    pub unload: fn(&mut Host, *mut ()),
    pub deinit: fn(&mut Host, *mut ()),
}

pub struct Symbols<Host, VTable> {
    pub lib: Library,
    api: Symbol<*mut ModuleAPI<Host, VTable>>,
}

impl<Host, VTable> Module<Host, VTable> {
    /// Creates a new library that can be reloaded at runtime
    ///
    /// [`path`] must be a dynamic library containing a `__MODULE`
    /// symbol, created using the [`init_module!`] macro.
    ///
    pub fn new(path: &'static Path, host: Host) -> Result<Self, Error> {
        let symbols = Self::load(path)?;
        let size = (unsafe { &**symbols.api }.size)();

        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(1))?;
        watcher.watch(path.parent().unwrap(), notify::RecursiveMode::NonRecursive)?;

        let mut module = Module {
            path: path.with_extension("dll").into_boxed_path(),
            state: vec![],
            symbols: Some(symbols),
            watcher,
            rx,
            host,
        };

        module.resize_state(size);

        if let Some(Symbols { ref mut api, .. }) = module.symbols {
            (unsafe { &***api }.init)(&mut module.host, Self::get_state(&mut module.state));
        }

        module.symbols = None;

        Ok(module)
    }

    pub fn reload(&mut self) -> Result<Option<&Symbols<Host, VTable>>, Error> {
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

    pub fn do_reload(&mut self) -> Result<Option<&Symbols<Host, VTable>>, Error> {
        if let Some(Symbols { ref mut api, .. }) = self.symbols {
            (unsafe { &***api }.unload)(&mut self.host, Self::get_state(&mut self.state));
        }

        self.symbols = None;

        let symbols = Self::load(&self.path)?;

        self.resize_state((unsafe { &**symbols.api }.size)());

        // TODO: Load module vtable
        (unsafe { &**symbols.api }.reload)(&mut self.host, Self::get_state(&mut self.state));
        self.symbols = Some(symbols);

        if let Some(symbols) = &self.symbols {
            Ok(Some(symbols))
        } else {
            Ok(None)
        }
    }

    fn resize_state(&mut self, size: usize) {
        self.state.resize((size + 7) / 8, 0);
    }

    fn get_state(buffer: &mut Vec<u64>) -> *mut () {
        buffer.as_mut_ptr() as *mut ()
    }

    #[cfg(windows)]
    fn load(path: &Path) -> Result<Symbols<Host, VTable>, Error> {
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

impl<Host, VTable> Symbols<Host, VTable> {
    fn new(path: &Path) -> Result<Self, Error> {
        unsafe {
            let library = Library::new(path)?;
            let api = library
                .get::<*mut ModuleAPI<Host, VTable>>(b"__MODULE")?
                .into_raw();

            Ok(Symbols { lib: library, api })
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
        host: $host:ty,
        state: $state:ty,
        vtable: $vtable:ty,
        init: $init:ident,
        reload: $reload:ident,
        update: $update:ident,
        unload: $unload:ident,
        deinit: $deinit:ident,
    ) => {
        fn cast<'a>(opaque_state: *mut ()) -> &'a mut $state {
            unsafe { &mut *(opaque_state as *mut $state) }
        }

        fn __init_module(host: &mut $host, opaque_state: *mut ()) {
            $init(host, cast(opaque_state))
        }

        fn __reload_module(host: &mut $host, opaque_state: *mut ()) -> $vtable {
            $reload(host, cast(opaque_state))
        }

        fn __update_module(host: &mut $host, opaque_state: *mut ()) {
            $update(host, cast(opaque_state))
        }

        fn __unload_module(host: &mut $host, opaque_state: *mut ()) {
            $unload(host, cast(opaque_state))
        }

        fn __deinit_module(host: &mut $host, opaque_state: *mut ()) {
            $deinit(host, cast(opaque_state))
        }

        #[no_mangle]
        pub static __MODULE: steadfast_core::module::ModuleAPI<$host, $vtable> =
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
    ($($libname:ident => $vtable:ident,)*) => {
        struct ModuleManager {
            $(
                $libname: steadfast_core::module::Module<steadfast_core::def::Host, $vtable>,
            )*
        }

        impl ModuleManager {
            pub fn new() -> Self {
                Self {
                    $(
                        $libname: steadfast_core::module::Module::new(std::path::Path::new(concat!("../target/debug/", stringify!($libname))), steadfast_core::def::Host::default()).expect("Failed to load libraries"),
                    )*
                }
            }

            pub fn reload(&mut self) -> () {
                $(
                    if let Ok(vtable) = self.$libname.reload() {
                        if let Some(symbols) = vtable {
                            self.$libname.host.$libname = Some($vtable::new(symbols));
                        }
                    }
                )*
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
