impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        let open = vec!['(', '{', '['];
        let close = vec![')', '}', ']'];

        

        for c in s.chars() {
            // init case
            if stack.len() == 0 {
                stack.push(c);
                continue;
            }
            
            // Already contains an char in the stack

            // Now check
            // If opener
            if open.contains(&c) {
                stack.push(c);
            }
            // If closer
            if close.contains(&c) {
                // Compare most recent opener
                let popped = stack.pop().unwrap();

                // if true it's a match and we can loop to next char
                // if not, then it's not a valid string
                let matching_pair = match popped {
                    '(' => c == ')',
                    '{' => c == '}',
                    '[' => c == ']',
                    _ => {return false;}
                };

                if !matching_pair {
                    return false;
                }
            }
        }

        // Valid string will leave an empty stack
        return stack.len() == 0;

    }
}
