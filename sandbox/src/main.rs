struct Test {
    x: u8,
    y: u8,
}

fn main() -> i32 {
    let s = Test {
        x: 5,
        y: 7,
    };

    if let Test{x,y:6} = s {
        println!("{},{}",x,6)
    }
    1_i32.add(2)
}

