use glium::{Display, glutin};
use display_info::DisplayInfo;
use glium::glutin::window::WindowBuilder;
use glium::glutin::{ContextBuilder, NotCurrent};
use glium::glutin::dpi::{LogicalSize, PhysicalSize};
use glium::glutin::platform::windows::EventLoopExtWindows;


// structure holding everything else we'll need
#[derive(Debug)]
pub struct Cropper {
    pub(crate) display: Display,
    pub(crate) display_info: DisplayInfo,
}

// where we do the cool stuff
impl Cropper {
    pub fn get_all() -> Vec<Cropper> {
        let display_info_list = DisplayInfo::all().unwrap();
        println!("{:?}", display_info_list);
        let mut cropper_list = Vec::new();
        for display_info in display_info_list {
            let display = Display::new::<NotCurrent, ()>(
                WindowBuilder::new()
                    .with_title("Screenshot")
                    .with_visible(false)
                    .with_always_on_top(true)
                    .with_decorations(true)
                    .with_max_inner_size(PhysicalSize::new(display_info.width, display_info.height))
                    .with_min_inner_size(PhysicalSize::new(display_info.width, display_info.height))
                    .with_resizable(false),
                ContextBuilder::new().with_vsync(true),
                &glutin::event_loop::EventLoop::new_any_thread(),
            ).unwrap();
            let cropper = Cropper {
                display,
                display_info,
            };
            println!("{:?}", cropper);
            cropper_list.push(cropper)
        }

        cropper_list
    }
}