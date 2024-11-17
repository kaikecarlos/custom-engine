use ash::{vk, Entry, Instance};
use winit::event_loop::ControlFlow;
use winit::keyboard::{Key, NamedKey};
use std::ffi::CString;

use winit::event::{ElementState, KeyEvent};
use winit::raw_window_handle::HasDisplayHandle;
use winit::window::Window;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn create_vulkan_instance(entry: &Entry, window: &Window) -> Instance {
    // Ash requires the strings to be in the CString format
    let app_name = CString::new("Custom Engine").unwrap();
    let engine_name = CString::new("Custom Engine").unwrap();
    // Enable the validation layers
    let layer_name = CString::new("VK_LAYER_KHRONOS_validation").unwrap();
    let layers_names_raw = vec![layer_name.as_ptr()];
    // We need to specify only:
    // - s_type (StructureType)
    // - p_next (std ptr)
    // - p_application_name (app_name as ptr)
    // - application_version (0)
    // - p_engine_name (engine_name as ptr)
    // - engine_version (0)
    // - api_version (vk::API_VERSION)
    let app_info = vk::ApplicationInfo {
        s_type: vk::StructureType::APPLICATION_INFO,
        p_next: std::ptr::null(),
        p_application_name: app_name.as_ptr(),
        application_version: 0,
        p_engine_name: engine_name.as_ptr(),
        engine_version: 0,
        api_version: vk::make_api_version(1, 0, 0, 0),
        ..Default::default()
    };
    // Now we need to specify the required extensions of our platform
    let extension_names =
        ash_window::enumerate_required_extensions(window.display_handle().unwrap().as_raw())
            .unwrap()
            .to_vec();

    let create_flags = if cfg!(any(target_os = "macos", target_os = "ios")) {
        vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
    } else {
        vk::InstanceCreateFlags::default()
    };

    // Instance create info
    let create_info = vk::InstanceCreateInfo::default()
        .application_info(&app_info)
        .enabled_layer_names(&layers_names_raw)
        .enabled_extension_names(&extension_names)
        .flags(create_flags);

    // Unfortunate unsafe block
    unsafe {
        let instance = entry.create_instance(&create_info, None)
            .expect("[INSTANCE]: Failed to create instance");
        return instance;
    }

}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Custom Engine")
        .build(&event_loop)
        .unwrap();
    // Initialize Vulkan and create an instance
    let entry = Entry::linked();
    let instance = create_vulkan_instance(&entry, &window);
    let _ = event_loop.run(move |event, elwp| {
        elwp.set_control_flow(ControlFlow::Poll);
        match event {
            Event::WindowEvent {
                event:
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                logical_key: Key::Named(NamedKey::Escape),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                elwp.exit();
            }
            Event::AboutToWait { .. } => {
                // draw_frame();
            },
            _ => (),
        }
    });
}
