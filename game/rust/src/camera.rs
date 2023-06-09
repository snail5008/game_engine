#![allow(dead_code)]

use crate::transform::Transform;

pub struct Camera {
    fov: f32,
    aspect: f32,
    near: f32,
    far: f32,
    transform: crate::transform::Transform,
    view: crate::matrices::Matrix4x4,
    projection: crate::matrices::Matrix4x4,
}

impl Camera {
    pub fn new(fov: f32, width: f32, height: f32) -> Self {
        Self {
            view: crate::matrices::Matrix4x4::identity(),
            transform: crate::transform::Transform::new(),
            projection: crate::matrices::Matrix4x4::perspective(fov, width / height, 0.1, 100.0),
            near: 0.1,
            far: 100.0,
            aspect: width / height,
            fov,
        }
    }

    pub fn update_winsize(&mut self, width: f32, height: f32) {
        self.aspect = width / height;
        self.projection.set(&crate::matrices::Matrix4x4::perspective(self.fov, self.aspect, self.near, self.far));
    }

    // pub fn view_mut(&mut self) -> &mut crate::matrices::Matrix4x4 {
    //     &mut self.view
    // }

    // pub fn view(&self) -> &crate::matrices::Matrix4x4 {
    //     &self.view
    // }

    pub fn mut_transform(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn projection_mut(&mut self) -> &mut crate::matrices::Matrix4x4 {
        &mut self.projection
    }

    pub fn projection(&self) -> &crate::matrices::Matrix4x4 {
        &self.projection
    }
}
