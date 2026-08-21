use rayon::prelude::*;
use crate::game::cpu_orbit::KeplerOrbit;
use crate::game::types::{Vector2D, Seconds};

pub struct ParallelSystemRunner;

impl ParallelSystemRunner {
    /// TASK: Dispatch orbital update across CPU thread pool with Rayon
    pub fn update_all_orbits(_orbits: &mut [(KeplerOrbit, Vector2D)], _current_time: Seconds) {
        todo!("Process Kepler orbits concurrently using Rayon")
    }
}
