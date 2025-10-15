use crate::{
    physics::{ASTRONOMICAL_UNIT, THE_EARTH, THE_MOON, THE_SUN},
    vectors::{State, StateVec, Vector},
};

pub fn create_planets() -> StateVec {
    StateVec(vec![THE_SUN, THE_EARTH, THE_MOON])
}

fn freeze_center_of_mass(bodies: Vec<State>) -> Vec<State> {
    let center_of_mass_vel = 1.0 / (bodies.iter().fold(0.0, |acc, curr| acc + curr.mass))
        * bodies.iter().fold(Vector { x: 0.0, y: 0.0 }, |acc, curr| {
            acc + curr.mass * curr.velocity
        });

    bodies
        .into_iter()
        .map(|b| State {
            velocity: b.velocity - center_of_mass_vel,
            ..b
        })
        .collect()
}

pub fn generate_two_body() -> StateVec {
    let body1 = State {
        mass: THE_SUN.mass,
        position: Vector {
            x: ASTRONOMICAL_UNIT,
            y: 0.0,
        },
        velocity: Vector { x: 0.0, y: -5e3 },
    };
    let body2 = State {
        mass: 0.7 * THE_SUN.mass,
        position: Vector {
            x: -ASTRONOMICAL_UNIT,
            y: 0.0,
        },
        velocity: Vector { x: 0.0, y: 4e3 },
    };

    StateVec(freeze_center_of_mass(vec![body1, body2]))
}

pub fn generate_three_body() -> StateVec {
    let body1 = State {
        mass: THE_SUN.mass,
        position: Vector {
            x: -ASTRONOMICAL_UNIT * 0.5,
            y: 0.0,
        },
        velocity: Vector { x: 0.0, y: -1.5e4 },
    };

    let body2 = State {
        mass: THE_SUN.mass * 0.8,
        position: Vector {
            x: ASTRONOMICAL_UNIT * 0.5,
            y: 0.0,
        },
        velocity: Vector { x: 0.0, y: 1.5e4 },
    };

    let body3 = State {
        mass: THE_SUN.mass * 0.4,
        position: Vector {
            x: 0.0,
            y: ASTRONOMICAL_UNIT * 0.3,
        },
        velocity: Vector { y: 0.0, x: -3.5e4 },
    };

    StateVec(freeze_center_of_mass(vec![body1, body2, body3]))
}
