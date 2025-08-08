
use crate::tofu::Tofu;

mod tofu {
pub struct Tofu {
    id: u16,
    name: Option<String>,
    silky: bool,
    smoky: bool,
}

impl Tofu {
    pub fn builder(id: u16) -> TofuBuilder {
        TofuBuilder::new(id)
    }
}

pub struct TofuBuilder {
    id: u16,
    name: Option<String>,
    silky: bool,
    smoky: bool,
}

impl TofuBuilder {
    pub fn new(id: u16) -> TofuBuilder {
        TofuBuilder {
            id,
            name: None,
            silky: false,
            smoky: false,
        }
    }

    pub fn build(self) -> Tofu {
        Tofu {
            id: self.id,
            name: self.name,
            silky: self.silky,
            smoky: self.smoky,
        }
    }

    pub fn name(mut self, name: String) -> TofuBuilder {
        self.name = Some(name);
        self
    }

    pub fn smoky(mut self, smoky: bool) -> TofuBuilder {
        self.silky = smoky;
        self
    }
}
}

fn main() {
    let mut smoky_tofu = Tofu::builder(0)
        .name("Smokey smoke tofu".to_owned())
        .smoky(true)
        .build();
    smoky_tofu.name = "asdf";

    let silken_tofu = Tofu::builder(1).smoky(true).build();
  
}
