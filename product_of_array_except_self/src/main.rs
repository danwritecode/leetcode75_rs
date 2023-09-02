fn main() {
    Solution::product_except_self(vec![1,2,3,4]);
}

struct Solution {}

impl Solution {
    pub fn product_except_self_naive(nums: Vec<i32>) -> Vec<i32> {
        let mut nums_product = vec![]; 

        for (i, n) in nums.iter().enumerate() {
            let lower_sum:i32 = nums[0..i].iter().product();
            let upper_sum:i32 = nums[i + 1..].iter().product();
            let sum = lower_sum * upper_sum;
            nums_product.push(sum);
        }

        return nums_product;
    }

    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let nums_len = nums.len();
        let end_index = nums_len - 1;
        let mut nums_product:Vec<i32> = vec![0; nums_len];

        let mut cur_product = 0;
        let mut mul_index = 0;
        let mut index = 0;

        loop {
            // determine cur_product base
            if mul_index == 0 {
                if mul_index != index {
                    cur_product = nums[mul_index]; 
                    mul_index += 1;
                    continue
                } else {
                    // this will only ever happen at 0,0 indexes
                    cur_product = nums[mul_index + 1];
                    mul_index += 2;
                    continue
                }
            }
            
            if index == mul_index {
                if index == end_index && mul_index == end_index {
                    nums_product[index] = cur_product;
                    break;
                }
                mul_index += 1;
                continue;
            }

            let mul_by = nums[mul_index];
            cur_product *= mul_by;

            if mul_index == end_index {
                nums_product[index] = cur_product;
                cur_product = 0;
                index += 1;
                mul_index = 0;
                continue;
            }

            if index == end_index && mul_index == end_index {
                nums_product[index] = cur_product;
                break;
            }

            mul_index += 1;
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
        assert_eq!(Solution::product_except_self(vec![1,2]), vec![1,2]);
        assert_eq!(Solution::product_except_self(vec![1,0]), vec![0,1]);
        assert_eq!(Solution::product_except_self(vec![1,2,3]), vec![6,3,2]);
    }
}
