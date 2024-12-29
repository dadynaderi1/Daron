use std::io::Read;

fn main() {
    for b in std::io::stdin().bytes() {
        let c = b.unwrap() as char;
        if c == 'q' {
            break;
        }
        print!("{}", c);
    }
}
