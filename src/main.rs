#![allow(dead_code)]

struct Foo<'a> {
    x: &'a i32,
}

fn main() {
    let x;
                              //  |
    {                         //  |
        let y = &5;           // ---+ y goes into scope
        let f = Foo { x: y }; // ---+ f goes into scope
        x = &f.x;             //  | | error here
    }                         // ---+ f and y go out of scope
                              //  |
    println!("{}", x);        //  |
}

// fn main() {
//     let x;
//
//     {
//         x = &"woah";
//         let f = Strukt { name: x };
//         x = &f.name;
//     }
//
//     println!("name: {}", x);
// }
//
// struct Strukt<'crazy_lifetime> {
//   name: &'crazy_lifetime str,
// }
//
// struct Foo<'a> {
//     x: &'a i32,
// }

// fn main() {
//     let x;                    // -+ x goes into scope
//                               //  |
//     {                         //  |
//         let y = &5;           // ---+ y goes into scope
//         let f = Foo { x: y }; // ---+ f goes into scope
//         x = &f.x;             //  | | error here
//     }                         // ---+ f and y go out of scope
//                               //  |
//     println!("{}", x);        //  |
// }                             // -+ x goes out of scope
