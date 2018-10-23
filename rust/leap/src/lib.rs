pub fn is_leap_year(year: i32) -> bool {

    if year % 4 != 0 {
        return false;
    }
    if year % 100 != 0 {
        return true;
    }
    return year % 400 == 0;
}
