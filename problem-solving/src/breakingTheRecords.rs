fn breakingRecords(scores: &[i32]) -> Vec<i32> {
    let mut counterLowest = 0;
    let mut counterHighest = 0;
    let mut lowestScore = scores[0];
    let mut highestScore = scores[0];
    for elem in scores {
        if lowestScore > *elem {
            counterLowest += 1;
            lowestScore = *elem;
        }
        if highestScore < *elem {
            counterHighest +=1;
            highestScore = *elem;
        }
    }
    return ([counterHighest,counterLowest].to_vec());

}