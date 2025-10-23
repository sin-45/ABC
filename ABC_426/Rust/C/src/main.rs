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
        n: i32,
        q: i32,
    }
    let mut l: i32 = 0;
    let mut dict: HashMap<i32, i32> = (1..n).map(|i| (i, 1)).collect();
    
   for _ in 0..q {
        let mut cnt: i32 = 0;
        input! {
            x: i32,
            y: i32,
        }
        for j in l..x+1 {
            cnt += dict.remove(&j).unwrap_or(0);
        }
        *dict.entry(y).or_insert(0) += cnt;
        println!("{}", cnt);
        l = max(l, x);
    }
} 
