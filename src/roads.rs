use bevy::prelude::*;

#[derive(Component)]
pub enum RoadMaterial {
    Dirt,
    Paved,
    Asfalt,
    Concrete
}

#[derive(Component)]
pub struct RoadSegment {
    position: (f32, f32),
}

#[derive(Component)]
pub struct Road {
    segments: Vec<RoadSegment>,
}

