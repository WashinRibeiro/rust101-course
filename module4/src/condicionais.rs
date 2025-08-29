#[test]
fn broken_test_greater_than() {
    let num = 10;
    // Corrija a condição para que o teste verifique se num é maior que 5.
    assert!(num > 5);
}

#[test]
fn broken_test_less_than() {
    let age = 18;
    // Corrija a condição para que o teste verifique se age é menor que 21.
    assert!(age < 21);
}

#[test]
fn broken_test_greater_than_or_equal_to() {
    let score = 75;
    // Corrija a condição para que o teste verifique se score é maior ou igual a 60.
    assert!(score >= 60);
}

#[test]
fn broken_test_less_than_or_equal_to() {
    let temp = 30;
    // Corrija a condição para que o teste verifique se temp é menor ou igual a 32.
    assert!(temp <= 32);
}

#[test]
fn broken_test_equality() {
    let name = "Rust";
    // Corrija a condição para que o teste verifique se name é igual a "Rust".
    assert!(name == "Rust");
}

#[test]
fn broken_test_inequality() {
    let is_open = false;
    // Corrija a condição para que o teste verifique se is_open não é verdadeiro.
    assert!(is_open != true);
}

#[test]
fn broken_test_if_else_logic() {
    let number = 10;
    let result = if number >= 10 {
        "Greater than 10"
    } else {
        "Less than or equal to 10"
    };
    // Corrija a expectativa para que o teste passe com o valor correto de result.
    assert_eq!(result, "Greater than 10");
}

#[test]
fn broken_test_multiple_conditions() {
    let x = 5;
    let y = 10;
    // Corrija a condição para verificar se x é menor que y E y é maior que 8.
    assert!(x < y && y > 8);
}

#[test]
fn broken_test_else_if() {
    let time = 12;
    let part_of_day = if time < 12 {
        "morning"
    } else if time < 18 {
        "afternoon"
    } else {
        "evening"
    };
    // Corrija a expectativa para que o teste passe com o valor correto.
    assert_eq!(part_of_day, "afternoon");
}

#[test]
fn broken_test_using_wrong_operator() {
    let a = 10;
    let b = 20;
    // Corrija o teste usando o operador correto para verificar se a é diferente de b.
    assert!(a != b);
}