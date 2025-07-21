
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

#[test]
fn test_if_statements() {
    let number = 45;

    if number < 50 {
        println!("tiadak lulus nilai kamu {}", number);
    }
    else if number > 50 && number < 75 {
        println!("remedial nilai kamu {}", number)
    }else {
        println!("kamu lulus nilai kamu {}", number)
    }
}

#[test] 
fn test_loop() {

    let mut count = 1;

    'outer: loop {
        
        let mut i = 1;

        loop {

            if count > 10 {
                break 'outer; // break for yang paling luar..
            }

            if i > 10 {
                break;
            }
            
            println!("{} * {} = {}", count, i, count*i);
            i += 1;
        }

        count += 1;
    }

    
}

#[test]
fn test_while_loop() {

    let mut count = 0;

    while count < 10 {
        count += 1;

        if count%2 == 0 {
            continue;
        }

        println!("perulangan ke : {}", count);
    }
}

#[test]
fn tes_for_loop() {
    let array = ["Marwan", "Wae", "Rust", "Programming"];
    for name in array {
        println!("Hello, {}!", name);
    }

    let count = array.len();
    for i in 0..count {
        println!("Perulangan ke: {}, Hello {}", i+1, array[i]);
    }

    for(i, name) in array.iter().enumerate() {
        println!("perulangan ke-{}, Hello {}", i, name);
    }

    println!("{:?}", array.iter().enumerate());
}

fn say_hello() {
    println!("Hello, Rust!");
}

#[test]
fn test_say_hello() {
    say_hello();
    say_hello();
    say_hello();

    let name = String::from("kucur");
    println!("name {}", name);
    println!("name {}", name);
    println!("name {}", name);
}

fn say_hello_name(first_name: String, last_name: String) {
    println!("Hello, {} {}!", first_name, last_name);
}

#[test]
fn test_say_hello_name() {
    say_hello_name(String::from("Marwan"), String::from("Wae"));
    say_hello_name(String::from("sean"), String::from("danish"));
}

fn factorial_loop(n: u32) -> u32 {
    let mut result = 1;

    for i in 1..=n {

        result *= i
    }

    result
}

#[test]
fn test_factorial_loop() {
    let fac_three = factorial_loop(3);
    let fac_five = factorial_loop(5);
    println!("result: {}", fac_three);
    println!("result: {}", fac_five);
}

fn factorial_recursive(n: u32) -> u32 {
    if n < 1 {
        return 1
    }

    n * factorial_recursive(n-1)
}

#[test]
fn test_factorial_recursive() {
    let res_fac = factorial_recursive(5);
    println!("factotial recursive {}", res_fac)
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    println!("bytes: {:?}", bytes);
    println!("b ' ': {:?}", b' ');
    for (i, &item) in bytes.iter().enumerate() {
        println!("i: {}, item: {}", i, item);
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
#[test]
fn test_first_word() {
    let name = String::from("sean danish alfath");
    let indx = first_word(&name);
    let first_name: &str = &name[0..indx];
    println!("{}", first_name);
}

    struct Person{
        first_name: String,
        last_name: String,
        age: u8
    }

#[test]
fn test_struct() {

    let marwan = Person{
        first_name: String::from("marwan"),
        last_name: String::from("wae"),
        age: 29
    };

    println!("first name: {}", marwan.first_name);
    println!("last name: {}", marwan.last_name);
    println!("age: {}", marwan.age);

    let first_name = String::from("danish");

    let danish = assign_person(&first_name, String::from("alfath"), 4);

    println!("{}, {}, {}", danish.first_name, danish.last_name, danish.age);
    println!("{}", first_name);
}

fn assign_person(first_name: &str, last_name: String, age: u8) -> Person {
    return Person {
        first_name: first_name.to_string(),
        last_name,
        age
    };
}

use std::collections::{BTreeMap, HashMap};

#[test]
fn test_hash_map() {
    let mut map = HashMap::<String, String>::new();
    map.insert(String::from("ani"),String::from("1") );
    map.insert(String::from("budi"),String::from("2") );
    map.insert(String::from("cici"),String::from("3") );
    map.insert(String::from("daru"),String::from("4") );
    map.insert(String::from("eko"),String::from("5") );
    map.insert(String::from("fery"),String::from("6") );
    map.insert(String::from("gina"),String::from("7") );
    map.insert(String::from("hilal"),String::from("8") );

    for (key, value)  in &map {
        println!("{} : {}", key, value);
    }

    println!("{:?}", map);
}

#[test]
fn test_btree_map() {
    let mut map = BTreeMap::<String, String>::new();
    map.insert(String::from("ani"),String::from("1") );
    map.insert(String::from("budi"),String::from("2") );
    map.insert(String::from("cici"),String::from("3") );
    map.insert(String::from("daru"),String::from("4") );
    map.insert(String::from("eko"),String::from("5") );
    map.insert(String::from("fery"),String::from("6") );
    map.insert(String::from("gina"),String::from("7") );
    map.insert(String::from("hilal"),String::from("8") );

    for (key, value)  in &map {
        println!("{} : {}", key, value);
    }

    println!("{:?}", map);
}

#[test]
fn test_iterator() {
    let array = [1,2,3,4,5,6];
    let iterator = array.iter();

    println!("{:?}", iterator);
}

fn connect_cache(host: Option<String>) -> Result<String, String> {
    match host {
        None => {
            return Err("No Cache host provider".to_string());
        }
        Some(host) => {
            return Ok(host);
        }
    }
}

#[test]
fn test_result() {
    // let cache = connect_cache(Some("localhost".to_string()));
    let cache = connect_cache(None);

    // println!("{:?}", cache.is_err());

    match cache {
        Ok(host) => {
            println!("Success connect to host: {}", host)
        }
        Err(error) => {
            println!("Error with message: {}", error)
        }
    }
}