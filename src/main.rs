use std::io;

mod airplane;
mod taxi;

fn main() {
    taxi::run();
    airplane::run();
}

pub fn read_f64() -> Result<f64,anyhow::Error> {
    let line = io::stdin().lines().next().expect("Could not get the next line.");
    Ok(line?.parse::<f64>().expect("Couldn't convert line to f64"))
}