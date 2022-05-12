use std::cmp::Ordering;

use rand::prelude::SliceRandom;
use rand::Rng;

pub fn random_vec(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut v: Vec<u32> = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(rng.gen());
    }
    v
}

pub fn shuffled_vec(n: u32) -> Vec<u32> {
    let mut vec: Vec<u32> = (1..(n + 1)).collect();
    vec.shuffle(&mut rand::thread_rng());
    vec
}


pub fn is_sorted<T: Ord>(v: &Vec<T>) -> bool {
    if v.len() <= 1 {
        return true;
    }
    for i in 1..v.len() {
        if v[i - 1] > v[i] {
            return false;
        }
    }
    true
}