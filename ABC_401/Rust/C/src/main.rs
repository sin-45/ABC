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
    }
    const MOD: u64 = 1_000_000_000;
    let mut vec: Vec<u64> = Vec::new();
    for _ in 0..k {
        vec.push(1);
    }
    let mut cnt: u64 = (k as u64) % MOD;
    for i in k..=n {
        cnt %= MOD; 
        vec.push(cnt);
        cnt += cnt - vec[i-k];
    }
    println!("{}", vec[n]);
}
