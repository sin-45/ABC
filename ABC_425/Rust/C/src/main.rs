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
        q: usize,
        a: [u32; n],
    }
    let mut point: usize = 0;
    let mut list: Vec<u32> = Vec::new();
    list.push(0);
    for i in 0..2*n {
        list.push(list[i] + a[i%n]);
    }
    for _ in 0..q {
        input! {
            p: u8,
        }
        if p == 1 {
            input! {
                c: usize,
            }
            point += c;
            point %= n;
        } else {
            input! {
                l: usize,
                r: usize,
            }
            let a: u32 = list[r+point] - list[l+point-1];
            println!("{}", a);
        }
    }
}