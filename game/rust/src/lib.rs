mod bindings;
use std::io::Write;

use bindings::{window, mesh::Mesh};
mod matrices;
mod dumbstuff;

extern "C" {
    pub fn window_create(width: u32, height: u32, title: *const u8);
}

fn window_create_bf(s: &mut dumbstuff::brainfuck::State, args: *const u8) {
    let arguments = dumbstuff::brainfuck::State::arguments(args, 3);
    let width: u32 = s.u32_from_idx(arguments[0] as usize);
    let height: u32 = s.u32_from_idx(arguments[1] as usize);
    let mut title: String = s.get_string(arguments[2] as usize);
    title.push('\0');
    unsafe { window_create(width, height, title.as_ptr()); }
}

#[no_mangle]
extern "C" fn game_main() {

    let mut state = dumbstuff::brainfuck::State::new("test_final.b");
    state.register_function("window_create", window_create_bf);
    while state.progchar() != '\0' {
        state.execute();
    }
    let mut memory: Vec<u8> = vec![];
    for i in 0..state.cell_count() {
        memory.push(state.get_cell(i));
    }

    let mut file = std::fs::File::create("memorydump.bin").unwrap();
    file.write_all(&memory[..]).unwrap();

    state.delete();

    let vertices: [f32; 18] = [
        0.0, 0.5, 0.0,    1.0, 0.0, 0.0,
        0.5, -0.5, 0.0,   0.0, 1.0, 0.0,
        -0.5, -0.5, 0.0,  0.0, 0.0, 1.0
    ];

    let mut mesh: Mesh = Mesh::new(&vertices, &[3, 3], "shaders/default.vert", "shaders/default.frag");
    mesh.model().translate(-0.25, 0.0, 0.0);
    // println!("{:#?}", mesh.model());

    while window::open() {
        window::frame_begin();
        
        mesh.draw();

        window::frame_end();
    }

    mesh.delete();
}
