//Arrays are of fixed length and elements are of the same data type
pub fn run(){

    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    numbers[0] = 99;
    println!("{:?}", numbers);

    //Get Array Length
    println!("Array length: {}", numbers.len());

    //


}