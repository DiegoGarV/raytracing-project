use std::f32::consts::PI;

use nalgebra_glm::Vec3;

pub struct Camera {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
    pub has_changed: bool
}

impl Camera {
    pub fn basis_change(&self, vector: &Vec3) -> Vec3{
        let forward: Vec3= (self.center - self.eye).normalize();
        let right: Vec3 = forward.cross(&self.up).normalize();
        let up: Vec3 = right.cross(&forward).normalize();
        let rotated = 
        vector.x*right +
        vector.y*up-
        vector.z*forward;
        rotated.normalize()
    }
    pub fn zoom(&mut self, delta: f32) {
        let direction = (self.center - self.eye).normalize();
        self.eye += direction * delta;
        self.has_changed = true;
    }
    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32){
        let radius_vec = self.eye-self.center;
        let radius = radius_vec.magnitude();
        let current_yaw = radius_vec.z.atan2(radius_vec.x);
        let new_yaw =(current_yaw+delta_yaw)%(2.0*PI);

        let projection_xy = (radius_vec.x*radius_vec.x + radius_vec.z*radius_vec.z).sqrt();
        let current_pitch = (-radius_vec.y).atan2(projection_xy);
        let new_pitch = (current_pitch+delta_pitch).clamp(-PI/2.0 + 0.1, PI/2.0 -0.1);
        
        let new_eye = self.center + Vec3::new(
            radius*new_yaw.cos() * new_pitch.cos(),
            -radius*new_pitch.sin(),
            radius*new_yaw.sin()*new_pitch.cos()
        );
        self.eye = new_eye;
        self.has_changed = true;
    }

    pub fn check_if_change(&mut self)-> bool{
        if self.has_changed {
            self.has_changed = false;
            true
        } else{
            false
        }
    }

}