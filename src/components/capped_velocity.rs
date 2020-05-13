use amethyst::ecs::{Component, DenseVecStorage};

use super::Velocity;

#[derive(Debug, Copy, Clone, Component)]
#[storage(DenseVecStorage)]
pub enum CappedVelocity {
    Orthogonal {
        max_x: f32,
        min_x: f32,
        max_y: f32,
        min_y: f32,
    },
    Magnitude {
        max: f32,
        min: f32,
    },
}

impl CappedVelocity {
    pub fn cap(&self, velo: Velocity) -> Velocity {
        match self {
            CappedVelocity::Orthogonal {
                max_x,
                min_x,
                max_y,
                min_y,
            } => Velocity {
                x: velo.x.min(*max_x).max(*min_x),
                y: velo.y.min(*max_y).max(*min_y),
            },
            CappedVelocity::Magnitude { max, min } => {
                let v = velo.as_vec2();
                let original_mag = v.magnitude();
                let mag = original_mag.min(*max).max(*min);

                if original_mag.is_normal() {
                    v.scale(mag / original_mag).into()
                } else {
                    v.into()
                }
            }
        }
    }

    pub fn new_ortho_abs(cap_x: f32, cap_y: f32) -> Self {
        CappedVelocity::Orthogonal {
            max_x: cap_x,
            min_x: -cap_x,
            max_y: cap_y,
            min_y: -cap_y,
        }
    }

    pub fn new_mag_abs(cap: f32) -> Self {
        CappedVelocity::Magnitude {
            max: cap,
            min: 0.0,
        }
    }
}
