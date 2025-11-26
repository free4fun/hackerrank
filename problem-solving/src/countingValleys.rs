fn countingValleys(steps: i32, path: &str) -> i32 {
    let mut valleys = 0;
    let mut level = 0;
    for step in path.chars() {
        if step == 'U' {
            level += 1;
            if level == 0 {  // if we arrive again to sea level we just leaved a valley
                valleys += 1;
            }
        } else {
            level -= 1;
        }
    }
    return valleys
}