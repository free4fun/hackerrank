fn pageCount(n: i32, p: i32) -> i32 {
    if n/2 < p { //from the end
        return n / 2 - p / 2;
    } else { //from beginning
        return p / 2;
    }
}