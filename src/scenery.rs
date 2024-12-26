use bevy::prelude::*;
use log;

#[derive(Component)]
struct GameCamera;

pub fn set_scenery(commands: &mut Commands) {
    log::info!("Setting up scenery");

    commands.spawn((
        Camera3d::default(),
        Transform {
            translation: Vec3::new(0.0, 50.0, 0.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
        .looking_at(Vec3::ZERO, -Vec3::Z),
        GameCamera,
    ));
}
