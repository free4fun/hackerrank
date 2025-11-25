fn bonAppetit(bill: &[i32], k: i32, b: i32) {
    let mut result = 0;
    for i in 0..bill.len() {
        if i != k as usize {
            result += bill[i];
        }
    }
    result = result/2;
    if result == b {
        println!("Bon Appetit");
    } else {
        println!("{}",b-result);
    }
}