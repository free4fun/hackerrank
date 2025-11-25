fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut max = 0;
    let mut result = 0i32;
    let mut sightedTimes = vec![0i32; 5];
    for elem in arr {
        let index = (elem - 1) as usize;
        sightedTimes[index] += 1;
        if sightedTimes[index] > max {
            max = sightedTimes[index];
            result = *elem;
        } else if (sightedTimes[index] == max) {
            if result > *elem {
                result = *elem;
            }
        } 
    }
    return result;
}