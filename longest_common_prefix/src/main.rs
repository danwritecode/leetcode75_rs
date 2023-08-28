fn main() {
    let list = vec!["t".to_string(), "testosterone".to_string(), "ter".to_string()];
    Solution::longest_common_prefix(list);
}

struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }

        let mut strs_mut = strs;

        let shortest_word_index = strs_mut
            .iter()
            .enumerate()
            .fold(0, |acc, (i, x)| {
                if x.len() < strs_mut[acc].len() {
                    return i;
                }
                return acc;
            });

        // make prefix the entire first word
        let mut prefix = strs_mut.remove(shortest_word_index).chars().collect::<Vec<char>>();

        for word in strs_mut {
            for (i, c) in word.chars().enumerate() {
                let prefix_index_char = prefix.get(i);

                match prefix_index_char {
                    Some(pic) => {
                        if pic != &c {
                            prefix = prefix[0..i].to_vec();
                            break;
                        }
                    },
                    None => break
                }
            }

            if prefix.len() == 0 {
                break;
            }
        }

        let prefix = prefix.iter().collect::<String>();
        return prefix;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_1() {
        let list = vec!["flower".to_string(),"flow".to_string(),"flight".to_string()];
        assert_eq!(Solution::longest_common_prefix(list), "fl");
    }

    #[test]
    fn test_2() {
        let list = vec!["dog".to_string(),"racecar".to_string(),"car".to_string()];
        assert_eq!(Solution::longest_common_prefix(list), "");
    }

    #[test]
    fn test_3() {
        let list = vec![];
        assert_eq!(Solution::longest_common_prefix(list), "");
    }

    #[test]
    fn test_4() {
        let list = vec!["test".to_string(), "testosterone".to_string(), "tiger".to_string()];
        assert_eq!(Solution::longest_common_prefix(list), "t");
    }

    #[test]
    fn test_5() {
        let list = vec!["ab".to_string(), "a".to_string()];
        assert_eq!(Solution::longest_common_prefix(list), "a");
    }
}
