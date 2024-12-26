use bevy::prelude::*;
use log;

mod settlement;
mod roads;
mod scenery;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .run();
}

fn startup(
    mut commands: Commands,
) {
    log::info!("Starting up");
    scenery::set_scenery(&mut commands);
}
