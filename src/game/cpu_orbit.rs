use crate::game::types::{Seconds, Vector2D};

pub struct KeplerOrbit {
    pub semi_major_axis: f64,        // orbit size, in sim distance units
    pub eccentricity: f64,           // 0 = circular, closer to 1 = more elongated
    pub mean_anomaly_at_epoch: f64,  // orbit "position" at t0 — MUST be rebased alongside t0
    pub gravity_factor: f64,         // Gravity of parent body (gravitational parameter)
    pub t0: Seconds,                 // reference epoch; rebase periodically to avoid f32 precision loss in (current_time - t0)
    pub omega: f64,                  // periapsis argument
}

impl KeplerOrbit {
    // Solve Kepler equation M = E - e*sin(E) to calculate position at current_time
    pub fn calculate_position_cpu(&self, _current_time: Seconds) -> Vector2D {
        let n = (self.gravity_factor / self.semi_major_axis.powi(3)).sqrt();
        let m: f64 = self.mean_anomaly_at_epoch + n * (_current_time.0 - self.t0.0);
        let mut e_anomaly = m;
        for _ in 0..5 {
            let f = e_anomaly - self.eccentricity * e_anomaly.sin() - m;
            let f_prime = 1.0 - self.eccentricity * e_anomaly.cos();
            let delta = f / f_prime;
            e_anomaly -= delta;
            if delta.abs() < 1e-6 {
                break;
            }
        }
        let beta = self.eccentricity / (1.0 + (1.0 - self.eccentricity * self.eccentricity).sqrt());
        let true_anom = e_anomaly + 2.0 * (beta * e_anomaly.sin() / (1.0 - beta * e_anomaly.cos())).atan();
        let r = self.semi_major_axis * (1.0 - self.eccentricity * e_anomaly.cos());

        let x_orb: f64 = r * true_anom.cos();
        let y_orb: f64 = r * true_anom.sin();
        let x = x_orb * self.omega.cos() - y_orb * self.omega.sin();
        let y = x_orb * self.omega.sin() + y_orb * self.omega.cos();

        Vector2D { x, y }
    }
}
