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
        s: String,
    }
    let mut x: Vec<u8> = s.chars().map(|x| x.to_digit(10).unwrap() as u8).collect();
    x.sort_unstable();
    let mut first: u8 = 10;
    let mut vec: Vec<u8> = Vec::new();
    for i in x {
        if i != 0 && first > i {
            first = i;
        } else {
            vec.push(i);
        }
    }
    println!("{}{:?}", first, vec.iter().format(""));
}
