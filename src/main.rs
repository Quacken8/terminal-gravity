use crate::display::{coords_function, draw_planets};
use crate::physics::{ASTRONOMICAL_UNIT, create_planets, generate_three_body, newtonian_d_dt};
use crate::rk4::step_rk4;
use ruscii::app::{App, State};
use ruscii::drawing::Pencil;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::terminal::Window;

mod display;
mod physics;
mod rk4;
mod vectors;

fn main() {
    run_app();
    // let simulation_result = simulate(365.0 * 24.0 * 60.0 * 60.0, 100.0);

    // for i in 0..simulation_result[0].len() {
    //     print!("{i};\t\t\t")
    // }
    // println!();

    // simulation_result.iter().for_each(|r| {
    //     r.iter().for_each(|position| {
    //         print!("{:.3e},\t{:.3e};", position.x, position.y);
    //     });
    //     println!();
    // });
}

fn run_app() {
    let mut app = App::default();

    let mut time = 0.0;
    let mut state = generate_three_body();

    let real_second_to_sim = (50 * 24 * 60 * 60) as f64;
    let target_framerate = 30.0;
    let steps_per_frame = 500;
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

        // for key_event in app_state.keyboard().last_key_events() {
        //     match key_event {
        //         KeyEvent::Pressed(Key::Esc) => app_state.stop(),
        //         KeyEvent::Pressed(Key::Q) => app_state.stop(),
        //         _ => (),
        //     }
        // }

        // let mut pencil = Pencil::new(window.canvas_mut());
        // pencil.draw_text(&format!("FPS: {}", 44), Vec2::xy(1, 1));
    });
}
