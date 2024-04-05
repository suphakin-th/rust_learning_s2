use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn sum(a: u32, b: u32) -> u32 {
    return a + b;
}

fn main() {
    let second_variable = 999;

    println!("second_variable is {}", second_variable);
    let mut _a_number = 1;
    _a_number = 2;
    println!("a_number is {}", _a_number);

    let x: i32 = 1;
    println!("1 in i32? {} {}", type_of(x), x);

    let y: u32 = 14;
    println!("14 in u32? {} {}", type_of(y), y);

    let thailand = 3.0;
    println!("3.0 in normal let? {} {}", type_of(thailand), thailand);

    let thailand_f: f32 = 4.0;
    println!("4.0 in f32? {} {}", type_of(thailand_f), thailand_f);

    let auu = 2 % 2;
    let auu_is_odd = auu == 0;
    println!("2 is odd? {:?}", auu_is_odd);

    // char
    let _i_am_a = 'a';

    // string literal
    let _s: &str = "Hello World!";

    // string allocated memory
    let _hello: &'static str = "hello";

    // ใช้ String::new()
    let mut string = String::new();
    string.push('H');

    // Heap
    let _greeting = String::from("Hello World");

    let score = 81;

    if score > 49 {
        println!("Passed!");
    } else {
        println!("F!");
    }

    for i in 0..10 {
        println!("{}", i);
    }

    let mut i = 0;

    while i < 10 {
        println!("hello");
        i += 1;
    }

    let my_tuple = ('A', 10, true);
    println!("{:?}", my_tuple);

    trait Frobnicate<T> {
        fn frobnicate(self) -> Option<T>;
    }
    let result = sum(10, 20);
    println!("10+20={}", result);

    let nums: [i32; 3] = [1, 2, 3];
    println!("{:?}", nums);

    // Best practice
    let arr: [i32; 3] = [1, 2, 3];
    println!("{:?}", arr);
    println!("{}", arr[1]);

    // basic for control Flow.
    let x = 42;

    if x < 42 {
        println!("less than 42");
    } else if x == 42 {
        println!("is 42");
    } else {
        println!("greater than 42");
    }
}
