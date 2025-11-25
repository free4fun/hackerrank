fn birthday(s: &[i32], d: i32, m: i32) -> i32 {
    let mut result = 0;
    for i in 0..s.len() {
        let mut sum = 0;
        let mut j = 0;    
        while (i+j < s.len() && j < m as usize) {
            sum = sum + s[i+j as usize];
            j += 1;
        }
        if sum == d {
            result += 1;
        }
    }
    return result;
}