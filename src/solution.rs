/* Thoughts on approaching the problem:
    First I thought to parse the array into subarrays which
    started at a wall and ended at a wall of equal or greater
    height, but soon realized this wouldn't cut it.

    After a bit of thinking I realized that the first and last
    indices can never hold water.

    Doing shit with crests. No idea what I'm doing.
*/

impl Solution {
    pub fn trap(heights: Vec<i32>) -> i32 {
        let len = heights.len();
        let last_index = len - 1;
        /* The constraints specify that 0 <= height[i] <= 10^5,
        so 10^5 + 1 is safe here */
        let mut prev_height = 100_001;
        let mut total_water = 0;
        for (curr_index, &curr_height) in heights.iter().enumerate() {
            println!("ITERATION NUMBER: {}", curr_index + 1);
            println!("current idx: {curr_index}, current height: {curr_height}");
            println!("total water before shit: {total_water}");
            prev_height = curr_height;
            if curr_index == 0 || curr_index == last_index {
                println!();
                continue;
            }
            let nearest_taller_left_crest_index =
                Self::get_nearest_taller_crest_index_left(&heights[..curr_index], curr_height);
            let nearest_taller_left_crest = heights[nearest_taller_left_crest_index];
            let nearest_taller_right_crest_index = Self::get_nearest_taller_crest_index_right(
                &heights,
                curr_index + 1,
                nearest_taller_left_crest,
            );
            let nearest_taller_right_crest = heights[nearest_taller_right_crest_index];
            println!(
                "nearest taller left crest: {nearest_taller_left_crest} at index {nearest_taller_left_crest_index}"
            );
            println!(
                "nearest taller right crest: {nearest_taller_right_crest} at index {nearest_taller_right_crest_index}"
            );
            if curr_height < nearest_taller_left_crest && nearest_taller_right_crest > curr_height {
                total_water += std::cmp::min(nearest_taller_left_crest, nearest_taller_right_crest)
                    - curr_height;
            }
            println!("total water after shit: {total_water}\n");
        }
        total_water
    }

    pub fn get_nearest_taller_crest_index_left(from_slice: &[i32], taller_than: i32) -> usize {
        let mut prev_height = i32::MIN;
        let mut prev_index = 0;
        let mut crests = Vec::new();
        for (curr_index, &curr_height) in from_slice.iter().enumerate().rev() {
            if curr_height > prev_height {
                prev_height = curr_height;
            }
            if curr_height < prev_height {
                if prev_height >= taller_than {
                    return prev_index;
                }
                crests.push(prev_index);
            }
            prev_index = curr_index;
        }
        let index = *crests.first().unwrap_or(&0);
        if from_slice[0] > from_slice[index] {
            return 0;
        }
        return index;
    }

    pub fn get_nearest_taller_crest_index_right(
        from_slice: &[i32],
        skip: usize,
        taller_than: i32,
    ) -> usize {
        let mut prev_height = i32::MIN;
        let mut prev_index = 0;
        let mut crests = Vec::new();
        let last_index = from_slice.len() - 1;
        for (curr_index, &curr_height) in from_slice.iter().enumerate().skip(skip) {
            if curr_height > prev_height {
                prev_height = curr_height;
            }
            if curr_height < prev_height {
                if prev_height >= taller_than {
                    return prev_index;
                }
                crests.push(prev_index);
            }
            prev_index = curr_index;
        }
        let index = *crests.first().unwrap_or(&0);
        if from_slice[last_index] > from_slice[index] {
            return last_index;
        }
        return index;
    }
}

pub struct Solution;
