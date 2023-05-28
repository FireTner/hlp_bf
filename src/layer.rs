use std::cmp;
use crate::comparator::comparator;



pub fn layer(left: u8, right: u8, leftm: bool, rightm: bool) -> [u8; 16] {
    let mut result: [u8; 16] = [0; 16];
    for x in 0u8..16 {
        result[x as usize] = cmp::max(comparator(x, left, leftm), comparator(right, x, rightm));
    }
    return result;
}

#[inline(always)]
pub fn apply(lut: Vec<[u8; 16]>, input: [u8; 16], index: usize) -> [u8; 16] {
    assert_ne!(lut.len(), 0);
    assert_eq!(input.len(), lut[0].len());

    let mut result: [u8; 16] = [0; 16];

    for i in 0..input.len() {
        result[i] = lut[index][input[i] as usize];
    }

    return result;
}