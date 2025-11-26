fn climbingLeaderboard(ranked: &[i32], player: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut ranked = ranked.to_vec();
    ranked.dedup(); // Remove duplicate scores from ranked (must be sorted for dedup to work correctly)
    for score in player.to_vec() {
        // Remove all ranked scores less than or equal to the player's score
        while let Some(&last) = ranked.last() {
            if score >= last {
                ranked.pop();
            } else {
                break;
            }
        }
        // The player's rank is the number of remaining ranked scores + 1
        result.push((ranked.len() + 1) as i32);
    }
    return result;
}