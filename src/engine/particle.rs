use std::fmt::Debug;

use super::physics::{PhysicalProperties, Physics, Vector3};
use super::simulation::SimulationState;

/// The common particle trait. It extends PhysicalProperties so that every particle
/// can be used in physics calculations.
pub trait Particle: PhysicalProperties + Send + Sync {
    /// Simulate this particle forward by dt.
    /// The simulation can use a reference to the physics engine and the current state.
    fn simulate(&self, dt: f64, physics: &Physics, state: &SimulationState) -> Box<dyn Particle>;

    /// Provides debugging information about the particle.
    fn debug_info(&self) -> String;
}

/// An electron particle.
#[derive(Debug)]
pub struct Electron {
    mass_val: f64,
    charge_val: f64,
    position_val: Vector3,
    velocity_val: Vector3,
}

impl Electron {
    pub fn new(position: Vector3, velocity: Vector3) -> Self {
        Electron {
            mass_val: 9.10938356e-31,
            charge_val: -1.60217662e-19,
            position_val: position,
            velocity_val: velocity,
        }
    }
}

impl PhysicalProperties for Electron {
    fn mass(&self) -> f64 {
        self.mass_val
    }
    fn charge(&self) -> f64 {
        self.charge_val
    }
    fn position(&self) -> Vector3 {
        self.position_val
    }
    fn velocity(&self) -> Vector3 {
        self.velocity_val
    }
}

impl Particle for Electron {
    fn simulate(&self, dt: f64, physics: &Physics, state: &SimulationState) -> Box<dyn Particle> {
        // Compute net force on this electron.
        let net_force = physics.compute_net_force(self, state);
        // Update velocity: v = v + (F/m) * dt.
        let acceleration = net_force.mul(1.0 / self.mass());
        let new_velocity = self.velocity().add(&acceleration.mul(dt));
        // Update position: p = p + v * dt.
        let new_position = self.position().add(&new_velocity.mul(dt));
        Box::new(Electron {
            mass_val: self.mass_val,
            charge_val: self.charge_val,
            position_val: new_position,
            velocity_val: new_velocity,
        })
    }

    fn debug_info(&self) -> String {
        format!("Electron at position: {:?}", self.position_val)
    }
}

/// A proton particle.
#[derive(Debug)]
pub struct Proton {
    mass_val: f64,
    charge_val: f64,
    position_val: Vector3,
    velocity_val: Vector3,
}

impl Proton {
    pub fn new(position: Vector3, velocity: Vector3) -> Self {
        Proton {
            mass_val: 1.6726219e-27,
            charge_val: 1.60217662e-19,
            position_val: position,
            velocity_val: velocity,
        }
    }
}

impl PhysicalProperties for Proton {
    fn mass(&self) -> f64 {
        self.mass_val
    }
    fn charge(&self) -> f64 {
        self.charge_val
    }
    fn position(&self) -> Vector3 {
        self.position_val
    }
    fn velocity(&self) -> Vector3 {
        self.velocity_val
    }
}

impl Particle for Proton {
    fn simulate(&self, dt: f64, physics: &Physics, state: &SimulationState) -> Box<dyn Particle> {
        let net_force = physics.compute_net_force(self, state);
        let acceleration = net_force.mul(1.0 / self.mass());
        let new_velocity = self.velocity().add(&acceleration.mul(dt));
        let new_position = self.position().add(&new_velocity.mul(dt));
        Box::new(Proton {
            mass_val: self.mass_val,
            charge_val: self.charge_val,
            position_val: new_position,
            velocity_val: new_velocity,
        })
    }

    fn debug_info(&self) -> String {
        format!("Proton at position: {:?}", self.position_val)
    }
}

/// A neutron particle.
#[derive(Debug)]
pub struct Neutron {
    mass_val: f64,
    position_val: Vector3,
    velocity_val: Vector3,
}

impl Neutron {
    pub fn new(position: Vector3, velocity: Vector3) -> Self {
        Neutron {
            mass_val: 1.674927471e-27,
            position_val: position,
            velocity_val: velocity,
        }
    }
}

impl PhysicalProperties for Neutron {
    fn mass(&self) -> f64 {
        self.mass_val
    }
    fn charge(&self) -> f64 {
        0.0 // Neutrons are neutral.
    }
    fn position(&self) -> Vector3 {
        self.position_val
    }
    fn velocity(&self) -> Vector3 {
        self.velocity_val
    }
}

impl Particle for Neutron {
    fn simulate(&self, dt: f64, physics: &Physics, state: &SimulationState) -> Box<dyn Particle> {
        let net_force = physics.compute_net_force(self, state);
        let acceleration = net_force.mul(1.0 / self.mass());
        let new_velocity = self.velocity().add(&acceleration.mul(dt));
        let new_position = self.position().add(&new_velocity.mul(dt));
        Box::new(Neutron {
            mass_val: self.mass_val,
            position_val: new_position,
            velocity_val: new_velocity,
        })
    }

    fn debug_info(&self) -> String {
        format!("Neutron at position: {:?}", self.position_val)
    }
}
