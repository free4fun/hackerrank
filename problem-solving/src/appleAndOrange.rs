fn countApplesAndOranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let mut houseA = 0;
    let mut houseO = 0;
    for element in apples {
        let mut distance = element + a;
        if (distance >= s && distance <= t) {
            houseA+=1;
        }
    }
    for element in oranges {
        let mut distance = element + b;
        if (distance >= s && distance <= t) {
            houseO+=1;
        }
    }
    println!("{}", houseA);
    println!("{}", houseO);
}
