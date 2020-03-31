//! ## 排序数组 ##
//! [原题目地址](https://leetcode-cn.com/problems/sort-an-array/) 难度：<b>中等</b>
//! ### 题目描述 ###

use rand::{thread_rng, Rng};

pub struct Solution;

impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        nums
    }

    /// 快速排序
    pub fn sort_array_quick_sort(mut nums: Vec<i32>) -> Vec<i32> {
        Self::quick_sort(&mut nums[..]);
        nums
    }

    fn quick_sort(nums: &mut [i32]) {
        if nums.len() <= 1 {
            return;
        }
        
        let split_index = Self::partition(nums);
        Self::quick_sort(&mut nums[0..split_index]);
        let end = nums.len();
        Self::quick_sort(&mut nums[split_index + 1..end]);
    }

    fn partition(nums: &mut [i32]) -> usize{
        if nums.len() <= 0 {
            panic!("array contains no elements");
        }

        let index = thread_rng().gen_range(0, nums.len());
        let end = nums.len() - 1;
        nums.swap(index, end);

        // small 表示下一个比选中的数组小数该放的位置，递增的
        let mut small = 0;
        for i in 0..nums.len()-1{
            if nums[i] < nums[end] {
                nums.swap(small, i);
                small += 1;
            } 
        }

        nums.swap(small, end);

        small
    }

    /// 冒泡排序
    pub fn sort_array_bubble_sort(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in 1..nums.len() - i {
                if nums[j] < nums[j - 1] {
                    nums.swap(j, j - 1);
                }
            }
        }
        nums
    }

    /// 选择排序
    pub fn sort_array_selection_sort(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            let mut max_val = nums[0];
            let mut max_index = 0;
            for j in 1..nums.len() - i {
                if nums[j] >= max_val {
                    max_val = nums[j];
                    max_index = j;
                }
            }
            let end = nums.len() - i - 1;
            nums.swap(max_index, end);
        }
        nums
    }

    /// 希尔排序
    pub fn sort_array_shell_sort(mut _nums: Vec<i32>) -> Vec<i32> {
        vec![]
    }

    /// 插入排序
    pub fn sort_array_insertion_sort(mut _nums: Vec<i32>) -> Vec<i32> {
        vec![]
    }

    /// 归并排序
    pub fn sort_array_merge_sort(mut _nums: Vec<i32>) -> Vec<i32> {
        vec![]
    }

    /// 堆排序
    pub fn sort_array_heap_sort(mut _nums: Vec<i32>) -> Vec<i32> {
        vec![]
    }
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_sort(){
        assert_eq!(Solution::sort_array_quick_sort(vec![1,4,6,0,3,12,7]), vec![0,1,3,4,6,7,12]);
        assert_eq!(Solution::sort_array_bubble_sort(vec![1,4,6,0,3,7,12]), vec![0,1,3,4,6,7,12]);
        assert_eq!(Solution::sort_array_selection_sort(vec![1,4,6,0,3,7,12]), vec![0,1,3,4,6,7,12]);
        assert_eq!(Solution::sort_array_insertion_sort(vec![1,4,6,0,3,7,12]), vec![0,1,3,4,6,7,12]);
    }
}

