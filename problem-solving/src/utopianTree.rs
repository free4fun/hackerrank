fn utopianTree(n: i32) -> i32 {
    let mut result = 1;
    for i in 0..n {
        if i%2 == 0 {
            result = result * 2;
        } else {
            result += 1;
        }
    }
    return result;
}