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
        let mut nums_product:Vec<i32> = nums.clone(); 
        let last_index = nums_len - 1;
        let mut cur_mul_by_index = 0;
        let mut loop_index = 0;

        loop {
            println!("loop_index {}", loop_index);
            let mul_by = nums[cur_mul_by_index];
            let n = nums_product[loop_index];

            if loop_index != cur_mul_by_index {
                println!("multiplying: {} by: {}", n, mul_by);
                let product = n * mul_by;
                nums_product[loop_index] = product;
                println!("updated nums_product: {:?}", nums_product);
            }
            
            if loop_index == last_index {
                if cur_mul_by_index == last_index {
                    break;
                }

                println!("loop_index matching last_index");
                loop_index = 0;
                cur_mul_by_index += 1;
                continue;
            }
            loop_index += 1;
        }

        println!("nums_product: {:?}", nums_product);
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
