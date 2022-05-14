use rand::prelude::SliceRandom;
use rand::Rng;

#[allow(dead_code)]
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

#[allow(dead_code)]
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


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn is_sorted_test() {
        assert_eq!(is_sorted(&vec![1, 2, 3, 4, 5]), true);
        assert_eq!(is_sorted(&vec![5, 4, 3, 2, 1]), false);
        assert_eq!(is_sorted(&vec![1, 1, 1, 1, 1]), true);
        assert_eq!(is_sorted(&vec![1, 1, 1, 1, 2]), true);
        assert_eq!(is_sorted(&vec![1, 1, 1, 2, 1]), false);
        assert_eq!(is_sorted(&vec![1]), true);
        assert_eq!(is_sorted::<u32>(&vec![]), true);
    }

    #[test]
    fn random_vec_test() {
        let v = random_vec(10);
        assert_eq!(v.len(), 10);
        assert_ne!(random_vec(10), random_vec(10));
    }
}