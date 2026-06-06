use std::collections::{HashMap, BinaryHeap};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counts: HashMap<i32, u32> = HashMap::new();

        // Get number frequency counts
        for num in nums {
            *counts.entry(num).or_insert(0) += 1;
        }   

        // Now get top k ocurring numbers
        let mut heap = BinaryHeap::new();
        for (num, count) in counts {
            heap.push((count, num));
        }

        let answer: Vec<i32> = (0..k).map(|_| heap.pop().unwrap().1).collect();
        answer
    }
}
