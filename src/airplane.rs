use anyhow;
use crate::read_f64;

pub fn run() {
    println!("How much is your airplane ticket?");
    let ticket_price = loop {
        match read_f64() {
            Ok(value) => break value,
            Err(e) => {
                println!("Invalid Number. {}", e);
                continue;
            }
        }
    };
    let tax = 1.20; // 20% Percent
    let final_price = ticket_price * tax; // For correct tax calculations
    println!("£{:.2} is the price of your airplane ticket!", final_price);
}