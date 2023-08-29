fn heapify(arr: &mut Vec<i32>, n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && arr[left] > arr[largest] {
        largest = left;
    }

    if right < n && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

fn insert_heap(arr: &mut Vec<i32>, v: i32) {
    if arr.is_empty() {
        arr.push(v);
    } else {
        arr.push(v);

        let n = arr.len();
        for i in (0..(n / 2)).rev() {
            heapify(arr, n, i);
        }
    }
}

fn delete_heap(arr: &mut Vec<i32>, v: i32) {
    let id = arr
        .iter()
        .position(|x| *x == v)
        .expect("Not found value in heap");

    let n = arr.len();
    arr.swap(id, n - 1);
    arr.pop();

    let n = arr.len();
    for i in (0..(n / 2)).rev() {
        heapify(arr, n, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heapify() {
        let mut arr = vec![3, 9, 2, 1, 4, 5];
        let n = arr.len();

        for i in (0..(n / 2)).rev() {
            heapify(&mut arr, n, i);
        }

        assert!(arr == vec![9, 4, 5, 1, 3, 2]);
    }

    #[test]
    fn test_insert_heap() {
        let mut arr = vec![];
        insert_heap(&mut arr, 1);
        assert!(arr == vec![1]);

        let mut arr1 = vec![9, 4, 5, 1, 3, 2];
        insert_heap(&mut arr1, 7);
        assert!(arr1 == vec![9, 4, 7, 1, 3, 2, 5]);
    }

    #[test]
    fn test_delete_heap() {
        let mut arr = vec![9, 4, 5, 1, 3, 2];
        delete_heap(&mut arr, 3);
        assert!(arr == vec![9, 4, 5, 1, 2]);

        delete_heap(&mut arr, 2);
        assert!(arr == vec![9, 4, 5, 1]);

        delete_heap(&mut arr, 4);
        assert!(arr == vec![9, 1, 5]);
    }
}
