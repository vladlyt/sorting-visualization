use rand::Rng;
use std::cmp::Ordering;
use rand::prelude::SliceRandom;

fn bubble_sort<T: Ord>(v: &mut [T]) {
    for i in 0..v.len() {
        for j in 0..(v.len() - i - 1) {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1)
            }
        }
    }
}

fn main() {}


// #[cfg(test)]
pub mod tests {
    use crate::*;
    //
    // pub fn random_vec(n: usize) -> Vec<i32> {
    //     let mut rng = rand::thread_rng();
    //     let mut v: Vec<i32> = Vec::with_capacity(n);
    //     for _ in 0..n {
    //         v.push(rng.gen());
    //     }
    //     v
    // }

    pub fn shuffled_vec(n: i32) -> Vec<i32> {
        let mut vec: Vec<i32> = (1..(n + 1)).collect();
        vec.shuffle(&mut rand::thread_rng());
        vec
    }


    fn is_sorted<T: Ord>(v: &[T]) -> bool {
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


    #[test]
    fn bubble_works_test() {
        let mut rand_vec = random_vec(20);
        let mut rand_slice = rand_vec.as_mut_slice();

        assert!(!is_sorted(rand_slice));
        bubble_sort(rand_slice);
        assert!(is_sorted(rand_slice));
    }
}
