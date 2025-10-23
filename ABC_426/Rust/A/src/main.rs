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
        mut x : String,
        mut y : String,
    }
    if x == "Lynx" {
        x = String::from("ZZZZ");
    }
    if y == "Lynx" {
        y = String::from("ZZZZ");
    }
    if x >= y {
        println!("Yes");
    } else {
        println!("No");
    }
}
