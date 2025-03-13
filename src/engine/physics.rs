use crate::engine::particles::ParticleTrait;

/// Generic update function for any particle that implements ParticleTrait.
pub fn update_particle<T: ParticleTrait>(particle: &mut T, delta_time: f32) {
    // Each particle type defines its own update logic.
    particle.update(delta_time);
}
