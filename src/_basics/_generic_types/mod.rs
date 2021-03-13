#[derive(Debug)]
struct Bag<T> {
    item: T,
}

#[test]
fn test_bag() {
    let i32_bag = Bag::<i32> { item: 42 };
    let bool_bag = Bag::<bool> { item: true };

    let float_bag = Bag { item: 3.14 };

    println!("{:?}, {:?}, {:?}", i32_bag, bool_bag, float_bag);
}
