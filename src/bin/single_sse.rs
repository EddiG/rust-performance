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
    for _ in 0..1000 {
        single_sse(&input);
    }
}
