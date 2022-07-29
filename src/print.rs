pub fn run(){
    //printing to console
    println!("hello there, from print.rs file");

    //Basic Arguments
    println!("{} is my {} name", "aricha", "second");

    //Positional Arguments
    println!("{0} is my {1} name and {0} is also my {2} name", "aricha", "second", "favourite");

    //Named Arguments
    println !(
     "{surName} is my {surNamePosition} name and {englishName} my {englishNamePosition} name",
     surName = "aricha",
     surNamePosition = "last",
     englishName = "samson",
     englishNamePosition = "first");

}