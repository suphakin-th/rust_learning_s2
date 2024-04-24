extern crate rand;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::any::type_name; // rand == 0.8.x

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

struct BagOfHolding<T> {
    item: T,
}

fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        println!("{}", type_of(i));
        Err(String::from("this is not the right number"))
    }
}

struct BagOfHoldingOpt<T> {
    item: Option<T>,
}

struct Location(f32, f32);

#[derive(Debug)]
enum RandomSpecies {
    Octopus,
    Fish,
    Clam,
    Crab,
}

// for rand version 0.8 ++
impl Distribution<RandomSpecies> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RandomSpecies {
        match rng.gen_range(0..=3) {
            0 => RandomSpecies::Octopus,
            1 => RandomSpecies::Fish,
            2 => RandomSpecies::Clam,
            _ => RandomSpecies::Crab,
        }
    }
}

struct SeaCreature {
    species: RandomSpecies,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

struct SexCreature {
    name: String,
    dick_inc: i32,
    dick_radius_cm: i32,
    dick_dec: String,
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

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("{:?}", number);

    // So fast!!!!
    let mut x = 0;
    loop {
        x += 1;
        println!("{:?}", x);
        if x == 42 {
            break;
        }
    }
    println!("{}", x);

    let mut x = 0;
    while x != 42 {
        x += 1;
    }
    println!("x is {}", x);

    let x = 42;

    match x {
        0 => {
            println!("found zero");
        }
        // match ทั้ง 1 และ 2
        1 | 2 => {
            println!("found 1 or 2!");
        }
        // match แบบ range ก็ได้
        3..=9 => {
            println!("found a number 3 to 9 inclusively");
        }
        // bind ใส่ตัวแปรก็ได้ด้วย
        matched_num @ 10..=100 => {
            println!("found {} number between 10 to 100!", matched_num);
        }
        // default match เอาไว้ดักทุกเคส คล้ายๆ switch case default
        _ => {
            println!("found something else!");
        }
    }

    // try struct (That not a function we call method)
    // Calling Methods
    // methods คล้ายๆกับ functions (ที่ใช้ fn) แต่ต่างกันตรง methods นั้น define ใน context ของ struct
    // จริงๆแล้ว methods ก็คือ functions ที่ทำใน struct (อาจจะฟังดูสับสน เหมือนหรือไม่เหมือนกันแน่? ดูตัวอย่างครับ)
    // static methods - คือเมธอดที่เป็นของชนิดตัวแปร เรียกด้วย :: เช่น String::from()
    // instance methods - คือเมธอดที่เป็นของตัวแปร ใช้ . เช่น s.len()

    let sealan = SexCreature {
        // String struct is also on stack,
        // but holds a reference to data on heap
        name: String::from("sealan"),
        dick_inc: 4,
        dick_radius_cm: 32,
        dick_dec: String::from("wiggle follow the force of water"),
    };

    let jade = SexCreature {
        name: String::from("jade the human"),
        dick_inc: 7,
        dick_radius_cm: 130,
        dick_dec: String::from("Stronghold of human who have big dick that can pierce your vegena daughter virgin vagina and made creampies with her."),
    };

    let engraved = Location(13.654542737935579, 100.60122523124825);

    println!(
        "{} have dick {} inc, dick radius {} cm, and {} and he love to hunt at this ({}, {})",
        jade.name, jade.dick_inc, jade.dick_radius_cm, jade.dick_dec, engraved.0, engraved.1
    );
    println!(
        "{} have dick {} inc, dick radius {} cm, and {}",
        sealan.name, sealan.dick_inc, sealan.dick_radius_cm, sealan.dick_dec
    );

    let food = String::from("hotdog");
    let result = match food.as_str() {
        "hotdog" => String::from("tadahhh"),
        _ => String::from("is not hotdog"),
    };

    println!("{:?}", result);

    let mut rng = rand::thread_rng();

    let ferris = SeaCreature {
        species: rng.gen(),
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };

    match ferris.species {
        RandomSpecies::Octopus => println!("{} is a octopus", ferris.name),
        RandomSpecies::Crab => println!("{} is a crab", ferris.name),
        RandomSpecies::Fish => println!("{} is a fish", ferris.name),
        RandomSpecies::Clam => println!("{} is a clam", ferris.name),
    }

    let i32_bag = BagOfHolding::<i32> { item: 42 };
    let bool_bag = BagOfHolding::<bool> { item: true };

    println!("i32_bag is {}", i32_bag.item);
    println!("bool_bag is {}", bool_bag.item);

    // เป็น None
    let _str_bag_opt = BagOfHoldingOpt::<&str> { item: None };

    // เป็น Some
    let _i32_bag_opt = BagOfHoldingOpt::<i32> { item: Some(42) };

    // println!("i32_bag is {}", _str_bag_opt);
    println!("bool_bag is {:?}", _i32_bag_opt.item);

    let data = do_something_that_might_fail(32);
    println!("bool_bag is {:?}", data);
    let data = do_something_that_might_fail(32);
    println!("bool_bag is {:?}", data);
    let data = do_something_that_might_fail(33);
    println!("bool_bag is {:?}", data);
    let data = do_something_that_might_fail(34);
    println!("bool_bag is {:?}", data);
    let data = do_something_that_might_fail(35);
    println!("bool_bag is {:?}", data);
    let data = do_something_that_might_fail(36);
    println!("bool_bag is {:?}", data);
    let data = do_something_that_might_fail(37);
    println!("bool_bag is {:?}", data);
    let data = do_something_that_might_fail(38);
    println!("bool_bag is {:?}", data);
    let data = do_something_that_might_fail(39);
    println!("bool_bag is {:?}", data);
    let data = do_something_that_might_fail(40);
    println!("bool_bag is {:?}", data);
    let data = do_something_that_might_fail(41);
    println!("bool_bag is {:?}", data);
    let data = do_something_that_might_fail(42);
    println!("bool_bag is {:?}", data);
}
