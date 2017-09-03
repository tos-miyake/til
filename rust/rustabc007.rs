fn assoc(key: &str, data: &[(&str, i32)]) -> i32 {
  for &(x, v) in data {
    if x == key {return v;}
  }
  -1
}

fn main() {
  let data = [("foo", 10), ("bar", 20), ("baz", 30)];
  println!("{}", assoc("foo", &data));
  println!("{}", assoc("bar", &data));
  println!("{}", assoc("baz", &data));
  println!("{}", assoc("oops", &data));
}
