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
        Self::quick_sort(&mut nums);
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
    pub fn sort_array_shell_sort(mut nums: Vec<i32>) -> Vec<i32> {

        nums
    }

    /// 插入排序
    pub fn sort_array_insertion_sort(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 1..nums.len() {
            let curr = nums[i];
            for j in (0..i).rev() {
                if nums[j] > curr {
                    nums[j + 1] = nums[j];
                    if j == 0 {
                        nums[j] = curr;
                    }
                } else {
                    nums[j + 1] = curr;
                    break;
                }
            }
        }
        nums
    }

    /// 归并排序
    pub fn sort_array_merge_sort(nums: Vec<i32>) -> Vec<i32> {
        Self::merge_sort(&nums)
    }

    fn merge_sort(nums: &[i32]) -> Vec<i32> {
        if nums.len() <= 1 {
            return nums.to_vec();
        }

        Self::merge(&Self::merge_sort(&nums[0..nums.len() / 2]), &Self::merge_sort(&nums[nums.len() / 2..]))
    }

    fn merge(left_seq: &[i32], right_seq: &[i32]) -> Vec<i32> {
        let mut merged = vec![];
        let mut i = 0;
        let mut j = 0;
        while i < left_seq.len() && j < right_seq.len() {
            if left_seq[i] <= right_seq[j] {
                merged.push(left_seq[i]);
                i += 1;
            } else {
                merged.push(right_seq[j]);
                j += 1;
            }
        }

        while i < left_seq.len() {
            merged.push(left_seq[i]);
            i += 1;
        }
        
        while j < right_seq.len() {
            merged.push(right_seq[j]);
            j += 1;
        }

        merged
    }

    /// 堆排序
    pub fn sort_array_heap_sort(mut nums: Vec<i32>) -> Vec<i32> {
        // 从最后一个非叶子节点开始，从右到左，从下到上调整堆
        let length = nums.len();
        for i in (0..=nums.len() / 2 - 1).rev() {
            Self::adjust_heap(&mut nums, i, length); 
        }

        for j in (1..nums.len()).rev() {
            nums.swap(0, j);
            Self::adjust_heap(&mut nums, 0, j);
        }

        nums
    }

    fn adjust_heap(nums: &mut [i32], mut index: usize, heap_size: usize) {
        let adj_val = nums[index];
        let mut k = index * 2 + 1;
        while k < heap_size {
            if k + 1 < heap_size && nums[k] < nums[k + 1] {
                k = k + 1;
            }

            if nums[k] > adj_val {
                nums.swap(k, index);
                index = k;
            }

            k = 2 * k + 1;
        }

        nums[index] = adj_val;
    }
}

fn main() {
    println!("Hello, world!");
    println!("{:?}", Solution::sort_array_heap_sort(vec![1,4,6,0,3,7,7,12]));
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
        assert_eq!(Solution::sort_array_merge_sort(vec![1,4,6,0,3,7,12]), vec![0,1,3,4,6,7,12]);
        assert_eq!(Solution::sort_array_heap_sort(vec![1,4,6,0,3,7,12]), vec![0,1,3,4,6,7,12]);
        assert_eq!(Solution::sort_array_shell_sort(vec![1,4,6,0,3,7,12]), vec![0,1,3,4,6,7,12]);
    }
}

