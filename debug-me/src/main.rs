struct MyStruct {
    prop: usize,
}

struct Point(f32, f32);

fn main() {
    let _a = 42;
    let _b = vec![0, 0, 0, 100];
    let _c = [1, 2, 3, 4, 5];
    let _d = 0x5ff;
    let _e = MyStruct { prop: 10 };
    let _p = Point(3.14, 3.14);
    
    println!("Hello, world!");
    println!(stringify!(_e.prop));
}