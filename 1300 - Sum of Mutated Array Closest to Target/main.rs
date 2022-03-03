// When given an array and a target, find the value that when every value in the
// given array larger than the value is changed, is summed to be the closest (in
// absolute difference) to the target

struct Solution {}

impl Solution {
    fn vec_sum(arr: &Vec<i32>, val: i32) -> i32 {
        // replaces the values of arr with val if the values are smaller
        let mut new_arr: Vec<i32> = Vec::new();
        for i in arr {
            if i > &val {
                new_arr.push(i32::from(&val + 0));
            } else {
                new_arr.push(i32::from(i + 0));
            }
        }
        new_arr.iter().sum()
    }
    fn main_loop(arr: Vec<i32>, target: i32, n: i32) -> i32 {
        let sum = Self::vec_sum(&arr, n);
        let next_sum = Self::vec_sum(&arr, n + 1);
        let sum_difference = (&target - &sum).abs();
        let next_sum_difference = (&target - &next_sum).abs();
        // Base Case
        if sum == target {
            return n;
        } else if next_sum == target {
            return n + 1;
        } else if next_sum > target {
            if sum_difference <= next_sum_difference {
                return n;
            } else {
                return n + 1;
            }
        } else if sum_difference == next_sum_difference {
            return n;
        } else {
            // Recursive Case
            return Self::main_loop(arr, target, n + 1);
        }
    }
    pub fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
        // The *value cannot be higher than the largest value in the given array.

        // Start with 0, and keep track of the sum.

        // If the next sum is closer to the target, get rid of the old sum

        // If the sum is greater than target, and the sum is farther away than the
        // previous sum, return previous sum.
        Self::main_loop(arr, target, 0)
    }
}

fn main() {
    println!("{} == 3?", Solution::find_best_value(vec![4, 9, 3], 10));
    println!("{} == 5?", Solution::find_best_value(vec![2, 3, 5], 10));
    println!(
        "{} == 11361?",
        Solution::find_best_value(vec![60864, 25176, 27249, 21296, 20204], 56803)
    );
    println!("{}", Solution::find_best_value(vec![2, 3, 5], 11))
}
