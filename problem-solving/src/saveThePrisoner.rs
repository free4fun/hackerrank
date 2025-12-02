fn saveThePrisoner(n: i32, m: i32, s: i32) -> i32 { 
    return(((s + m - 2 ) % n) + 1);
}