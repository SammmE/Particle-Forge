use bevy::prelude::*;
use crate::engine::physics::{calculate_next_state, commit_state};
use crate::engine::particles::{Proton, Neutron, Electron};

/// System for calculating the next state.
pub fn calculate_next_state_system(
    mut proton_query: Query<&mut Proton>,
    mut neutron_query: Query<&mut Neutron>,
    mut electron_query: Query<&mut Electron>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    for mut proton in proton_query.iter_mut() {
        calculate_next_state(&mut proton, dt);
    }
    for mut neutron in neutron_query.iter_mut() {
        calculate_next_state(&mut neutron, dt);
    }
    for mut electron in electron_query.iter_mut() {
        calculate_next_state(&mut electron, dt);
    }
}

/// System for committing the next state.
pub fn commit_state_system(
    mut proton_query: Query<&mut Proton>,
    mut neutron_query: Query<&mut Neutron>,
    mut electron_query: Query<&mut Electron>,
) {
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