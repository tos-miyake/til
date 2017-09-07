trait Distance {
    fn distance(&self, p: &Self) -> f64;
}

struct Point {
    x: f64, y: f64
}

impl Point {
    fn new(x1: f64, y1: f64) -> Point {
        Point {x: x1, y: y1}
    }
}

impl Distance for Point {
    fn distance(&self, p: &Point) -> f64 {
        let dx = self.x - p.x;
        let dy = self.y - p.y;
        (dx * dx + dy * dy).sqrt()
    }
}

struct Point3D {
    x: f64, y: f64, z: f64
}

impl Point3D {
    fn new(x1: f64, y1: f64, z1: f64) -> Point3D {
        Point3D { x: x1, y: y1, z: z1 }
    }
}

impl Distance for Point3D {
    fn distance(&self, p: &Point3D) -> f64 {
        let dx = self.x - p.x;
        let dy = self.y - p.y;
        let dz = self.z - p.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

fn main() {
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(10.0, 10.0);
    let p3 = Point3D::new(0.0, 0.0, 0.0);
    let p4 = Point3D::new(10.0, 10.0, 10.0);
    println!("{}", p1.distance(&p2));
    println!("{}", p3.distance(&p4));
}
