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
        n: i32,
    }
    for _ in 0..n {
        input! {
            s: char,
        }
        let d: i32 = s as i32;
        match s {
            'a'..='z' => {
                println!("小文字です: {}", s);
            }
            'A'..='Z' => {
                println!("大文字です: {}", s);
            }
            '0'..='9' => {
                println!("数字です: {}", s);
            }
            _ => {
                println!("other");
            }
        }
        print(d);
    }
}

fn print(s: i32) {
    println!("{}", s);
}