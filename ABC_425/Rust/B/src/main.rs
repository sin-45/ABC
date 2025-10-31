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
        a: [i32; n],
    }
    let mut ans: bool = true;
    let mut p: BTreeSet<i32> = BTreeSet::new();
    let mut ans_list: Vec<i32> = Vec::new();
    for i in 1..=n {
        p.insert(i as i32);
    }
    for i in 0..n {
        if a[i] != -1 {
            if p.insert(a[i]) {
                ans = false;
            } else {
                p.remove(&a[i]);
            }
        }
    }
    if !ans {
        println!("No");
        return;
    }
    println!("Yes");
    let p: Vec<i32> = p.into_iter().collect();
    let mut cnt: usize = 0;
    for i in 0..n {
        if a[i] == -1 {
            ans_list.push(p[cnt]);
            cnt += 1;
        } else {
            ans_list.push(a[i]);
        }
    }
    println!("{}", ans_list.iter().join(" "));
}