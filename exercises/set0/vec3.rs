// Imports std::cmp::Ordering into scope.
// Documentation is here: http://doc.rust-lang.org/stable/std/cmp/enum.Ordering.html
// Ordering is a enum representing the possibly relationship between ordered values.
use std::cmp::Ordering;

#[derive(Debug)]
struct Vec3 { x: i32, y: i32, z: i32 }

impl Vec3 {
    fn new(x: i32, y: i32, z: i32) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }
}

fn main() {
    let x = Vec3::new(1,2,3);
    let y = Vec3::new(3, 2, 1);
}
