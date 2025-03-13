mod engine;

use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_system(calculate_next_state_system.label("calculate"))
        .add_system(commit_state_system.after("calculate"))
        .run();
}
