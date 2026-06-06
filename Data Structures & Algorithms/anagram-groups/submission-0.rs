use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for s in strs {
            let mut s_map: [u8; 26] = [0; 26];
            for c in s.chars() {
                s_map[(c as usize) % 26] += 1;
            }

            anagrams.entry(s_map).or_insert(vec![]).push(s);

        }

        // Loop through anagrams and format final output
        let mut answer: Vec<Vec<String>> = anagrams.into_values().collect();
        answer
    }
}
