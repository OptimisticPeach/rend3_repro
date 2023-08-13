use std::collections::HashSet;
use winit::event::{ElementState, KeyboardInput, MouseScrollDelta, VirtualKeyCode};

#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
pub enum InputKind {
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    DirForward,
    DirRight,
    DirLeft,
    DirBack,
    DirDown,
    DirUp,
    CamSwitch,
}

#[derive(Default)]
pub struct InputManager {
    pub pressed: HashSet<InputKind>,
    pub just_pressed: HashSet<InputKind>,
    pub just_released: HashSet<InputKind>,
    pub zoom: f32,
    pub escape_requested: bool,
}

impl InputManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn key_event(&mut self, event: KeyboardInput) {
        use InputKind::*;

        let code = match event.virtual_keycode {
            Some(x) => x,
            _ => return,
        };

        let code = match code {
            VirtualKeyCode::Up => ArrowUp,
            VirtualKeyCode::Down => ArrowDown,
            VirtualKeyCode::Left => ArrowLeft,
            VirtualKeyCode::Right => ArrowRight,
            VirtualKeyCode::W => DirForward,
            VirtualKeyCode::D => DirRight,
            VirtualKeyCode::S => DirBack,
            VirtualKeyCode::A => DirLeft,
            VirtualKeyCode::LShift | VirtualKeyCode::RShift => DirDown,
            VirtualKeyCode::Space => DirUp,
            VirtualKeyCode::C => CamSwitch,
            VirtualKeyCode::Escape => {
                self.escape_requested = true;
                return;
            }
            _ => return,
        };

        if event.state == ElementState::Pressed {
            self.pressed.insert(code);
            self.just_pressed.insert(code);
        } else {
            self.pressed.remove(&code);
            self.just_released.insert(code);
        }
    }

    pub fn zoom_event(&mut self, event: MouseScrollDelta) {
        self.zoom -= match event {
            MouseScrollDelta::LineDelta(_, lines) => lines / 5.0,
            MouseScrollDelta::PixelDelta(pos) => (pos.y / 3.0) as f32,
        };
    }

    pub fn pressed(&self, key: InputKind) -> bool {
        self.pressed.contains(&key)
    }

    pub fn just_pressed(&self, key: InputKind) -> bool {
        self.just_pressed.contains(&key)
    }

    pub fn just_released(&self, key: InputKind) -> bool {
        self.just_released.contains(&key)
    }

    pub fn reset_frame(&mut self) {
        self.zoom = 0.0;
        self.just_pressed.clear();
        self.just_released.clear();
    }
}
