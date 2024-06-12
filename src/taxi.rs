use crate::read_f64;

pub fn run() {
    println!("How far are your travelling? (In miles)");
    let miles = loop {
        match read_f64() {
            Ok(value) => break value,
            Err(e) => {
                println!("Invalid Number. {}", e);
                continue;
            }
        }
    };
    let rate_per_mile = 1.20; //£1.20 per mile to be charged.
    let tax = 1.20; //20% tax rate.
    let final_price = (miles * rate_per_mile) * tax;
    println!("£{:.2} is the total cost for your trip!",final_price)
}