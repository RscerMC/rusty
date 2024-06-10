use std::io;

fn main() {
    println!("How much is your airplane ticket?");
    let ticket_price = read_i32() as f64;
    let tax = 0.2; // 20% Percent
    let final_price = ticket_price * (1.0 + tax);
    println!("£{:.2} is the price of your airplane ticket!", final_price);
}

fn read_i32() -> i32 {
    let line = io::stdin().lines().next().unwrap().unwrap();
    line.parse().unwrap()
}
