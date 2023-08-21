fn quick_sort(arr: &mut Vec<i32>, left_most_idx: isize, right_most_idx: isize) {
    if left_most_idx < right_most_idx {
        let pivot_idx = partition(arr, left_most_idx, right_most_idx);
        quick_sort(arr, left_most_idx, pivot_idx - 1);
        quick_sort(arr, pivot_idx, right_most_idx);
    }
}

fn partition(arr: &mut Vec<i32>, left_most_idx: isize, right_most_idx: isize) -> isize {
    let pivot_idx = right_most_idx;

    let mut store_idx = left_most_idx - 1;
    for i in left_most_idx..right_most_idx {
        if arr[i as usize] < arr[pivot_idx as usize] {
            store_idx += 1;
            arr.swap(i as usize, store_idx as usize);
        }
    }
    arr.swap(pivot_idx as usize, (store_idx + 1) as usize);
    store_idx + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut arr = vec![8, 7, 6, 1, 0, 9, 2];
        quick_sort(&mut arr, 0, 6);

        println!("arr: {arr:?}");

        let expected = vec![0, 1, 2, 6, 7, 8, 9];
        assert!(arr == expected);
    }
}
