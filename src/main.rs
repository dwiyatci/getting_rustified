mod exercises;
mod hello_world;

use exercises::*;

fn main() {
    // println!("Hello, world!");
    hello_world::hello();

    // exercise_01::invoke();
    // exercise_02::invoke();

    // let mut s = String::from("foo");
    // change_str(&mut s);
    // println!("{}", s);

    // let s = "foo";
    // let t = &mut s;
    // *t = "bar";
    // println!("{}", s);

    // let mut x = 42;
    // let y = &mut x;
    // y = 43;
    // *y = 43;
    // println!("{}", y);
    //
    // let mut s = String::from("foo");
    // let t = &mut s;
    // *t = String::from("bar");
    // println!("{}", s);
}

// fn change_str(x: &mut String) {
//     x.push_str(", bar");
//     *x = String::from("baz");
//     println!("{}", x);
//     println!("{} {}", x, *x);
//     println!("{} {} {}", x, *x, **x);
//
//     let y = &x;
//     println!("{} {} {}", y, *y, **y);
// }
