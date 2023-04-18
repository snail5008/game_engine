pub mod engine {
    extern "C" {
        fn engine_read_file(filename: *const u8) -> *mut u8;
        fn engine_sum_u32(vector: *const u32, vector_element_count: u32) -> u32;
    }
    pub fn sum_u32(vector: &[u32]) -> u32 {
        unsafe { engine_sum_u32(vector.as_ptr(), vector.len() as u32) }
    }
    pub fn read_file(filename: &str) -> &str {
        let contents: *const u8 = unsafe { engine_read_file(filename.as_ptr()) };
        unsafe { std::ffi::CStr::from_ptr(contents as *const i8) }.to_str().unwrap()
    }
}

pub mod window {
    extern "C" {
        fn window_open() -> bool;
        fn window_frame_begin();
        fn window_frame_end();
    }

    pub fn open() -> bool {
        unsafe { window_open() }
    }

    pub fn frame_begin() {
        unsafe { window_frame_begin() }
    }

    pub fn frame_end() {
        unsafe { window_frame_end() }
    }
}

pub mod mesh {
    use std::ffi::c_void;
    use crate::bindings::engine;

    extern "C" {
        fn renderer_mesh_create(vertices: *const f32, vertex_count: u32, layout_location_count: u32, vertex_layout: *const u32, vertex_shader_path: *const u8, fragment_shader_path: *const u8) -> *mut c_void;
        fn renderer_mesh_destroy(mesh: *mut c_void);
        fn renderer_mesh_draw(mesh: *mut c_void);
    }

    pub struct Mesh {
        mesh: *mut c_void
    }

    impl Mesh {
        pub fn new(vertices: &[f32], vertex_layout: &[u32], vertex_shader_path: &str, fragment_shader_path: &str) -> Self {
            let mut vertex_shader_path: String = String::from(vertex_shader_path);
            let mut fragment_shader_path: String = String::from(fragment_shader_path);
            vertex_shader_path.push('\0');
            fragment_shader_path.push('\0');
            Mesh {
                mesh: unsafe { renderer_mesh_create(vertices.as_ptr(), vertices.len() as u32 / engine::sum_u32(&vertex_layout), vertex_layout.len() as u32, vertex_layout.as_ptr(), vertex_shader_path.as_ptr(), fragment_shader_path.as_ptr()) }
            }
        }
        pub fn delete(&self) {
            unsafe { renderer_mesh_destroy(self.mesh); }
        }
        pub fn draw(&self) {
            unsafe { renderer_mesh_draw(self.mesh); }
        }
    }
}
