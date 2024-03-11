#[no_mangle]
fn unroll_2_scalar(input: &[u32]) -> u32 {
    let count = input.len();
    let mut sum1: u32 = 0;
    let mut sum2: u32 = 0;
    for idx in (0..count).step_by(2) {
        sum1 += input[idx];
        sum2 += input[idx + 1];
    }
    sum1 + sum2
}

fn main() {
    const COUNT: usize = 8 * 1024;
    let input = vec![1u32; COUNT];
    for _ in 0..1000 {
        unroll_2_scalar(&input);
    }
}
