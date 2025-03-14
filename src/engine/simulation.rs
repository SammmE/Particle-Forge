use super::particle::Particle;

/// Represents a simulation state holding a collection of particles.
pub struct SimulationState {
    pub particles: Vec<Box<dyn Particle>>,
}

impl SimulationState {
    pub fn new() -> Self {
        SimulationState {
            particles: Vec::new(),
        }
    }

    pub fn add_particle(&mut self, particle: Box<dyn Particle>) {
        self.particles.push(particle);
    }
}