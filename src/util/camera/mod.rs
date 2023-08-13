use buttery::TransformComponent;
use crate::util::input_manager::{InputKind, InputManager};
use glam::Mat4;

pub mod flycam;
pub mod orbitcam;

pub enum Mode {
    Fly,
    Orbit,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Orbit
    }
}

// #[derive(Default)]
pub struct Camera {
    pub mode: Mode,
    pub flycam: flycam::FlyCamera,
    pub orbcam: orbitcam::OrbitCam,
}

impl Default for Camera {
    fn default() -> Self {
        use buttery::TransformComponent;
        use glam::Quat;

        Self {
            mode: Mode::default(),
            flycam: flycam::FlyCamera::default(),
            orbcam: orbitcam::OrbitCam {
                up: TransformComponent::new(0.04, Quat::from_xyzw(
                    -0.1463432,
                    -0.87882996,
                    0.3139044,
                    0.3281854,
                )),
                inclination: TransformComponent::new(0.04, 0.93079555),
                distance: TransformComponent::new(
                    0.03,
                    6.1281543,
                ),
                target_height: TransformComponent::new(
                    0.01,
                    6.1639004,
                ),
                min: TransformComponent::new(
                    0.01,
                    4.2,
                ),
            }
        }
    }
}

impl Camera {
    pub fn drive(&mut self, time: f32) -> Mat4 {
        match self.mode {
            Mode::Fly => self.flycam.drive(time),
            Mode::Orbit => self.orbcam.drive(time),
        }
    }

    pub fn input(&mut self, input: &InputManager) {
        if input.just_pressed(InputKind::CamSwitch) {
            match self.mode {
                Mode::Fly => self.mode = Mode::Orbit,
                Mode::Orbit => self.mode = Mode::Fly,
            }
        }

        match self.mode {
            Mode::Fly => self.flycam.input(input),
            Mode::Orbit => self.orbcam.input(input),
        }
    }
}
