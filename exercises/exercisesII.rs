#[derive(Debug)]
enum MyOption<T> {
    MyNone,
    MySome(T)
}

use MyOption::*;

impl<T> MyOption<T> {
    fn map<U, F>(self, f: F) -> MyOption<U>
        where F: FnOnce(T) -> U {
            panic!("implement me")
    }

    fn and_then<U, F>(self, f: F) -> MyOption<U>
        where F: FnOnce(T) -> MyOption<U> {
            panic!("implement me")
    }
}

fn main() {
    let x = MySome(1).map(|x| x + 1);
    let y = MySome(2).and_then(|y| {
        if y > 10 {
            MyNone
        } else {
            MySome(y + 1)
        }
    });

    println!("{:?}", x);
    println!("{:?}", y);
}
