struct Point3D {
  x: f64, y: f64, z: f64
}

fn distance(p1: &Point3D, p2: &Point3D) -> f64 {
  let dx = p1.x - p2.x;
  let dy = p1.y - p2.y;
  let dz = p1.z - p2.z;
  (dx * dx + dy * dy + dz * dz).sqrt()
}

fn main() {
  let p1 = Point3D {x:0.0, y:0.0, z:0.0};
  let p2 = Point3D {x:10.0, y:10.0, z:10.0};
  println!("{}", distance(&p1, &p2));
}
