
//traditional Struct version
struct Color{
    red: u8,
    green: u8,
    blue: u8,
}

//Tupple struct
struct Color2(u8, u8, u8);

pub fn run(){
    let mut c = Color{
        red: 255,
        green: 0,
        blue: 0,
    };

    c.green = 55;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut c2 = Color2(0, 0, 255);

    c2.0 = 55;

    println!("Color2: {} {} {}", c2.0, c2.1, c2.2);

}