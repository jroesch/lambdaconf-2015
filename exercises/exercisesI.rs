struct FillMeIn;

#[derive(Debug)]
struct Vec3 { x: i32, y: i32, z: i32 }

fn vec_eq(v1: &Vec3, v1: &Vec3) -> bool {
    v1.x == v2.x &&
    v1.y == v2.y &&
    v1.z == v2.z
}

// The dot product of two vectors is simply the sum of each index multiplied.
// Ex. (1 2 3) dot (4 5 6) = (1 * 4) + (2 * 5) + (3 * 6)


fn vec_dot_product(v1: FillMeIn, v2: FillMeIn) -> FillMeIn { FillMeIn }

fn vec3_exercise1() {
    let x = Vec3::new(1,2,3);
    let y = Vec3::new(3,2,1);
    let is_eq = vec_eq(&x, &y);
    println!("{:?}", x); // error value moved
    println!("{:?}", y); // error value moved
}

fn vec3_exercise2() {
    let x = Vec3::new(1,2,3);
    let y = Vec3::new(3,2,1);
    let is_eq = vec_eq(&x, &y);
    println!("{:?}", x); // error value moved
    println!("{:?}", y); // error value moved
}

fn main() {
    vec_exercise1();
}
