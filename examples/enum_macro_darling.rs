use template::EnumFromDarling;

#[derive(EnumFromDarling, Debug)]
enum Direction {
    Up(DirectionUp<u32>),
    Down,
    Left(u8),
    Right,
}
#[derive(Debug)]
struct DirectionUp<T> {
    speed: T,
}
fn main() {
    let dir = Direction::Up(DirectionUp { speed: 100 });
    let x = DirectionUp { speed: 100 };
    println!("{:?}", dir);
    let y: Direction = x.into();
    println!("{:?}", y);
    let z: Direction = Direction::Down.into();
    println!("{:?}", z);
    let w: Direction = 100.into();
    println!("{:?}", w);
}
