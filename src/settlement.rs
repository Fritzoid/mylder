use crate::roads::Road;

struct Settlement {
    name: String,
    location: (f32, f32),
    population: u32,
    roads: Vec<Road>,
}

