use crate::game::types::{Seconds, Vector2D};

pub struct KeplerOrbit {
    pub semi_major_axis: f64,
    pub eccentricity: f64,
    pub mean_anomaly_at_epoch: f64,
    pub gravity_factor: f64,
    pub t0: Seconds,
    pub omega: f64,
}

impl KeplerOrbit {
    // Solve Kepler equation M = E - e*sin(E) to calculate position at current_time
    pub fn calculate_position(&self, _current_time: Seconds) -> Vector2D {
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
        let true_anom = 2.0
            * ((1.0 + self.eccentricity).sqrt() * (e_anomaly / 2.0).sin())
                .atan2((1.0 - self.eccentricity).sqrt() * (e_anomaly / 2.0).cos());
        let r = self.semi_major_axis * (1.0 - self.eccentricity * e_anomaly.cos());

        let x_orb: f64 = r * true_anom.cos();
        let y_orb: f64 = r * true_anom.sin();
        let x = x_orb * self.omega.cos() - y_orb * self.omega.sin();
        let y = x_orb * self.omega.sin() + y_orb * self.omega.cos();

        Vector2D { x, y }
    }
}
