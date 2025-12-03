fn circularArrayRotation(a: &[i32], k: i32, queries: &[i32]) -> Vec<i32> {
    let mut a: Vec<i32> = a.to_vec();
    for i in 0..(k % (a.len() as i32)) {
        let last = a.pop().unwrap();
        a.insert(0, last);
    }
    let mut result: Vec<i32> = Vec::new();
    for i in queries {
        result.push(a[*i as usize]);
    }
    return result;
}