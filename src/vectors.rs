use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct State {
    pub position: Vector,
    pub velocity: Vector,
    pub mass: f64,
}

#[derive(Clone, Debug)]
pub struct StateVec(pub Vec<State>);

impl AddAssign for StateVec {
    fn add_assign(&mut self, rhs: Self) {
        self.0
            .iter_mut()
            .zip(rhs.0.iter())
            .for_each(|(s, o)| *s += *o);
    }
}

impl Add for StateVec {
    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Vec::with_capacity(self.0.len().min(rhs.0.len()));

        self.0
            .iter()
            .zip(rhs.0.iter())
            .for_each(|(a, b)| res.push(*a + *b));

        StateVec(res)
    }

    type Output = StateVec;
}

impl Mul<StateVec> for f64 {
    type Output = StateVec;

    fn mul(self, rhs: StateVec) -> StateVec {
        let mut res = Vec::with_capacity(rhs.0.len());
        for s in rhs.0.iter() {
            res.push(self * *s);
        }
        StateVec(res)
    }
}

impl Mul<State> for f64 {
    type Output = State;

    fn mul(self, rhs: State) -> State {
        State {
            position: self * rhs.position,
            velocity: self * rhs.velocity,
            mass: self * rhs.mass,
        }
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl AddAssign for State {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Add for State {
    fn add(self, rhs: State) -> State {
        State {
            position: self.position + rhs.position,
            velocity: self.velocity + rhs.velocity,
            mass: self.mass + rhs.mass,
        }
    }

    type Output = State;
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vector {
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-1.0 * rhs)
    }
    type Output = Vector;
}

impl Add for Vector {
    fn add(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }

    type Output = Vector;
}
