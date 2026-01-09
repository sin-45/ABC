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
    }
    let mut d: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    let mut cnt: u32 = 0;
    let mut let_cnt: Vec<i32> = vec![0; m];
    for i in 0..m {
        input! {
            k: usize,
            a: [usize; k],
        }
        let_cnt[i] = k as i32;
        for &j in &a {
            d[j-1].insert(i);
        }
    }
    input! {
        b: [usize; n],
    }
    for &i in &b {
        for &j in &d[i-1] {
            let_cnt[j] -= 1;
            if let_cnt[j] == 0 {
                cnt += 1;
            }
        }
        println!("{}", cnt);
    }
}
