fn insertion_sort(arr: &mut Vec<i32>) {
    let n = arr.len();

    for i in 1..n {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut arr = vec![-2, 45, 0, 11, -9];
        insertion_sort(&mut arr);

        let expected = vec![-9, -2, 0, 11, 45];
        assert!(arr == expected);
    }
}
