#[test]
fn show_types() {
    let _x = 2.0;
    println!("_x type is f64");

    let _y: f32 = 3.0;
    println!("_y type is f32");

    let _c = 'c';
    println!("_c type is char");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = tup;
    println!("The value of _x, _y, _z is {}, {}, {}", _x, _y, _z);

    let _a = [1, 2, 3, 4, 5];
    println!("_a type is a list");

    let _b: [i32; 5] = [1, 2, 3, 4, 5];
    println!("_b is {:?}", _b);

    let _c = [3; 5];
    println!("_c is {:?}", _c);
}

#[test]
fn _vecotrs() {
    let mut i32_vec = Vec::<i32>::new();
    i32_vec.push(1);
    i32_vec.push(2);
    i32_vec.push(3);

    let mut float_vec = Vec::new();
    float_vec.push(1.3);
    float_vec.push(1.4);
    float_vec.push(1.5);

    let string_vec = vec![String::from("Hello"), String::from("World")];
    for word in string_vec.iter() {
        println!("{}", word);
    }
}
