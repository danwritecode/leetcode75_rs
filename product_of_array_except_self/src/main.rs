fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut nums_product = vec![]; 

        for (i, n) in nums.iter().enumerate() {
            let lower_sum:i32 = nums[0..i].iter().product();
            let upper_sum:i32 = nums[i + 1..].iter().product();
            let sum = lower_sum * upper_sum;
            nums_product.push(sum);
        }

        return nums_product;
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::product_except_self(vec![1,2,3,4]), vec![24,12,8,6]);
        assert_eq!(Solution::product_except_self(vec![-1,1,0,-3,3]), vec![0,0,9,0,0]);
    }
}
