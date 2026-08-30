use crate::math::Vector2D;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KeplerOrbit {
    pub semi_major_axis: f32,       // orbit size, in sim distance units
    pub gravity_factor: f32,        // gravity of parent body (gravitational parameter)
    pub t0: u32,                    // reference epoch in hours; safe as integer up to ~1000 years at day resolution

    pub eccentricity: u16,          // 0 = circular, closer to 1 = more elongated
    pub mean_anomaly_at_epoch: u16, // orbit "position" at t0 — MUST be rebased alongside t0
    pub omega: u16,                 // periapsis argument (angle, radians)
    pub _pad0: u16,
}


fn orbits_to_bytes(orbits: &[KeplerOrbit]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            orbits.as_ptr() as *const u8,
            std::mem::size_of_val(orbits),
        )
    }
}

impl KeplerOrbit {
    // Solve Kepler equation M = E - e*sin(E) to calculate position at current_time

    pub fn eccentricity_f32(&self) -> f32 {
        self.eccentricity as f32 / 65535.0
    }

    pub fn mean_anomaly_at_epoch_f32(&self) -> f32 {
        (self.mean_anomaly_at_epoch as f32 / 65535.0) * std::f32::consts::TAU
    }

    pub fn omega_f32(&self) -> f32 {
        (self.omega as f32 / 65535.0) * std::f32::consts::TAU
    }

    pub fn calculate_position(&self, current_time: u32) -> Vector2D {
        let delta_t = (current_time - self.t0) as f32;

        let eccentricity = self.eccentricity_f32();
        let mean_anomaly_at_epoch = self.mean_anomaly_at_epoch_f32();
        let omega = self.omega_f32();

        let n = (self.gravity_factor / self.semi_major_axis.powi(3)).sqrt();
        let m = mean_anomaly_at_epoch + n * delta_t;

        let mut e_anomaly = m;
        for _ in 0..5 {
            let f = e_anomaly - eccentricity * e_anomaly.sin() - m;
            let f_prime = 1.0 - eccentricity * e_anomaly.cos();
            let delta = f / f_prime;
            e_anomaly -= delta;
            if delta.abs() < 1e-6 {
                break;
            }
        }

        let true_anom = 2.0
            * ((1.0 + eccentricity).sqrt() * (e_anomaly / 2.0).sin())
                .atan2((1.0 - eccentricity).sqrt() * (e_anomaly / 2.0).cos());
        let r = self.semi_major_axis * (1.0 - eccentricity * e_anomaly.cos());

        let x_orb = r * true_anom.cos();
        let y_orb = r * true_anom.sin();
        let x = x_orb * omega.cos() - y_orb * omega.sin();
        let y = x_orb * omega.sin() + y_orb * omega.cos();

        Vector2D { x, y }
    }
}