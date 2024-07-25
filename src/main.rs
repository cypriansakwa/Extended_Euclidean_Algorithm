fn main() {
    let a = 1420778888777788778; // First number
    let b = 71378678965888776656; // Second number

    let (gcd, x, y) = extended_euclidean(a, b);

    println!("The GCD of {} and {} is {}", a, b, gcd);
    println!("The coefficients x and y are {} and {} respectively", x, y);
}

fn extended_euclidean(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        return (a, 1, 0);
    }

    let (gcd, x1, y1) = extended_euclidean(b, a % b);

    let x = y1;
    let y = x1 - (a / b) * y1;

    (gcd, x, y)
}