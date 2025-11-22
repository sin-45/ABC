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
        a: u32,
        b: u32,
        s: Chars,
    }
    let mut cnt: u32 = 0;
    let mut l: usize = 0;
    let mut r: usize = 0;
    let mut a_cnt: u32 = 0;
    let mut b_cnt: u32 = 0;

    while r != n {
        let mark: &str = "l";
        if b_cnt >= b {
            if a_cnt >= a {
                cnt += a_cnt - a + 1;
            }
        }
        if mark == "l" {
            if s[l] == 'a'{
                a_cnt += 1;
            } else {
                b_cnt += 1;
            }
            l += 1;
        } else {
            if s[r] == 'a'{
                a_cnt += 1;
            } else {
                b_cnt += 1;
            }
            r += 1;
        }
    }
    println!("{}", cnt);
}