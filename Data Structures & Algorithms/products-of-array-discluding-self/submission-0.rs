impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        // println!{"Nums: {:?}", nums};
        let left_prods = prefix_sum(&nums, false);
        let right_prods = prefix_sum(&nums, true);

        // println!("left_prods: {:?}", left_prods);
        // println!("right_prods: {:?}", right_prods);

        let mut answers = vec![0; nums.len()];
        // Manually update first and last
        answers[0] = right_prods[1];
        answers[nums.len() - 1] = left_prods[left_prods.len() - 2];

        // Loop to calculate any in the middle
        if nums.len() > 2 {
            for i in 1..nums.len() - 1 {
                answers[i] = left_prods[i - 1] * right_prods[i + 1];
            }
        }
        
        answers
    }
}

pub fn prefix_sum(arr: &Vec<i32>, backwards: bool) -> Vec<i32> {
    let len = arr.len();
    let mut prefix_prods = vec![0; len];

    // Forwards
    if !backwards {
        prefix_prods[0] = arr[0];

        for i in 1..len {
            prefix_prods[i] = prefix_prods[i - 1] * arr[i];
            // println!("Forwards prefixes: {:?}", prefix_prods);
        }
        
    } 
    // Backwards
    else { 
        prefix_prods[len - 1] = arr[len - 1];
        
        for i in (0..len - 1).rev() {
            prefix_prods[i] = prefix_prods[i + 1] * arr[i];
            // println!("Backwards prefixes: {:?}", prefix_prods);
        }
    }
    
    prefix_prods
}
