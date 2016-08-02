/// Returns the longest increasing subsequence if a list.
/// That is the longest sequence (p_i) s.t. for all i, p_i < p_(i + 1)
/// and list[p_i] < list[p_(i + 1)]
pub fn longest_increasing_subsequence<T: Ord>(list: &Vec<T>) -> usize {
    let mut lis_ending_at_i: Vec<usize> = vec![0; list.len() + 1];
    let mut curr_i: usize = 1;
    let mut curr_max: usize = 0;
    for x in list {
        let mut max: usize = 1;
        for j in 0..curr_i {
            max = if x > &list[j] && max < lis_ending_at_i[j] {
                lis_ending_at_i[j]
            } else {
                max
            }
        };
        lis_ending_at_i[curr_i] = max + 1;
        curr_max = if curr_max < max + 1 {
            max + 1
        } else {
            curr_max
        };
        curr_i = curr_i + 1;
    }
    curr_max
}

#[cfg(test)]
mod test {
    use super::longest_increasing_subsequence;

    #[test]
    fn basic() {
        let res: usize = longest_increasing_subsequence(&vec!(2, 4, 3, 5, 1, 7, 6, 9, 8));
        assert_eq!(5, res);
    }

    #[test]
    fn empty_list() {
        let res: usize = longest_increasing_subsequence(&Vec::<i32>::new());
        assert_eq!(0, res);
    }
}