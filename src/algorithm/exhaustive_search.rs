const MAX: i32 = 20_000_000;

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

#[allow(dead_code)]
fn chapter_problem3_1(a: Vec<i32>, v: i32) -> i32 {
    let mut founded_id = -1;
    for (i, n) in a.iter().enumerate() {
        if v == *n {
            founded_id = i as i32;
        }
    }

    founded_id
}

#[allow(dead_code)]
fn chapter_problem3_2(a: Vec<i32>, v: i32) -> i32 {
    let mut cnt = 0;
    for n in a {
        if v == n {
            cnt += 1;
        }
    }

    cnt
}

#[allow(dead_code)]
fn chapter_problem3_3(a: Vec<i32>) -> i32 {
    let mut min = MAX;
    let mut second_min = MAX;
    for n in a {
        if min > n {
            second_min = min;
            min = n;
        }
    }

    second_min
}

#[allow(dead_code)]
fn chapter_problem3_4(a: Vec<i32>) -> i32 {
    let mut min = MAX;
    let mut max = 0;
    for n in a {
        if min > n {
            min = n;
        }

        if n > max {
            max = n;
        }
    }

    max - min
}

#[allow(dead_code)]
fn chapter_problem3_5(mut a: Vec<i32>) -> i32 {
    let mut cnt = 0;

    while a.iter().all(|n| n % 2 == 0) {
        a = a.iter().map(|n| n / 2).collect();
        cnt += 1;
    }

    cnt
}

#[allow(dead_code)]
fn chapter_problem3_6(k: i32, n: i32) -> i32 {
    let mut cnt = 0;
    for x in 0..k + 1 {
        for y in 0..k + 1 {
            let z = n - (x + y);
            if 0 <= z && z <= k {
                cnt += 1;
            }
        }
    }

    cnt
}

#[allow(dead_code)]
fn chapter_problem3_7(n: i64) -> i64 {
    let s = n.to_string();

    let mut sum = 0;
    for bit in 0..1 << (s.len() - 1) {
        let mut tmp: i64 = 0;

        for i in 0..(s.len() - 1) {
            tmp *= 10;
            tmp += s.chars().nth(i).unwrap() as i64 - '0' as i64;

            if (bit & (1 << i)) != 0 {
                sum += tmp;

                tmp = 0;
            }
        }

        tmp *= 10;
        tmp += s.chars().nth(s.len() - 1).unwrap() as i64 - '0' as i64;
        sum += tmp;
    }

    sum
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

    #[test]
    fn test_chapter_problem3_1() {
        assert_eq!(chapter_problem3_1(vec![4, 3, 12, 7, 11, 7], 7), 5);
        assert_eq!(chapter_problem3_1(vec![4, 3, 12, 7, 11], 8), -1);
    }

    #[test]
    fn test_chapter_problem3_2() {
        assert_eq!(chapter_problem3_2(vec![4, 3, 12, 11], 7), 0);
        assert_eq!(chapter_problem3_2(vec![4, 3, 12, 7, 11], 7), 1);
        assert_eq!(chapter_problem3_2(vec![4, 3, 12, 7, 11, 7], 7), 2);
    }

    #[test]
    fn test_chapter_problem3_3() {
        assert_eq!(chapter_problem3_3(vec![4, 3, 12, 11]), 4);
        assert_eq!(chapter_problem3_3(vec![4, 3, 12, 11, 2]), 3);
    }

    #[test]
    fn test_chapter_problem3_4() {
        assert_eq!(chapter_problem3_4(vec![4, 3, 12, 11]), 9);
        assert_eq!(chapter_problem3_4(vec![4, 3, 12, 11, 2]), 10);
    }

    #[test]
    fn test_chapter_problem3_5() {
        assert_eq!(chapter_problem3_5(vec![8, 12, 24, 32]), 2);
        assert_eq!(chapter_problem3_5(vec![4, 3, 12, 11, 2]), 0);
    }

    #[test]
    fn test_chapter_problem3_6() {
        assert_eq!(chapter_problem3_6(2, 2), 6);
        assert_eq!(chapter_problem3_6(5, 15), 1);
    }

    #[test]
    fn test_chapter_problem3_7() {
        assert_eq!(chapter_problem3_7(125), 176);
        assert_eq!(chapter_problem3_7(9999999999), 12656242944);
    }
}
