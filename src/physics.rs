use crate::{rk4::step_rk4, vectors::State, vectors::StateVec, vectors::Vector};

static NEWTONIAN_G: f64 = 6.6743e-11;
static TOO_CLOSE: f64 = 2.0 * 6.95700e8; // Solar diameter
pub static ASTRONOMICAL_UNIT: f64 = 149597870700.0;

pub const THE_SUN: State = State {
    mass: 1.98892e30,
    position: Vector { x: 0.0, y: 0.0 },
    velocity: Vector { x: 0.0, y: 0.0 },
};

pub const THE_EARTH: State = State {
    mass: 5.9722e24,
    position: Vector {
        x: ASTRONOMICAL_UNIT,
        y: 0.0,
    },
    velocity: Vector {
        x: 0.0,
        y: 2.9681753092730e04,
    },
};

pub const THE_MOON: State = State {
    position: Vector {
        x: 1.1 * ASTRONOMICAL_UNIT,
        y: 0.0,
    },
    velocity: Vector {
        x: 0.0,
        y: 0.8 * 2.9681753092730e04,
    },
    mass: THE_EARTH.mass,
};

fn magnitude(v: Vector) -> f64 {
    v.x.hypot(v.y)
}

fn newtonian_force(me: &State, other_planet: &State) -> Vector {
    let from_me_to_other = other_planet.position - me.position;
    NEWTONIAN_G * me.mass * other_planet.mass
        / magnitude(from_me_to_other)
        / magnitude(from_me_to_other)
        / magnitude(from_me_to_other)
        * from_me_to_other
}

fn newtonian_energy(me: &State, other_planet: &State) -> f64 {
    let from_me_to_other = other_planet.position - me.position;
    -NEWTONIAN_G * me.mass * other_planet.mass / magnitude(from_me_to_other)
}

pub fn newtonian_d_dt(state: StateVec, _t: f64) -> StateVec {
    let planets = state.0;
    let mut res = Vec::with_capacity(planets.len());

    for planet in planets.iter() {
        let mut force = Vector { x: 0.0, y: 0.0 };
        for other_planet in planets.iter() {
            if magnitude(planet.position - other_planet.position) < TOO_CLOSE {
                continue;
            }
            force += newtonian_force(planet, other_planet);
        }

        res.push(State {
            mass: 0.0,
            position: planet.velocity,
            velocity: (1.0 / planet.mass) * force,
        });
    }

    StateVec(res)
}

pub fn total_energy(state: &StateVec) -> f64 {
    let planets = &state.0;
    let kinetic = planets.iter().fold(0.0, |acc, curr| {
        acc + 0.5 * curr.mass * magnitude(curr.velocity) * magnitude(curr.velocity)
    });

    let mut potential = 0.0;
    for i in 0..planets.len() {
        let me = planets[i];
        for other_planet in planets.iter().skip(i + 1) {
            potential += newtonian_energy(&me, other_planet);
        }
    }

    kinetic + potential
}
