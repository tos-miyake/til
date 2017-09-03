fn main() {
  let ref x = 100;
  match x {
    &y => println!("{}",y)
  }
  match *x {
    y => println!("{}", y)
  }
  let mut z = 200;
  match z {
    ref mut y => {
      *y = 300;
      println!("{}",y)
    }
  }
}
