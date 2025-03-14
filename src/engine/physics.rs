use std::f64::consts::PI;
use std::fmt::Debug;

use super::particle::Particle;
use super::simulation::SimulationState;

use rayon::prelude::*;

/// A simple 3D vector for positions and velocities.
#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    pub fn add(&self, other: &Vector3) -> Vector3 {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn sub(&self, other: &Vector3) -> Vector3 {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn mul(&self, scalar: f64) -> Vector3 {
        Vector3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }

    pub fn magnitude_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(&self) -> Vector3 {
        let mag = self.magnitude();
        if mag == 0.0 {
            Vector3::zero()
        } else {
            self.mul(1.0 / mag)
        }
    }

    pub fn zero() -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }
}

/// A trait exposing the physical properties needed for force calculations.
pub trait PhysicalProperties: Debug {
    fn mass(&self) -> f64;
    fn charge(&self) -> f64;
    fn position(&self) -> Vector3;
    fn velocity(&self) -> Vector3;
}

/// The Physics struct encapsulates various interactions between particles.
pub struct Physics {
    gravitational_constant: f64,
    coulomb_constant: f64,
    strong_force_constant: f64,
    weak_force_constant: f64,
}

impl Physics {
    pub fn new() -> Self {
        Physics {
            gravitational_constant: 6.67430e-11,
            coulomb_constant: 8.9875517923e9,
            strong_force_constant: 1.0e2, // Placeholder value for strong force.
            weak_force_constant: 1.0e-5,  // Placeholder value for weak force.
        }
    }

    /// Compute gravitational force between two particles.
    pub fn gravitational_force(
        &self,
        mass1: f64,
        mass2: f64,
        pos1: Vector3,
        pos2: Vector3,
    ) -> Vector3 {
        let r_vec = pos2.sub(&pos1);
        let distance_sq = r_vec.magnitude_squared();
        if distance_sq == 0.0 {
            return Vector3::zero();
        }
        let force_magnitude = self.gravitational_constant * mass1 * mass2 / distance_sq;
        r_vec.normalize().mul(force_magnitude)
    }

    /// Compute electrostatic force between two charged particles.
    pub fn electrostatic_force(
        &self,
        charge1: f64,
        charge2: f64,
        pos1: Vector3,
        pos2: Vector3,
    ) -> Vector3 {
        let r_vec = pos2.sub(&pos1);
        let distance_sq = r_vec.magnitude_squared();
        if distance_sq == 0.0 {
            return Vector3::zero();
        }
        let force_magnitude = self.coulomb_constant * charge1 * charge2 / distance_sq;
        r_vec.normalize().mul(force_magnitude)
    }

    /// Placeholder for the strong nuclear force.
    /// In reality, the strong force is non-classical and extremely short-ranged.
    pub fn strong_force(&self, pos1: Vector3, pos2: Vector3) -> Vector3 {
        let r_vec = pos2.sub(&pos1);
        let distance = r_vec.magnitude();
        // Only significant at extremely short distances (e.g., ~1e-15 meters).
        if distance > 1e-15 || distance == 0.0 {
            return Vector3::zero();
        }
        let force_magnitude = self.strong_force_constant / (distance * distance);
        r_vec.normalize().mul(force_magnitude)
    }

    /// Placeholder for the weak nuclear force.
    /// This is generally negligible in classical particle simulations.
    pub fn weak_force(&self, _pos1: Vector3, _pos2: Vector3) -> Vector3 {
        Vector3::zero()
    }

    /// Example collision response.
    pub fn collision_response(&self, particle1: &dyn Particle, particle2: &dyn Particle) {
        println!(
            "Collision detected between {} and {}",
            particle1.debug_info(),
            particle2.debug_info()
        );
        // In a full simulation, you’d adjust velocities to conserve momentum/energy.
    }

    /// Compute the net force on a particle by summing interactions with all other particles.
    pub fn compute_net_force(
        &self,
        particle: &dyn PhysicalProperties,
        state: &SimulationState,
    ) -> Vector3 {
        let mut net_force = Vector3::zero();
        for other in &state.particles {
            // Avoid self-interaction by checking pointer equality.
            if std::ptr::eq(
                particle as *const _ as *const u8,
                (&**other) as *const _ as *const u8,
            ) {
                continue;
            }
            net_force = net_force.add(&self.gravitational_force(
                particle.mass(),
                other.mass(),
                particle.position(),
                other.position(),
            ));
            net_force = net_force.add(&self.electrostatic_force(
                particle.charge(),
                other.charge(),
                particle.position(),
                other.position(),
            ));
            net_force = net_force.add(&self.strong_force(particle.position(), other.position()));
            net_force = net_force.add(&self.weak_force(particle.position(), other.position()));
        }
        net_force
    }
}
