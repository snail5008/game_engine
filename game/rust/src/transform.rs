use crate::matrices::Matrix4x4;

pub struct Transform {
    translation: (f32, f32, f32),
    rotation:    (f32, f32, f32),
    scale:       (f32, f32, f32),
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            translation: (0.0, 0.0, 0.0),
            rotation:    (0.0, 0.0, 0.0),
            scale:       (0.0, 0.0, 0.0),
        }
    }
    pub fn generate_matrix(&self) -> Matrix4x4 {
        let mut matrix = Matrix4x4::identity();
        matrix.mut_data()[12] += self.translation.0;
        matrix.mut_data()[13] += self.translation.1;
        matrix.mut_data()[14] += self.translation.2;
        matrix
    }
    pub fn translate(&mut self, pos: (f32, f32, f32)) {
        self.translation.0 += pos.0;
        self.translation.1 += pos.1;
        self.translation.2 += pos.2;
    }
    pub fn set_translation(&mut self, pos: (f32, f32, f32)) {
        self.translation = pos;
    }
}
