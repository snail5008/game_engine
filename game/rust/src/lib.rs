mod bindings;
mod matrices;
mod scripting;
mod camera;
mod transform;
use std::f32::consts::PI;
use bindings::window;
use matrices::Matrix4x4;

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
    
    let mut cam = camera::Camera::new(PI / 4.0, 800.0, 600.0);
    let mut mesh = bindings::mesh::Mesh::new(&vertices, &[3, 3], "shaders/default.vert", "shaders/default.frag");

    cam.mut_transform().translate((0.0, 0.0, -3.0));

    mesh.mut_transform().translate((0.0, 0.5, 0.0));
    mesh.mut_transform().set_scale((0.5, 0.5, 0.5));

    while window::open() {
        mesh.mut_transform().scale(((bindings::engine::time() * 10.0).sin() / 10.0, (bindings::engine::time() * 10.0).cos() / 50.0, (bindings::engine::time() * 10.0).cos() / 50.0));
        // let a = (bindings::engine::time() / 100.0).sin();
        mesh.mut_transform().rotate((0.01, 0.01, 0.01));

        window::frame_begin();

        cam.update_winsize(window::width(), window::height());
        
        mesh.draw(&cam);

        window::frame_end();
    }

    mesh.delete();
}
