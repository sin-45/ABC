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
        a: [u32; n],
    }
    let mut dict: HashMap<u32, usize> = HashMap::new();
    for i in 0..n {
        *dict.entry(a[i]).or_insert(0) += 1;
    }
    let mut ans: usize = 0;
    for (k, v) in &dict {
        if *v == 1 {
            continue;
        }
        let cnt: usize = n - v;
        ans += cnt * (*v * (*v - 1) / 2);
    }
    println!("{}", ans);
}
