use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, usize> = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            let other = target - num;

            if let Some(other_idx) = seen.get(&other) {
                return vec![*other_idx as i32, i as i32];
            }

            // Wasn't found, save and move on
            seen.insert(*num, i);
        }

        vec![]
    }
}
