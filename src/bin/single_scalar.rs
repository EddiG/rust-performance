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
    for _ in 0..1000 {
        single_scalar(&input);
    }
}
