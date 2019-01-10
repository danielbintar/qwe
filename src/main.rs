extern crate kiss3d;
extern crate nalgebra as na;

use na::{Vector3, UnitQuaternion};
use kiss3d::window::Window;
use kiss3d::light::Light;

fn main() {
    let mut window = Window::new("qwe");
    let mut cube   = window.add_cube(0.5, 0.5, 0.5);

    cube.set_color(1.0, 0.0, 0.0);

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    while window.render() {
        cube.prepend_to_local_rotation(&rot);
    }
}
