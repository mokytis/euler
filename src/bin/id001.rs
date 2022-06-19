fn is_multiple_3_5(num: u32) -> bool {
    return num % 3 == 0 || num % 5 == 0;
}

fn solve(num: u32) -> u32 {
    let mut sum = 0;
    for i in 1..num {
        if is_multiple_3_5(i) {
            sum += i;
        }
    }
    return sum;
}

fn main() {
    let ans = solve(1000);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    #[test]
    fn sum_below_ten() {
        assert_eq!(23, super::solve(10));
    }
}
