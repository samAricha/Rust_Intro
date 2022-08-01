pub fn run(){
    let guest: (&str, &str, i8) = ("Olwenya", "Kenyan", 97);

    println!("{} is {}% {}", guest.0, guest.2, guest.1);
}