#![allow(dead_code, unused_variables)]

fn main() {
    // The Problem:
    //      * I acquire a handle to some kind of resource.
    //      * I lend you a reference to the resource.
    //      * I decide I’m done with the resource, and deallocate it,
    //          while you still have your reference.
    //      * You decide to use the resource.

    // foo2();
    // foo3();
    integer_copy_confusion();
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
}
