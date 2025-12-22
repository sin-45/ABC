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
        n: u64,
        m: u32,
    }
    let mut ans: u64 = 0;
    let mut over: bool = false;
    for i in 0..m+1 {
        if let Some(p) = n.checked_pow(i) {
            if let Some(next_p) = ans.checked_add(p) {
                if next_p > 1_000_000_000 {
                    over = true;
                    break;
                }
                ans = next_p;
            } else {
                over = false;
                break;
            }
        } else {
            over = false;
            break;
        }
    }
    if over {
        println!("inf");
    } else {
        println!("{}", ans);
    }
}
