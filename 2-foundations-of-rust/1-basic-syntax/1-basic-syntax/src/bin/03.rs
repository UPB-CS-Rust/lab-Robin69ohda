fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    // TODO
    let mut a = 0;
    let mut b = 99;
    for i in 0..8 {
        if input[i] > a {
            a = input[i];
        } else if input[i] < b {
            b = input [i];
        }

    }

    println!("{} is largest and {} is smallest", a, b);
}
