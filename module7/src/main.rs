use std::{fmt::{self, Display, Formatter}, u8};

enum Result {
    Ok(u8),
    Err(String),
}

impl Display for Result {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Result::Ok(number) => write!(f, "Funcionou: {}", number),
            Result::Err(msg) => write!(f, "Erro: {}", msg),
        }
    }
}

fn half_item(item: u8) -> Result {
    if item % 2 == 0 {
        Result::Ok(item / 2)
    } else {
        Result::Err(format!("O item {} não pode ser dividido ao meio", item))
    }
}

fn main() {
    let result: Result = half_item(9);
    let result2: Result = half_item(12);
    
    println!("{}", result);
    println!("{}", result2);
}
