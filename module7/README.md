# Estruturas Avançadas

## Structs
Em Rust, uma struct (estrutura) é uma forma de criar tipos personalizados que agrupam diferentes dados sob um mesmo nome. Ela permite organizar informações relacionadas de maneira clara e eficiente, facilitando o gerenciamento e a manipulação desses dados no programa.

Exemplo:
```rust
struct People {
    name: String,
    age: u8,
}
```

A struct **People** define um tipo que representa uma pessoa, contendo dois campos: name, que é uma string para armazenar o `nome`, e `age`, que é um número inteiro sem sinal (`u8`) para armazenar a idade. Assim, ao invés de trabalhar com variáveis separadas para nome e idade, você pode criar uma instância de `People` e acessar esses dados de forma agrupada e organizada.

Além de definir os campos, podemos implementar métodos para a struct usando o bloco impl. No exemplo abaixo, criamos dois métodos:

- `new`: um método associado que serve como "construtor", facilitando a criação de novas instâncias da struct. O uso de `Self` indica que o retorno será do próprio tipo da struct (`People`).
- `is_adult`: um método que recebe uma referência à instância (`&self`) e retorna se a pessoa é adulta (idade maior ou igual a 18).

```rust
impl People {
    fn new(name: String, age: u8) -> Self {
        People { name, age }
    }

    fn is_adult(&self) -> bool {
        self.age >= 18
    }
}
```

No `main`, mostramos como criar uma instância usando o método `new` e como acessar os métodos e campos da struct:

```rust
fn main() {
    let people1: People = People::new("Washington".to_string(), 23);

    println!("Você é adulto? Resposta: {}", people1.is_adult());
    println!("Meu nome é {}, e eu tenho {} anos", people1.name, people1.age);
}
```

**Observações importantes:**
- O bloco `impl` permite adicionar funcionalidades à struct, tornando o código mais organizado e reutilizável.
- O uso de `Self` dentro do `impl` é equivalente ao nome da própria struct.
- O parâmetro `&self` nos métodos indica que estamos acessando os dados da instância atual, sem modificar seu estado.

Structs são fundamentais em Rust para modelar entidades do mundo real, promover organização e garantir segurança no tratamento dos dados.

---

#