fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn reverse_words_naive(s: String) -> String {
        let mut words = s
            .split(' ')
            .filter(|&x| x != "")
            .map(|x| x.trim())
            .collect::<Vec<&str>>();

        let mut rev_words = words.clone();
        let mut index = 0;

        while words.len() >= 2 {
            let end_index = (rev_words.len() - 1) - index;
            rev_words.swap(index, end_index);

            // remove last and first
            words.pop();
            words.remove(0);

            index += 1;
        }

        let rev_words = rev_words
            .into_iter()
            .map(|w| format!(" {}", w))
            .collect::<String>()
            .trim()
            .to_string();

        return rev_words;
    }

    pub fn reverse_words(s: String) -> String {
        let rev = s.split_whitespace().rev().collect::<Vec<&str>>();
        rev.join(" ")
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::reverse_words("the sky is blue".to_string()), "blue is sky the");
        assert_eq!(Solution::reverse_words("  hello world  ".to_string()), "world hello");
        assert_eq!(Solution::reverse_words("a good   example".to_string()), "example good a");
        assert_eq!(Solution::reverse_words("".to_string()), "");
        assert_eq!(Solution::reverse_words("word".to_string()), "word");
        assert_eq!(Solution::reverse_words("word foo".to_string()), "foo word");
        assert_eq!(Solution::reverse_words("foo bar baz buzz fuzz".to_string()), "fuzz buzz baz bar foo");
    }
}
