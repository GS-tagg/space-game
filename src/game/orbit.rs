use crate::game::types::{Vector2D, Seconds};

pub struct KeplerOrbit {
    pub primary_body_entity: u64,
    pub semi_major_axis: f64,
    pub eccentricity: f64,
    pub orbital_period: f64,
    pub mean_anomaly_at_epoch: f64,
}

impl KeplerOrbit {
    /// TASK: Solve Kepler equation M = E - e*sin(E) to calculate position at current_time
    pub fn calculate_position(&self, _current_time: Seconds) -> Vector2D {
        todo!("Calculate 2D position along orbital ellipse")
    }
}
