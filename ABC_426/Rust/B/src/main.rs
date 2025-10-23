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
    let mut dict: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        let temp = dict.entry(c).or_insert(0);
        *temp += 1;
    }
    for (str, cnt) in &dict {
        if *cnt == 1 {
            println!("{}", str);
        }
    }
}
