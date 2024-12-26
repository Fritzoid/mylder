use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use log;

pub static SCENERY_SCALE: f32 = 1.0;

#[derive(Debug, Clone, Copy)]
pub struct ScaledCoordinate(pub f32);

impl ScaledCoordinate {
    pub fn new(coordinate: f32) -> Self {
        Self(coordinate * SCENERY_SCALE)
    }

    pub fn scaled(&self) -> f32 {
        self.0
    }

    pub fn set(&mut self, coordinate: f32) {
        self.0 = coordinate * SCENERY_SCALE;
    }
}

impl From<f32> for ScaledCoordinate {
    fn from(coordinate: f32) -> Self {
        Self::new(coordinate)
    }
}

#[derive(Component)]
struct GameCamera;

pub fn set_scenery(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    log::info!("Setting up scenery");

    commands.spawn((
        Camera3d::default(),
        Transform {
            translation: Vec3::new(6.0, 6.0, 6.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
        .looking_at(Vec3::ZERO, -Vec3::Z),
        PanOrbitCamera::default(),
        GameCamera,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(
            ScaledCoordinate(5.0).scaled(),
            ScaledCoordinate(5.0).scaled(),
        ))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 15.0, 0.0),
    ));
}
