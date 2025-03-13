use bevy::prelude::*;
use crate::engine::physics::{calculate_next_state, commit_state};
use crate::engine::particles::{Proton, Neutron, Electron};

/// System to perform a two-phase simulation step:
/// 1. Calculate the next state for all particles.
/// 2. Commit the next state as the current state.
pub fn simulation_step(
    mut proton_query: Query<&mut Proton>,
    mut neutron_query: Query<&mut Neutron>,
    mut electron_query: Query<&mut Electron>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();

    // Phase 1: Calculate the next state based on current state
    for mut proton in proton_query.iter_mut() {
        calculate_next_state(&mut proton, delta_time);
    }
    for mut neutron in neutron_query.iter_mut() {
        calculate_next_state(&mut neutron, delta_time);
    }
    for mut electron in electron_query.iter_mut() {
        calculate_next_state(&mut electron, delta_time);
    }

    // Phase 2: Commit the next state to current state
    for mut proton in proton_query.iter_mut() {
        commit_state(&mut proton);
    }
    for mut neutron in neutron_query.iter_mut() {
        commit_state(&mut neutron);
    }
    for mut electron in electron_query.iter_mut() {
        commit_state(&mut electron);
    }
}
