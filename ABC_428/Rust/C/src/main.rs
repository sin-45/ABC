use proconio::input;
use std::collections::HashSet; // set -> hash
use std::collections::HashMap; // 連想配列 -> hash
use std::collections::BinaryHeap; // priority_queue -> 二分ヒープ
use std::collections::BTreeSet; // set -> 平衡二分木 (log n)
use std::collections::BTreeMap; // 連想配列 -> 平行二分木 (log n)
use std::collections::VecDeque; // VecDeque(queue)
use std::collections::LinkedList; // list -> あまり便利ではないかも
use std::cmp::Reverse;
use std::cmp::min;
use std::cmp::max;
use itertools::Itertools;
use itertools::iproduct;

fn main() {
    input! {
        q: u32,
    }
    let mut que: VecDeque<i32> = VecDeque::new();
    let mut que_cnt: VecDeque<i32> = VecDeque::new();
    que.push_back(0);
    que_cnt.push_back(0);
    for i in 0..q {
        input! {
            p: u8,
        }
        if p == 1 {
            input! {
                c: char,
            }
            let val: i32 = if c == '(' {1} else {-1};
            let idx: usize = que.len() - 1;
            que.push_back(que[idx] + val);
            que_cnt.push_back(min(que[idx], que_cnt[idx]));
        } else {
            que.pop_back();
            que_cnt.pop_back();
        }
        let idx: usize = que.len() - 1;
        if que[idx] == que_cnt[idx] && que[idx] == 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
