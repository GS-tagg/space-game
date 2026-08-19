#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    // Vector addition
    pub fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
    
    // Vector subtraction
    pub fn sub(self, rhs: Self) -> Self {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }

    // Scalar multiplication
    pub fn scale(self, factor: f64) -> Self {
        Self { x: self.x * factor, y: self.y * factor }
    }

    // Calculate vector magnitude (length)
    pub fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    // Return normalised unit vector
    pub fn normalise(self) -> Self {
        let len = self.length();
        if len > f64::EPSILON {
            let inv_len = 1.0 / len;
            Self::new(self.x * inv_len, self.y * inv_len)
        } else {
            Self::new(0.0, 0.0)
        }
        
    }
}
///    fn normalise(self) -> Self {
        let len_squared = self.dot(self);

         len_squared > 0.0 {
            let inv_len = len_squared.sqrt().recip();

            Self::new(self.x * inv_len, self.y * inv_len, self.z * inv_len)
        } else {
            Self::new(0.0, 0.0, 0.0)
        }
    }

#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    pub data: [f32; 16],
}

impl Mat4 {
    /// TASK: Construct a 2D Orthographic Projection matrix for rendering HUD elements
    /// TIP: Maps screen pixel coordinates (0..width, 0..height) to GPU clip space (-1.0..1.0)
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32) -> Self {
        todo!("Construct 4x4 orthographic projection matrix")
    }
}
