pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut steps = 0;

    while n != 1 {
        // Check for potential overflow before performing arithmetic operations
        if n % 2 == 0 {
            // Check if dividing n by 2 will result in overflow
            if let Some(new_n) = n.checked_div(2) {
                n = new_n;
            } else {
                return None; // Return None if overflow occurs
            }
        } else {
            // Check if multiplying n by 3 and adding 1 will result in overflow
            if let Some(new_n) = n.checked_mul(3).and_then(|x| x.checked_add(1)) {
                n = new_n;
            } else {
                return None; // Return None if overflow occurs
            }
        }
        steps += 1;
    }

    Some(steps)
}
