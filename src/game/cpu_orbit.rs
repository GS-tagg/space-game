use crate::game::types::Seconds;
use crate::math::Vector2D;

pub struct KeplerOrbit {
    pub semi_major_axis: f32,       // orbit size, in sim distance units
    pub eccentricity: f32,          // 0 = circular, closer to 1 = more elongated
    pub mean_anomaly_at_epoch: f32, // orbit "position" at t0 — MUST be rebased alongside t0
    pub gravity_factor: f32,        // gravity of parent body (gravitational parameter)
    pub t0: u32, // reference epoch in hours; safe as integer up to ~1000 years at day resolution
    pub omega: f32, // periapsis argument (angle, radians)
}

impl KeplerOrbit {
    // Solve Kepler equation M = E - e*sin(E) to calculate position at current_time
    pub fn calculate_position(&self, current_time: u32) -> Vector2D {
        let delta_t = (current_time - self.t0) as f32; // integer sub, then cast

        let n = (self.gravity_factor / self.semi_major_axis.powi(3)).sqrt();
        let m = self.mean_anomaly_at_epoch + n * delta_t;

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

        let true_anom = 2.0
            * ((1.0 + self.eccentricity).sqrt() * (e_anomaly / 2.0).sin())
                .atan2((1.0 - self.eccentricity).sqrt() * (e_anomaly / 2.0).cos());
        let r = self.semi_major_axis * (1.0 - self.eccentricity * e_anomaly.cos());

        let x_orb = r * true_anom.cos();
        let y_orb = r * true_anom.sin();
        let x = x_orb * self.omega.cos() - y_orb * self.omega.sin();
        let y = x_orb * self.omega.sin() + y_orb * self.omega.cos();

        Vector2D { x, y }
    }
}
