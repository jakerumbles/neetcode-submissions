impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // First check length of s and t is same
        if s.len() != t.len() {
            return false;
        }

        // Count letter occurence in each string
        let mut seen_s: HashMap<char, u32> = HashMap::new();
        let mut seen_t: HashMap<char, u32> = HashMap::new();

        for l in s.chars() {
            *seen_s.entry(l).or_insert(0) += 1;
        }

        for l in t.chars() {
            *seen_t.entry(l).or_insert(0) += 1;
        }

        seen_s == seen_t
    }
}
