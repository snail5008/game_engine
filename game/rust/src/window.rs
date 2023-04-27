
use crate::bindings::*;
use std::{ptr::null_mut};

pub struct Window {
    initialised: bool,
    window: *mut GLFWwindow
}

trait ToCStr {
    fn c_string(&self) -> *const i8;
}

// pub fn ruststr_to_c(string: &str) -> *mut u8 {
//     let mut c_str = Vec::with_capacity(string.len() + 1);
//     c_str.extend(string.bytes());
//     c_str.push(0);
//     let ptr = c_str.as_mut_ptr();
//     std::mem::forget(c_str);
//     ptr
// }

impl ToCStr for &str {
    fn c_string(&self) -> *const i8 {
        let mut c_str: Vec<u8> = Vec::with_capacity(self.len() + 1);
        c_str.extend(self.bytes());
        c_str.push(0);
        let ptr = c_str.as_mut_ptr();
        std::mem::forget(c_str);
        ptr as *const i8
    }
}

impl Window {
    pub fn new(size: (u32, u32), title: &str) -> Result<Self, &str> {
        if unsafe { glfwInit() } == 0 {
            return Err("Failed to initialise GLFW");
        }
        
        unsafe { glfwWindowHint(GLFW_OPENGL_PROFILE as i32, GLFW_OPENGL_CORE_PROFILE as i32) };
        unsafe { glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR as i32, 3 as i32) };
        unsafe { glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR as i32, 3 as i32) };
        let window = unsafe { glfwCreateWindow(
            size.0 as i32,
            size.1 as i32,
            title.c_string(),
            null_mut::<GLFWmonitor>(),
            null_mut::<GLFWwindow>()
        ) };

        if window == null_mut::<GLFWwindow>() {
            return Err("Failed to create window");
        }

        unsafe { glfwMakeContextCurrent(window) };

        if unsafe { gladLoadGL() } == 0 {
            return Err("Failed to load OpenGL");
        }

        unsafe { glfwSwapInterval(0) };
        unsafe { crate::bindings::glad_glEnable(GL_DEPTH_TEST) };

        Ok(Window {
            initialised: true,
            window,
        })
    }
    pub fn delete(&self) {
        unsafe { glfwDestroyWindow(self.window) };
        unsafe { glfwTerminate() };
    }
}
