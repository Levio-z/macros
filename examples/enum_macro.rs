use template::EnumFrom;

#[derive(EnumFrom, Debug)]
enum Direction<T> {
    Up(DirectionUp<T>),
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
    let y: Direction<u32> = x.into();
    println!("{:?}", y);
    let z: Direction<u32> = Direction::Down.into();
    println!("{:?}", z);
    let w: Direction<u32> = 100.into();
    println!("{:?}", w);
}
