mod bindings;
use bindings::{window, mesh::Mesh};

#[no_mangle]
extern "C" fn game_main() {
    let vertices: [f32; 15] = [
        0.0, 1.0,    1.0, 0.0, 0.0,
        1.0, -1.0,   0.0, 1.0, 0.0,
        -1.0, -1.0,  0.0, 0.0, 1.0
    ];

    let mesh: Mesh = Mesh::new(&vertices, &[2, 3], "shaders/default.vert", "shaders/default.frag");

    while window::open() {
        window::frame_begin();
        
        mesh.draw();

        window::frame_end();
    }

    mesh.delete();
}
