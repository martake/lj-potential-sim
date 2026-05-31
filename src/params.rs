use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SimParams {
    pub r_zone: f32,
    pub r_shield: f32,
    pub repulsion_strength: f32,
    pub attraction_strength: f32,
    pub electron_noise: f32,
    pub dt: f32,
    pub damping: f32,
    pub max_velocity: f32,
    pub atom_count: usize,
    pub paused: bool,
}

impl Default for SimParams {
    fn default() -> Self {
        Self {
            r_zone: 30.0,
            r_shield: 60.0,
            repulsion_strength: 5000.0,
            attraction_strength: 50.0,
            electron_noise: 2.0,
            dt: 0.016,
            damping: 0.999,
            max_velocity: 200.0,
            atom_count: 16,
            paused: false,
        }
    }
}
