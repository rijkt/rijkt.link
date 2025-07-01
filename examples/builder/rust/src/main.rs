
struct Account {
    id: String,
    name: Option<String>,
    suspended: bool
    
}

impl Account {
    fn builder(id: String,) -> AccountBuilder {
        AccountBuilder { id, name: None, suspended: false} // move default values into build?
    }
}


struct AccountBuilder {
        id: String,
    name: Option<String>,
    suspended: bool

}

impl AccountBuilder {

    fn build(self) -> Account {
        Account { id: self.id, name: self.name, suspended: self.suspended}
    }

    fn name(mut self, name: String) -> AccountBuilder {
        self.name = Some(name);
        self
    }
}


fn main() {
    let mut a = Account::builder("test".to_owned()).build();
    a.name = None // TODO: forbid
}