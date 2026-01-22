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
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            wp: [[i64; 2]; n],
        }
        let mut p_cnt: i64 = 0;
        let mut a_vec: Vec<i64> = Vec::new();
        for i in 0..n {
            p_cnt += wp[i][1];
            a_vec.push(wp[i][0] + wp[i][1]);
        }
        a_vec.sort();
        for i in 0..n {
            p_cnt -= a_vec[i];
            if p_cnt < 0 {
                println!("{}", i);
                break
            }
        }
    }
}
