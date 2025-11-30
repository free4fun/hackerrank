fn designerPdfViewer(h: &[i32], word: &str) -> i32 {
    let max_height = word
        .chars()
        .map(|c| h[(c as u8 - b'a') as usize])
        .max()
        .unwrap_or(0);
    let result = max_height * word.len() as i32;
    return result;
}