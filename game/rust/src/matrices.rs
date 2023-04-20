#![allow(dead_code)]

#[derive(Debug)]
pub struct Matrix4x4 {
    data: [f32; 16]
}

impl Matrix4x4 {
    pub fn identity() -> Self {
        Matrix4x4 {
            data: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ]
        }
    }

    pub fn as_ptr(&self) -> *const f32 {
        self.data.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut f32 {
        self.data.as_mut_ptr()
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.data[12] += x;
        self.data[13] += y;
        self.data[14] += z;
    }
}
