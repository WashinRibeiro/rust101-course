#[test]
fn broken_test_function_with_parameters() {
    let result = add_two(2);
    // Corrija a função `add_two` para que o teste passe, somando 2 ao argumento.
    assert_eq!(result, 4);

    // Função para correção:
    fn add_two(x: i32) -> i32 {
        // Correção necessária aqui.
        x + 2
    }
}

#[test]
fn broken_test_function_return() {
    let empty_string = create_empty_string();
    // Corrija a função `create_empty_string` para que o teste passe, retornando uma String vazia.
    assert_eq!(empty_string, "");

    // Função para correção:
    fn create_empty_string() -> String {
        "".to_string()
    }
}

#[test]
fn broken_test_scope_within_functions() {
    let outer_var = 100;
    let result = multiply_by_two(outer_var);
    // Corrija a função `multiply_by_two` e/ou a chamada para refletir o escopo correto.
    assert_eq!(result, 200);

    // Função para correção:
    fn multiply_by_two(x: i32) -> i32 {
       let inner_var = x * 2;
       inner_var
       // Correção necessária aqui.
    }
}

#[test]
fn broken_test_function_without_return_type() {
    let greeting = say_hello();
    // Corrija a função `say_hello` para que o teste passe, retornando uma saudação.
    assert_eq!(greeting, "Hello!");

    // Função para correção:
    fn say_hello() -> String {
        "Hello!".to_string()
       // Correção necessária aqui.
    }
}

#[test]
fn broken_test_passing_reference_to_function() {
    let mut value = 10;
    increase_by_one(&mut value);
    // Corrija a função `increase_by_one` para que o teste passe, incrementando o valor por referência.
    assert_eq!(value, 11);

    // Função para correção:
    fn increase_by_one(x: &mut i32) {
        *x += 1;
       // Correção necessária aqui.
    }
}