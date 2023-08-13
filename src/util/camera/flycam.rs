use crate::util::input_manager::{InputKind, InputManager};
use buttery::{Scaffold, TransformComponent, Translate};
use glam::{Mat4, Quat, Vec2, Vec3};

pub struct FlyCamera {
    position: TransformComponent<Translate<Vec3>>,
    rotate: TransformComponent<Translate<Vec2>>,
    pub speed: TransformComponent<Translate<f32>>,
}

impl FlyCamera {
    pub fn drive(&mut self, time: f32) -> Mat4 {
        self.speed.drive(time);

        self.rotate
            .begin(|rot| Mat4::from_rotation_y(rot.y) * Mat4::from_rotation_x(rot.x))
            .and_then(&mut self.position, Mat4::from_translation)
            .drive(time)
            .inverse()
    }

    pub fn input(&mut self, input: &InputManager) {
        self.speed.target *= 1.0 - input.zoom / 5.0;
        let speed = self.speed.current;
        let rotation = self.rotate.current;
        let rotation = Quat::from_rotation_y(rotation.y);

        let mut move_delta = Vec3::ZERO;
        if input.pressed(InputKind::DirForward) {
            move_delta += rotation.mul_vec3(Vec3::Z);
        }
        if input.pressed(InputKind::DirBack) {
            move_delta += rotation.mul_vec3(-Vec3::Z);
        }
        if input.pressed(InputKind::DirRight) {
            move_delta += rotation.mul_vec3(Vec3::X);
        }
        if input.pressed(InputKind::DirLeft) {
            move_delta += rotation.mul_vec3(-Vec3::X);
        }
        if input.pressed(InputKind::DirUp) {
            move_delta += rotation.mul_vec3(Vec3::Y);
        }
        if input.pressed(InputKind::DirDown) {
            move_delta += rotation.mul_vec3(-Vec3::Y);
        }

        move_delta *= speed;
        self.position.target += move_delta;

        let off = 0.04;

        let mut rotate = self.rotate.target;
        if input.pressed(InputKind::ArrowUp) {
            rotate.x = (rotate.x - off).max(-std::f32::consts::FRAC_PI_2);
        }
        if input.pressed(InputKind::ArrowDown) {
            rotate.x = (rotate.x + off).min(std::f32::consts::FRAC_PI_2);
        }
        if input.pressed(InputKind::ArrowRight) {
            rotate.y += off;
        }
        if input.pressed(InputKind::ArrowLeft) {
            rotate.y -= off;
        }
        self.rotate.target = rotate;
    }
}

impl Default for FlyCamera {
    fn default() -> Self {
        FlyCamera {
            position: TransformComponent::new(0.001, Vec3::new(0.0, 0.0, -8.0)),
            rotate: TransformComponent::new(0.001, Vec2::ZERO),
            speed: TransformComponent::new_zoom(0.4),
        }
    }
}
