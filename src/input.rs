use rayon::iter::repeat;
use std::collections::HashSet;
use winit::{
    event::{
        self, ElementState, KeyEvent,
        WindowEvent::{self, KeyboardInput},
    },
    keyboard::{KeyCode, PhysicalKey},
};

#[derive(Default)]
pub struct InputState {
    pressed_keys: HashSet<KeyCode>,
    new_pressed_keys: HashSet<KeyCode>,
}

impl InputState {
    pub fn new() -> Self {
        Self::default()
    }
    //eturns true if the event was a keyboard event.
    pub fn update(&mut self, event: &WindowEvent) -> bool {
        if let WindowEvent::KeyboardInput {
            event:
                KeyEvent {
                    physical_key: PhysicalKey::Code(code),
                    state,
                    repeat,
                    ..
                },
            ..
        } = event
        {
            match state {
                ElementState::Pressed => {
                    if !*repeat {
                        self.new_pressed_keys.insert(*code);
                    }
                    self.pressed_keys.insert(*code);
                }
                ElementState::Released => {
                    self.pressed_keys.remove(code);
                }
            }
            true
        } else {
            false
        }
    }
    // clears 1 frame actions (call at the end of every frame)
    pub fn end_frame(&mut self) {
        self.new_pressed_keys.clear();
    }

    // is this key pressed?
    pub fn is_held(&self, key: KeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }

    // was the key pressed this frame?
    pub fn is_pressed(&self, key: KeyCode) -> bool {
        self.new_pressed_keys.contains(&key)
    }
}
