impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.len() == 1 {
            return true;
        }

        let s: Vec<char> = s.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect();
        println!("{:?}", s);
        
        if s.len() == 0 {
            return true;
        }

        // Create pointers
        let mut a = 0;
        let mut b = if s.len() > 0 {
            s.len() - 1
        } else {
            0
        };

        while a <= b {
            println!("a: {a}, b: {b}");
            if s[a] != s[b] {
                return false;
            }

            a += 1;
            if b == 0 {
                break;
            }
            b -= 1;
        }

        return true;
    }
}
