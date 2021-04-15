use libloading::{Library, Symbol};
use std::error::Error;
use std::{ffi::CString, os::raw::c_char};

fn main() -> Result<(), Box<dyn Error>> {
    let lib = unsafe { Library::new("./steadfast.dll")? };
    unsafe {
        let greet: Symbol<fn(name: &str)> = lib.get(b"greet")?;
        greet("Bob");
    }

    Ok(())
}

// mod editor;
//
// use crate::editor::{Editor, Message};
// use anyhow::Result;
// use std::borrow::{Borrow, BorrowMut};
// use std::ffi::CString;
// use std::ops::Deref;
// use std::sync::Arc;
// use steadfast_vulkan::api::EntryPointers;
// use steadfast_vulkan::InstanceExtensions;
// use steadfast_vulkan::{vulkan_loader, ApplicationInfo, Instance, InstanceError, Window};
// use steadfast_vulkan::{InstanceLayers, Version};
// use steadfast_vulkan::{LogicalDevice, PhysicalDevice};
// use winit::dpi::LogicalSize;
// use winit::event::{Event, WindowEvent};
// use winit::event_loop::{ControlFlow, EventLoop};
// use winit::window::WindowBuilder;
//
// #[macro_use]
// extern crate log;
// extern crate pretty_env_logger;
//
// fn main() -> Result<()> {
//     pretty_env_logger::init();
//
//     let instance = create_instance()?;
//
//     let window = create_devices(instance.clone());
//
//     instance.run();
//
//     Ok(())
// }
//
// fn create_instance() -> Result<Arc<Instance>> {
//     let info = ApplicationInfo {
//         application_name: env!("CARGO_PKG_NAME").into(),
//         application_version: Version::from_cargo(),
//         engine_name: env!("CARGO_PKG_NAME").into(),
//         engine_version: Version::from_cargo(),
//         api_version: Version::new(1, 2, 0),
//     };
//
//     Instance::new(Some(info), InstanceExtensions::default())
// }
//
// fn create_devices(instance: Arc<Instance>) -> Result<Box<Window>> {
//     let device = LogicalDevice::new(instance.clone());
//
//     let window = device.window();
//     device.recreate_device();
//
//     window
// }
//
// // fn update(editor: &mut Editor) {
// //     editor.update();
// // }
// //
// // // PhysicalDevice::enumerate(&instance);
// //     // let surface =
// //
// //     // let monitor = event_loop.primary_monitor().unwrap();
// //     // let size = LogicalSize::new(monitor.size().width, monitor.size().height);
// //
// //     // let window = WindowBuilder::new()
// //     //     .with_maximized(true)
// //     //     .with_inner_size(size)
// //     //     .with_title("haha")
// //     //     .build(&event_loop)
// //     //     .unwrap();
// //
// //     let mut editor = Editor::new();
