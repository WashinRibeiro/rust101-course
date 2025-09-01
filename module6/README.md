# Lists e Maps

## Vetores
Uma lista, chamada no Rust como Slice tem essa estrutura:
```rust
let x = [1, 2, 3];
```

Já um vetor, segue a seguinte estrutura: 
```rust
let y: Vec<u8> = vec![1, 2, 3];
```

A grosso modo, um vetor tem mais funcionalidades que uma lista e uma forma diferente de alocar a memória.
O dado de um vetor tem que ser homogênio, ou seja:
```rust
let x: Vec<u8> = vec![1, 2, 3]; // Correto
let y: Vec<u8> = vec![1, 2, "Olá"]; // Errado
let z: Vec<u8> = vec![-200, 5, 1000]; // Errado
```

### Conhecendo alguns métodos
#### iter() / iter_mut() / for_each()
    
O método `iter()` retorna um iterador de referências imutáveis (&T) para os elementos do vetor. Isso significa que você pode ler os valores, mas não pode modificá-los diretamente, permitindo percorrer seus elementos de forma segura e eficiente. 

O método `iter_mut()` é usado em Rust para iterar sobre um vetor permitindo modificar seus elementos durante a iteração. Esse método retorna referências mutáveis (&mut T).

O método `for_each()` é usado para iterar sobre cada elemento de um iterador, aplicando uma função a cada item. Ele não retorna uma nova coleção, apenas executa a função fornecida para cada elemento.

**OBS.:** Como entramos no escopo da função, se não enviarmos como uma referência, nosso vetor será excluído, e vai sumir da memória.

```rust
// Sem modificação no vetor (Leitura dos valores)
fn main() {
    let x: Vec<i8> = vec![1, 2, 90];
    x.iter().for_each(sum_one);
    println!("{:?}", x);
}

fn sum_one(x: &i8) {
    let y = x + 1;
    println!("{y}");
}
```

```rust
// Com modificação no vetor
fn main() {
    let x: Vec<i8> = vec![1, 2, 90];
    x.iter_mut().for_each(sum_one);
    println!("{:?}", x);
}

fn sum_one(x: &mut i8) {
    *x += 1;
}
```

#### map() / collect()
O método `map` em Rust é usado para transformar cada elemento de um iterador usando uma função. No seu exemplo:

```rust
let y = x.iter().map(sum_one);
```
`x.iter()` cria um iterador sobre os elementos do vetor x.
`map(sum_one)` aplica a função **sum_one** a cada elemento do iterador.

**O que map retorna?**
O `map` retorna um novo iterador, não um vetor ou lista. Esse iterador irá produzir os valores transformados quando for consumido (por exemplo, usando `collect()` para criar um vetor).

```rust
fn main() {
    let x: Vec<i8> = vec![1, 2, 90];
    let y: Vec<i8> = x.iter().map(sum_one).collect();
    println!("{:?}", y); // Saída: [2, 3, 91]```
}

fn sum_one(x: &i8) -> i8 {
    x + 1
}
```

**Resumo:**
- `map` transforma cada elemento.
- Retorna um iterador, não uma coleção pronta.
- Para obter um vetor, use `collect()`.


#### into_inter() / filter()

`into_iter()`
- **O que faz:** Transforma a coleção (Vec<i8>) em um iterador que consome os elementos. Ou seja, depois de usar into_iter(), você não pode mais usar o vetor original, pois ele foi "movido".
- **Por quê usar:** Permite processar cada elemento da coleção de forma eficiente, usando métodos de iteradores como filter(), map(), etc.

`filter()`
**O que faz:** Recebe uma função (ou closure) que retorna **true** ou **false** para cada elemento do iterador. Só os elementos para os quais a função retorna true são mantidos.

Exemplo:
```rust
fn main() {
    let x: Vec<i8> = vec![1, 2, 90, 44, 85, 63, 24];
    // into_iter() transforma x em um iterador
    // filter(get_par) mantém apenas os pares
    let y: Vec<i8> = x.into_iter().filter(get_par).collect();
    println!("{:?}", y); // [2, 90, 44, 24]
}

fn get_par(x: &i8) -> bool {
    x % 2 == 0
}
```

**OBS.:** Como utilizamos o `into_iter()`, o x não pode ser mais consumido após o filter(). Para resolver isso podemos utilizar um `x.clone()`.
Exemplo:
```rust
fn main() {
    let x: Vec<i8> = vec![1, 2, 90, 44, 85, 63, 24];
    let y: Vec<i8> = x.clone().into_iter().filter(get_par).collect();
    println!("Lista antes: {:?}", x); // Lista antes: [1, 2, 90, 44, 85, 63, 24]
    println!("Lista depois: {:?}", y); // Lista depois: [2, 90, 44, 24]
}

