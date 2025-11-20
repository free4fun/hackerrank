fn gradingStudents(grades: &[i32]) -> Vec<i32> {
    let mut res : Vec<i32> = Vec::new();
    for original in grades {
        if (original < &38) {
            res.push(*original);
        } else {
            if (original % 5 < 3) {
                let mut num = *original;
                while (num % 5 != 0) {
                    num +=1;
                }
                res.push(num);
            }
        }
    }
    return res;
}