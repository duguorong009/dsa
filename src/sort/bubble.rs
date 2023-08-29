fn bubble_sort(arr: &mut Vec<i32>) {
    let n = arr.len();

    // loop to access each array element
    for i in 0..n - 1 {
        // check if swapping occurs
        let mut swapped = false;

        // loop to compare array element
        for j in 0..(n - i - 1) {
            // compare 2 adjacent elements
            // change > to < to sort in descending order
            if arr[j] > arr[j + 1] {
                // swapping occurs if elements are not in
                // the intended order
                arr.swap(j, j + 1);

                swapped = true;
            }
        }

        // no swapping means the array is already sorted
        // so no need for further comparison
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = vec![-2, 45, 0, 11, -9];
        bubble_sort(&mut arr);

        assert!(arr == vec![-9, -2, 0, 11, 45]);
    }
}
