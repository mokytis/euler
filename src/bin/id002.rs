use euler::utils::seq::Fibonacci;

fn solve(limit: u32) -> u32 {
   Fibonacci::new()
        .take_while(|n| n < &limit)
        .filter(|n| n % 2 == 0)
        .sum()
}

fn main() {
    let ans = solve(4000000);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    #[test]
    fn fib_sum_even_under_100() {
        assert_eq!(44, super::solve(100));
    }
}
