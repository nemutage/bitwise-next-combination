fn next_combination(sub: i8) -> i8 {
    let least = sub & -sub;
    let left = sub + least;
    let right = ((sub & !left) / least) >> 1;
    left | right
}

fn main() {
    let n = 5;
    let k = 3;
    let mut bit = (1 << k) - 1;

    while bit < (1 << n) {
        println!("{:05b}", bit);
        bit = next_combination(bit);
    }
}
