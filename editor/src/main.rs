mod editor;

use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, WindowEvent};
use winit::window::WindowBuilder;
use winit::dpi::LogicalSize;
use crate::editor::{Editor, Message};
use steadfast_vulkan::instance::instance::{Instance, InstanceError};
use steadfast_vulkan::device::physical_device::PhysicalDevice;

fn update(editor: &mut Editor) {
    editor.update();
}

fn main() -> Result<(), InstanceError> {
    let extensions = steadfast_vulkan::window::required_extensions();

    let instance = Instance::new(extensions)?;
    PhysicalDevice::enumerate(&instance);



    let event_loop = EventLoop::new();

    let monitor = event_loop.primary_monitor().unwrap();
    let size = LogicalSize::new(monitor.size().width, monitor.size().height);

    let window = WindowBuilder::new().with_maximized(true).with_inner_size(size).with_title("haha").build(&event_loop).unwrap();

    let mut editor = Editor::new();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, ..} => {
            match event {
                WindowEvent::CloseRequested => {
                    editor.sender.send(Message::Exit { force: false }).unwrap();
                },
                WindowEvent::Resized(size) => {
                    // engine.renderer.set_frame_size(size.into());
                    //                     engine.user_interface.send_message(WidgetMessage::width(
                    //                         editor.root_grid,
                    //                         MessageDirection::ToWidget,
                    //                         size.width as f32,
                    //                     ));
                    //                     engine.user_interface.send_message(WidgetMessage::height(
                    //                         editor.root_grid,
                    //                         MessageDirection::ToWidget,
                    //                         size.height as f32,
                    //                     ));
                }
                _ => (),
            }

            // if let Some(os_event) = translate_event(&event) {
                // engine.user_interface.process_os_event(&os_event);
            // }
        },
        Event::MainEventsCleared => {
            update(
                &mut editor,
            );
            //                 &mut engine,
            //                 &mut elapsed_time,
            //                 fixed_timestep,
            //                 &clock,

            if editor.exit {
                *control_flow = ControlFlow::Exit;
            }
        },
        Event::RedrawRequested(_) => {
            // engine.render(fixed_timestep).unwrap();
        },
        Event::LoopDestroyed => {
            // rg3d::core::profiler::print();
        },
        _ => *control_flow = ControlFlow::Poll,
    })
}
