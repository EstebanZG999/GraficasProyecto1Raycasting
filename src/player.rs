use nalgebra::Vector2;

pub struct Player {
    pub pos: Vector2<f32>,
    pub a: f32,  // Ángulo de vista
    pub fov: f32, // Campo de visión
}
