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
    }
    let mut a_list: [u16; 101] = [0; 101];
    let mut b_list: [u16; 101] = [0; 101];
    for i in 0..n {
        input! {
            a: usize,
            b: u16,
        }
        a_list[a] += b;
        b_list[a] += 1;
    }
    for i in 1..101 {
        if b_list[i] != 0 {
            println!("{}", a_list[i] as f64 / b_list[i] as f64);
        }
    }
}
