enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn move_person(m: Movement){
    match m{
        Movement::Up => println!("Moving up"),
        Movement::Down => println!("Moving Down"),
        Movement::Left => println!("Moving Left"),
        Movement::Right => println!("Moving Right"),
    }
}


pub fn run(){
    let person1 = Movement::Up;
    let person2 = Movement::Down;
    let person3 = Movement::Right;
    let person4 = Movement::Left;

    move_person(person1);
    move_person(person2);
    move_person(person3);
    move_person(person4);
}