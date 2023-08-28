fn main() {
    let zipped = Solution::merge_alternately("Foo".to_string(), "bar".to_string());
    println!("zipped: {}", zipped);
}


struct Solution {}

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut word1_chars = word1.chars();
        let mut word2_chars = word2.chars();
        let mut zipped = "".to_string();

        loop {
            match (word1_chars.next(), word2_chars.next()) {
                (None, None) => break,
                (Some(c1), Some(c2)) => {
                    zipped.push(c1);
                    zipped.push(c2);
                },
                (Some(c1), None) => zipped.push(c1),
                (None, Some(c2)) => zipped.push(c2)
            }
        }

        return zipped;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::merge_alternately("abc".to_string(), "pqr".to_string()), "apbqcr");
        assert_eq!(Solution::merge_alternately("ab".to_string(), "pqrs".to_string()), "apbqrs");
        assert_eq!(Solution::merge_alternately("abcd".to_string(), "pq".to_string()), "apbqcd");
    }
}
