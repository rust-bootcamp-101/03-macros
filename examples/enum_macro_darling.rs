use macros::EnumFromDarling;

#[allow(unused)]
#[derive(Debug, EnumFromDarling)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(u32),
    Right,
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp {
    speed: usize,
}

fn main() {
    let up: Direction = DirectionUp::new(42).into();
    println!("{:?}", up);

    let left: Direction = 10.into();
    println!("{:?}", left);
}

impl DirectionUp {
    fn new(speed: usize) -> Self {
        Self { speed }
    }
}
