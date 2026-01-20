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
        a: [u16; n],
    }
    let mut b: Vec<u16> = Vec::new();
    b.push(0);
    for i in 0..n {
        b.push(b[i] + a[i]);
    }
    let mut cnt: u32 = 0;
    for i in 0..n {
        for j in i+1..n+1 {
            let mut t: bool = true;
            let c = b[j] - b[i];
            for k in i..j {
                if c % a[k] == 0 {
                    t = false;
                    break;
                }
            }
            if t {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}
