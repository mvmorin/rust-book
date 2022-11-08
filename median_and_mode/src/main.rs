use rand::{Rng,thread_rng,distributions::Uniform};
use std::collections::HashMap;

#[allow(dead_code)]
fn get_type<T>(_:& T) -> &'static str {
    std::any::type_name::<T>()
}

struct SortedVec<T>{
    vec: Vec<T>
}

impl<T: Copy + Ord> SortedVec<T> {
    fn from(v: &Vec<T>) -> Self {
        let mut v_sorted = v.clone();
        v_sorted.sort_unstable();
        SortedVec{vec: v_sorted}
    }

    fn median(&self) -> T {
        // let len = self.vec.len();
        // if len % 2 == 0 {
        //     let up = len/2;
        //     let down = len/2-1;
        //     (self.vec[up] + self.vec[down])/2
        // } else {
        //     self.vec[len/2]
        // }
        self.vec[self.vec.len()/2]
    }
}

fn main() {
    const N: usize = 7;
    const RANGE: core::ops::Range<i64> = -1..2;

    let list =
        thread_rng().
        sample_iter(Uniform::from(RANGE)).
        take(N).
        collect::<Vec<i64>>();
    println!("list: {:?}",list);

    println!("median: {:?}", median(&list));

    println!("mode: {:?}", mode(&list));
}

fn median<T: Copy + Ord>(v: & Vec<T>) -> T {
    SortedVec::from(v).median()
}

fn mode<T: Copy + std::hash::Hash + std::cmp::Eq + std::fmt::Debug>(v: &Vec<T>) -> T {
    let mut counts : HashMap<T,usize>= HashMap::new();

    for &el in v.iter() {
        let count = counts.entry(el).or_insert(0);
        *count += 1;
    }

    println!("{:?}",counts);

    let mut max_count = 0usize;
    let mut max_val = v[0];

    for (val,count) in counts {
        if count > max_count {
            max_count = count;
            max_val = val;
        }
    }

    max_val
}
