fn meets_input(num: i64) -> bool {
    // Move from right to left
    let mut tmp = num;
    let mut prev_digit = tmp % 10;
    tmp = tmp / 10;
    let mut has_pair = false;
    let mut prev_prev_digit: i64 = -1;

    while tmp > 0 {
        let current_digit = tmp % 10;
        // make sure digit is increasing
        if current_digit > prev_digit {
            return false;
        }
        // Check if the digit is in a pair not in a larger group
        if current_digit == prev_digit {
            let next_digit = tmp / 10 % 10;
            if prev_digit != prev_prev_digit && current_digit != next_digit {
                has_pair = has_pair || true;
            }
        }

        tmp = tmp / 10;
        prev_prev_digit = prev_digit;
        prev_digit = current_digit;
    }

    has_pair
}

fn main() {
    let mut count = 0;
    for combination in 168630..=718098 {
        if meets_input(combination) {
            count += 1;
        }
    }
    println!("count: {}", count);
}