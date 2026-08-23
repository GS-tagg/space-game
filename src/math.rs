#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    // Vector addition
    pub fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }

    // Vector subtraction
    pub fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }

    // Scalar multiplication
    pub fn scale(self, factor: f64) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    // Calculate vector magnitude (length)
    pub fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    // Return normalised unit vector
    pub fn normalise(self) -> Self {
        let len = self.length();
        //epsilon is essentially 0
        if len > f64::EPSILON {
            let inv_len = 1.0 / len;
            Self::new(self.x * inv_len, self.y * inv_len)
        } else {
            Self::new(0.0, 0.0)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
    pub data: [f64; 16],
}

impl Mat4 {
    pub fn identity() -> Self {
        Self {
            data: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    // 2D Orthographic Projection matrix for UI and map rendering.
    pub fn orthographic(left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64) -> Self {
        let m00 = 2.0 / (right - left);
        let m11 = 2.0 / (top - bottom);
        let m22 = -2.0 / (far - near);
        let m30 = -(right + left) / (right - left);
        let m31 = -(top + bottom) / (top - bottom);
        let m32 = -(far + near) / (far - near);
        let m33 = 1.0;

        Self {
            data: [
                m00, 0.0, 0.0, 0.0, 0.0, m11, 0.0, 0.0, 0.0, 0.0, m22, 0.0, m30, m31, m32, m33,
            ],
        }
    }

    // 2D Translation Matrix.

    pub fn from_translation(x: f64, y: f64, z: f64) -> Self {
        Self {
            data: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, x, y, z, 1.0,
            ],
        }
    }

    // 2D Scale Matrix.
    pub fn from_scale(scale_x: f64, scale_y: f64) -> Self {
        Self {
            data: [
                scale_x, 0.0, 0.0, 0.0, 0.0, scale_y, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
                1.0,
            ],
        }
    }

    // Multiply two 4x4 matrices (self * rhs).
    // Final Matrix = Projection * View * Model
    pub fn mul(&self, rhs: &Self) -> Self {
        let mut result = Self::identity();
        for row in 0..4 {
            for col in 0..4 {
                result.data[col * 4 + row] = self.data[0 * 4 + row] * rhs.data[col * 4 + 0]
                    + self.data[1 * 4 + row] * rhs.data[col * 4 + 1]
                    + self.data[2 * 4 + row] * rhs.data[col * 4 + 2]
                    + self.data[3 * 4 + row] * rhs.data[col * 4 + 3];
            }
        }
        result
    }

    // Convert matrix data to a flat array suitable for shader uniform buffers.
    pub fn to_cols_array(&self) -> [f64; 16] {
        self.data
    }
}
