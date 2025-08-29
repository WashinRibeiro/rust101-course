# Variáveis em Rust

- Em Rust, uma **variável** é um identificador associado a um valor que pode ser usado em seu programa.
- Por padrão, as variáveis em Rust são imutáveis, o que significa que uma vez que um valor é atribuído a uma variável, ele não pode ser alterado.
- A imutabilidade é uma das muitas garantias de segurança que Rust oferece.

- Para declarar uma variável, utilizamos a palavra-chave `let`.
- Se você quiser que uma variável seja mutável você pode usar `let mut`.
- Rust também permite "shadowing", onde você pode declarar uma nova variável com o mesmo nome, efetivamente criando uma nova variável.

## Como dar nome às coisas?

Em Rust usamos "snake_case" como padrão para variáveis e funções, por exemplo:

**Exemplos Corretos:**

- nome_de_uma_variavel
- total_de_itens
- preco_do_cafe

**Exemplos Incorretos:**

- NomeDeUmaVariavel (isto é "PascalCase", usado para tipos em Rust)
- outroNomeEscritoComCamelCase (isto é "camelCase", comumente usado em outras linguagens como JavaScript)
- variável_com_acento (acentos e caracteres especiais não são recomendados)
- 1_inicial (números não devem ser usados no início dos nomes de variáveis)

**Dicas:**

- Seja descritivo, mas conciso.
- Os nomes devem refletir o propósito da variável ou o valor que ela guarda.
- Evite usar "números mágicos" diretamente no código; dê-lhes um nome significativo.


----

# Bits e Bytes

- 1 bit é a unidade básica de informação na computação e pode ter um de dois valores: 0 ou 1.

A realação entre bits e byts pode ser confusa mesmo, mas pense o seguinte:
- 1 duzia = 12 ovos
- 1 hora = 60 minutos
- **1 byte = 8 bits**

Os computadores armazenam todos os dados como bits e usam bytes para representar informações maiores
Exemplo:
0 --> 00000000
1 --> 00000001
2 --> 00000010

Um byte pode ir de 000000000 até 11111111.
Ou seja, 1 byte pode representar 256 valores diferentes (2^8), desde 0 até 255 em decimal.


#### Arquitetura de armazenamento

Há duas arquiteturas de computadores principais quanto ao armazenamento de bytes:
- **Big-endian**: os bytes mais significativos (o "big end") são armazenados primeiro
- **Little-endian**: os bytes menos significativos (o "litte end") são armazenados *primeiro*

```
Exemplo:
258 em binário é: 00000001 00000010

No formato **big-endian**, os bytes são armazenados assim:
Primeiro: 00000001 (byte mais significativo)
Depois: 00000010 (byte menos significativo)
Na memória: [01] [02]

No formato little-endian, os bytes são armazenados assim:
Primeiro: 00000010 (byte menos significativo)
Depois: 00000001 (byte mais significativo)
Na memória: [02] [01]
```


#### Problemas de segurança

**Overflow** ocorre quando tentamos armazenar um número maior do que o máximo permitido pelo tipo de dados.

***Exemplo prático em Rust**: Se você somar 1 ao valor máximo de um tipo u8 (que é 255), ele volta para 0:*
```rust
let x: u8 = 255;
let y = x + 1; // Overflow! Resultado: 0
```


**Underflow** ocorre quando tentamos armazenar um número menor do que o mínimo permitido pelo tipo de dados.

***Exemplo prático em Rust**: Se você subtrair 1 do valor mínimo de um tipo u8 (que é 0), ele vai para 255:*
```rust
let a: u8 = 0;
let b = a - 1; // Underflow! Resultado: 255
```

---

# Tipos
Rust é uma linguagem de programação com tipagem estática e forte, o que significa que:

**Tipagem estática**
- O tipo de cada variável é determinado em tempo de compilação e não muda.
- Uma variável declarada com um tipo específico não pode ser reatribuída a um valor de outro tipo sem uma conversão explícita.

**Tipagem forte**
- Rust é rigoroso com as operações entre tipos.
- Não é possível transformar diretamente um número e uma string sem converter explicitamente um deles para o tipo compatível do outro.

O compilador de Rust é inteligente o suficiente para inferir o tipo de muitas variáveis, o que significa que nem sempre é necessário especificar o tipo explicitamente.

## Vamos conhecer os tipos:

#### Unsigned Integer (Inteiro sem sinal)
Exemplo: 
```rust
    let a: u8 = 5; // Ele funciona de 0 a 255 (pois tem 8 bits == 1 byte)
    let b: u16 = 500; // 16 bits sem sinal
    let c: u32 = 1_000_000; // 32 bits sem sinal
    let d: u64 = 1_000_000_000_000; // 64 bits sem sinal
    let e: u128 = 1_000_000_000_000_000_000; // 128 bits com sinal
```

#### Signed Integer (Inteiro com sinal)
Nesse caso ele pega a metade das possibilidades de valores negativos e a outra metade de valores positivos.
Exemplo: 
```rust
    let a: i8 = -5; // Ele funciona de -128 a 127 (pois tem 8 bits == 1 byte e o 0 conta)
    let b: i16 = 500; // 16 bits sem sinal
    let c: i32 = -1_000_000; // 32 bits sem sinal
    let d: i64 = 1_000_000_000_000; // 64 bits sem sinal
    let e: i128 = -1_000_000_000_000_000_000; // 128 bits com sinal
```

#### Floating Point (Ponto Flutuantes --> Número com vírgula)
Exemplo:
```rust
    let x: f32 = 3.14; // 32 bits com sinal
    let y: f64 = 3.14_159_265_358_979; // 64 bits com sinal
```

#### Booleano (Verdadeiro ou Falso)
Exemplo:
```rust
    let x: bool = true;
    let y: bool = false;
```

#### Character
OBS: Sempre com aspas simples
Exemplo: 
```rust
    let x: char = 'a';
    let y: char = '&';
```

#### Tuplas (Estrutura de dados de elementos mistos - Heterogênio)
Exemplo: 
```rust
    let x: (i32, f64, &str) = (84, 2.71, "Texto");
```

#### Slice / Array / Vetor (Fatias de valores com o mesmo tipo - Homogênio)
Exemplo: 
```rust
    let x: [i32; 5] = [1, 2, 3, 4, 5];
    let y 
```

#### Strings
OBS: Sempre com aspas simples
Exemplo:
```rust
    let x: &str = "Hello, Nearx!";
    let y: String = "oi".to_string();
```

#### Analisar tamanho de bytes dos tipos
```rust
println!("{}", std::mem::size_of::<i8>());
println!("{}", std::mem::size_of::<i16>());
println!("{}", std::mem::size_of::<(i16, i16, i8)>());
```