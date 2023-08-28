fn main() {
    let foo = Solution::gcd_of_strings("ABCABCABC".to_string(), "ABCAAA".to_string());
    println!("{}", foo);
}

struct Solution {}

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let max_len = std::cmp::max(str1.len(), str2.len());
        let str1_len = str1.len();
        let str2_len = str2.len();
        let str1_chars = str1.chars().collect::<Vec<char>>();
        let str2_chars = str2.chars().collect::<Vec<char>>();

        let mut gcd = 0;
        for i in (1..max_len + 1).rev() {
            match (str1_len % i, str2_len % i) {
                (0, 0) => {
                    gcd = i;     
                    break;
                },
                _ => ()
            }
        }

        let str1_gcd = str1[..gcd].to_string();
        let str2_gcd = str2[..gcd].to_string();

        if str1_gcd == str2_gcd {
            let str1_chunked:Vec<String> = str1_chars.chunks(gcd).map(|c| c.to_vec().iter().collect::<String>()).collect();
            let str2_chunked:Vec<String> = str2_chars.chunks(gcd).map(|c| c.to_vec().iter().collect::<String>()).collect();

            for c in str1_chunked {
                if c != str1_gcd {
                    return "".to_string();
                }
            }

            for c in str2_chunked {
                if c != str1_gcd {
                    return "".to_string();
                }
            }

            return str1_gcd;
        }

        return "".to_string();
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string()), "ABC".to_string());
        assert_eq!(Solution::gcd_of_strings("ABABAB".to_string(), "ABAB".to_string()), "AB".to_string());
        assert_eq!(Solution::gcd_of_strings("LEET".to_string(), "CODE".to_string()), "".to_string());
        assert_eq!(Solution::gcd_of_strings("ABCDEF".to_string(), "ABC".to_string()), "".to_string());
        assert_eq!(Solution::gcd_of_strings("ABCABCABC".to_string(), "ABCAAA".to_string()), "".to_string());
    }
}
