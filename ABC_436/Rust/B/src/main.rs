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
        n: usize
    }
    let mut vec: Vec<Vec<u32>> = vec![vec![0; n]; n];
    let mut r: usize = 0;
    let mut c: usize = (n-1)/2;
    let mut k: u32 = 1;
    vec[r][c] = k;
    for _ in 0..n*n-1 {
        if vec[(n+r-1) % n][(c+1) % n] == 0 {
            vec[(n+r-1) % n][(c+1) % n] = k+1;
            r = (n+r-1) % n;
            c = (c+1) % n;
        } else {
            vec[(r+1) % n][c] = k+1;
            r = (r+1) % n;
        }
        k += 1;
    }
    for row in &vec {
        println!("{}", row.iter().join(" "));
    }
}
