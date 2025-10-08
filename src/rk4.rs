use std::ops::{Add, AddAssign, Mul};

pub fn step_rk4<T>(current_state: &mut T, d_dt: fn(state: T, t: f64) -> T, t: f64, dt: f64) -> f64
where
    T: Add<Output = T> + AddAssign + Clone,
    f64: Mul<T, Output = T>,
{
    let k1 = d_dt(current_state.clone(), t);

    let k2 = d_dt(dt / 2.0 * k1.clone() + current_state.clone(), t + dt / 2.0);

    let k3 = d_dt(dt / 2.0 * k2.clone() + current_state.clone(), t + dt / 2.0);

    let k4 = d_dt(dt * k3.clone() + current_state.clone(), t + dt);

    *current_state += (dt / 6.0) * (k1 + 2.0 * k2 + 2.0 * k3 + k4);

    t + dt
}
