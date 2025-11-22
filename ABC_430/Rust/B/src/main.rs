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
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        grid: [Chars; n]
    }
    for i in &grid {
        println!("{:?}", i);
    }
    let mut set: HashSet<String> = HashSet::new();
    for i in 0..n-m+1 {
        for j in 0..n-m+1 {
            let mut temp: String = "".to_string();
            for k in 0..m {
                for l in 0..m {
                    temp += &grid[i+k][j+l].to_string();
                }
            }
            set.insert(temp);
        }
    }
    println!("{}", set.len());
}
