use crate::engine::particles::ParticleTrait;

/// Generic function to compute the next state for any particle.
pub fn calculate_next_state<T: ParticleTrait>(particle: &mut T, delta_time: f32) {
    particle.calculate_next_state(delta_time);
}

/// Generic function to commit the computed next state as the current state.
pub fn commit_state<T: ParticleTrait>(particle: &mut T) {
    particle.commit_state();
}
