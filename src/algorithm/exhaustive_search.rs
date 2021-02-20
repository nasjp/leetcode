#[allow(dead_code)]
fn liner_exist(a: Vec<i32>, v: i32) -> bool {
    for n in a {
        if v == n {
            return true;
        }
    }

    false
}

#[allow(dead_code)]
fn liner_index(a: Vec<i32>, v: i32) -> i32 {
    for (i, n) in a.iter().enumerate() {
        if v == *n {
            return i as i32;
        }
    }

    -1
}

#[allow(dead_code)]
fn liner_min(a: Vec<i32>) -> i32 {
    const MAX: i32 = 20_000_000;
    let mut min = MAX;

    for n in a {
        if min > n {
            min = n
        }
    }

    min
}

#[allow(dead_code)]
fn liner_pair(a: Vec<i32>, b: Vec<i32>, k: i32) -> i32 {
    for n in &a {
        for m in &b {
            if n + m >= k {
                return n + m;
            }
        }
    }

    -1
}

#[allow(dead_code)]
fn liner_combination(a: Vec<i32>, w: i32) -> bool {
    // 2^N 通りの部分集合
    for bit in 0..(1 << a.len()) {
        // 部分集合に含まれる要素の和
        let mut sum = 0;

        for (i, n) in a.iter().enumerate() {
            // a[i] が部分集合に含まれているかどうか
            if (bit & (1 << i)) != 0 {
                sum += n
            }
        }

        if sum == w {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_liner_exist() {
        assert!(liner_exist(vec![4, 3, 12, 7, 11], 7));
        assert!(!liner_exist(vec![4, 3, 12, 7, 11], 8));
    }

    #[test]
    fn test_liner_index() {
        assert_eq!(liner_index(vec![4, 3, 12, 7, 11], 7), 3);
        assert_eq!(liner_index(vec![4, 3, 12, 7, 11], 8), -1);
    }

    #[test]
    fn test_liner_min() {
        assert_eq!(liner_min(vec![4, 3, 12, 7, 11]), 3);
        assert_eq!(liner_min(vec![4, 3, 12, 7, 11, 2]), 2);
    }

    #[test]
    fn test_liner_pair() {
        assert_eq!(liner_pair(vec![8, 5, 4], vec![4, 1, 9], 10), 12);
    }

    #[test]
    fn test_liner_combination() {
        assert_eq!(liner_combination(vec![1, 2, 4, 5, 11], 10), true);
        assert_eq!(liner_combination(vec![1, 5, 8, 11], 10), false);
    }
}
