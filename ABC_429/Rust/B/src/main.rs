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
        m: u32,
        a: [u32; n],
    }
    let sum_a: u32 = a.iter().sum();
    let mut t: bool = false;
    for i in 0..n {
        if sum_a - &a[i] == m {
            t = true;
        }
    }
    if t {
        println!("Yes");
    } else {
        println!("No");
    }
}