fn get_par(x: &i8) -> bool {
    x % 2 == 0
}
```

#### all() / any()
O código abaixo demonstra o uso dos métodos `all()` e `any()` em iteradores do Rust, que são úteis para verificar condições em coleções. No exemplo, temos um vetor de inteiros x e uma função chamada get_par, que retorna true se o número for par.

O método `all()` verifica se todos os elementos do vetor satisfazem a condição definida pela função get_par. Ou seja, ele retorna true apenas se **TODOS** os números forem pares. Como o vetor contém números ímpares, o resultado é false.

Já o método `any()` retorna true se pelo menos um elemento do vetor satisfaz a condição. Neste caso, como o número 90 é par, o resultado é true.

Esses métodos são muito úteis para realizar verificações rápidas em coleções, tornando o código mais legível e conciso. Uma possível armadilha para iniciantes é esquecer que all() exige que todos os elementos atendam à condição, enquanto any() basta que apenas um atenda.

```rust
fn main() {
    let x: Vec<i8> = vec![1, 7, 90, 45];
    
    let y: bool = x.iter().all(get_par);
    let z: bool = x.iter().any(get_par);

    println!("{}", y); // false
    println!("{}", z); // true
}

fn get_par(x: &i8) -> bool {
    x % 2 == 0
}
```

---

## HashMap
Um HashMap em Rust é uma estrutura de dados que armazena pares de **chave** e **valor**, permitindo acesso rápido aos valores por meio de suas chaves. Ele funciona de forma semelhante ao tipo **Record do TypeScript** ou a uma **tabela do Excel**, onde cada linha possui uma chave única associada a um valor. Essa estrutura é muito útil quando você precisa buscar, inserir ou remover dados de forma eficiente, usando uma chave para identificar cada valor. No contexto de Rust, o HashMap é amplamente utilizado para organizar informações de maneira flexível e dinâmica, facilitando operações como contagem de elementos, agrupamento de dados ou armazenamento de configurações.

Como o Hashmap é uma estrutura mais robusta, precisamos importar dos pacotes do Rust:
```rust
use std::collections::HashMap;
```

O método `insert()` é utilizado para adicionar pares de chave e valor em um HashMap. No exemplo, cada chamada de `insert()` insere um nome (convertido para String) como chave e uma idade (i8) como valor. O HashMap precisa ser mutável (mut) porque estamos modificando seu conteúdo.

O macro `dbg!` serve para imprimir o valor de uma expressão no console de forma detalhada, facilitando a depuração. Ao usar `dbg!(a)`, o conteúdo do HashMap é exibido no terminal, mostrando todas as chaves e valores inseridos. Isso é útil para verificar rapidamente o estado da estrutura durante o desenvolvimento.

```rust
fn main() {
    let mut a: HashMap<String, i8> = HashMap::new();
    
    a.insert("lucas".to_string(), 26);
    a.insert("andre".to_string(), 30);
    a.insert("joão".to_string(), 30);
    
    dbg!(a);
    
    // Terminal (Resultado)
    // a = {
    //   "andre": 30,        
    //   "joão": 30,
    //   "lucas": 26,        
    // }    
}
```

Podemos consultar um **valor** específico, buscando pela **chave** a partir do método `get()`. Como o exemplo abaixo:
```rust
println!("{:?}", a.get("joão")); // Some(30)
```

Além disso podemos remover uma linha dessa estrutura utilizando o `remove()`

```rust
a.remove("joão");
dbg!(a); 

// Terminal (Resultado)
// a = {
//   "andre": 30,        
//   "lucas": 26,        
// } 
```

Para atualizar um valor, também podemos utilizar o insert() com a chave do valor que queremos modificar:

```rust
a.insert("andre".to_string(), 55);
dbg!(a); 

// Terminal (Resultado)
// a = {
//   "andre": 55,        
//   "lucas": 26,        
// }
```


## Strings
- **`String` é diferente de `&str`:** String é um tipo que representa uma string mutável e alocada dinamicamente. Já `&str` é uma fatia de string, geralmente imutável e com tamanho fixo.

- **Implementação interna:** Internamente, `String` é um wrapper sobre um vetor de bytes (Vec<u8>). Isso permite que ela armazene qualquer sequência de bytes válida em UTF-8.

- **UTF-8:** Rust usa UTF-8 para representar strings, então cada caractere pode ocupar mais de um byte. Por isso, acessar diretamente um índice como name[2] não funciona, pois pode cortar um caractere no meio.

- **Conversão entre tipos:** Você pode converter um `&str` para String usando `.to_string()` ou `String::from()`.

- **Manipulação:**
Para modificar uma String, métodos como `push_str()` permitem adicionar texto ao final.

Exemplo prático: 
```rust
fn main() {
    let mut nome = String::from("washin");
    nome.push_str(" ribeiro");
    println!("{}", nome); // Saída: washin ribeiro

    // Não é possível acessar um caractere por índice diretamente:
    // println!("{}", nome[2]); // Erro!
    // Para acessar caracteres, use métodos como .chars():
    println!("{}", nome.chars().nth(2).unwrap()); // Saída: s
}
```

**OBS.:** Nunca use índices diretamente em String, sempre utilize métodos como .chars() para trabalhar com caracteres.