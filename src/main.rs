#![allow(dead_code, unused_variables)]

fn main() {
    // The Problem:
    //      * I acquire a handle to some kind of resource.
    //      * I lend you a reference to the resource.
    //      * I decide I’m done with the resource, and deallocate it,
    //          while you still have your reference.
    //      * You decide to use the resource.

    foo2();
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
        println!("dropping Person {} with age: ", self.age);
    }
}
