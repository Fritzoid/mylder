use crate::scenery::ScaledCoordinate;
use bevy::prelude::*;
use once_cell::sync::OnceCell;

pub enum RoadMaterial {
    Dirt,
    Paved,
    Asfalt,
    Concrete,
}

static ROAD_MESH: OnceCell<Cuboid> = OnceCell::new();

#[derive(Component)]
pub struct RoadSegment {
    position: (f32, f32),
    material: RoadMaterial,
    length: usize,
    mesh: Cuboid,
}

impl RoadSegment {
    pub fn spawn(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        commands.spawn((
            Mesh3d(meshes.add(self.mesh)),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_translation(Vec3::new(self.position.0, 0.0, self.position.1))
                * Transform::from_scale(Vec3::new(1.0 /*self.length as f32 */, 1.0, 1.0)),
        ));
    }
}

impl Default for RoadSegment {
    fn default() -> Self {
        let rs = ROAD_MESH.get_or_init(|| {
            Cuboid::new(
                ScaledCoordinate(0.5).scaled(),
                ScaledCoordinate(0.05).scaled(),
                ScaledCoordinate(0.5).scaled(),
            )
        });
        RoadSegment {
            position: (0.0, 0.0),
            material: RoadMaterial::Dirt,
            length: 10,
            mesh: rs.clone(),
        }
    }
}