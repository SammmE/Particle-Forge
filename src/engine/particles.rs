use bevy::prelude::*;

/// A common trait for particle behavior.
pub trait ParticleTrait {
    fn mass(&self) -> f32;
    fn charge(&self) -> f32;
    fn position(&self) -> Vec3;
    fn velocity(&self) -> Vec3;
    fn update(&mut self, delta_time: f32);
}

/// Proton particle
#[derive(Component, Debug, Clone)]
pub struct Proton {
    pub position: Vec3,
    pub velocity: Vec3,
}

impl Proton {
    pub const MASS: f32 = 1.6726219e-27;
    pub const CHARGE: f32 = 1.60217662e-19;

    pub fn new(position: Vec3, velocity: Vec3) -> Self {
        Self { position, velocity }
    }
}

impl ParticleTrait for Proton {
    fn mass(&self) -> f32 {
        Self::MASS
    }
    fn charge(&self) -> f32 {
        Self::CHARGE
    }
    fn position(&self) -> Vec3 {
        self.position
    }
    fn velocity(&self) -> Vec3 {
        self.velocity
    }
    fn update(&mut self, delta_time: f32) {
        // Update logic (e.g., simple Euler integration)
        self.position += self.velocity * delta_time;
    }
}

/// Neutron particle
#[derive(Component, Debug, Clone)]
pub struct Neutron {
    pub position: Vec3,
    pub velocity: Vec3,
}

impl Neutron {
    pub const MASS: f32 = 1.674927471e-27;
    pub const CHARGE: f32 = 0.0;

    pub fn new(position: Vec3, velocity: Vec3) -> Self {
        Self { position, velocity }
    }
}

impl ParticleTrait for Neutron {
    fn mass(&self) -> f32 {
        Self::MASS
    }
    fn charge(&self) -> f32 {
        Self::CHARGE
    }
    fn position(&self) -> Vec3 {
        self.position
    }
    fn velocity(&self) -> Vec3 {
        self.velocity
    }
    fn update(&mut self, delta_time: f32) {
        self.position += self.velocity * delta_time;
    }
}

/// Electron particle
#[derive(Component, Debug, Clone)]
pub struct Electron {
    pub position: Vec3,
    pub velocity: Vec3,
}

impl Electron {
    pub const MASS: f32 = 9.10938356e-31;
    pub const CHARGE: f32 = -1.60217662e-19;

    pub fn new(position: Vec3, velocity: Vec3) -> Self {
        Self { position, velocity }
    }
}

impl ParticleTrait for Electron {
    fn mass(&self) -> f32 {
        Self::MASS
    }
    fn charge(&self) -> f32 {
        Self::CHARGE
    }
    fn position(&self) -> Vec3 {
        self.position
    }
    fn velocity(&self) -> Vec3 {
        self.velocity
    }
    fn update(&mut self, delta_time: f32) {
        self.position += self.velocity * delta_time;
    }
}
