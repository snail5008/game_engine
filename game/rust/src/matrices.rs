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

    pub fn zero() -> Self {
        Matrix4x4 {
            data: [
                0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ]
        }
    }

    pub fn new(data: [f32; 16]) -> Self {
        Matrix4x4 { data }
    }

    // idk how this works, but it does
    pub fn perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f: f32 = 1.0 / (fovy / 2.0).tan();
        let range_inv: f32 = 1.0 / (near - far);
        let m00: f32 = f / aspect;
        let m11: f32 = f;
        let m22: f32 = (near + far) * range_inv;
        let m23: f32 = -1.0;
        let m32: f32 = near * far * range_inv * 2.0;
    
        Self {
            data: [
                m00, 0.0, 0.0, 0.0,
                0.0, m11, 0.0, 0.0,
                0.0, 0.0, m22, m23,
                0.0, 0.0, m32, 0.0,
            ]
        }
    }
    
    pub fn data(&self) -> &[f32; 16] {
        &self.data
    }

    pub fn mut_data(&mut self) -> &mut [f32; 16] {
        &mut self.data
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

    pub fn mul(&self, rhs: &Self) -> Self {
        let mut matrix = Self::zero();
        for i in 0..4 {
            for j in 0..4 {
                matrix.mut_data()[i * 4 + j] = self.data()[i * 4] * rhs.data()[j] + self.data()[i * 4 + 1] * rhs.data()[j + 4] + self.data[i * 4 + 2] * rhs.data()[j + 8] + self.data()[i * 4 + 3] * rhs.data()[j + 12];
            }
        }
        matrix
    }

    // pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
    //     self.data[12] += x;
    //     self.data[13] += y;
    //     self.data[14] += z;
    // }

    pub fn set(&mut self, rhs: &Self) {
        for i in 0..16 {
            self.data[i] = rhs.data[i];
        }
    }

}
