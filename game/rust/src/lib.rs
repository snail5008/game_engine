mod bindings;
mod matrices;
mod scripting;
mod camera;
mod transform;
mod ui;
use std::f32::consts::PI;
use bindings::window;

extern "C" {
    pub fn window_create(width: u32, height: u32, title: *const u8);
}

#[no_mangle]
extern "C" fn game_main() {
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

    extern "C" {
        fn rand() -> i32;
        fn srand(s: u32);
    }
    
    unsafe { srand(0); }
    let mut shape_positions = Vec::new();
    for i in 0..100 {
        let x = (unsafe { rand() } as f32 / std::u32::MAX as f32 - 0.25) * 30.0;
        let y = (unsafe { rand() } as f32 / std::u32::MAX as f32 - 0.25) * 30.0;
        let z = (unsafe { rand() } as f32 / std::u32::MAX as f32 - 0.25) * 30.0;
        shape_positions.push((x, y, z));
    }
    
    let mut cam = camera::Camera::new(PI / 4.0, 800.0, 600.0);
    cam.mut_transform().translate((0.0, 0.0, -10.0));

    let mut weird_shape = bindings::mesh::Mesh::new(&vertices, &[3, 3], "shaders/default.vert", "shaders/default.frag");
    weird_shape.mut_transform().set_scale((0.5, 0.5, 0.5));

    let mut floor = bindings::mesh::Mesh::new(&vertices, &[3, 3], "shaders/default.vert", "shaders/default.frag");
    floor.mut_transform().set_scale((10.0, 0.1, 10.0));
    floor.mut_transform().set_translation((0.0, -5.0, 0.0));

    while window::open() {
        weird_shape.mut_transform().set_scale(((((bindings::engine::time() * 10.0).sin() + 1.5)), (((bindings::engine::time() * 20.0).cos() + 1.5) / 4.0), (((bindings::engine::time() * 20.0).cos() + 1.5) / 4.0)));
        weird_shape.mut_transform().set_rotation((bindings::engine::time(), bindings::engine::time(), bindings::engine::time()));

        println!("{}", (((bindings::engine::time() * 10.0).sin() + 1.0)));

        window::frame_begin();
        cam.update_winsize(window::width(), window::height());
        
        for i in 0..shape_positions.len() {
            weird_shape.mut_transform().set_translation(shape_positions[i]);
            weird_shape.draw(&cam);
        }
        for i in 0..shape_positions.len() {
            weird_shape.mut_transform().set_translation((-shape_positions[i].0, -shape_positions[i].1, -shape_positions[i].2))
        }

        floor.draw(&cam);

        window::frame_end();
    }

    weird_shape.delete();
    floor.delete();
}
