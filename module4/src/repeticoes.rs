#[test]
fn broken_test_while_loop() {
    let mut number = 0;
    // Corrija o loop para que incremente number até que seja menor que 5.
    while number > 5 {
        number += 1;
    }
    assert_eq!(number, 5);
}

#[test]
fn broken_test_infinite_loop_with_break() {
    let mut counter = 0;
    loop {
        counter += 1;
        // Corrija o ponto de interrupção para sair do loop quando counter for igual a 3.
        if counter > 3 {
            break;
        }
    }
    assert_eq!(counter, 3);
}

#[test]
fn broken_test_for_loop_over_range() {
    let mut sum = 0;
    // Corrija o loop for para somar valores de 1 a 5, inclusive.
    for i in 1..=4 {
        sum += i;
    }
    assert_eq!(sum, 15); // 1+2+3+4+5 = 15
}

#[test]
fn broken_test_for_loop_over_array() {
    let numbers = [1, 2, 3, 4, 5];
    let mut sum = 0;
    // Corrija o loop para somar os valores do array numbers.
    for &num in numbers.iter() {
        sum += num;
    }
    assert_eq!(sum, 10); // Soma incorreta.
}

#[test]
fn broken_test_while_true_with_condition() {
    let mut num = 10;
    // Corrija o loop para decrementar num até 5 usando um loop infinito com condição de saída.
    loop {
        if num <= 5 {
            break;
        }
        num -= 1;
    }
    assert_eq!(num, 5);
}

#[test]
fn broken_test_loop_control_variables() {
    let mut count = 0;
    for _ in 0..5 {
        count += 1;
    }
    // Corrija a expectativa para que o teste reflita o correto controle de iteração.
    assert_eq!(count, 6); // Contagem de iterações incorreta.
}

#[test]
fn broken_test_nested_loops() {
    let mut product = 1;
    for i in 1..3 {
        // Loop externo.
        for j in 1..4 {
            // Loop interno.
            product *= j;
        }
    }
    // Corrija a expectativa para o produto correto das iterações dos loops aninhados.
    assert_eq!(product, 36); // Resultado do produto incorreto.
}

#[test]
fn broken_test_loop_with_continue() {
    let mut sum = 0;
    for i in 1..=10 {
        if i % 2 == 0 {
            continue;
        } // Deve somar apenas números ímpares.
        sum += i;
    }
    // Corrija a expectativa para que o teste reflita a soma correta de números ímpares.
    assert_eq!(sum, 25); // Soma de ímpares incorreta.
}

#[test]
fn broken_test_while_decrement() {
    let mut number = 5;
    // Corrija o loop para decrementar number até 0.
    while number < 0 {
        number -= 1;
    }
    assert_eq!(number, 0);
}

#[test]
fn broken_test_for_loop_with_index() {
    let letters = ['a', 'b', 'c', 'd', 'e'];
    let mut count = 0;
    // Corrija o loop for para contar o número de letras no array.
    for letter in letters.iter() {
        count += 1;
    }
    assert_eq!(count, 4); // Contagem de letras incorreta.
}