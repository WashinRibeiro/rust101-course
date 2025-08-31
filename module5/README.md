# Gerenciamento de memória em Rust

O gerenciamento de memória em Rust é um dos aspectos mais inovadores da linguagem.
Rust utiliza um sistema de **propriedade** com regras de **empréstimo** que garantem a segurança de memória em tempo de compilação.

Cada valor tem um "proprietário" (ownership), que determina o quando ele será apagado.
Quando a proprietário sai de escopo, o valor é descartado.

Rust também implementa o conceito de "borrowing" (empréstimo) e "slices" para referenciar partes de dados sem tomar a propriedade, permitindo acesso seguro e eficiente.

## Regras de gerenciamento de memória em Rust:

### Propriedade (Ownership):

- Cada valor em Rust tem uma variável chamada sua "proprietária" (owner).
- Há exatamente uma proprietária por valor em qualquer momento.
- Quando a proprietária sai do escopo, o valor é descartado.

### Transferência de Propriedade (Move):

- Quando um valor é atribuído a outra variável, a propriedade é transferida para a nova variável, e a variável original não pode mais ser usada. Esse comportamento é conhecido como "move".
- Este conceito previne acessos simultâneos à mesma área de memória, o que pode levar a condições de corrida ou erros.

## Empréstimo (Borrowing):

- Rust permite referências a valores sem tomar sua propriedade, conhecido como "borrowing".
- As referências podem ser imutáveis (&T) ou mutáveis (&mut T), mas com restrições:
  - Só pode existir um uma referência mutável por vez.
  - Referências imutáveis não podem coexistir com referências mutáveis.
  - Em qualquer momento, você pode ter OU uma referência mutável ou quantas referências imutáveis quiser
  - As referências precisam ser validas SEMPRE.

### Tempo de Vida (Lifetimes):

- Rust precisa garantir que todas as referências sejam válidas pelo tempo que forem usadas.
- "Lifetimes" são anotações usadas em Rust para garantir que referências não sobrevivam à validade dos dados aos quais elas apontam.
- "Lifetimes" são inferidas pelo compilador em muitos casos, mas em situações complexas, elas precisam ser especificadas manualmente para assegurar que não haja dangling references.

### Slice Type:

- "Slices" permitem referenciar uma sequência contígua de elementos em uma coleção em vez de toda a coleção.
- "Slices" são um tipo de empréstimo que não tem propriedade sobre os dados aos quais se referem, garantindo acesso seguro a uma parte de um array, por exemplo, sem risco de acessar dados fora dos limites.

# Funções

As funções em Rust são blocos de código que realizam tarefas específicas e podem ser reutilizadas em diferentes partes do programa. Elas são fundamentais para organizar e modularizar o código, tornando-o mais legível e mantível.
Em Rust, as funções são definidas com a palavra-chave `fn`, seguida por um nome, parênteses que podem conter argumentos, e, opcionalmente, um tipo de retorno após a seta `->`:

```rust
fn minha_funcao(meu_parametro: i32) -> i32 {
  let meu_retorno = meu_parametro + 1;

  meu_retorno
}
```

Os escopos dentro das funções permitem criar variáveis que são acessíveis apenas dentro delas, garantindo que o espaço de nomes global não seja poluído.

## Características importantes:

- Parâmetros e Retornos: As funções podem receber dados (parâmetros) e retornar um resultado. Os tipos de dados dos parâmetros e do valor de retorno devem ser especificados.
- Escopo: Variáveis declaradas dentro de uma função estão no escopo local da função e não são acessíveis fora dela.
- Expressões e Declarações: Rust é uma linguagem baseada em expressões, o que significa que quase tudo retorna um valor. A última expressão de uma função pode ser seu valor de retorno, sem necessidade de usar a palavra-chave `return`.

# Funções Nativas

Em Rust, além das funções definidas pelo usuário, existem várias funções nativas e macros que são extremamente úteis para diversas tarefas como:

- debugging
- formatação de strings
- manipulação de vetores
- controle de fluxo do programa.

Macros são um recurso poderoso em Rust que permitem escrever código que gera outro código. Focaremos em algumas funções nativas e macros úteis, sem entrar em detalhes profundos sobre como as macros funcionam.

## Funções Nativas e Macros:

- assert!: Usado para testes e verifica se uma condição é verdadeira. Se não for, o programa panica.
- format!: Similar a println!, mas retorna a string formatada em vez de imprimi-la na saída padrão.
- vec!: Macro utilizada para criar um novo vetor contendo os valores especificados.
- panic!: Causa o término imediato do programa exibindo uma mensagem de erro fornecida. Útil para testar condições de erro.
- println! e print!: Macros para imprimir mensagens na saída padrão, com println! adicionando uma nova linha no final.
- dbg!: Macro que imprime o nome da variável, seu valor e a localização no código onde dbg! foi chamado, útil para debugging.
- todo!: Macro que marca partes do código que ainda precisam ser implementadas. O programa panica se essa parte do código for alcançada.
