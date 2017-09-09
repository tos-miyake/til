fn find_if<T>(pred: fn(&T) -> bool, table: &[T]) -> Option<&T> {
    for x in table {
        if pred(x) {return Some(x); }
    }
    None
}

fn main() {
    fn eventp(x: &i32) -> bool { x % 2 == 0 }
    fn oddp(x: &i32) -> bool { x % 2 != 0 }
    let a = [2, 4, 6, 8, 10];
    match find_if(eventp, &a) {
        Some(v) => println!("{}",v),
        None => println!("None")
    }
    match find_if(oddp, &a) {
        Some(v) => println!("{}", v),
        None => println!("None")
    }
}
