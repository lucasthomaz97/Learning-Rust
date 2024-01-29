fn main() {
    let _integer: i8 = -25;
    let _unnegative_integer: u8 = 25;
    let _floating_point: f64 = 10.64;
    let _boolean: bool = false; // could be 0 for false or 1 for true
    let _character: char = 'l';

    let mut tup: (i32, bool, char) = (14, true, 'B');
    println!("{}", tup.0);
    tup.0 = 28;
    println!("{}", tup.0);

    let mut arr: [i32;5] = [-10, 25, 22, -50, 77];
    println!("{}",arr[4]);
    arr[4] = arr[4]*-1;
    println!("{}",arr[4]);

    let x: u8 = 8;
    let _y: u32 = x.into();
}