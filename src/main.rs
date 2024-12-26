use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use log;

mod settlement;
mod roads;
mod scenery;

use settlement::Settlement;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, startup)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>> 
) {
    log::info!("Starting up");
    scenery::set_scenery(&mut commands, &mut meshes, &mut materials);
    let s = Settlement::new("Morges".to_string(), (0.0, 0.0));
    s.spawn(&mut commands, &mut meshes, &mut materials);
}
