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
        n: usize,
        k: usize,
        s: String,
    }
    let mut hash: BTreeMap<&str, u16> = BTreeMap::new();
    for i in 0..n-k+1 {
        let slice: &str = &s[i..i+k];
        *hash.entry(slice).or_insert(0) += 1;
    }
    let max: u16 = *hash.values().max().unwrap();
    println!("{}", max);
    for (key, value) in &hash {
        if *value == max {
            print!("{} ", key);
        }
    }
    println!("");
}
