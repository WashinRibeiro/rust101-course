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

## Traits
Traits em Rust são similares a "interfaces" em outras linguagens. Eles definem um conjunto de métodos que tipos podem implementar. Ou seja, um trait descreve um comportamento que pode ser compartilhado entre diferentes structs ou enums. Se um tipo implementa um trait, ele precisa fornecer uma implementação para os métodos definidos nesse trait.

Traits ajudam a garantir que diferentes tipos possam ser usados de forma intercambiável, desde que implementem o mesmo conjunto de comportamentos.

... 

Aqui criamos o trait `Comer`, que exige que qualquer tipo que o implemente forneça o método comendo, que recebe uma referência à instância (`&self`) e retorna um `bool`.

**Definição do Trait**
```rust
trait Comer {
    fn comendo(&self) -> bool;
}
```

Criamos duas structs: Car e People. Cada uma representa uma entidade diferente.


```rust
struct Car {
    name: String,
    year: u16
}

struct People {
    name: String,
    age: u8
}
```

**Implementação do Trait**

Aqui, estamos dizendo que `People` implementa o trait `Comer`. Ou seja, pessoas podem "comer", então você fornece uma implementação para o método `comendo`, que sempre retorna `true`.

Note que você não implementou `Comer` para `Car`, pois faz sentido: carros não comem.

```rust
impl Comer for People {
    fn comendo(&self) -> bool {
        true
    }   
}
```

Na main, criamos uma instância de `People` e chamamos o método `comendo()`, que retorna `true` e imprime isso.

```rust
fn main() {
    let p: People = People {
        name: "Joaquim".to_string(),
        age: 6
    };

    println!("{}", p.comendo())
}
```

Se precisarmos dar um `println!()` no `p` que foi criado, vamos ter o erro:
```bash
error[E0277]: `People` doesn't implement `std::fmt::Display`
  --> src\main.rs:37:20
   |
37 |     println!("{}", p);
   |               --   ^ `People` cannot be formatted with the default formatter
   |               |
   |               required by this formatting parameter
   |
   = help: the trait `std::fmt::Display` is not implemented for `People`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more 
info)
```

Esse erro acontece porque o println! com {} exige que o tipo implemente o trait Display.

Por padrão, o Rust não sabe como transformar a sua struct (People) em uma String legível. Ele só sabe imprimir tipos básicos (como i32, String, etc.) que já implementam Display.

O trait Display define justamente como um tipo deve ser convertido para texto “bonito” e legível para humanos. Quando você implementa esse trait, está dizendo explicitamente ao compilador como formatar a struct quando alguém usar {} em macros como println!.

**Solução:**
```rust
// Importação
use std::fmt::{Display, Formatter, Result};

// Implementação
impl Display for People {
    fn fmt(&self, f: &mut Formatter) -> Result<> {
        write!(f, "Nome: {}, Idade: {}", self.name, self.age)
    }
}
```

**Resumindo:**
- O erro ocorre porque o compilador não sabe imprimir sua struct.
- O trait Display fornece o contrato para ensinar o compilador a fazer isso.

---

## Enums
Enums, ou "enumerations", são tipos especiais em Rust que permitem definir um conjunto de valores possíveis para uma variável. Eles são úteis para representar estados, opções ou categorias distintas dentro do seu programa. Com enums, você pode criar tipos que só aceitam valores previamente definidos, tornando o código mais seguro e fácil de entender.

Por exemplo, um enum pode ser usado para representar o estado de uma conexão (Conectado, Desconectado, Em Espera) ou o resultado de uma operação (Sucesso, Erro). Além disso, enums em Rust podem armazenar dados associados a cada variante, tornando-os ainda mais poderosos e flexíveis para modelar situações do mundo real.

```rust
#[derive(Debug)]

enum Order {
    OrderDone,
    PaymentDone,
    OrderSent,
    OrderDelivery
}

fn main() {
    dbg!(Order::OrderDelivery);
}
```

No exemplo acima, eu defini um `enum` chamado `Order`, que representa diferentes estados de um pedido: **OrderDone**, **PaymentDone**, **OrderSent** e **OrderDelivery**. Enums são úteis para modelar situações onde uma variável pode assumir apenas um valor dentre várias opções pré-definidas, tornando o código mais seguro e legível.

Além disso, usei a anotação `#[derive(Debug)]` para permitir que o `enum` seja impresso facilmente no console usando macros como `dbg!`, o que facilita a depuração.

No próximo código, criei a função `get_number`, que retorna um `Option<u8>`. O tipo `Option` é um enum padrão do Rust que representa a possibilidade de ausência de valor: retorna `Some(x)` se o número for menor que 10, ou `None` caso contrário. Isso obriga o tratamento explícito de casos onde pode não haver valor, evitando erros comuns de acesso a dados inexistentes.

---

## Patters Match
O "Pattern Match" (ou simplesmente "match") em Rust é uma ferramenta poderosa para comparar e destrinchar valores, especialmente enums e tipos opcionais. Ele permite que você escreva código que analisa diferentes possibilidades de um valor, executando ações específicas para cada caso.

O match funciona como um "switch" mais inteligente, mas com muito mais poder: ele pode extrair dados, combinar múltiplos padrões e garantir que todos os casos sejam tratados, tornando o código mais seguro e legível.

Por exemplo, ao trabalhar com um enum ou um Option, você pode usar o match para decidir o que fazer dependendo do valor:

```rust
let resultado = Some(10);

match resultado {
    Some(valor) => println!("O valor é {}", valor),
    None => println!("Não há valor"),
}
```

No exemplo acima, o match verifica se o resultado é `Some` ou `None` e executa o código apropriado para cada caso.

**O pattern match é essencial em Rust para:**
- Tratar diferentes variantes de enums
- Lidar com valores opcionais (`Option`)
- Garantir que todos os casos possíveis sejam cobertos
- Escrever código mais seguro, claro e idiomático

Em resumo, o match é uma das principais formas de controle de fluxo em Rust, permitindo que você escreva lógica robusta e sem surpresas!

...

O código abaixo, mostra como usar `enums` e `pattern match` para tratar diferentes resultados de uma operação em Rust.

```rust
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
    let result1: Result = half_item(9);
    let result2: Result = half_item(12);
    
    println!("{}", result1);
    println!("{}", result2);
}
```

Primeiro, eu defini um enum chamado `Result`, que pode ser `Ok(u8)` para indicar sucesso com um valor, ou `Err(String)` para indicar erro com uma mensagem. Depois, implementei o trait `Display` para o enum, usando o `match` para decidir como formatar cada variante: se for `Ok`, imprime o valor; se for `Err`, imprime a mensagem de erro.

A função `half_item` recebe um número e tenta dividir por 2. Se o número for par, retorna `Result::Ok` com o resultado. Se for ímpar, retorna `Result::Err` com uma mensagem explicando o erro.

No main, no result1 chamo `half_item(9)` e imprimo o resultado. Como 9 é ímpar, o programa mostra:
Erro: O item 9 não pode ser dividido ao meio;

Já no result2, como 12 é par, imprimo:
Funcionou: 6