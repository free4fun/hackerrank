pub fn pickingNumbers(a: &[i32]) -> i32 {
    let mut freq = vec![0; 101];
    for num in a {
        freq[*num as usize] += 1;
    }
    let mut result = 1;
    for i in 0..100 {
        let temp = freq[i] + freq[i + 1];
        if temp > result {
            result = temp;
        }
    }
    return result;
}