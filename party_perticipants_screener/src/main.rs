use party_perticipants_screener::visitor_info::{Visitor, VisitorAction};
fn main() {
    let mut visitor_list: Vec<Visitor> = Vec::new();
    visitor_list.push(Visitor::new("Bert", 45, VisitorAction::Accept));
    visitor_list.push(Visitor::new("Steve", 15, VisitorAction::Refuse));
    visitor_list.push(Visitor::new(
        "Jon",
        18,
        VisitorAction::AcceptWithNote {
            note: "Jon is a minor".to_string(),
        },
    ));
    visitor_list.push(Visitor::new("Pete", 30, VisitorAction::Probation));
    Visitor::process_visitor(&visitor_list, &take_name());
}

fn take_name() -> String {
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    name.trim().to_string().to_lowercase()
}
