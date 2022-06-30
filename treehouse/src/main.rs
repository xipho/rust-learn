use std::io::stdin;

fn main() {
    let mut visitor_list = vec![
        Visitor::new("bert", "Hello, Bert, enjoy your treehouse!"),
        Visitor::new("steve", "Hi, Steve, your milk is in the fridge!"),
        Visitor::new("fred", "Wow, who invited Fred?")
    ];
    loop {
        println!("Hello, enter your name please: ");
        let name = ask_for_name();
        let known_visitor = visitor_list
            .iter()
            .find(|visitor| visitor.name == name);
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not in visitor list.", name);
                    visitor_list.push(Visitor::new(&name, "New friend"))
                }
            }
        }
    }
}

fn ask_for_name() -> String {

    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line!");
    your_name.trim().to_lowercase()
}

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}