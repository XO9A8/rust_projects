#![allow(dead_code)]
pub mod visitor_info {
    pub enum VisitorAction {
        Accept,
        AcceptWithNote { note: String },
        Refuse,
        Probation,
    }
    pub struct Visitor {
        pub name: String,
        age: i8,
        pub action: VisitorAction,
    }
    impl Visitor {
        pub fn new(name: &str, age: i8, action: VisitorAction) -> Self {
            Self {
                name: name.to_lowercase(),
                age,
                action,
            }
        }
        pub fn feedback(&self) {
            match &self.action {
                VisitorAction::Accept => println!("Welcome to the adventure, {}!", self.name),
                VisitorAction::AcceptWithNote { note } => {
                    println!("Welcome to the adventure, {}! Note: {}", self.name, note)
                }
                VisitorAction::Refuse => println!("You are not allowed here, {}!", self.name),
                VisitorAction::Probation => println!("You are on probation, {}!", self.name),
            }
        }
        pub fn process_visitor(visitor_list: &Vec<Visitor>, name: &str) {
            let known_visitor = visitor_list.iter().find(|v| v.name == name);
            match known_visitor {
                Some(x) => Visitor::feedback(&x),
                None => println!("{} is not on the visitor list.", name),
            }
        }
    }
}
