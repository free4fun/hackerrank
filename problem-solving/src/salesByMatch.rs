fn sockMerchant(n: i32, ar: &[i32]) -> i32 {
    let mut sock_color = vec![false; 101];
    let mut result = 0;
    for i in 0..n {
        let color = ar[i as usize] as usize;
        if sock_color[color] { // second sock of this color
            result += 1;
            sock_color[color] = false;
        } else  {
            sock_color[color] = true; // first sock of this color
        }
    }
    return result;
}