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
        m: usize,
        k: usize,
        mut h: [i64; n],
        mut b: [i64; m],
    }
    h.sort_unstable();
    b.sort_unstable();
    let mut idx: usize = 0;
    for i in 0..m {
        if h[idx] <= b[i] {
            idx += 1;
        }
        if idx as usize == n {
            break
        }
    }
    if idx >= k {
        println!("Yes");
    } else {
        println!("No")
    }
}