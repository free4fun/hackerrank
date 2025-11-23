fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    let x = x1 - x2;
    let v = v2 - v1;
    if (x != 0) && (v == 0){
        return("NO").to_string();
    } else if (x < 0) && (v > 0) {
        return("NO").to_string();
    } else if (x > 0) && (v < 0) {
        return("NO").to_string();
    } else if ((x1-x2)%(v2-v1)) == 0 {
        return("YES").to_string();
    } else {
        return("NO").to_string();
    }
}