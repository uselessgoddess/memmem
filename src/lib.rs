use std::cmp;

pub mod naive {
    pub fn memmem(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        if !needle.is_empty() {
            haystack.windows(needle.len()).position(|window| window == needle)
        } else {
            return None;
        }
    }
}
