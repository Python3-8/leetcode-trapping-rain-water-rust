mod solution;

#[cfg(test)]
mod tests {
    use super::solution::Solution;

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
    fn test_get_left_crest_indices() {
        assert_eq!(
            Solution::get_left_crest_indices(&[7, 1, 2, 3, 4, 5, 6, 7, 8, 0, 10], 8),
            vec![]
        );
    }

    #[test]
    fn test_get_right_crest_indices() {
        assert_eq!(
            Solution::get_right_crest_indices(&[7, 1, 2, 3, 4, 5, 6, 7, 0, 1, 0, 10], 8),
            vec![9, 11]
        );
    }

    #[test]
    fn test_get_relevant_crest_indices() {
        // assert_eq!(
        //     Solution::get_relevant_crest_indices(&[5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5], 5),
        //     Some((0, 10))
        // );
        // assert_eq!(
        //     Solution::get_relevant_crest_indices(&[0, 1, 2, 3, 4, 5, 4, 3, 2, 1, 0], 5),
        //     None
        // );
        assert_eq!(
            Solution::get_relevant_crest_indices(&[5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5], 5),
            (0, 10)
        );
        assert_eq!(
            Solution::get_relevant_crest_indices(&[0, 1, 2, 3, 4, 5, 4, 3, 2, 1, 0], 5),
            (0, 10)
        );
    }
}

fn main() {}
