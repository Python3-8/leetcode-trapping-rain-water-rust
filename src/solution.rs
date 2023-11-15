/* Thoughts on approaching the problem:
    First I thought to parse the array into subarrays which
    started at a wall and ended at a wall of equal or greater
    height, but soon realized this wouldn't cut it.

    After a bit of thinking I realized that the first and last
    indices can never hold water.

    Doing shit with crests. No idea what I'm doing.

    Yeah I thought about it this morning and came up with some
    stuff that I thought would work. After a lot of fighting
    with my code, now it's working for the two test cases on
    LeetCode. Idk if I can submit it though

    I GOT IT WORKING! IT EVEN WORKS WITH MY OWN TEST CASE!!!

    Lmao I was wondering why LeetCode was giving me 1,500 ms,
    then I realized I left in the println!() calls I used for
    debugging
*/

impl Solution {
    pub fn trap(heights: Vec<i32>) -> i32 {
        let len = heights.len();
        let last_idx = len - 1;
        let mut total_water = 0;
        for (curr_idx, &curr_height) in heights.iter().enumerate() {
            if curr_idx == 0 || curr_idx == last_idx {
                continue;
            }
            let (left_crest_idx, right_crest_idx) =
                Self::get_relevant_crest_indices(&heights, curr_idx);
            let (left_crest, right_crest) = (heights[left_crest_idx], heights[right_crest_idx]);
            if curr_height < left_crest && right_crest > curr_height {
                let change = std::cmp::min(left_crest, right_crest) - curr_height;
                if change > 0 {
                    total_water += change;
                }
            }
        }
        total_water
    }

    pub fn get_left_crest_indices(heights: &[i32], left_of: usize) -> Vec<usize> {
        let taller_than = heights[left_of];
        let len = heights.len();
        let mut crest_indices = Vec::new();
        let mut prev_idx = 0;
        let mut prev_height = -1;
        for (curr_idx, &curr_height) in heights[..left_of].iter().enumerate().rev() {
            if curr_height < prev_height && prev_height > taller_than {
                crest_indices.push(prev_idx);
            }
            // (prev_idx, prev_height) = (curr_idx, curr_height);
            // Apparently in the Rust version that LeetCode uses, the
            // above statement is unstable
            prev_idx = curr_idx;
            prev_height = curr_height;
        }
        if len >= 2 {
            let first = heights[0];
            if first > heights[1] && first > taller_than {
                crest_indices.push(0);
            }
        }
        crest_indices
    }

    pub fn get_right_crest_indices(heights: &[i32], right_of: usize) -> Vec<usize> {
        let taller_than = heights[right_of];
        let len = heights.len();
        let mut crest_indices = Vec::new();
        let mut prev_idx = 0;
        let mut prev_height = -1;
        for (curr_idx, &curr_height) in heights.iter().enumerate().skip(right_of + 1) {
            if curr_height < prev_height && prev_height > taller_than {
                crest_indices.push(prev_idx);
            }
            // (prev_idx, prev_height) = (curr_idx, curr_height);
            prev_idx = curr_idx;
            prev_height = curr_height;
        }
        if len >= 2 {
            let last_index = len - 1;
            let last_height = heights[last_index];
            if last_height > heights[len - 2] && last_height > taller_than {
                crest_indices.push(last_index);
            }
        }
        crest_indices
    }

    pub fn get_relevant_crest_indices(heights: &[i32], from: usize) -> (usize, usize) {
        let left_crest_indices = Self::get_left_crest_indices(heights, from);
        let right_crest_indices = Self::get_right_crest_indices(heights, from);
        let mut tallest_left_idx = 0;
        for &lidx in left_crest_indices.iter() {
            if heights[lidx] > heights[tallest_left_idx] {
                tallest_left_idx = lidx;
            }
        }
        let mut tallest_right_idx = heights.len() - 1;
        for &ridx in right_crest_indices.iter() {
            if heights[ridx] > heights[tallest_right_idx] {
                tallest_right_idx = ridx;
            }
        }
        (tallest_left_idx, tallest_right_idx)
    }
}

pub struct Solution;
