fn main() {
    let foo = Solution::gcd_of_strings("LEET".to_string(), "CODE".to_string());
    println!("{}", foo);
}

struct Solution {}

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let max_len = std::cmp::max(str1.len(), str2.len());
        let str1_len = str1.len();
        let str2_len = str2.len();

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
            return str1_gcd; // doesn't matter which you return 
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
    }
}
