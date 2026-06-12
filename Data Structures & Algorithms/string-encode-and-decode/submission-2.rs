impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut acc = String::new();
        let len = strs.len();
        for (i, s) in strs.into_iter().enumerate() {
            let hex_string: String = s.as_bytes()
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect();
            println!("Pushed \"{}\"", hex_string);

            // Empty string case
            if hex_string.len() == 0 {
                acc.push_str("!");
            } else {
                acc.push_str(&hex_string);
            }
            
            if i + 1 < len {
                acc.push('Î');
            }
        }

        acc
    }

    pub fn decode(s: String) -> Vec<String> {
        if s.len() == 0 {
            vec![]
        } else {
            // Get back to vec
            let strs: Vec<String> = s.split('Î').map(|s| {
                // Empty string case
                if s == "!" {
                    return String::from("");
                } else {
                    hex_to_string(s).unwrap()
                }
            }).collect();

            // Decode hex strings

            strs
        }

    }
}

fn hex_to_string(hex: &str) -> Option<String> {
    // A valid hex string representing bytes must have an even length
    if hex.len() % 2 != 0 {
        return None;
    }

    let mut bytes = Vec::new();
    for i in (0..hex.len()).step_by(2) {
        // Slice the string every 2 characters and convert to a u8
        let byte_str = &hex[i..i + 2];
        if let Ok(byte) = u8::from_str_radix(byte_str, 16) {
            bytes.push(byte);
        } else {
            return None; // Invalid hex character encountered
        }
    }

    // Convert the decoded bytes to a UTF-8 String
    String::from_utf8(bytes).ok()
}
