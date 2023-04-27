pub mod bindings;
mod matrices;
mod scripting;
mod camera;
mod transform;
mod ui;
mod window;
use std::f32::consts::PI;

#[no_mangle]
extern "C" fn main() -> i32 {
    // vertices from learnopengl.com
    let vertices: [f32; 216] = [
        -0.5, -0.5, -0.5,   0.0, 0.0, 0.0,
        0.5, -0.5, -0.5,    0.0, 0.0, 0.0,
        0.5,  0.5, -0.5,    0.0, 0.0, 0.0,
        0.5,  0.5, -0.5,    0.0, 0.0, 0.0,
        -0.5,  0.5, -0.5,   0.0, 0.0, 0.0,
        -0.5, -0.5, -0.5,   0.0, 0.0, 0.0,

        -0.5, -0.5,  0.5,   0.0, 0.0, 1.0,
        0.5, -0.5,  0.5,    0.0, 0.0, 1.0,
        0.5,  0.5,  0.5,    0.0, 0.0, 1.0,
        0.5,  0.5,  0.5,    0.0, 0.0, 1.0,
        -0.5,  0.5,  0.5,   0.0, 0.0, 1.0,
        -0.5, -0.5,  0.5,   0.0, 0.0, 1.0,

        -0.5,  0.5,  0.5,   0.0, 1.0, 0.0,
        -0.5,  0.5, -0.5,   0.0, 1.0, 0.0,
        -0.5, -0.5, -0.5,   0.0, 1.0, 0.0,
        -0.5, -0.5, -0.5,   0.0, 1.0, 0.0,
        -0.5, -0.5,  0.5,   0.0, 1.0, 0.0,
        -0.5,  0.5,  0.5,   0.0, 1.0, 0.0,

        0.5,  0.5,  0.5,    0.0, 1.0, 1.0,
        0.5,  0.5, -0.5,    0.0, 1.0, 1.0,
        0.5, -0.5, -0.5,    0.0, 1.0, 1.0,
        0.5, -0.5, -0.5,    0.0, 1.0, 1.0,
        0.5, -0.5,  0.5,    0.0, 1.0, 1.0,
        0.5,  0.5,  0.5,    0.0, 1.0, 1.0,

        -0.5, -0.5, -0.5,   1.0, 0.0, 0.0,
        0.5, -0.5, -0.5,    1.0, 0.0, 0.0,
        0.5, -0.5,  0.5,    1.0, 0.0, 0.0,
        0.5, -0.5,  0.5,    1.0, 0.0, 0.0,
        -0.5, -0.5,  0.5,   1.0, 0.0, 0.0,
        -0.5, -0.5, -0.5,   1.0, 0.0, 0.0,

        -0.5,  0.5, -0.5,   1.0, 0.0, 1.0,
        0.5,  0.5, -0.5,    1.0, 0.0, 1.0,
        0.5,  0.5,  0.5,    1.0, 0.0, 1.0,
        0.5,  0.5,  0.5,    1.0, 0.0, 1.0,
        -0.5,  0.5,  0.5,   1.0, 0.0, 1.0,
        -0.5,  0.5, -0.5,   1.0, 0.0, 1.0,
    ];
    
    let window = window::Window::new((800, 600), "Game").unwrap();
    window.delete();

    // // NOTE: CHECK HOW DELTA TIME WORKS IN GODOT

    // let mut cam = camera::Camera::new(PI / 4.0, 800.0, 600.0);
    // cam.mut_transform().translate((0.0, 0.0, -10.0));

    // let mut weird_shape = bindings::mesh::Mesh::new(&vertices, &[3, 3], "shaders/default.vert", "shaders/default.frag");
    // weird_shape.mut_transform().set_scale((0.5, 0.5, 0.5));

    // let mut floor = bindings::mesh::Mesh::new(&vertices, &[3, 3], "shaders/default.vert", "shaders/default.frag");
    // floor.mut_transform().set_scale((10.0, 0.1, 10.0));
    // floor.mut_transform().set_translation((0.0, -5.0, 0.0));

    // weird_shape.mut_transform().set_scale(((((bindings::engine::time() * 10.0).sin() + 1.5)), (((bindings::engine::time() * 20.0).cos() + 1.5) / 4.0), (((bindings::engine::time() * 20.0).cos() + 1.5) / 4.0)));
    // while window::open() {
    //     // weird_shape.mut_transform().set_rotation((bindings::engine::time(), bindings::engine::time(), bindings::engine::time()));
    //     weird_shape.mut_transform().scale((100.0 * window::frame_time() as f32, 0.0, 0.0));

    //     // println!("{}", (((bindings::engine::time() * 10.0).sin() + 1.0)));
    //     println!("{}", window::frame_time());

    //     window::frame_begin();
    //     cam.update_winsize(window::width(), window::height());
        
    //     weird_shape.draw(&cam);

    //     floor.draw(&cam);

    //     window::frame_end();
    // }

    // weird_shape.delete();
    // floor.delete();

    0
}
