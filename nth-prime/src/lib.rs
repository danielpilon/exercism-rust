pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    (2..)
        .filter(|num| is_prime(*num))
        .nth((n - 1) as usize)
        .unwrap()
}

fn is_prime(num: u32) -> bool {
    if num % 2 == 0 {
        false
    } else {
        until_next_prime(num, 3)
    }
}

fn until_next_prime(num: u32, divisor: u32) -> bool {
    if num < (divisor * divisor) {
        true
    } else if num % divisor == 0 {
        false
    } else {
        until_next_prime(num, divisor + 2)
    }
}
