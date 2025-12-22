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
    }
    let mut ans: u64 = 0;
    for i in 1..=2 {
        let mut l: u64 = 0;
        let mut r: u64 = 10_000_000_000;
        let p = 2_u64.pow(i as u32);
        while l+1 != r {
            let mid: u64 = (l + r) / 2;
            let is_ok = mid.checked_mul(mid)
                            .and_then(|m2| m2.checked_mul(p))
                            .map(|val| val <= n)
                            .unwrap_or(false);

            if is_ok {
                l = mid;
            } else {
                r = mid;
            }
        }
        ans += l as u64;
    }
    println!("{}", ans);
}
