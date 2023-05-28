use crate::layer::layer;
use itertools::Itertools;

pub fn gen_lut(states: usize) -> Vec<[u8; 16]> {
    let mut result: Vec<[u8; 16]> = vec![];

    for n in 0..1024u16 {
        let left: u16 = n & 0xF;
        let right: u16 = (n & 0xF0) >> 4;
        let leftmode: bool = n & 0x100 != 0;
        let rightmode: bool = n & 0x200 != 0;
        
        let layer: [u8; 16] = layer(left as u8, right as u8, leftmode, rightmode);

        // println!("{} {} {} {} ==> {:?}", left, right, leftmode, rightmode, layer);
        
        if layer.iter().unique().count() < states {
            continue;
        }

        if !result.contains(&layer) {
            result.push(layer);
        }
    }

    println!("{}", result.len());
    return result;
}

pub fn gen_pairs(lut: &Vec<[u8; 16]>, states: usize) -> Vec<Vec<(u8, [u8; 16])>> {
    return vec![];
}