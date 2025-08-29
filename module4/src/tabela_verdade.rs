#[test]
fn broken_test_logical_and() {
    let a = true;
    let b = false;
    // Corrija a condição para que o teste reflita o funcionamento do operador AND.
    assert!(a || b);
}

#[test]
fn broken_test_logical_or() {
    let a = false;
    let b = false;
    // Corrija a condição para que o teste reflita o funcionamento do operador OR.
    assert!(a && b);
}

#[test]
fn broken_test_logical_not() {
    let a = true;
    // Corrija a condição para que o teste reflita o funcionamento do operador NOT.
    assert!(a);
}

#[test]
fn broken_test_combined_logical_expressions() {
    let a = true;
    let b = false;
    let c = true;
    // Corrija a expressão para que o teste reflita a combinação correta dos operadores lógicos.
    assert!(!(a && b) || c);
}

#[test]
fn broken_test_logical_and_with_equals() {
    let x = 5;
    let y = 10;
    // Corrija a condição para que o teste passe, combinando operadores lógicos com comparação.
    assert!(x < 10 && y == 10);
}

#[test]
fn broken_test_logical_or_with_not() {
    let x = 5;
    let is_even = x % 2 == 0;
    // Corrija a expressão para que o teste verifique se x não é par OU x é menor que 10.
    assert!(!is_even && x > 10);
}

#[test]
fn broken_test_negation_of_boolean() {
    let flag = true;
    // Corrija a condição para testar a negação correta de flag.
    assert!(!flag == false);
}

#[test]
fn broken_test_multiple_negations() {
    let truthy = false;
    // Corrija a expressão para que o teste passe, considerando múltiplas negações.
    assert!(!(!(!truthy)));
}

#[test]
fn broken_test_logical_and_priorities() {
    let a = true;
    let b = false;
    let c = true;
    // Corrija a expressão para refletir corretamente as prioridades dos operadores lógicos.
    assert!(a && b || c);
}

#[test]
fn broken_test_logical_expression_complex() {
    let x = 20;
    let y = 15;
    // Corrija a expressão para que o teste verifique se x é maior que 10 E y é menor que 20 OU y é igual a 15.
    assert!(x > 10 && (y < 20 || y != 15));
}