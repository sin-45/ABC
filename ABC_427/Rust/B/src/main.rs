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


fn f(t: u32) -> u32 {
    t.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>()
}

fn main() {
    input! {
        n: usize,
    }
    let mut list: Vec<u32> = Vec::new();
    list.push(1);
    for i in 0..n {
        list.push(list[i] + f(list[i]));
    }
    println!("{}", list[n-1]);
}
