fn greatestCommonDivisor(mut a:i32, mut b:i32) -> i32 {
    while (b > 0) {
        let temp = b;
        b = a%temp;
        a = temp;
    }
    return a;
}

fn greatestCommonDivisorInArray(arr:&[i32]) -> i32 {
    let mut result = arr[0];
    for elem in arr {
        result = greatestCommonDivisor(result, *elem);
    }
    return result;
}

fn leastCommonMultiple(a: i32, b: i32) -> i32 {
    return (a*b/greatestCommonDivisor(a,b));
}

fn leastCommonMultipleInArray(arr: &[i32]) -> i32 {
    let mut result = arr[0];
    for elem in arr {
        result = leastCommonMultiple(result, *elem);
    }
    return result;
}

fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    let mut lcm = leastCommonMultipleInArray(a);
    let mut gcd = greatestCommonDivisorInArray(b);
    let mut counter = 0;
    let mut multiple_of_lcm = lcm;
    while multiple_of_lcm <= gcd {
        if (gcd%multiple_of_lcm == 0) {
            counter += 1;
        }
        multiple_of_lcm += lcm;
    }
    return counter;
}