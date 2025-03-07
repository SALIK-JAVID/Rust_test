use std::io;

// Function to calculate GCD (HCF) using Euclidean algorithm
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Function to calculate LCM using GCD
fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn main() {
    let mut numbers = Vec::new();

    println!("Enter five positive integers:");

    for i in 0..5 {
        let mut input = String::new();
        println!("Enter number {}: ", i + 1);
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        // Convert input to integer
        let num: u64 = input.trim().parse().expect("Please enter a valid number");
        numbers.push(num);
    }

    // Compute GCD (HCF) for all numbers
    let mut hcf = numbers[0];
    for &num in &numbers[1..] {
        hcf = gcd(hcf, num);
    }

    // Compute LCM for all numbers
    let mut lcm_result = numbers[0];
    for &num in &numbers[1..] {
        lcm_result = lcm(lcm_result, num);
    }

    // Display results
    println!("\nThe Highest Common Factor (HCF) is: {}", hcf);
    println!("The Lowest Common Multiple (LCM) is: {}", lcm_result);
}