use crate::simulation::simulate;

mod rk4;
mod simulation;
mod vectors;

fn main() {
    let simulation_result = simulate(365.0 * 24.0 * 60.0 * 60.0, 100.0);

    for i in 0..simulation_result[0].len() {
        print!("{i};\t\t\t")
    }
    println!();

    simulation_result.iter().for_each(|r| {
        r.iter().for_each(|position| {
            print!("{:.3e},\t{:.3e};", position.x, position.y);
        });
        println!();
    });
}
