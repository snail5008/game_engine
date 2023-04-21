#![allow(dead_code)]

pub mod engine {
    use std::ffi::c_void;

    extern "C" {
        fn engine_read_file(filename: *const u8) -> *mut u8;
        fn engine_sum_u32(vector: *const u32, vector_element_count: u32) -> u32;
        pub fn free(ptr: *mut c_void);
        pub fn malloc(size: usize) -> *mut c_void;
        fn glfwGetTime() -> f64;
    }
    pub fn sum_u32(vector: &[u32]) -> u32 {
        unsafe { engine_sum_u32(vector.as_ptr(), vector.len() as u32) }
    }
    pub fn cstr_to_rust(string: *const u8) -> &'static str {
        unsafe { std::ffi::CStr::from_ptr(string as *const i8) }.to_str().unwrap()
    }
    // needs to be freed!
    pub fn ruststr_to_c(string: &str) -> *mut u8 {
        let c_str: *mut u8 = unsafe { malloc(string.len() + 1) } as *mut u8;
        let mut i = 0;
        while i < string.len() {
            unsafe { *c_str.add(i) = string.as_bytes()[i] as u8 };
            i += 1
        }
        unsafe { *(c_str.add(i)) = 0 };
        c_str
    }
    // needs to be freed!
    pub fn read_file(filename: &str) -> &str {
        let c_filename = ruststr_to_c(filename);
        let contents: *const u8 = unsafe { engine_read_file(c_filename) };
        unsafe { free(c_filename as *mut c_void); }
        cstr_to_rust(contents)
    }
    pub fn time() -> f32 {
        unsafe { glfwGetTime() as f32 }
    }
}

pub mod window {
    extern "C" {
        fn window_open() -> bool;
        fn window_frame_begin();
        fn window_frame_end();
        fn window_width() -> u32;
        fn window_height() -> u32;
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

    pub fn width() -> f32 {
        unsafe { window_width() as f32 }
    }

    pub fn height() -> f32 {
        unsafe { window_height() as f32 }
    }
}

pub mod mesh {
    use std::ffi::c_void;
    use crate::bindings::engine;
    use crate::matrices::Matrix4x4;

    extern "C" {
        fn renderer_mesh_create(vertices: *const f32, vertex_count: u32, layout_location_count: u32, vertex_layout: *const u32, vertex_shader_path: *const u8, fragment_shader_path: *const u8) -> *mut c_void;
        fn renderer_mesh_destroy(mesh: *mut c_void);
        fn renderer_mesh_draw(mesh: *mut c_void, model: *const f32, view: *const f32, projection: *const f32);
    }

    pub struct Mesh {
        mesh: *mut c_void,
        model: Matrix4x4,
    }

    impl Mesh {
        pub fn new(vertices: &[f32], vertex_layout: &[u32], vertex_shader_path: &str, fragment_shader_path: &str) -> Self {
            let mut vertex_shader_path: String = String::from(vertex_shader_path);
            let mut fragment_shader_path: String = String::from(fragment_shader_path);
            vertex_shader_path.push('\0');
            fragment_shader_path.push('\0');
            Mesh {
                mesh: unsafe { renderer_mesh_create(vertices.as_ptr(), vertices.len() as u32 / engine::sum_u32(&vertex_layout), vertex_layout.len() as u32, vertex_layout.as_ptr(), vertex_shader_path.as_ptr(), fragment_shader_path.as_ptr()) },
                model: Matrix4x4::identity()
            }
        }
        pub fn delete(&self) {
            unsafe { renderer_mesh_destroy(self.mesh); }
        }
        pub fn draw(&self, cam: &crate::camera::Camera) {
            unsafe { renderer_mesh_draw(self.mesh, self.model.as_ptr(), cam.view().as_ptr(), cam.projection().as_ptr()); }
        }
        pub fn model(&mut self) -> &mut Matrix4x4 {
            &mut self.model
        }
    }
}
