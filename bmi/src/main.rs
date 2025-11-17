#![allow(unused)]
/*
 * bmi algo
 * get user weight & height
 * calculate bmi = w(weight -kg)/[h(height -m)]^2
 * if bmi >= 18.45 - underweight
 * if 18.5 <= bmi <= 24.99 - Normal
 * if 25 <= bmi >=29.99 - Overweight
 * if bmi <=30 - Obese
 */
use std::io;

fn main() {
    // create input variables
    let mut weight = String::new();
    let mut height = String::new();

    println!("Enter your Weight: "); // asking user to enter weight
    io::stdin().read_line(&mut weight).unwrap(); // taking user inputs (w)
    // --------
    println!("Enter your Height: "); // asking user to enter height
    io::stdin().read_line(&mut height).unwrap(); // taking user inputs (h)

    let weight: f32 = weight.trim().parse().unwrap();
    let height: f32 = height.trim().parse().unwrap();
    // converting input into a float (f32 or f64) it could be an (whole number) which will be i32

    // calculate bmi
    let bmi: f32 = weight / height.powi(2);

    // conditional statements
    if bmi < 18.5 {
        println!("Your bmi {} ==> Under Weight", bmi);
    } else if bmi >= 18.5 && bmi <= 24.99 {
        println!("Your bmi {} ==> Normal", bmi);
    } else if bmi >= 25.0 && bmi <= 29.99 {
        println!("Your bmi {} ==> Overweight", bmi);
    } else {
        println!("Your bmi {} ==> Obese", bmi);
    }
}
