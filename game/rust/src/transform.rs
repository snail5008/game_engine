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
            scale:       (1.0, 1.0, 1.0),
        }
    }
    pub fn generate_matrix(&self) -> Matrix4x4 {
        let mut matrix = Matrix4x4::identity();
        let mut translation = Matrix4x4::identity();
        let mut scale = Matrix4x4::identity();

        translation.mut_data()[12] = self.translation.0;
        translation.mut_data()[13] = self.translation.1;
        translation.mut_data()[14] = self.translation.2;

        scale.mut_data()[0] = self.scale.0;
        scale.mut_data()[5] = self.scale.1;
        scale.mut_data()[10] = self.scale.2;
        

        // made using google images of matrix rotations, I don't understand any of this
        // in the future, probably try to get a more intuitive understanding of matrix
        // transformations in general, also could most likely be made WAY more efficient,
        // so definitely come back to this entire function at some point, especially
        // if it becomes a bottleneck

        let cos_x = self.rotation.0.cos();
        let sin_x = self.rotation.0.sin();
        let cos_y = self.rotation.1.cos();
        let sin_y = self.rotation.1.sin();
        let cos_z = self.rotation.2.cos();
        let sin_z = self.rotation.2.sin();
        
        let mut x_rot = Matrix4x4::identity();
        x_rot.mut_data()[5] = cos_x;
        x_rot.mut_data()[6] = -sin_x;
        x_rot.mut_data()[9] = sin_x;
        x_rot.mut_data()[10] = cos_x;

        let mut y_rot = Matrix4x4::identity();
        y_rot.mut_data()[0] = cos_y;
        y_rot.mut_data()[2] = sin_y;
        y_rot.mut_data()[8] = -sin_y;
        y_rot.mut_data()[10] = cos_y;

        let mut z_rot = Matrix4x4::identity();
        z_rot.mut_data()[0] = cos_z;
        z_rot.mut_data()[1] = -sin_z;
        z_rot.mut_data()[4] = sin_z;
        z_rot.mut_data()[5] = cos_z;

        matrix.set(&matrix.mul(&scale).mul(&x_rot).mul(&y_rot).mul(&z_rot).mul(&translation));
        
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
    pub fn scale(&mut self, factor: (f32, f32, f32)) {
        self.scale.0 += factor.0;
        self.scale.1 += factor.1;
        self.scale.2 += factor.2;
    }
    pub fn set_scale(&mut self, factor: (f32, f32, f32)) {
        self.scale = factor;
    }
    pub fn rotate(&mut self, by: (f32, f32, f32)) {
        self.rotation.0 += by.0;
        self.rotation.1 += by.1;
        self.rotation.2 += by.2;
    }
}
