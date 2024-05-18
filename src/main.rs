/*Collections are data structures that store multiple values.

They are stored on the heap: 
the data does not need to be known at compile time
and can grow or shrink as the program runs.

A vector allows you to store a variable number
of values next to each other.

*/
fn main(){
    //create a vector
    let vec1: Vec<i32> = Vec::new();
    println!("{:?}", vec1);

    //create a vector with initial values
    let vec2 = vec![1, 2, 3, 4, 5];
    println!("{:?}", vec2);

    //update a vector
    let mut vec3: Vec<i32> = Vec::new();

    vec3.push(1);
    vec3.push(2);
    vec3.push(3);
    println!("{:?}", vec3);

    vec3.pop();
    println!("{:?}", vec3);

    //accessing elements in a vector

    //reading elements
    let vec4 = vec!['a', 'b', 'c', 'd', 'e'];

    // let fourth: &char = &vec4[30];
    // println!("The fourth element is: {}", fourth);

    //using get method
    let fifth: Option<&char> = vec4.get(100);

    match fifth {
        Some(&fifth) => println!("The fifth element is: {}", fifth),
        None => println!("There is no fifth element."),
    }

    //iterating over the values in a vector
    let vec5 = vec![1, 2, 3, 4, 5];
    for i in &vec5 {
        println!("{}", i);
    }

    //iterating over mutable references
    let mut vec6 = vec![10, 20, 30, 40, 50];
    for i in &mut vec6 {
        *i += 1;
    }

    println!("{:?}", vec6);

    //store multiple types in a vector using enums

    #[derive(Debug)]
    #[allow(dead_code)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Int(6),
        SpreadsheetCell::Int(9),
    ];

    println!("{:?}", row);


} // all the vectors go out of scope and are freed here