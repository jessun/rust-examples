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
