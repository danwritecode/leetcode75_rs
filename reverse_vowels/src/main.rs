fn main() {
    println!("Hello, world!");
}


struct Solution {}

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut vowels = s
            .chars()
            .enumerate()
            .filter(|(_i, c)| is_vowel(c))
            .map(|(i, c)| (c, i))
            .collect::<Vec<(char, usize)>>();

        let mut s_chars = s.chars().collect::<Vec<char>>();
        let mut index = 0;

        while vowels.len() >= 2 {
            index += 1;
            let c = s.chars().nth(index).unwrap();

            if is_vowel(&c) {
                let first = vowels[0];
                let last = vowels[vowels.len() - 1];
                s_chars.swap(first.1, last.1);

                // remove first and last so that they don't get processed again
                vowels.pop();
                vowels.remove(0);
            }

        }

        return s_chars.iter().collect::<String>();
    }
}

pub fn is_vowel(letter: &char) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    vowels.contains(&letter)
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::reverse_vowels("hello".to_string()), "holle");
        assert_eq!(Solution::reverse_vowels("leetcode".to_string()), "leotcede");
        assert_eq!(Solution::reverse_vowels("unicorn".to_string()), "onicurn");
        assert_eq!(Solution::reverse_vowels("".to_string()), "");
        assert_eq!(Solution::reverse_vowels("ao".to_string()), "oa");
        assert_eq!(Solution::reverse_vowels("a".to_string()), "a");
        assert_eq!(Solution::reverse_vowels("ab".to_string()), "ab");
        assert_eq!(Solution::reverse_vowels("aA".to_string()), "Aa");
    }
}
