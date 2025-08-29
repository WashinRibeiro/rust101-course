//! # Variáveis em Rust
//!
//! Em Rust, uma **variável** é um identificador associado a um valor que pode ser usado em seu programa. 
//! Por padrão, as variáveis em Rust são imutáveis, o que significa que uma vez que um valor é atribuído a uma variável, 
//! ele não pode ser alterado. A imutabilidade é uma das muitas garantias de segurança que Rust oferece.
//!
//! Para declarar uma variável, utilizamos a palavra-chave `let`. Se você quiser que uma variável seja mutável, 
//! você pode usar `let mut`. Rust também permite "shadowing", onde você pode declarar uma nova variável com o mesmo nome, 
//! efetivamente criando uma nova variável.

#[test] 
fn test_como_declarar_variaveis() {
    // Variáveis são declaradas com `let` e seus valores são imutáveis por padrão.
    let nome_da_variavel = "Valor da variável";
    assert_eq!(nome_da_variavel, "Valor da variável");
}

#[test]
fn test_definindo_uma_variavel() {
    // Este teste verifica se podemos atribuir um valor inteiro a uma variável.
    let x = 10;
    assert_eq!(x, 10);
}

#[test]
fn test_mudando_o_valor_de_uma_variavel() {
    // Variáveis mutáveis podem ter seus valores alterados.
    let mut y = 5;
    y = y + 5;
    assert_eq!(y, 10);
}

#[test]
fn test_variavel_shadowing() {
    // Rust permite shadowing de variáveis, o que significa redeclarar uma variável com o mesmo nome.
    let x = 5;
    let x = x + 5; // Shadowing: 'x' é agora 10, não 5.
    assert_eq!(x, 10);
}

#[test]
fn test_aprendendo_sobre_tests() {
    // Testes ajudam a verificar se seu código está funcionando como esperado.
    // Este teste intencionalmente falhará, mostrando como o teste pode ser usado para pegar erros.
    let minha_var = 12;
    // Corrija o valor esperado para que o teste passe.
    assert_eq!(minha_var, 11); // O valor esperado deve ser igual ao valor da variável.
}