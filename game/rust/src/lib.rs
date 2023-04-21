mod bindings;
use bindings::{window, mesh::Mesh};
mod matrices;
mod dumbstuff;

#[no_mangle]
extern "C" fn game_main() {

    let mut state = dumbstuff::brainfuck::State::new("test.b");
    while state.progchar() != '\0' {
        state.execute();
    }
    state.delete();

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
