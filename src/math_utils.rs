pub const fn binomial_coefficient(n: usize, k: usize) -> usize {
    if n < k {
        return 0;
    }

    factorial(n) / ((factorial(n - k)) * factorial(k))
}

const fn factorial(n: usize) -> usize {
    match n {
        0usize | 1usize => 1,
        2usize..=20usize => factorial(n - 1usize) * n,
        _ => 0,
    }
}
