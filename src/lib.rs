use std::cmp;

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
