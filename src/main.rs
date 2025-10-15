use crate::display::{coords_function, draw_planets};
use crate::initial_conditions::{generate_three_body, generate_two_body};
use crate::physics::{ASTRONOMICAL_UNIT, newtonian_d_dt, total_energy};
use crate::rk4::step_rk4;
use ruscii::app::{App, State};
use ruscii::drawing::Pencil;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::Window;

mod display;
mod initial_conditions;
mod physics;
mod rk4;
mod vectors;

fn main() {
    run_app();
}

fn run_app() {
    let mut app = App::default();

    let mut time = 0.0;
    let mut state = generate_two_body();

    let real_second_to_sim = (50 * 24 * 60 * 60) as f64;
    let target_framerate = 30.0;
    let steps_per_frame = 10000;
    let dt = real_second_to_sim / target_framerate / (steps_per_frame as f64);

    app.run(move |app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }

        for _ in 0..steps_per_frame {
            time = step_rk4(&mut state, newtonian_d_dt, time, dt);
        }

        let window_dimensions = window.canvas().dimension();
        let mut pencil = Pencil::new(window.canvas_mut());
        let to_terminal_coords = coords_function(window_dimensions, 4.0 * ASTRONOMICAL_UNIT);

        draw_planets(&state, to_terminal_coords, &mut pencil, window_dimensions);

        pencil.draw_text(
            &format!("Total energy: {:4e} J", total_energy(&state)),
            Vec2::xy(1, 1),
        );
    });
}
