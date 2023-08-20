fn merge_sort(arr: &[i32]) -> Vec<i32> {
    let n = arr.len();

    if n == 1 {
        vec![arr[0]]
    } else {
        // divide & conquer
        let mid = n / 2;
        let lhs = &arr[0..mid];
        let rhs = &arr[mid..n];
        let sorted_lhs = merge_sort(lhs);
        let sorted_rhs = merge_sort(rhs);

        // combine
        let mut res = vec![];
        let mut i = 0;
        let mut j = 0;
        while i < sorted_lhs.len() && j < sorted_rhs.len() {
            if sorted_lhs[i] <= sorted_rhs[j] {
                res.push(sorted_lhs[i]);
                i += 1;
            } else {
                res.push(sorted_rhs[j]);
                j += 1;
            }
        }
        if i == sorted_lhs.len() {
            res.extend(sorted_rhs[j..].iter());
        } else {
            res.extend(sorted_lhs[i..].iter());
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let arr = vec![1, 5, 10, 12, 6, 9];
        let sorted_arr = merge_sort(&arr);

        let expected = vec![1, 5, 6, 9, 10, 12];
        assert!(sorted_arr == expected);
    }
}
