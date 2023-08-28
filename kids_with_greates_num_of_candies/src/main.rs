fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let largest = candies.sort();
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::kids_with_candies(vec![2,3,5,1,3], 3), vec![true, true, true, false, true]);
    }

    // #[test]
    // fn it_works_based() {
    // }
}
