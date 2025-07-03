struct Tofu {
    id: u16,
    name: Option<String>,
    silky: bool,
    smoky: bool,
}

impl Tofu {
    fn builder(id: u16) -> TofuBuilder {
        TofuBuilder::new(id)
    }
}

struct TofuBuilder {
    id: u16,
    name: Option<String>,
    silky: bool,
    smoky: bool,
}

impl TofuBuilder {
    fn new(id: u16) -> TofuBuilder {
        TofuBuilder {
            id,
            name: None,
            silky: false,
            smoky: false,
        }
    }

    fn build(self) -> Tofu {
        Tofu {
            id: self.id,
            name: self.name,
            silky: self.silky,
            smoky: self.smoky,
        }
    }

    fn name(mut self, name: String) -> TofuBuilder {
        self.name = Some(name);
        self
    }

    fn smoky(mut self, smoky: bool) -> TofuBuilder {
        self.silky = smoky;
        self
    }
}

fn main() {
    let smoky_tofu = Tofu::builder(0)
        .name("Smokey smoke tofu".to_owned())
        .smoky(true)
        .build();

    let silken_tofy = Tofu::builder(1).smoky(true).build();
}
