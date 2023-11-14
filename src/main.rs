mod solution;

use solution::Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode_case_1() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let output = 6;
        assert_eq!(Solution::trap(height), output);
    }

    #[test]
    fn leetcode_case_2() {
        let height = vec![4, 2, 0, 3, 2, 5];
        let output = 9;
        assert_eq!(Solution::trap(height), output);
    }

    #[test]
    fn test_get_tallest_index() {
        println!("testing left");
        assert_eq!(
            Solution::get_nearest_taller_crest_index_left(&[0, 1, 0, 2, 1, 0], 0),
            3
        );
        assert_eq!(
            Solution::get_nearest_taller_crest_index_left(&[1, 2, 3, 4, 5, 0, 1, 2, 4, 0], 0),
            8
        );
        println!("testing right");
        assert_eq!(
            Solution::get_nearest_taller_crest_index_right(&[0, 1, 0, 2, 1, 0], 0, 0),
            1
        );
        assert_eq!(
            Solution::get_nearest_taller_crest_index_right(&[1, 2, 3, 4, 5, 0, 1, 2, 4, 0], 0, 0),
            4
        );
    }
}

fn main() {}
