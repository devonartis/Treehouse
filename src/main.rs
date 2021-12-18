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
    
    let mut visitor_list = vec![
        Visitor::new("divine","Hello Divine,  enjoy the rust house"),
        Visitor::new("steve", "Hi Steve, Your milk is in the fridge"),
        Visitor::new("s.johnson", "Stacie, I am so glad you have made it"),
    ];


    loop {
        
        println!("Hello, what's your name ? (Leave empty and press enter to quit");
        
        let name = what_is_your_name();
        
        let known_visitor = visitor_list
        .iter()
        .find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet_vistor(),

            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor list.", name);
                    visitor_list.push(Visitor::new(&name, "New Friend"));
                }
            }
        }
        
        


      
    }

    println!("The final list of visitors: ");
    println!("{:#?}", visitor_list);
    
    
    
    
    
}
