mod bindings;
mod matrices;
mod scripting;
mod camera;
mod transform;
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
    
    let mut cam = camera::Camera::new(PI / 4.0, 800.0, 600.0);
    let mut mesh = bindings::mesh::Mesh::new(&vertices, &[3, 3], "shaders/default.vert", "shaders/default.frag");

    cam.mut_transform().translate((0.0, 0.0, -3.0));
    mesh.mut_transform().translate((0.0, 1.0, 0.0));
    
    while window::open() {
        mesh.mut_transform().translate(((bindings::engine::time() * 10.0).sin() / 10.0, 0.0, 0.0));

        window::frame_begin();

        cam.update_winsize(window::width(), window::height());
        
        mesh.draw(&cam);

        window::frame_end();
    }

    mesh.delete();
}
