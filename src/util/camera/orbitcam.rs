use crate::util::input_manager::{InputKind, InputManager};
use buttery::{Rotate, TransformComponent, Translate};
use glam::{Mat4, Quat, Vec3};
use std::ops::Neg;

#[derive(Debug)]
pub struct OrbitCam {
    pub up: TransformComponent<Rotate>,
    pub inclination: TransformComponent<Translate<f32>>,
    pub distance: TransformComponent<Translate<f32>>,
    // pub speed: TransformComponent<Translate<f32>>,
    pub target_height: TransformComponent<Translate<f32>>,
    pub min: TransformComponent<Translate<f32>>,
}

impl OrbitCam {
    pub fn drive(&mut self, time: f32) -> Mat4 {
        // self.speed.drive(time);

        let up = self.up.drive(time);
        let incl = self.inclination.drive(time);
        let dist = self.distance.drive(time);
        let height = self.target_height.drive(time);
        let min = self.min.drive(time);

        let arm = dist * Quat::from_rotation_x(incl).mul_vec3(Vec3::Z);
        let mut pos = Vec3::Y * height - arm;
        let pos_len = pos.length();
        if pos_len < min {
            pos.y += min - pos_len;
        }
        let cam =
            Mat4::from_rotation_translation(Quat::from_rotation_x(incl), pos);

        (Mat4::from_quat(up) * cam).inverse()
    }

    pub fn input(&mut self, input: &InputManager) {
        let (mut yaw, mut pitch) = (0.0, 0.0);

        if input.pressed(InputKind::ArrowUp) {
            pitch -= 1.0;
        }

        if input.pressed(InputKind::ArrowDown) {
            pitch += 1.0;
        }

        if input.pressed(InputKind::ArrowLeft) {
            yaw -= 1.0;
        }

        if input.pressed(InputKind::ArrowRight) {
            yaw += 1.0;
        }

        yaw *= 0.05;
        pitch *= 0.08;

        let mut delta_zoom = input.zoom;
        if input.pressed(InputKind::DirUp) {
            delta_zoom += 0.2;
        } else if input.pressed(InputKind::DirDown) {
            delta_zoom -= 0.2;
        }

        self.distance.target *= 1.0 + delta_zoom * 0.2;
        self.up.target = (self.up.target * Quat::from_rotation_y(yaw)).normalize();
        self.inclination.target =
            (self.inclination.target + pitch).clamp(0.0, std::f32::consts::FRAC_PI_2);

        let (mut up, mut right) = (0.0, 0.0);

        if input.pressed(InputKind::DirRight) {
            right -= 1.0;
        }

        if input.pressed(InputKind::DirLeft) {
            right += 1.0;
        }

        if input.pressed(InputKind::DirForward) {
            up += 1.0;
        }

        if input.pressed(InputKind::DirBack) {
            up -= 1.0;
        }

        let want_distance = self.distance.current;

        let speed_scl = 0.04 * ((want_distance.sqrt().neg().exp() + 1.0).recip() * 2.0 - 1.0);

        let axis = Vec3::Y.cross(Vec3::new(right * -speed_scl, 0.0, up * speed_scl));
        let len = axis.length();
        if len == 0.0 {
            return;
        }

        self.up.target = (self.up.target * Quat::from_scaled_axis(axis)).normalize();
    }

    pub fn target(&self) -> Vec3 {
        self.up.current.mul_vec3(Vec3::Y)
    }
}

impl Default for OrbitCam {
    fn default() -> Self {
        OrbitCam {
            up: TransformComponent::new_rotate(Quat::IDENTITY),
            inclination: TransformComponent::new_angle(0.0),
            distance: TransformComponent::new_zoom(4.0),
            target_height: TransformComponent::new(0.01, 6.0 * 0.7),
            min: TransformComponent::new(0.01, 6.0 * 0.7),
        }
    }
}
