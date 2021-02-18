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
}
