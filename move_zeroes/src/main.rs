fn main() {
    let mut foo = vec![0,1,0,3,12];
    Solution::move_zeroes_v2(&mut foo);
    println!("{:?}", foo);
}

struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut zeez = vec![];
        nums.retain(|n| {
            if *n == 0 {
                zeez.push(0);
                false
            } else {
                true
            }
        });

        nums.append(&mut zeez);
    }
    
    pub fn move_zeroes_v2(nums: &mut Vec<i32>) {
        let mut zeez = vec![];
        let mut index = 0;
        
        loop {
            if index == nums.len() - 1 {
                break;
            }
            
            let num = nums[index];
            if num == 0 {
                zeez.push(nums.remove(index));
                index = 0; 
                continue
            }

            index += 1;
        }

        nums.append(&mut zeez);
    }
}
