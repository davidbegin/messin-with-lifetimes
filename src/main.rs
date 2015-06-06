#![allow(dead_code)]

fn main() {
    println!("Messin with Lifetimes");

    let y = &"durf"; // this is the same as `let _y = 5; let y = &_y;`

    let strukt = Strukt { name: y };

    println!("{}", strukt.name);

    bar();
    foo();
}

fn bar<'a_lifetime>() {
    println!("This has a lifetime");
}

fn foo<'another_lifetime>() {
    println!("This also has a lifetime");
}

struct Strukt<'crazy_lifetime> {
  name: &'crazy_lifetime str,
}
