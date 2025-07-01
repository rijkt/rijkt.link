
struct Account {
    id: &'static str,
    name: Option<String>,
    suspended: bool
    
}

impl Account {
    fn new(id: &'static str,) -> Self {
        Self { id, name: None, suspended: false}
    }
}



fn main() {
    let a = Account::new("test");
}