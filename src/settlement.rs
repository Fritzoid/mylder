use crate::roads::RoadSegment;
use bevy::prelude::*;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Location(f32, f32);

#[derive(Component)]
struct Roads(Vec<RoadSegment>);

#[derive(Bundle)]
pub struct Settlement {
    name: Name,
    location: Location,
    roads: Roads,
}

impl Settlement {
    pub fn new(name: String, location: (f32, f32)) -> Self {
        Self {
            name: Name(name),
            location: Location(location.0, location.1),
            roads: Roads(vec![RoadSegment::default()]),
        }
    }

    pub fn spawn(
        &self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        for rs in &self.roads.0 {
            rs.spawn(commands, meshes, materials);
        }
    }
}
