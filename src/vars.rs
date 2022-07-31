pub fn run(){
    let sur_name = "Aricha";
    let mut marks = 50;
    println!("my surname is {} and I scored {}%", sur_name, marks);
    marks = 60;
    let marks2 = marks;
    println!("my surname is {} and I scored {}%", sur_name, marks2);

    //Define Constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assingning multiple vars
    let (her_name, her_age) = ("Hannah", 44);
    println!("{} is {} years old", her_name, her_age)
}