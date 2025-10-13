use ruscii::{drawing::Pencil, spatial::Vec2};

use crate::vectors::{StateVec, Vector};

pub fn coords_function(
    dimensions: Vec2,
    large_radius: f64,
) -> impl Fn(Vector) -> ruscii::spatial::Vec2 {
    let mut ratio = 0.0;

    if dimensions.x > dimensions.y {
        ratio = (dimensions.x as f64) / large_radius;
    } else {
        ratio = (dimensions.y as f64) / large_radius;
    }

    move |v: Vector| Vec2 {
        x: (v.x / ratio) as i32 - dimensions.x / 2,
        y: (v.y / ratio) as i32 - dimensions.y / 2,
    }
}

pub fn draw_planets(
    state: StateVec,
    to_terminal_coords: impl Fn(Vector) -> ruscii::spatial::Vec2,
    mut pencil: Pencil,
) {
    let planet_chars = ["☀️", "🌍"];

    for (state, symbol) in state.0.iter().zip(planet_chars.iter()) {
        pencil.draw_text(&symbol, to_terminal_coords(state.position));
    }
}
