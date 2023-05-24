
fn is_match(s: String, p: String) -> bool {
    if s.len() == 0 {
        return true
    }

    println!("{s}");
    println!("{p}");
    true
}


fn main() {
    let str = String::from("fafavaf");
    let p = String::from(".*");
    println!("{}", is_match(str, p));
}