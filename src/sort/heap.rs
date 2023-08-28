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

fn heap_sort(arr: &mut Vec<i32>) {
    let n = arr.len();

    // build max-heap
    for i in (0..(n / 2) - 1).rev() {
        heapify(arr, n, i);
    }

    for i in (0..n).rev() {
        arr.swap(0, i);

        // heapify the root element to get highest element at root origin
        heapify(arr, i, 0);
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

    #[test]
    fn test_heap_sort() {
        let mut arr = vec![1, 12, 9, 5, 6, 10];
        heap_sort(&mut arr);

        assert!(arr == vec![1, 5, 6, 9, 10, 12]);
    }
}
