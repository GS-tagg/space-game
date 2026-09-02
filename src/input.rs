use rayon::iter::repeat;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::{Duration, Instant};
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Default)]
pub struct InputState {
    // Keyboard tracking
    pressed_keys: HashSet<KeyCode>,
    new_pressed_keys: HashSet<KeyCode>,
    press_started_at: HashMap<KeyCode, Instant>,
    press_duration: HashMap<KeyCode, Duration>,

    // -- Mouse tracking 
    pressed_mouse_buttons: HashSet<MouseButton>,
    new_pressed_mouse_buttons: HashSet<MouseButton>,
    mouse_button_started_at: HashMap<MouseButton, Instant>,
    mouse_button_duration: HashMap<MouseButton, Duration>,

    cursor_position: PhysicalPosition<f64>,
    cursor_delta: (f64, f64),
    scroll_delta: (f32, f32),
}

impl InputState {
    pub fn new() -> Self {
        Self::default()
    }

    // Returns `true` if the event was handled by InputState.
    pub fn update(&mut self, event: &WindowEvent) -> bool {
        match event {
            // --- Keyboard Input ---
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state,
                        repeat,
                        ..
                    },
                ..
            } => {
                match state {
                    ElementState::Pressed => {
                        if !*repeat && !self.pressed_keys.contains(code) {
                            self.new_pressed_keys.insert(*code);
                            self.press_started_at.insert(*code, Instant::now());
                        }
                        self.pressed_keys.insert(*code);
                    }
                    ElementState::Released => {
                        if self.pressed_keys.contains(code) {
                            let start = self
                                .press_started_at
                                .remove(code)
                                .unwrap_or_else(Instant::now);
                            let duration = start.elapsed();
                            self.press_duration.insert(*code, duration);

                            if crate::config::config().input_debug_enabled {
                                println!(
                                    "input debug: key={:?} released after {:.2}ms",
                                    code,
                                    duration.as_secs_f32() * 1000.0
                                );
                            }
                        }
                        self.pressed_keys.remove(code);
                    }
                }
                true
            }

            // Mouse Buttons
            WindowEvent::MouseInput { state, button, .. } => {
                match state {
                    ElementState::Pressed => {
                        if !self.pressed_mouse_buttons.contains(button) {
                            self.new_pressed_mouse_buttons.insert(*button);
                            self.mouse_button_started_at.insert(*button, Instant::now());
                        }
                        self.pressed_mouse_buttons.insert(*button);
                    }
                    ElementState::Released => {
                        if self.pressed_mouse_buttons.contains(button) {
                            let start = self
                                .mouse_button_started_at
                                .remove(button)
                                .unwrap_or_else(Instant::now);
                            let duration = start.elapsed();
                            self.mouse_button_duration.insert(*button, duration);

                            if crate::config::config().input_debug_enabled {
                                println!(
                                    "input debug: mouse_button={:?} released after {:.2}ms",
                                    button,
                                    duration.as_secs_f32() * 1000.0
                                );
                            }
                        }
                        self.pressed_mouse_buttons.remove(button);
                    }
                }
                true
            }

            // Cursor Motion and Delta 
            WindowEvent::CursorMoved { position, .. } => {
                let dx = position.x - self.cursor_position.x;
                let dy = position.y - self.cursor_position.y;
                self.cursor_delta = (dx, dy);
                self.cursor_position = *position;
                true
            }

            // scrollwheel
            WindowEvent::MouseWheel { delta, .. } => {
                match delta {
                    MouseScrollDelta::LineDelta(x, y) => {
                        self.scroll_delta = (*x, *y);
                    }
                    MouseScrollDelta::PixelDelta(pos) => {
                        self.scroll_delta = (pos.x as f32, pos.y as f32);
                    }
                }
                true
            }

            _ => false,
        }
    }

    pub fn end_frame(&mut self) {
        self.new_pressed_keys.clear();
        self.new_pressed_mouse_buttons.clear();
        self.cursor_delta = (0.0, 0.0);
        self.scroll_delta = (0.0, 0.0);
    }

    // prints mouse input
    pub fn debug(&self) {
        println!("--- InputState Debug Snapshot ---");
        println!("Pressed Keys: {:?}", self.pressed_keys);
        for key in &self.pressed_keys {
            println!("  Key {:?} held for {:.2}ms", key, self.key_duration(*key).as_secs_f32() * 1000.0);
        }

        println!("Pressed Mouse Buttons: {:?}", self.pressed_mouse_buttons);
        for button in &self.pressed_mouse_buttons {
            println!("  Mouse {:?} held for {:.2}ms", button, self.mouse_button_duration(*button).as_secs_f32() * 1000.0);
        }

        println!("Cursor Position: ({:.1}, {:.1})", self.cursor_position.x, self.cursor_position.y);
        println!("Cursor Delta: ({:.2}, {:.2})", self.cursor_delta.0, self.cursor_delta.1);
        println!("Scroll Delta: ({:.2}, {:.2})", self.scroll_delta.0, self.scroll_delta.1);
        println!("---------------------------------");
    }

    // keyboard
    pub fn is_held(&self, key: KeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }

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

    // mouse
    pub fn is_mouse_held(&self, button: MouseButton) -> bool {
        self.pressed_mouse_buttons.contains(&button)
    }

    pub fn is_mouse_pressed(&self, button: MouseButton) -> bool {
        self.new_pressed_mouse_buttons.contains(&button)
    }

    pub fn mouse_button_duration(&self, button: MouseButton) -> Duration {
        if self.pressed_mouse_buttons.contains(&button) {
            self.mouse_button_started_at
                .get(&button)
                .map(|start| start.elapsed())
                .unwrap_or_default()
        } else {
            self.mouse_button_duration
                .get(&button)
                .copied()
                .unwrap_or_default()
        }
    }

    pub fn cursor_position(&self) -> PhysicalPosition<f64> {
        self.cursor_position
    }

    pub fn cursor_delta(&self) -> (f64, f64) {
        self.cursor_delta
    }

    pub fn scroll_delta(&self) -> (f32, f32) {
        self.scroll_delta
    }
}