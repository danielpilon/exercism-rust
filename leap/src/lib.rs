pub fn is_leap_year(year: u64) -> bool {
    let is_div_by = |x: u64, y: u64| x % y == 0;
    is_div_by(year, 400) || (!is_div_by(year, 100) && is_div_by(year, 4))
}
