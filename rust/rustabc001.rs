fn main() {
  let a = 123;
  let ra = &a;
  let al = ra * 10;
  println!("{}", ra);
  println!("{}", al);

  let mut b = 456;
  println!("{}", b);
  {
    let mut rb = &mut b;
    //let rb1 = &b;
    //println!("{}", b);
    println!("{}", rb);
    *rb = 1000;
  }
  println!("{}", b);
}
