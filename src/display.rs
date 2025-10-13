use ruscii::{drawing::Pencil, spatial::Vec2};

use crate::{
    physics::{THE_EARTH, THE_SUN},
    vectors::{State, StateVec, Vector},
};

pub fn coords_function(
    dimensions: Vec2,
    large_radius: f64,
) -> impl Fn(Vector) -> ruscii::spatial::Vec2 {
    let ratio = (if dimensions.x > dimensions.y {
        dimensions.x as f64
    } else {
        dimensions.y as f64
    }) / large_radius;

    move |v: Vector| Vec2 {
        x: (v.x * ratio) as i32 + dimensions.x / 2,
        y: (v.y / 2.0 * ratio) as i32 + dimensions.y / 2,
    }
}

fn is_in_bounds(position: Vec2, dimensions: Vec2) -> bool {
    position.x > 0 && position.y > 0 && position.x < dimensions.x && position.y < dimensions.y
}

pub fn draw_planets(
    state: &StateVec,
    to_terminal_coords: impl Fn(Vector) -> ruscii::spatial::Vec2,
    pencil: &mut Pencil,
    window_dimensions: Vec2,
) {
    for (i, state) in state.0.iter().enumerate() {
        let draw_position = to_terminal_coords(state.position);
        if is_in_bounds(draw_position, window_dimensions) {
            pencil.draw_text(planet_char(state, i), draw_position);
        }
    }
}

fn planet_char(planet: &State, i: usize) -> &str {
    let small_bodies = [
        "🌕", "🌙", "🌒", "🌓", "🌔", "🌖", "🌗", "🌘", "☄️", "🪨", "🌑",
    ];
    let planets = [
        "🌍", "🪐", "🌎", "🌏", "🟤", "🔴", "🟢", "🔵", "🟣", "🟠", "🟡",
    ];
    let stars = [
        "☀️", "⭐", "🌟", "💫", "✨", "🌞", "🟠", "🟡", "🔵", "🟣", "⚪",
    ];

    if planet.mass > 0.3 * THE_SUN.mass {
        stars[i % stars.len()]
    } else if planet.mass > 0.3 * THE_EARTH.mass {
        planets[i % stars.len()]
    } else {
        small_bodies[i % stars.len()]
    }
}
