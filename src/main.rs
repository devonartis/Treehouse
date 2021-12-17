use std::io::stdin;
use std::fmt::Debug;

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_ascii_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_vistor(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();

    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    
        your_name
            .trim()
            .to_lowercase()
}



fn main() {
    
    println!("Hello, what's your name");
    //let mut allow_them_in = false;
    let name = what_is_your_name();
    
    let visitor_list = [
        Visitor::new("divine","Hello Divine,  enjoy the rust house"),
        Visitor::new("steve", "Hi Steve, Your milk is in the fridge"),
        Visitor::new("s.johnson", "Stacie, I am so glad you have made it"),
    ];
   
    //println!("{:?}",visitor_list);

    
    let known_visitor = visitor_list
        .iter()
        .find(|visitor| visitor.name == name);

    match known_visitor {
        Some(visitor) => visitor.greet_vistor(),
        None => println!("You are not on the visitor list. Please leave !!!")
    }
    
    //println!("{:?}", &known_visitor);

    /*
    for visitor in &vistor_list {
          if visitor == &name {
              allow_them_in = true;
          }
    }
    
    if allow_them_in {
        println!("Welcome to my Treehouse, {}", name);
    }else {
        println!("Sorry, you aren't on the list.");
    }

    */
}
