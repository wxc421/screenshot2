mod cropper;


#[cfg(test)]
mod tests {
    use std::{thread, time};
    use display_info::DisplayInfo;
    use glium::{Display, glutin};
    use glium::glutin::ContextBuilder;
    use glium::glutin::dpi::{LogicalPosition, LogicalSize, PhysicalPosition, Position};
    use glium::glutin::dpi::Position::Logical;
    use glium::glutin::window::WindowBuilder;
    use crate::cropper::Cropper;
    use super::*;

    #[test]
    fn b() {
        let cropper_list = Cropper::get_all();
        for cropper in cropper_list {
            // cropper.display.gl_window().window().set_outer_position(LogicalPosition::new(cropper.display_info.x, cropper.display_info.y));
            println!("cropper.display_info.x:{},cropper.display_info.y:{}", cropper.display_info.x, cropper.display_info.y);
            cropper.display.gl_window().window().set_outer_position(PhysicalPosition::new(cropper.display_info.x, cropper.display_info.y));
            cropper.display.gl_window().window().set_inner_size(LogicalSize::new(cropper.display_info.width, cropper.display_info.height));
            cropper.display.gl_window().window().set_max_inner_size(Some(LogicalSize::new(cropper.display_info.width, cropper.display_info.height)));
            cropper.display.gl_window().window().set_min_inner_size(Some(LogicalSize::new(cropper.display_info.width, cropper.display_info.height)));
            cropper.display.gl_window().window().set_visible(true);
        }
        let ten_millis = time::Duration::from_secs(10);
        thread::sleep(ten_millis);
    }
}
