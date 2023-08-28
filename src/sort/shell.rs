fn shell_sort(arr: &mut Vec<i32>, n: usize) {
    let mut interval = n / 2;
    while interval > 0 {
        for i in interval..n {
            let temp = arr[i];
            let mut j = i;
            while j >= interval && arr[j - interval] > temp {
                arr[j] = arr[j - interval];
                j -= interval;
            }
            arr[j] = temp;
        }

        interval /= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_sort() {
        let mut arr = vec![9, 8, 3, 7, 5, 6, 4, 1];
        let n = arr.len();
        shell_sort(&mut arr, n);

        assert!(arr == vec![1, 3, 4, 5, 6, 7, 8, 9]);
    }
}
