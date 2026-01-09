use rand::Rng;

use crate::game::Rand;

pub fn rand_slice<T: Copy>(rng: &mut Rand, slice: &[T]) -> T {
    let i = rng.random_range(0..slice.len());
    slice[i]
}

pub fn remove_random<T>(rng: &mut Rand, v: &mut Vec<T>) -> T {
    let i = rng.random_range(0..v.len());
    v.remove(i)
}
