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
    let mut set: HashSet<(u32, u32)> = HashSet::new();
    let mut cnt: u32 = 0;
    for _ in 0..m {
        input! {
            r: u32,
            c: u32,
        }
        let mut t: bool = true;
        for i in 0..2 {
            for j in 0..2 {
                if set.contains(&(r+i, c+j)) {
                    t = false;
                }
            }
        }
        if t {
            cnt += 1;
            for i in 0..2 {
                for j in 0..2 {
                    set.insert((r+i, c+j));
                }
            }
        }
    }
    println!("{}", cnt);
}
