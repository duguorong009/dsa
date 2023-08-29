use std::cmp::Ordering;

fn bucket_sort(arr: &mut Vec<f32>) {
    let n = arr.len();

    // Create n buckets
    let mut buckets = vec![vec![]; n];

    // Insert elements into their respective buckets
    for i in 0..n {
        let idx = (10.0 * arr[i]) as usize;
        buckets[idx].push(arr[i]);
    }

    // Sort the elements in each bucket
    for i in 0..n {
        buckets[i].sort_by(|a, b| {
            if *a < *b {
                Ordering::Less
            } else if *a == *b {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        });
    }

    // Get the sorted element
    for (i, elem) in buckets.concat().into_iter().enumerate() {
        arr[i] = elem;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_sort() {
        let mut arr = vec![0.42, 0.32, 0.23, 0.52, 0.25, 0.47, 0.51];
        let expected = vec![0.23, 0.25, 0.32, 0.42, 0.47, 0.51, 0.52];

        bucket_sort(&mut arr);

        assert!(arr == expected);
    }
}
