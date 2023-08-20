fn selection_sort(arr: &mut Vec<i32>) {
    let n = arr.len();

    for i in 0..(n - 1) {
        let mut min_idx = i;
        for j in (i + 1)..n {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        arr.swap(i, min_idx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut arr = vec![-2, 45, 0, 11, -9];
        selection_sort(&mut arr);

        let expected = vec![-9, -2, 0, 11, 45];
        assert!(arr == expected);
    }
}
