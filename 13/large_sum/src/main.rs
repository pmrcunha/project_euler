mod data;

fn main() {
    // I'll build a vec with the sum of each digit from the different numbers
    let mut sums = Vec::new();
    sums.resize(50, 0); // must consider all digits to be correct

    // For each number
    for line in data::DIGITS.lines() {
        let mut index = 0;
        // I thought I only cared about the first 11 digits (and it works), but I need them all to be correct
        if let Some(l) = line.trim().get(0..sums.len()) {
            // For each digit
            for c in l.chars() {
                match c.to_digit(10) {
                    Some(n) => {
                        sums[index] += n; // in each element of the vector, I add all digits at this index
                        index += 1;
                    }
                    None => (),
                }
            }
        }
    }
    // println!("Sum of each digit of each number is: {:?}", sums);

    let mut result = Vec::new();
    let mut rest = 0;
    loop {
        let n = sums.pop();
        match n {
            Some(n) => {
                /* I add the sum of the digits to the eventual rest carried from
                 * the previous value. Each sum of digits is a decimal value of the
                 * final result, so I calculate the modulo to get the unit for that
                 * decimal value, and divide by 10 to get the rest that should be
                 * carried to the higher decimal value.
                 * (basically I calculate the addition as I would do by hand)
                 */

                let r = (n + rest) % 10;
                result.insert(0, r.to_string()); // I could also push and use result.reverse() later
                rest = (n + rest) / 10;
            }
            None => {
                result.insert(0, rest.to_string()); // The rest of the last value has to be carried over to the next decimal
                break;
            }
        }
    }
    if let Some(result_string) = result.join("").get(0..10) {
        // trim the result to the first 10 digits
        println!("The result is {}", result_string);
    }
}
