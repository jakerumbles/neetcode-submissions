impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut point1 = 0;
        let mut point2 = numbers.len() - 1;

        while point1 < point2 {
            let sum = numbers[point1] + numbers[point2];
            if sum == target {
                return vec![(point1 as i32) + 1i32, (point2 as i32) + 1i32];
            }

            // Keep searching
            // Invariant: numbers vec is sorted increasing
            // Lower point 2 if sum is too high
            if sum > target {
                point2 -= 1;
            } 
            // Increase point1 if sum is too low
            else {
                point1 += 1;
            }
        }

        // Rust requires this but should never reach this as always guaranteed to be a solution
        return vec![point1 as i32, point2 as i32];
    }
}
