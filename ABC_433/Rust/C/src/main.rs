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
        s: String,
    }
    let mut num_list: Vec<u8> = Vec::new();
    let mut cnt_list: Vec<u32> = Vec::new();
    let mut index: usize = 0;
    num_list.push(100);
    cnt_list.push(0);
    for i in s.chars() {
        let a: u8 = i.to_digit(10).unwrap() as u8;
        if a != num_list[index] {
            index += 1;
            num_list.push(a);
            cnt_list.push(1);
        } else {
            cnt_list[index] += 1;
        }
    }
    let mut ans: u64 = 0;
    for i in 0..num_list.len()-1 {
        if num_list[i] + 1 == num_list[i+1] {
            ans += min(cnt_list[i], cnt_list[i+1]) as u64;
        }
    }
    println!("{}", ans);
}
