use std::io;

fn main() {
    println!("How much is your airplane ticket?");
    let ticket_price = read_f64();
    let tax = 1.2; // 20% Percent
    let final_price = ticket_price * tax; //For correct tax calculations
    println!("£{:.2} is the price of your airplane ticket!", final_price);
}

fn read_f64() -> f64 {
    let line = io::stdin().lines().next().unwrap();
    line.parse().unwrap()
}
