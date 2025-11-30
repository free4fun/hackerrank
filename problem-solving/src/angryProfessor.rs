fn angryProfessor(k: i32, a: &[i32]) -> String {
    let mut counter = 0;
    for elem in a {
        if *elem <= 0 {
            counter += 1;
        }
        if k <= counter {
            return("NO").to_string();
        }
    }
    return("YES").to_string()
}