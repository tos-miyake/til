fn find<T:PartialEq + Copy>(key: T, data: &[T]) -> bool {
    for &x in data {
        if key == x { return true; }
    }
    false
}

fn assoc<T:PartialEq + Copy, U>(key: T, data: &[(T, U)]) -> Option<&U> {
    for &(x, ref v) in data {
        if key == x { return Some(v); }
    }
    None
}

fn main() {
    let data = [1,2,3,4,5,6,7,8];
    println!("{}", find(8, &data));
    println!("{}", find(0, &data));

    let data1 = ["foo", "bar", "baz"];
    println!("{}", find("baz", &data1));
    println!("{}", find("oops", &data1));

    let data2 = [("foo", 100), ("bar", 200), ("baz", 300)];
    match assoc("baz", &data2) {
        Some(v) => println!("{}", v),
        None => println!("None")
    }
    match assoc("oops", &data2) {
        Some(v) => println!("{}", v),
        None => println!("None")
    }
}
