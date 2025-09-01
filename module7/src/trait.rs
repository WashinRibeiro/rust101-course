use std::fmt::{Display, Formatter, Result};

trait Comer {
    fn comendo(&self) -> bool;
}

struct Car {
    name: String,
    year: u16
}

struct People {
    name: String,
    age: u8
}

impl Comer for People {
    fn comendo(&self) -> bool {
        true
    }   
}

impl Display for People {
    fn fmt(&self, f: &mut Formatter) -> Result<> {
        write!(f, "Nome: {}, Idade: {}", self.name, self.age)
    }
}

fn main() {

    let p: People = People {
        name: "Joaquim".to_string(),
        age: 6
    };

    println!("{}", p.comendo());
    println!("{}", p);
}