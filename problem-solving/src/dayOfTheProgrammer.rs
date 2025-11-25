fn dayOfProgrammer(year: i32) -> String {
    if (year == 1918) {
        return "26.09.1918".to_string();
    }
    else if ((year <= 1917) && (year % 4 == 0)) || ((year % 400 == 0) || (year % 4 == 0) && (year % 100 != 0)) {
        return (format!("12.09.{}", year));
    } else {
        return (format!("13.09.{}", year));
    }
}