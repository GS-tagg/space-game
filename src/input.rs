use rayon::iter::repeat;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::{Duration, Instant};
use winit::{
    event::{ElementState, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

#[derive(Default)]
pub struct InputState {
    pressed_keys: HashSet<KeyCode>,
    new_pressed_keys: HashSet<KeyCode>,
    press_started_at: HashMap<KeyCode, Instant>,
    press_duration: HashMap<KeyCode, Duration>,
}

impl InputState {
    pub fn new() -> Self {
        Self::default()
    }
    //is keyboard event?
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
                        self.press_started_at.entry(*code).or_insert_with(Instant::now);
                    }
                    self.pressed_keys.insert(*code);
                }
                ElementState::Released => {
                    if self.pressed_keys.contains(code) {
                        let start = self.press_started_at.remove(code).unwrap_or_else(Instant::now);
                        self.press_duration.insert(*code, start.elapsed());
                    }
                    self.pressed_keys.remove(code);
                }
            }

            if crate::config::config().input_debug_enabled {
                if let Some(code) = self.pressed_keys.iter().find(|&&key| key == *code) {
                    let duration = self
                        .press_started_at
                        .get(code)
                        .map(|start| start.elapsed())
                        .unwrap_or_else(|| Duration::ZERO);
                    println!(
                        "input debug: key={:?} pressed={} duration={:.2}ms",
                        code,
                        true,
                        duration.as_secs_f32() * 1000.0
                    );
                }
            }

            return true;
        }

        false
    }

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

    pub fn key_duration(&self, key: KeyCode) -> Duration {
        if self.pressed_keys.contains(&key) {
            self.press_started_at
                .get(&key)
                .map(|start| start.elapsed())
                .unwrap_or_default()
        } else {
            self.press_duration.get(&key).copied().unwrap_or_default()
        }
    }
}
