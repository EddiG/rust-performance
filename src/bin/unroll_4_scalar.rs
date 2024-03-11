#[no_mangle]
fn unroll_4_scalar(input: &[u32]) -> u32 {
    let count = input.len();
    let mut sum1: u32 = 0;
    let mut sum2: u32 = 0;
    let mut sum3: u32 = 0;
    let mut sum4: u32 = 0;
    for idx in (0..count).step_by(4) {
        sum1 += input[idx];
        sum2 += input[idx + 1];
        sum3 += input[idx + 2];
        sum4 += input[idx + 3];
    }
    sum1 + sum2 + sum3 + sum4
}

fn main() {
    const COUNT: usize = 8 * 1024;
    let input = vec![1u32; COUNT];
    for _ in 0..1000 {
        unroll_4_scalar(&input);
    }
}
