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
use std::io::{stdout, Write};
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }
    let mut score: Vec<u8> = vec![0; n];
    for c in 0..m {
        let mut zero: u8 = 0;
        let mut one: u8 = 0;
        for i in 0..n {
            if s[i][c] == '0' {
                zero += 1;
            } else {
                one += 1;
            }
        }
        if zero == 0 || one == 0 {
            continue
        }
        let key: char = 
        if zero > one {'1'}
        else {'0'};
        for i in 0..n {
            if s[i][c] == key {
                score[i] += 1;

            }
        }
    }
    let max: u8 = *score.iter().max().unwrap();
    for i in 0..n {
        if score[i] == max {
            print!("{} ", i+1);
        }
    }
    println!();
}