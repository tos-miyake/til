fn foo(&(a,b): &(i32,i32)) {
  println!("{},{}",a,b);
}

fn main() {
  let x = (1,2);
  foo(&x);
}
