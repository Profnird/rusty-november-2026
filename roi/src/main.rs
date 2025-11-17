/*
 * Return On Investment
 * roi = (net profit / investment cost) * 100
 */
#![allow(unused)]

fn calc_roi(net_profit: f32, investment_cost: f32) -> Option<f32> {
    // check if investment cost is not zero so not to crash the program
    if investment_cost == 0.0 {
        None
    } else {
        Some((net_profit / investment_cost) * 100.0)
    }
}

fn main() {
    let roi = calc_roi(56.0, 56.0).unwrap();
    println!("Your Return On Investment is {:.2}% ", roi);
    //{:.2}% will format the roi into 2 decimal places with a percentage
}
