use std::ops::Div;

fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut flowerbed_planting = flowerbed.clone();
        let mut planted = 0;

        if n == 0 {
            return true;
        }

        if flowerbed.len() == 1 && n == 1 {
            let flower = flowerbed.first().unwrap();
            if flower == &0 {
                return true;
            }

            if flower == &1 {
                return false;
            }
        }
        
        for (i, &f) in flowerbed.iter().enumerate() {
            if f == 1 {
                continue; 
            }
            let next = flowerbed_planting.get(i + 1);

            if i == 0 && next.is_some() {
                let next = next.unwrap();
                if next == &0 {
                    planted += 1;
                    flowerbed_planting[i] = 1;
                    continue;
                }
            }

            let prev = flowerbed_planting.get(i - 1);

            match (prev, next) {
                (Some(p), Some(n)) => {
                    if p == &0 && n == &0 {
                        planted += 1;
                        flowerbed_planting[i] = 1;
                        continue;
                    }
                },
                (Some(p), None) => {
                    if p == &0 {
                        planted += 1;
                        flowerbed_planting[i] = 1;
                        continue;
                    }
                }
                (_, _) => continue
            }
        }

        return planted >= n;
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        // assert_eq!(Solution::can_place_flowers(vec![1,0,0,0,1], 1), true);
        // assert_eq!(Solution::can_place_flowers(vec![1,0,0,0,1], 2), false);
        // assert_eq!(Solution::can_place_flowers(vec![0,0,0,0,0], 3), true);
        // assert_eq!(Solution::can_place_flowers(vec![0,0,0,0,0], 4), false);
        // assert_eq!(Solution::can_place_flowers(vec![0,0,0,0,0,0,0,0,0], 5), true);
        // assert_eq!(Solution::can_place_flowers(vec![0,0,0,0,0,0,0,0,0], 6), false);
        // assert_eq!(Solution::can_place_flowers(vec![1,0,0,0,0,1], 2), false);
        // assert_eq!(Solution::can_place_flowers(vec![0], 1), true);
        // assert_eq!(Solution::can_place_flowers(vec![1], 1), false);
        assert_eq!(Solution::can_place_flowers(vec![0,0,0,1,0,1,0,0,1,0,0,1,0,1,0,0,0,1,0,0,1,0,1,0,1,0,0,1,0,1,0,1,0,0,1,0,0,0,0,1,0,0,0,1,0,1,0,0,0,1,0,0,0,1,0,0,1,0,0,0,0,1,0,0,1,0,0,0,0,0,1,0,1,0,1,0,0,0,0,0,1,0,1,0,0,1,0,0,1,0,1,0,0,1,0,0,0,1,0,1,0,1,0,0,1,0,0,0,1,0,0,0,1,0,1,0,1,0,1,0,0,1,0,1,0,1,0,0,0,0,1,0,0,1,0,1,0,1,0,1,0,1,0,1,0,0,1,0,0,1,0,0,1,0,0,0,0,1,0,0,0,0,0,0,1,0,1,0,1,0,0,0,1,0,0,1,0,0,1,0,0,1], 26), false);
    }

}
