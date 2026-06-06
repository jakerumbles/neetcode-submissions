use std::collections::HashSet;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        if nums.len() == 0 {
            return false;
        }

        let mut seen = HashSet::new();
        for num in &nums {
            if seen.contains(num) {
                return true;
            }

            seen.insert(num);
        }

        return false;
    }
}
