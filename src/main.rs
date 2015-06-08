#![allow(dead_code, unused_variables)]

extern crate type_printer;

fn main() {
    // The Problem:
    //      * I acquire a handle to some kind of resource.
    //      * I lend you a reference to the resource.
    //      * I decide I’m done with the resource, and deallocate it,
    //          while you still have your reference.
    //      * You decide to use the resource.

    // foo2();
    // foo3();
    // integer_copy_confusion();

    // println!("println de-references the interoplated variable")
    // println!("Owned Seven {}", owned_seven());
    // wha();
    // what_happens_when(); // it doesn't compile thats what!

    // enum_enum_enums();
    lets_take_it_further();
}

fn foo() {
    let v = vec![1, 2, 3];
}

// when foo is called, v is allocated on the heap
// then at the end of foo,
// v goes out of scope
// and is deallocated from the heap

// let use drop to see this
fn foo2() {
    // println!("v is being bound to a new Person struct");
    println!("the start of foo2");
    let v = Person { age: 32 };
    println!("the end of foo2")
}

struct Person {
    age: i32
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("dropping Person with age: {}", self.age);
    }
}

fn foo3() {
    // So I can assign something to many variables!
    // its just who every got it last

    let x = Person { age: 32 };
    let y = x;
    let z = y;

    // this won't compile!
    // let j = x;

    // because x gave away ownership to y
}

// any function that doesn't take a borrowed value, takes ownership
//
// and it will have to be returned to return ownership
//
// so unless the input value is the return value,
// it needs to be borrowed, if you want to use it after the passing
// it into a function

fn integer_copy_confusion() {
    let v = 1;
    let x = v;

    // this compiles!
    let j = v + 3;

    // because i32 implements Copy?

    let x = Person { age: 17 };
    let y = x;

    // won't compile because Person does not implement Copy
    // let z = x;

    let p = PersonWithCopy { age: 18 };
    let i = p;
    let q = p;
}

fn owned_seven() -> Box<i32> {
    let three : Box<i32> = Box::new(3);
    let four : Box<i32> = Box::new(4);
    Box::new(*three + *four)
}

fn wha() {

    // so could I saw I am assigning three to the address of 3
    // or where on the stack 3 was allocated at?
    let three = &3;
    let four = &4;

    // so ha! I don't need to deference for println!
    //
    // there needs to be an easier way see this
    //
    // or a more strict println?
    println!("{}", three + four);

    type_printer::print_type_of(&three);

}

#[derive(Copy, Clone)]
struct PersonWithCopy {
    age: i32
}

fn what_happens_when() {
    println!("
        what happens when I try to derive copy
        on structs who have Boxes as attributes
    ");

    // can't compile because Box doesn't implement copy
    // #[derive(Copy, Clone)]
    // struct Email { user_id: i32, reply_count: Box<i32> }

    // let e = Email { user_id: 1 };
    //
    // let x = e;
    //
    // let z = e;
}

fn enum_enum_enums() {
    let x = MyEnum::X(10);
    let y = MyEnum::Y(Box::new(99));

    match_and_print(x);
    match_and_print(y);
}

fn match_and_print(e: MyEnum) {
    match e {
        MyEnum::X(x) => println!("{}", x),
        MyEnum::Y(y) => println!("{}", y)
    }
}

enum MyEnum {
    X(i32),
    Y(Box<i32>)
}


// Ok so this is a very important distinction
//
// if we pass a borrowed Enum,
// then we can not print a box variable since it is borrowed
// and does not have copy and clone implemented
//
// but if we let the function take ownership, then it can just deference it,
// since it knows it owns it

fn lets_take_it_further() {
    // so what about this ref thang
    match_and_print_2(&MyEnum::X(500));
}

fn match_and_print_2(e: &MyEnum) {
    match e {
        &MyEnum::X(x) => println!("x: {}", x),
        &MyEnum::Y(ref y) => println!("y: {}", y)
    }
}
