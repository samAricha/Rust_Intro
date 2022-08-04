struct Organization{  
    first_name: String,
    last_name: String,

}

impl Organization{
    //construct organization
    fn new(first: &str, last: &str) -> Organization{
        Organization {
            first_name : first.to_string(),
            last_name : last.to_string()
        }
        
    }

    //Get Full Name
    fn full_name(&self) -> String{
        format!("Organization {} {}", self.first_name, self.last_name)
    }

    //Set Last Name
    fn set_last_name(&mut self, last: &str){
        self.last_name = last.to_string();
    }

    //Name to Tuple
    fn to_tuple(self) -> (String, String){
        (self.first_name, self.last_name)
    }
}

pub fn run(){
    let mut org = Organization::new("Git", "Hub");
    println!("The {} {} Organization", org.first_name, org.last_name);
    println!("{}", org.full_name());//printing the name using the full name
    org.set_last_name("Lab");
    println!("{}", org.full_name());//printing the name using the full name

    println!("Tuple: {:?}", org.to_tuple());

}