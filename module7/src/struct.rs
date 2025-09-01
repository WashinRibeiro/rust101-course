struct People {
    name: String,
    age: u8,
}

impl People {
    fn new(name: String, age: u8) -> Self {
        People { name, age }
    }

    fn is_adult(&self) -> bool {
        self.age >= 18
    } 
}

fn main() {
    let people1: People = People::new("Washington".to_string(), 23);
    let people1: People = People::new("Washington".to_string(), 23);

    println!("Você é adulto? Resposta: {}", people1.is_adult());
    println!("Meu nome é {}, e eu tenho {} anos", people1.name, people1.age);
}
