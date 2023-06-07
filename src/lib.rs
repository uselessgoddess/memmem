pub mod naive {
    pub fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        if !needle.is_empty() {
            haystack.windows(needle.len()).position(|window| window == needle)
        } else {
            None
        }
    }
}

pub mod kmp {
    fn kmp_prefix_function(pattern: &[u8]) -> Vec<usize> {
        let len = pattern.len();
        let mut prefix_function = vec![0; len];

        for i in 1..len {
            let mut j = prefix_function[i - 1];

            while j > 0 && pattern[i] != pattern[j] {
                j = prefix_function[j - 1];
            }

            if pattern[i] == pattern[j] {
                j += 1;
            }

            prefix_function[i] = j;
        }

        prefix_function
    }

    pub fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        let needle_len = needle.len();
        let haystack_len = haystack.len();

        if needle_len == 0 {
            return Some(0);
        }

        if needle_len > haystack_len {
            return None;
        }

        let prefix_function = kmp_prefix_function(needle);
        let mut j = 0;

        for i in 0..haystack_len {
            while j > 0 && haystack[i] != needle[j] {
                j = prefix_function[j - 1];
            }

            if haystack[i] == needle[j] {
                j += 1;
            }

            if j == needle_len {
                return Some(i - needle_len + 1);
            }
        }

        None
    }
}

pub mod bm {
    fn calculate_jump_table(pattern: &[u8]) -> Vec<usize> {
        let pattern_length = pattern.len();

        let mut jump_table = vec![pattern_length; 256];

        for (i, &ch) in pattern.iter().enumerate() {
            jump_table[ch as usize] = pattern_length - 1 - i;
        }

        jump_table
    }

    pub fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        // Prepare
        let needle_length = needle.len();
        let haystack_length = haystack.len();

        if needle_length == 0 {
            return Some(0);
        }

        if needle_length > haystack_length {
            return None;
        }

        // Precompute jump table
        let jump_table = calculate_jump_table(needle);

        // Main loop
        let mut i = 0usize;
        while i < haystack_length - needle_length + 1 {
            let chunk = &haystack[i..i + needle_length];

            if chunk == needle {
                return Some(i);
            }

            let &mismatch_char = chunk.last().unwrap();
            let jump_distance = jump_table[mismatch_char as usize];
            i += jump_distance;
        }

        None
    }
}

#[test]
fn foo() {
    let haystack = "Привет, я ищу подстроку в этой строке!";
    let needle = "подстроку";

    match bm::find(haystack.as_bytes(), needle.as_bytes()) {
        Some(index) => println!("Подстрока \"{}\" найдена на позиции {}", needle, index),
        None => println!("Подстрока \"{}\" не найдена в строке", needle),
    }
}
