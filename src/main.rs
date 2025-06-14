fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello, test!");
    println!("This is a test function.");
}

#[test]
fn test_variables() {
    let mut name = "marwan";

    println!("Hello, {}!", name);

    name = "marwanwae";

    println!("Hello, {}!", name);

    let username = name;
    println!("Username is: {}", username);
    println!("name is now: {}", name);

    let num32 = 2000000000;
    println!("The number is: {}", num32);
    let num64: i64 = 10;
    println!("The number is: {}", num64);

    let float = 10.2;
    println!("The float is: {}", float);

    let conv_num32 = num32 as i64;
    println!("The converted number is: {}", conv_num32);

    let car = 'S';
    println!("The car character is: {}", car);

    let mut string_name: String = String::from("Marwan");
    println!("The string name is: {}", string_name);
    string_name = String::from("Marwan Wae");

    let move_string: String = string_name;
    println!("The moved string is: {}", move_string);
    // println!("The original string name is now: {}", string_name); // error karena string_name telah dipindahkan

    let mut num_mut = 10;
    println!("The mutable number is: {}", num_mut);
    num_mut = 20;
    let move_num: i32 = num_mut;
    println!("The moved mutable number is: {}", move_num);
    println!("The original mutable number is now: {}", num_mut); 

}

#[test]
fn test_tuples() {
    // Tuples in Rust can hold multiple values of different types.
    let tuple = (1, 2.5, "Hello");
    println!("Tuple values: {:?}", tuple);
    println!("Second element: {}", tuple.1);
    // Accessing tuple elements
    let (x, _, z) = tuple;
    println!("x: {}, y: 0, z: {}", x, z);
    
    let single_element_tuple = (42,);
    println!("Single element tuple: {:?}", single_element_tuple);
    
    let empty_tuple = ();
    println!("Empty tuple: {:?}", empty_tuple);

    let mut tuple_mut = (1, 2, 3);
    tuple_mut.0 = 10;
    tuple_mut.1 = 20;
    tuple_mut.2 = 30;
    println!("Modified tuple: {:?}", tuple_mut);
}

fn unit() {
    // The unit type `()` is used when a function does not return a value.
    println!("This function returns the unit type.");
}

#[test]
fn test_unit() {
    // Calling the unit function
    let result =unit();
    println!("Result of unit function: {:?}", result);

    let empty_tuple = ();
    println!("Empty tuple: {:?}", empty_tuple);
}

#[test]
fn test_arrays() {
    // Arrays in Rust are fixed-size collections of elements of the same type.
    let arr = [1, 2, 3, 4, 5];
    println!("Array: {:?}", arr);
    
    // Accessing array elements
    println!("First element: {}", arr[0]);
    println!("Second element: {}", arr[1]);
    
    let len_arr = arr.len();
    println!("Length of array: {}", len_arr);

    // Array with a specific size
    let arr_fixed: [i32; 5] = [10; 5];
    println!("Fixed size array: {:?}", arr_fixed);

    let mut arr_mut = [1, 2, 3];
    arr_mut[0] = 10;
    arr_mut[1] = 20;
    arr_mut[2] = 30;
    println!("Modified array: {:?}", arr_mut);
    

    let array_dim = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    println!("2D Array: {:?}", array_dim);

    let array_3d = [[[1, 2], [3, 4]], [[5, 6], [7, 8]]];
    println!("3D Array: {:?}", array_3d);
}


#[test]
fn test_constants() {
    // Constants in Rust are immutable and must have a type annotation.
    const PI: f64 = 3.14159;
    println!("Value of PI: {}", PI);

    // Constants can be defined at any scope, including inside functions.
    const MAX_SIZE: usize = 100;
    println!("Maximum size: {}", MAX_SIZE);

    // Constants can also be used in expressions.
    let radius = 5.0;
    let area = PI * radius * radius;
    println!("Area of circle with radius {}: {}", radius, area);

    const VERSION: &str = "1.0.0";
    println!("Version: {}", VERSION);
}

const MAXIMUM: i64 = 1000; 

#[test]
fn test_variable_scope() {
    // Variable scope in Rust determines where a variable can be accessed.
    let x = 10;
    {
        let y = 20; // y is only accessible within this block
        println!("Inside block: x = {}, y = {}", x, y);
        println!("this maximum is: {}", MAXIMUM);
    }
    // println!("Outside block: y = {}", y); // This would cause an error

    // Shadowing allows re-declaring a variable with the same name.
    let x = x + 5; // x is now 15
    println!("After shadowing: x = {}", x);

    // Variables can also be declared with different types.
    let z: f64 = 3.14;
    println!("z as f64: {}", z);
}