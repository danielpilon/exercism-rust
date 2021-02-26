pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|n| has_multiple(n, factors))
        .fold(0, |acc, m| acc + m)
}

fn has_multiple(n: &u32, factors: &[u32]) -> bool {
    factors.iter().any(|f| *f > 0 && n % *f == 0)
}
