use proconio::input;
use std::collections::HashSet; // set -> hash
use std::collections::HashMap; // 連想配列 -> hash
use std::collections::BinaryHeap; // priority_queue -> 二分ヒープ
use std::collections::BTreeSet; // set -> 平衡二分木 (log n)
use std::collections::BTreeMap; // 連想配列 -> 平行二分木 (log n)
use std::collections::VecDeque; // Deque(queue)
use std::collections::LinkedList; // list -> あまり便利ではないかも
use std::cmp::Reverse;
use std::cmp::min;
use std::cmp::max;
use itertools::Itertools;
use itertools::iproduct;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        a: [[u8; w]; h],
        b: [u8; n],
    }
    let mut cnt_vec: Vec<u32> = vec!{0; n};
    let vec_set: Vec<HashSet<u8>> = a
        .into_iter()
        .map(|row| row.into_iter().collect())
        .collect();

    
    for i in 0..h {
        for c in 0..n {
            if vec_set[i].contains(&b[c]) {
                cnt_vec[i] += 1;
            } 
        }
    }
    println!("{}", cnt_vec.iter().max().unwrap());
}
