fn count_sort(arr: &Vec<i32>) -> Vec<i32> {
    let size = arr.len();
    if size < 2 {
        return arr.clone();
    }

    let mut output = vec![0; size];

    let max = *arr.iter().max().unwrap();
    let mut count = vec![0; (max + 1) as usize];

    for i in 0..size {
        count[arr[i] as usize] += 1;
    }

    for i in 1..=(max as usize) {
        count[i] += count[i - 1];
    }

    for i in (0..size).rev() {
        output[count[arr[i] as usize] - 1] = arr[i];
        count[arr[i] as usize] -= 1;
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_sort() {
        let arr = vec![4, 2, 2, 8, 3, 3, 1];
        let sorted = count_sort(&arr);

        let expected = vec![1, 2, 2, 3, 3, 4, 8];
        assert!(sorted == expected);
    }
}
