use bevy::prelude::*;
use crate::engine::physics::update_particle;
use crate::engine::particles::{Proton, Neutron, Electron};

/// System to update the simulation for all particle types.
pub fn simulation_step(
    mut proton_query: Query<&mut Proton>,
    mut neutron_query: Query<&mut Neutron>,
    mut electron_query: Query<&mut Electron>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();

    for mut proton in proton_query.iter_mut() {
        update_particle(&mut proton, delta_time);
    }

    for mut neutron in neutron_query.iter_mut() {
        update_particle(&mut neutron, delta_time);
    }

    for mut electron in electron_query.iter_mut() {
        update_particle(&mut electron, delta_time);
    }
}
