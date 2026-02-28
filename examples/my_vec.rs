use template::my_vec;

fn main() {
    let v = my_vec![1, 2, 3, 4, 5];
    println!("{:?}", v);

    let v2 = my_vec!["a", "b", "c"];
    println!("{:?}", v2);

    let v3 = my_vec![1; 3];
    println!("{:?}", v3);
}
