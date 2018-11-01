mod data;

fn main() {
    // I'll build a vec with the sum of each digit from the different numbers
    let mut sums = Vec::new();
    sums.resize(11, 0);
    // For each number
    for line in data::DIGITS.lines() {
        let mut counter = 0;
        // we only care about the first 11 digits
        if let Some(l) = line.trim().get(0..sums.len()) {
            // For each digit
            for c in l.chars() {
                match c.to_digit(10) {
                    Some(n) => {
                        sums[counter] += n;
                        counter += 1;
                    }
                    None => (),
                }
            }
        }
    }
    // println!("Sum of the first 11 digits is: {:?}", sums);

    let mut result = Vec::new();
    let mut rest = 0;
    loop {
        let n = sums.pop();
        match n {
            Some(n) => {
                let r = (n + rest) % 10;
                result.push(r.to_string());
                rest = (n + rest) / 10;
            }
            None => {
                result.push(rest.to_string());
                break;
            }
        }
    }
    result.reverse();
    let result_string = result.join("");
    println!("The result is {}", result_string);
}
