mod comparator;
mod layer;
mod lut;

use std::result;

use lut::gen_lut;
use lut::gen_pairs;
use layer::apply;

fn main() {
    let states = 0;
    let lut: Vec<[u8; 16]> = gen_lut(states);
    let _pairs: Vec<Vec<(u8, [u8; 16])>> = gen_pairs(&lut, states);
    let _result: Vec<usize> = find(&lut);
}

fn find(lut: &Vec<[u8; 16]>) -> Vec<usize> {
    let mut result: Vec<usize> = vec![0];
    let mut m = 0;

    let now = std::time::Instant::now();

    loop {
        m += 1;

        for i in 0..result.len() {
            result[i] += 1;
            if result[i] != lut.len() {
                break;
            }
            result[i] = 0;
            if result.len() == i+1 {
                println!("{}; new depth reached: {} in {}s", m, result.len(), now.elapsed().as_secs());
                result.push(0);
            }
        }
    }

    return vec![];
}

