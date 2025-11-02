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
        q: usize,
        mut a: [i64; n],
        mut b: [i64; n],
    }
    let mut min: i64 = 0;
    for i in 0..n {
        min += if a[i] < b[i] {a[i]} else {b[i]};
    }
    for i in 0..q {
        input! {
            c: char,
            mut x: usize,
            v: i64,
        }
        x -= 1;
        min -= if a[x] < b[x] {a[x]} else {b[x]};
        if c == 'A' {
            a[x] = v;  
        } else {
            b[x] = v;
        }
        min += if a[x] < b[x] {a[x]} else {b[x]};
        println!("{}", min);
    }
}