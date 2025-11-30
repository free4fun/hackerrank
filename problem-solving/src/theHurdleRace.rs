fn hurdleRace(k: i32, height: &[i32]) -> i32 {
    // Find the maximum height using iterator
    let max_height = *height.iter().max().unwrap_or(&0);
    // Return the number of doses needed, or 0 if not needed
    (max_height - k).max(0)
}