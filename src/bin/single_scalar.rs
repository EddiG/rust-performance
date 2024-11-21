#[no_mangle]
fn single_scalar(input: &[u32]) -> u32 {
    let mut sum: u32 = 0;
    for value in input {
        sum += value
    }
    sum
}

fn main() {
    const COUNT: usize = 8 * 1024;
    let input = vec![1u32; COUNT];

    println!("Array size: {} elements", COUNT);
    println!("Each element value: 1");
    println!("Expected sum: {}", COUNT);

    // Time the scalar summation
    let start = std::time::Instant::now();
    let iterations = 1000;

    let mut final_sum;
    for i in 0..iterations {
        final_sum = single_scalar(&input);
        if i == 0 {
            // Print result of first iteration
            println!("Actual sum using scalar addition: {}", final_sum);
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
