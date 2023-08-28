fn heapify(arr: &mut Vec<i32>, n: usize, i: usize) {
    // Find largest among root, left child and right child
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && arr[left] > arr[largest] {
        largest = left;
    }

    if right < n && arr[right] > arr[largest] {
        largest = right;
    }

    // swap and continue heapifying if root is not largest
    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heapify() {
        let mut arr = vec![1, 10, 3];
        let n = arr.len();
        heapify(&mut arr, n, 0);

        assert!(arr == vec![10, 1, 3]);
    }
}
