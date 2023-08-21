use std::collections::HashMap;

fn fibonacci_using_dynamic_top_down(n: usize) -> usize {
    let mut memo: HashMap<usize, usize> = HashMap::new();
    memo.insert(0, 0);
    memo.insert(1, 1);

    fn fib(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
        if !memo.contains_key(&n) {
            let res = fib(n - 1, memo) + fib(n - 2, memo);
            memo.insert(n, res);
        }

        *memo.get(&n).unwrap()
    }

    let res = fib(n, &mut memo);
    res
}

fn fibonacci_using_dynamic_bottom_up(n: usize) -> usize {
    let mut memo: Vec<usize> = vec![0; n + 1];
    memo[0] = 0;
    memo[1] = 1;

    for i in 2..(n + 1) {
        memo[i] = memo[i - 1] + memo[i - 2];
    }
    memo[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_1() {
        let res = fibonacci_using_dynamic_top_down(10);
        assert!(res == 55);
    }

    #[test]
    fn test_fib_2() {
        let res = fibonacci_using_dynamic_bottom_up(10);
        assert!(res == 55);
    }
}
