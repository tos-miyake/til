#[derive(Debug, PartialEq)]
struct Pair<T, U> {
    fst: T, snd: U
}

impl <T, U> Pair<T,U> {
    fn new(a: T, b: U) -> Pair<T,U> {
        Pair { fst: a, snd: b }
    }
}

fn main() {
    let p1 = Pair::new(1,2.0);
    let p2 = Pair::new(1,3.0);
    let p3 = Pair::new("foo", 100);
    println!("{:?}", p1);
    println!("{:?}", p2);
    println!("{:?}", p3);
    println!("{}", p1 == p2);
    println!("{}", p3 == p3);
}
