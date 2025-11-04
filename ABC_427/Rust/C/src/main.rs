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
        eg: [(u16, u16); m],
    }
    let mut ans: u16 = m as u16;
    for i in 0..2_i32.pow(n as u32) {
        let mut cnt: u16 = 0;
        for (u, v) in &eg {
            if ((i >> u) & 1) == ((i >> v) & 1) {
                cnt += 1;
            }
        }
        ans = min(ans, cnt);
    }
    println!("{}", ans);
}
