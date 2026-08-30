use std::fs;
use std::sync::{LazyLock, RwLock, RwLockReadGuard, RwLockWriteGuard};

const CONFIG_PATH: &str = "game.conf";

#[derive(Clone, Copy, Debug)]
pub struct RuntimeConfig {
    pub fps_tracker_enabled: bool,
    pub vsync_enabled: bool,
    pub target_fps: u16,
    pub input_debug_enabled: bool,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            fps_tracker_enabled: true,
            vsync_enabled: true,
            target_fps: 60,
            input_debug_enabled: false,
        }
    }
}

impl RuntimeConfig {
    pub fn clamp(&mut self) {
        if self.target_fps == 0 {
            self.target_fps = 1;
        }
    }
}

pub static CONFIG: LazyLock<RwLock<RuntimeConfig>> =
    LazyLock::new(|| RwLock::new(RuntimeConfig::default()));

pub fn config() -> RwLockReadGuard<'static, RuntimeConfig> {
    CONFIG.read().expect("config lock poisoned")
}

pub fn config_mut() -> RwLockWriteGuard<'static, RuntimeConfig> {
    CONFIG.write().expect("config lock poisoned")
}

pub fn load_runtime_config() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(CONFIG_PATH).unwrap_or_else(|_| {
        let default = RuntimeConfig::default();
        let text = format!(
            "fps_tracker_enabled={}\nvsync_enabled={}\ntarget_fps={}\ninput_debug_enabled={}\n",
            default.fps_tracker_enabled,
            default.vsync_enabled,
            default.target_fps,
            default.input_debug_enabled
        );
        fs::write(CONFIG_PATH, &text).expect("failed to create default config");
        text
    });

    let mut cfg = RuntimeConfig::default();

    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let line = line.split('#').next().unwrap_or(line).trim();
        if line.is_empty() {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            continue;
        };

        match key.trim() {
            "fps_tracker_enabled" => {
                cfg.fps_tracker_enabled = value.trim().eq_ignore_ascii_case("true");
            }
            "vsync_enabled" => {
                cfg.vsync_enabled = value.trim().eq_ignore_ascii_case("true");
            }
            "target_fps" => {
                if let Ok(value) = value.trim().parse::<u16>() {
                    cfg.target_fps = value;
                }
            }
            "input_debug_enabled" => {
                cfg.input_debug_enabled = value.trim().eq_ignore_ascii_case("true");
            }
            _ => {}
        }
    }

    cfg.clamp();
    *CONFIG.write().expect("config lock poisoned") = cfg;
    Ok(())
}

pub fn save_runtime_config() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = *config();
    let text = format!(
        "fps_tracker_enabled={}\nvsync_enabled={}\ntarget_fps={}\ninput_debug_enabled={}\n",
        cfg.fps_tracker_enabled,
        cfg.vsync_enabled,
        cfg.target_fps,
        cfg.input_debug_enabled
    );
    fs::write(CONFIG_PATH, text)?;
    Ok(())
}

pub fn set_fps_tracker_enabled(enabled: bool) {
    config_mut().fps_tracker_enabled = enabled;
}

pub fn set_vsync_enabled(enabled: bool) {
    config_mut().vsync_enabled = enabled;
}

pub fn set_target_fps(fps: u16) {
    let mut cfg = config_mut();
    cfg.target_fps = fps.max(1);
}

pub fn set_input_debug_enabled(enabled: bool) {
    config_mut().input_debug_enabled = enabled;
}
