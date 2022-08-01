pub fn run(){
    let mut hello = String::from("hello ");

    //push char
    hello.push('W');
    //push string
    hello.push_str("orld");

   
    //capacity in bytes
    println!("Capacity: {}", hello.capacity());

    //check if empty
    println!("Is hello empty: {}", hello.is_empty());

    //check if contains world
    println!("Contains World: {}", hello.contains("World"));

    //replacing a word
    println!("Replace: {}", hello.replace("World", "Mombasa"));

    //Loop through strings by white space
    for word in hello.split_whitespace(){
        println!("splitting: {}", word);
    }

    //printing
    println!("{} \nLength: {}\n\n\n",hello, hello.len());


    //creating a string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //Assertion Testing
    assert_eq!(2, s.len());
    assert_eq!(s.capacity(), 10);

    println!("{}", s)

}