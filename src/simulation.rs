use crate::{rk4::step_rk4, vectors::State, vectors::StateVec, vectors::Vector};

static NEWTONIAN_G: f64 = 6.6743e-11;
static TOO_CLOSE: f64 = 2.0 * 6.95700e8; // Solar diameter

fn create_planets() -> StateVec {
    let the_sun = State {
        mass: 1.98892e30,
        position: Vector { x: 0.0, y: 0.0 },
        velocity: Vector { x: 0.0, y: 0.0 },
    };

    let the_earth = State {
        mass: 5.9722e24,
        position: Vector {
            x: 149597870700.0,
            y: 0.0,
        },
        velocity: Vector {
            x: 0.0,
            y: 2.9681753092730e04,
        },
    };

    StateVec(vec![the_sun, the_earth])
}

fn magnitude(v: Vector) -> f64 {
    v.x.hypot(v.y)
}

fn newtonian_force(me: &State, other_planet: &State) -> Vector {
    let from_me_to_other = -1.0 * me.position + other_planet.position;
    -NEWTONIAN_G * me.mass * other_planet.mass
        / magnitude(from_me_to_other)
        / magnitude(from_me_to_other)
        / magnitude(from_me_to_other)
        * from_me_to_other
}

fn newtonian_d_dt(state: StateVec, _t: f64) -> StateVec {
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
            velocity: force,
        });
    }

    StateVec(res)
}

pub fn simulate(end_time: f64, steps: f64) -> Vec<Vec<Vector>> {
    let mut time = 0.0;
    let dt = end_time / steps;
    let mut state = create_planets();
    let mut res = Vec::with_capacity(steps.ceil() as usize);
    res.push(state.0.iter().map(|s| s.position).collect());

    while time < end_time {
        time = step_rk4(&mut state, newtonian_d_dt, time, dt);
        res.push(state.0.iter().map(|s| s.position).collect());
    }

    res
}
