use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let second_variable = 999;

    println!("second_variable is {}", second_variable);
    let mut _a_number = 1;
    _a_number = 2;
    println!("a_number is {}", _a_number);

    let x:i32 = 1;
    println!("1 in i32? {} {}", type_of(x), x);

    let y:u32 = 14;
    println!("14 in u32? {} {}", type_of(y), y);

    let thailand = 3.0;
    println!("3.0 in normal let? {} {}", type_of(thailand), thailand);

    let thailand_f:f32 = 4.0;
    println!("4.0 in f32? {} {}", type_of(thailand_f), thailand_f);

    let auu = 2 % 2;
    let auu_is_odd = auu == 0;
    tracing::info!("2 is odd? {:?}", auu_is_odd);

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
}
