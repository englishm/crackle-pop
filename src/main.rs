fn crackle_pop(n: i32) -> String {
    let mut buf = String::with_capacity("CracklePop\n".len());

    if n % 3 == 0 {
        buf.push_str("Crackle")
    }
    if n % 5 == 0 {
        buf.push_str("Pop")
    }
    if buf.len() == 0 {
        buf.push_str(&n.to_string())
    }
    buf.push('\n');
    return buf;
}

fn main() {
    for n in 1..=100 {
        print!("{}", crackle_pop(n));
    }
}
