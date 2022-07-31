pub fn run(){
    let x:i32 = 344;
    let y:i64 = 455;
    //let is_present = false;
    //let is_absent: bool = true;
    let is_greater: bool = 10 > 5;

    let a1 = 'a';
    let face = '\u{1F600}';


    //Finding Max Size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
   // println!("Max f32: {}", std::f32::MAX);
    //println!("Max f64: {}", std::f64::MAX)

    println!("{:?}", (x, y ,is_greater , a1, face ));
}