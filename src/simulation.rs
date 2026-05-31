use egui::{Pos2, Vec2};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::params::SimParams;

#[derive(Clone)]
pub struct Atom {
    pub pos: Pos2,
    pub vel: Vec2,
    pub electron_offset: Vec2,
    pub force_acc: Vec2,
}

pub struct SimState {
    pub atoms: Vec<Atom>,
    rng: SmallRng,
    pub bounds: Vec2,
}

impl SimState {
    pub fn new(atom_count: usize, bounds: Vec2) -> Self {
        let mut rng = SmallRng::from_os_rng();
        let atoms = Self::create_atoms(atom_count, bounds, &mut rng);
        Self {
            atoms,
            rng,
            bounds,
        }
    }

    pub fn reset(&mut self, atom_count: usize) {
        self.atoms = Self::create_atoms(atom_count, self.bounds, &mut self.rng);
    }

    pub fn update(&mut self, params: &SimParams) {
        if params.paused {
            return;
        }

        let noise = params.electron_noise;
        let r_zone = params.r_zone;
        let r_shield = params.r_shield;

        // Step 1: Electron fluctuation
        for atom in &mut self.atoms {
            atom.electron_offset.x += self.rng.random_range(-noise..=noise);
            atom.electron_offset.y += self.rng.random_range(-noise..=noise);

            let len = atom.electron_offset.length();
            if len > r_zone {
                atom.electron_offset *= r_zone / len;
            }
        }

        // Step 2: Pairwise force calculation
        for atom in &mut self.atoms {
            atom.force_acc = Vec2::ZERO;
        }

        let n = self.atoms.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let delta = self.atoms[j].pos - self.atoms[i].pos;
                let r = delta.length().max(0.1);

                if r > 2.0 * r_shield {
                    continue;
                }

                let dir = delta / r;

                let force = if r < 2.0 * r_zone {
                    // Repulsion
                    let overlap = (2.0 * r_zone - r).max(0.0);
                    let ratio = overlap / r_zone;
                    let magnitude = params.repulsion_strength * ratio * ratio;
                    dir * magnitude
                } else if r < 2.0 * r_shield {
                    // Attraction
                    let exposure_i = -self.atoms[i].electron_offset.dot(dir);
                    let exposure_j = self.atoms[j].electron_offset.dot(dir);
                    let net_exposure = (exposure_i + exposure_j).max(0.0);
                    let dist_factor =
                        1.0 - (r - 2.0 * r_zone) / (2.0 * r_shield - 2.0 * r_zone);
                    let magnitude = params.attraction_strength * net_exposure / r_zone * dist_factor;
                    dir * magnitude
                } else {
                    Vec2::ZERO
                };

                // Newton's 3rd law: attraction pulls together, repulsion pushes apart
                // For repulsion: force points from i toward j, so i gets pushed in -force dir
                // For attraction: force points from i toward j, so i gets pulled in +force dir
                // The sign conventions are embedded in the magnitude calculation:
                //   repulsion magnitude > 0 pushes apart, attraction magnitude > 0 pulls together
                // Repulsion: atoms[i] -= force, atoms[j] += force (pushes apart)
                // Attraction: atoms[i] += force, atoms[j] -= force (pulls together)
                if r < 2.0 * r_zone {
                    // Repulsion: push apart
                    self.atoms[i].force_acc -= force;
                    self.atoms[j].force_acc += force;
                } else {
                    // Attraction: pull together
                    self.atoms[i].force_acc += force;
                    self.atoms[j].force_acc -= force;
                }
            }
        }

        // Step 3: Integration
        let dt = params.dt;
        let max_vel = params.max_velocity;
        let bx = self.bounds.x;
        let by = self.bounds.y;

        for atom in &mut self.atoms {
            atom.vel += atom.force_acc * dt;
            atom.vel *= params.damping;

            // Clamp velocity magnitude
            let speed = atom.vel.length();
            if speed > max_vel {
                atom.vel *= max_vel / speed;
            }

            atom.pos += atom.vel * dt;

            // Elastic boundary reflection
            if atom.pos.x < 0.0 {
                atom.pos.x = -atom.pos.x;
                atom.vel.x = atom.vel.x.abs();
            } else if atom.pos.x > bx {
                atom.pos.x = 2.0 * bx - atom.pos.x;
                atom.vel.x = -atom.vel.x.abs();
            }

            if atom.pos.y < 0.0 {
                atom.pos.y = -atom.pos.y;
                atom.vel.y = atom.vel.y.abs();
            } else if atom.pos.y > by {
                atom.pos.y = 2.0 * by - atom.pos.y;
                atom.vel.y = -atom.vel.y.abs();
            }

            atom.force_acc = Vec2::ZERO;
        }
    }

    pub fn atoms(&self) -> &[Atom] {
        &self.atoms
    }

    fn create_atoms(count: usize, bounds: Vec2, rng: &mut SmallRng) -> Vec<Atom> {
        (0..count)
            .map(|_| Atom {
                pos: Pos2::new(
                    rng.random_range(0.0..bounds.x),
                    rng.random_range(0.0..bounds.y),
                ),
                vel: Vec2::ZERO,
                electron_offset: Vec2::ZERO,
                force_acc: Vec2::ZERO,
            })
            .collect()
    }
}
