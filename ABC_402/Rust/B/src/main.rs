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
        q: usize,
    }
    let mut list: Vec<u32> = Vec::new();
    let mut cnt: usize = 0;
    for _ in 0..q {
        input! {
            c: u8,
        }
        if c == 1 {
            input! {
                x: u32,
            }
            list.push(x);
        } else {
            println!("{}", list[cnt]);
            cnt += 1;
        }
    }
}
