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
        x: u32,
        n: usize,
        w: [u32; n],
        q: usize,
    }
    let mut t: Vec<bool> = vec![false; n];
    let mut cnt: u32 = x;
    for i in 0..q {
        input! {
            mut p: usize,
        }
        p -= 1;
        if t[p] {
            cnt -= w[p];
        } else {
            cnt += w[p];
        }
        t[p] = !t[p];
        println!("{}", cnt);
    }
}