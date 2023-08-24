fn counting_sort(arr: &mut Vec<i32>, size: usize, place: usize) {
    let mut output = vec![0; size + 1];
    let max = *arr.iter().max().unwrap();

    let mut count = vec![0; max as usize + 1];

    // Calculate the count of elements
    for i in 0..size {
        count[((arr[i] as usize) / place) % 10] += 1;
    }

    // Calculate cumulative count
    for i in 1..10 {
        count[i] += count[i - 1];
    }

    // Place the elements in sorted order
    for i in (0..size).rev() {
        output[count[((arr[i] as usize) / place) % 10] - 1] = arr[i];
        count[((arr[i] as usize) / place) % 10] -= 1;
    }

    for i in 0..size {
        arr[i] = output[i];
    }
}

fn radix_sort(arr: &mut Vec<i32>, size: usize) {
    let max = *arr.iter().max().unwrap();

    // Apply counting sort to sort elements based on place value.
    let mut place = 1;
    while max / place > 0 {
        counting_sort(arr, size, place as usize);
        place *= 10;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radix_sort() {
        let mut arr = vec![121, 434, 564, 23, 1, 45, 788];
        let size = arr.len();
        radix_sort(&mut arr, size);

        let expected = vec![1, 23, 45, 121, 434, 564, 788];
        assert!(arr == expected);
    }
}
