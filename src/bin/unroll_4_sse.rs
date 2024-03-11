#![feature(portable_simd)]
use std::simd::prelude::*;

#[no_mangle]
fn unroll_4_sse(input: &[u32]) -> u32 {
    let count = input.len();
    let mut sum1 = u32x64::splat(0);
    let mut sum2 = u32x64::splat(0);
    let mut sum3 = u32x64::splat(0);
    let mut sum4 = u32x64::splat(0);
    for idx in (0..count).step_by(64 * 4) {
        let slice1 = unsafe { input.get_unchecked(idx..(idx + 64)) };
        let slice2 = unsafe { input.get_unchecked((idx + 64)..(idx + 64 * 2)) };
        let slice3 = unsafe { input.get_unchecked((idx + 64 * 2)..(idx + 64 * 3)) };
        let slice4 = unsafe { input.get_unchecked((idx + 64 * 3)..(idx + 64 * 4)) };
        sum1 += u32x64::from_slice(slice1);
        sum2 += u32x64::from_slice(slice2);
        sum3 += u32x64::from_slice(slice3);
        sum4 += u32x64::from_slice(slice4);
    }

    (sum1 + sum2 + sum3 + sum4).reduce_sum()
}

fn main() {
    const COUNT: usize = 8 * 1024;
    let input = vec![1u32; COUNT];
    for _ in 0..1000 {
        unroll_4_sse(&input);
    }
}
