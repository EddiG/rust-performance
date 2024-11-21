#![feature(portable_simd)]
use std::simd::prelude::*;

#[no_mangle]
fn single_sse(input: &[u32]) -> u32 {
    let count = input.len();
    let mut sum = u32x64::splat(0);
    for idx in (0..count).step_by(64) {
        let slice = unsafe { input.get_unchecked(idx..(idx + 64)) };
        sum += u32x64::from_slice(slice);
    }

    sum.reduce_sum()
}

fn main() {
    const COUNT: usize = 8 * 1024;
    let input = vec![1u32; COUNT];

    println!("Array size: {} elements", COUNT);
    println!("Each element value: 1");
    println!("Expected sum: {}", COUNT);

    // Time the SIMD summation
    let start = std::time::Instant::now();
    let iterations = 1000;

    let mut final_sum;
    for i in 0..iterations {
        final_sum = single_sse(&input);
        if i == 0 {
            // Print result of first iteration
            println!("Actual sum using SIMD: {}", final_sum);
        }
    }

    let duration = start.elapsed();
    println!("\nPerformance metrics:");
    println!("Total time for {} iterations: {:?}", iterations, duration);
    println!(
        "Average time per iteration: {:?}",
        duration / iterations as u32
    );
}
