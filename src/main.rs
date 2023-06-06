fn main() {
    for n in 1..=100 {
        if n % 3 == 0 && n % 5 == 0 {
            println!("CracklePop");
            continue;
        }
        if n % 3 == 0 {
            println!("Crackle");
            continue;
        }
        if n % 5 == 0 {
            println!("Pop");
            continue;
        }
        println!("n: {}", n);
    }
}
