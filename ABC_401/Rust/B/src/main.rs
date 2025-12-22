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
        n: u8,
    }
    let mut login: bool = false;
    let mut cnt: u8 = 0;
    for _ in 0..n {
        input! {
            s: String,
        }
        if s == "login" {
            login = true;
        } else if s == "logout" {
            login = false;
        } else if s == "private" && !login {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
