fn divisibleSumPairs(n: i32, k: i32, ar: &[i32]) -> i32 {
    let mut result = 0;
    let mut arRest = vec![0i32; k as usize];
    for elem in ar {
        let rest = ((elem % k) + k) % k;
        let complement = (k - rest) % k;
        result += arRest[complement as usize];
        arRest[rest as usize] +=1;
    }
    return result;
}