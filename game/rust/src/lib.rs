mod bindings;
use bindings::{window, mesh::Mesh};
mod matrices;

#[no_mangle]
extern "C" fn game_main() {

    let vertices: [f32; 18] = [
        0.0, 0.5, 0.0,    1.0, 0.0, 0.0,
        0.5, -0.5, 0.0,   0.0, 1.0, 0.0,
        -0.5, -0.5, 0.0,  0.0, 0.0, 1.0
    ];

    let mut mesh: Mesh = Mesh::new(&vertices, &[3, 3], "shaders/default.vert", "shaders/default.frag");
    mesh.model().translate(-0.25, 0.0, 0.0);
    println!("{:#?}", mesh.model());

    while window::open() {
        window::frame_begin();
        
        mesh.draw();

        window::frame_end();
    }

    mesh.delete();
}
