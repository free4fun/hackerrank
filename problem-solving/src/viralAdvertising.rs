fn viralAdvertising(n: i32) -> i32 {
    let mut result = 0;
    let mut people = 5;
    for day in 0..n {
        people = people/2;
        result += people;
        people = 3*people;
    }
    return result;
}