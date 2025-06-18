#![forbid(unsafe_code)]

pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {

    if k == 0 {
        return vec![vec![]];
    }

    let n = arr.len();
    
    if k > n {
        return vec![];
    }
    let mut result = combinations(&arr[1..], k - 1);

    for x in &mut result {
        x.insert(0, arr[0]);
    }

    result.extend(combinations(&arr[1..], k));
    result
}
