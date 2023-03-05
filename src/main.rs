// tests rust's standard library square root function against some other algorithm(s?) in terms of speed.
// Calculates mean squared error of my sqrt calculations by squaring and comparing to the original number
// I did not make this to shit on rust at all btw, I just wanted to test out less accurate sqrt functions to see
// the difference in speed and used rust in order to get more practice
// NOTE -- According to exo, rust stdlib sqrt takes advantage of hardware square root instructions. No way im beating that gg
// RESULTS: my implementation of the babylonian method was much slower and much less accurate
// TODO: change arrays to vectors to allow for larger trial sizes
//       add more approximation methods
//       to run use --release for more accurate benchmarking

use rand::Rng;
use std::{time::Instant};

const NUM_TRIALS: usize = 100000;
const BABYL_AGREEING_DECIMALS: f64 = 3.0;


fn main() {
    let mut babylonian_results: [f64; NUM_TRIALS] = [0.0; NUM_TRIALS];
    let mut rust_results: [f64; NUM_TRIALS] = [0.0; NUM_TRIALS];
    let mut inputs: [f64; NUM_TRIALS] = [0.0; NUM_TRIALS];

    println!("\n\nRUNNING SQUARE ROOT CALCULATIONS ON {} RANDOM NUMBERS\n", NUM_TRIALS);

    // populate random array
    for i in 0..NUM_TRIALS {
        inputs[i] = rand::thread_rng().gen_range(0.0 .. 100.0);
    }
    

    // calculate sqrt using custom algorithm
    let start = Instant::now();
    for i in 0..NUM_TRIALS {
        babylonian_results[i] = bablyonian_sqrt(inputs[i]);
    }
    let elapsed = start.elapsed();
    println!("Babylonian algorithm took {} ns", elapsed.as_nanos());


    // calculate sqrt using rust std algorithm
    let start = Instant::now();
    for i in 0..NUM_TRIALS {
        rust_results[i] = inputs[i].sqrt();
    }
    let elapsed = start.elapsed();
    println!("Rust's algorithm took     {} ns", elapsed.as_nanos());


    // calculate and print MSE
    println!("Mean Squared Error of babylonian algorithm was {}", get_mse(inputs, babylonian_results));
    println!("Mean Squared Error of Rust's algorithm was     {}", get_mse(inputs, rust_results));

}

//test using the babylonian method, assumes non-negative input
fn bablyonian_sqrt(number: f64) -> f64 {
    //custom sqrt algorithm here
    if number == 0.0 || number == 1.0 {
        return number;
    }
    let mut prev: f64 = 0.0;
    let mut x = number / 2.0;
    while f64::abs(prev - x) > f64::powf(10.0, -BABYL_AGREEING_DECIMALS) {
        prev = x;
        x = (x + number/x) / 2.0;
    }
    return x;
}

// calculate the mean squared error of my algorithm using when squaring the test square root
// not exactly sure if MSE is the right metric to use here but it should give a good idea of how far off my calculation is
fn get_mse(base: [f64; NUM_TRIALS], test: [f64; NUM_TRIALS]) -> f64 {
    //MSE = 1/n * sum((base_i - test_i)^2) for i = 1 .. n
    let mut sum: f64 = 0.0;
    for i in 0..NUM_TRIALS {
        let test_val: f64 = test[i] * test[i];
        sum += (base[i] - test_val).powf(2.0);
    }

    return (1.0 / NUM_TRIALS as f64) * sum;
}