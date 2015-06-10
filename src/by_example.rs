pub fn main() {
    println!("\nLifetimes by example\n");

    box_vs_stack();
}

fn box_vs_stack() {
    let stack_int: i32 = 5;
    let box_int: Box<i32> = Box::new(5);
    let ref_to_box: &i32 = &*box_int;

    // println!("ref_to_box: {}", ref_to_box);


    // If you are assigning a variable, and allocated some resources,
    // in a block, you can not assign that value to the outer variable
    // because its liftime will end when the block ends
    //
    // so it is shorter than the variable it is being assigned to.

    // AKA
    // let ref_to_another_box: &i32 = {
    //     let another_boxed_integer = Box::new(3);
    //     &*another_boxed_integer
    // };
}
