fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max = candies.iter().max().unwrap();

        let candy_res = candies
            .iter()
            .map(|c| return &(c + extra_candies) >= max)
            .collect::<Vec<bool>>();

        return candy_res;
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
