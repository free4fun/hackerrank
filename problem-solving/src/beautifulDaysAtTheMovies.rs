fn beautifulDays(i: i32, j: i32, k: i32) -> i32 {
    let mut result = 0;
    for elem in i..=j {
        let reverse: String = elem.to_string().chars().rev().collect();
        if (elem - reverse.parse::<i32>().unwrap()) % k == 0 {
            result +=1;
        }
    }
    return result;
}